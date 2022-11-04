// Cross program invocation

use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
    rc::Rc,
};

use log::{debug, warn};
use rustc_middle::{
    mir::{
        BinOp, Body, Operand, PlaceRef, ProjectionElem, Rvalue, SourceInfo, Terminator,
        TerminatorKind, VarDebugInfoContents,
    },
    ty::{Ty, TyCtxt, TyKind},
};

use crate::{
    reporter::{Report, VulnerabilityType},
    source_info,
    wpa::{FullPlaceRef, StateMachine, StateTransitor},
};

const SPL_ID: &'static str = "spl_token::id";
const SPL_INSTRUCTION: &'static str = "spl_token::instruction";
const PUBKEY: &'static str = "::Pubkey";
const CMP_NE: &'static str = "std::cmp::PartialEq::ne";
const CMP_EQ: &'static str = "std::cmp::PartialEq::eq";

#[derive(Clone, PartialEq, Eq)]
pub struct ProgramId {
    pub is_key: bool,
    pub external: bool,
    pub checked: Rc<Cell<bool>>,
}

impl Default for ProgramId {
    fn default() -> Self {
        Self {
            is_key: false,
            external: false,
            checked: Rc::new(Cell::new(false)),
        }
    }
}

impl core::fmt::Debug for ProgramId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProgramId")
            .field("is_key", &self.is_key)
            .field("checked", &self.checked)
            .field("ptr", &self.checked.as_ptr())
            .finish()
    }
}

fn is_pubkey<'tcx>(place: PlaceRef<'tcx>, body: &Body<'tcx>, tcx: TyCtxt<'tcx>) -> bool {
    let ty = place.ty(body, tcx).ty.peel_refs();
    if let TyKind::Adt(adt_def, _substs) = ty.kind() {
        let ty_str = tcx.def_path_str(adt_def.did);
        if ty_str.find(PUBKEY) != None {
            return true;
        }
    }
    false
}

impl<'tcx> StateTransitor<'tcx> for ProgramId {
    fn compute_rvalue(
        rvalue: &Rvalue<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, ProgramId>,
        src_info: SourceInfo,
    ) -> Self {
        //debug!("CPIChecker--compute_rvalue: {:?}", rvalue);
        match rvalue {
            Rvalue::Use(op) => {
                // debug!("updating use: {:?}", op);
                return Self::compute_operand(op, body, tcx, sm, src_info);
            }
            Rvalue::Ref(_, _, place) => {
                // debug!("updating ref: {:?}", place);
                return Self::compute_place(place.as_ref(), body, tcx, sm, Some(src_info));
            }
            Rvalue::Cast(_, op, _ty) => {
                return Self::compute_operand(op, body, tcx, sm, src_info);
            }
            Rvalue::BinaryOp(opnd, box (lhs, rhs)) => match opnd {
                BinOp::Ne | BinOp::Eq => {
                    let mut update_op_st_fn = |op| {
                        let st = Self::compute_operand(op, body, tcx, sm, src_info);
                        if st.is_key {
                            // let mut checked = st.checked.borrow_mut();
                            // if let Operand::Move(op_place) | Operand::Copy(op_place) = op {
                            //     let op_fp =
                            //         FullPlaceRef::new(body.source.def_id(), op_place.as_ref());
                            //     checked.insert(op_fp);
                            // }
                            st.checked.set(true);
                        }
                        // debug!("state for op: {:?} is {:?}", op, st);
                    };
                    update_op_st_fn(lhs);
                    update_op_st_fn(rhs);
                    // debug!("binop: {:?}, {:?}", lhs, rhs);
                }
                _ => {}
            },
            _ => {}
        }
        Default::default()
    }

    fn compute_operand(
        op: &Operand<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, ProgramId>,
        src_info: SourceInfo,
    ) -> Self {
        //debug!("CPIChecker--compute_operand: {:?}", op);
        match op {
            Operand::Copy(from) | Operand::Move(from) => {
                Self::compute_place(from.as_ref(), body, tcx, sm, Some(src_info))
            }
            Operand::Constant(_from) => {
                // debug!("CPIChecker--compute_operand: {:?}", op);
                Default::default()
            }
        }
    }

    fn compute_place(
        place: PlaceRef<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, ProgramId>,
        src_info: Option<SourceInfo>,
    ) -> Self {
        // debug!("CPIChecker--compute_place: {:?}", place);
        if let Some(st) = sm.get_state(FullPlaceRef::new(body.source.def_id(), place)) {
            // debug!("place: {:?} has state {:?}", place, st);
            return st.clone();
        }

        let mut cur = place;
        let mut full_place = FullPlaceRef::new(body.source.def_id(), cur);
        while sm.get_state(full_place).is_none() {
            if let Some((base, _projs)) = cur.last_projection() {
                full_place = FullPlaceRef::new(body.source.def_id(), base);
                cur = base;
            } else {
                break;
            }
        }
        if let Some(st) = sm.get_state(full_place) {
            // let local_index = place.local.index();

            if is_pubkey(place, body, tcx) {
                let mut external = st.external.clone();
                let mut checked = st.checked.clone();
                sm.update(place, st.clone(), body, tcx);
                let st = Self {
                    is_key: true,
                    external: external,
                    checked: checked,
                };
                return st;
            }
        }
        Default::default()
    }

    fn compute_terminator(
        terminator: &Terminator<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, ProgramId>,
        src_info: SourceInfo,
    ) -> Self {
        //debug!("CPIChecker--compute_terminator: {:?}", terminator);
        if let TerminatorKind::Call {
            func,
            args,
            destination,
            ..
        } = &terminator.kind
        {
            if let Operand::Constant(box c) = func {
                if let TyKind::FnDef(f, _substs) = c.literal.ty().kind() {
                    let func_name = tcx.def_path_str(*f);
                    if func_name.find(SPL_ID) != None {
                        if let Some(tup) = destination {
                            // debug!("destination: {:?}", destination);
                            let mut st = Self::compute_place(tup.0.as_ref(), body, tcx, sm, Some(src_info));
                            if st.is_key {
                                st.checked.set(true);
                                st.external = false;
                            }
                            return st.clone();
                        }
                    } else if func_name.find(SPL_INSTRUCTION) != None {
                        let program_id = &args[0];
                        let st = Self::compute_operand(program_id, body, tcx, sm, src_info);
                        // debug!("State in place: {:?}, {:?}", program_id, st);
                        if st.is_key {
                            if let Operand::Move(id_place) | Operand::Copy(id_place) = program_id {
                                let id_pf =
                                    FullPlaceRef::new(body.source.def_id(), id_place.as_ref());

                                if !st.checked.get() {
                                    // debug!("cpi error");
                                    let call_stack =
                                        Report::call_stack_formatter(tcx, sm.get_call_stack());
                                    Report::new_bug(
                                        tcx,
                                        VulnerabilityType::CrossProgramInvocation,
                                        "Critical".to_string(),
                                        tcx.def_path_str(body.source.def_id()),
                                        source_info::get_source_file(tcx, src_info.span)
                                            .unwrap_or("".to_string()),
                                        source_info::get_source_lines(tcx, body.span)
                                            .unwrap_or("".to_string()),
                                        call_stack,
                                        "UnResolved".to_string(),
                                        None,
                                        None,
                                        None,
                                    );
                                }
                            }
                        }
                    } else if func_name == CMP_EQ || func_name == CMP_NE {
                        assert_eq!(args.len(), 2);
                        debug!("Opnd: {:?}, {:?}, {:?}", func_name, args[0], args[1]);
                        let mut update_op_st_fn = |op| {
                            let st = Self::compute_operand(op, body, tcx, sm, src_info);
                            if st.is_key {
                                // let mut checked = st.checked.borrow_mut();
                                // if let Operand::Move(op_place) | Operand::Copy(op_place) = op {
                                //     let op_fp =
                                //         FullPlaceRef::new(body.source.def_id(), op_place.as_ref());
                                //     checked.insert(op_fp);
                                // }
                                st.checked.set(true);
                            }
                            debug!("Update op: {:?}, st: {:?}", op, st);
                        };
                        update_op_st_fn(&args[0]);
                        update_op_st_fn(&args[1]);
                    }
                }
            }
        }
        Default::default()
    }

    fn init_place(
        place: PlaceRef<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, ProgramId>,
    ) -> Self {
        if is_pubkey(place, body, tcx) {
            let mut st = Self::default();
            st.is_key = true;
            return st;
        }
        Default::default()
    }

    fn merge(
        &self,
        other: Self,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, ProgramId>,
    ) -> Self {
        Self {
            is_key: self.is_key | other.is_key,
            external: self.external | other.external,
            checked: Rc::new(Cell::new(self.checked.get() || other.checked.get())),
        }
    }
}
