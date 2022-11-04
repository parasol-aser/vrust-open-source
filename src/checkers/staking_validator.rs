//! Check if missing staking validator check.
//! Steps:
//! 1. Check at the callsite of delegate_stake, which has three arguments,
//!    the last one is the key of the validator vote account
//! 2. Get the name of the last argument, check if it has been checked before.
//! We don't care the name or other information of this argument. This should make
//! the checking more robust.

use crate::{
    reporter::{Report, VulnerabilityType},
    source_info,
    wpa::StateMachine,
};
use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
    rc::Rc,
};

use log::{debug, warn};
use rustc_middle::{
    mir::{BinOp, Body, Operand, PlaceRef, Rvalue, TerminatorKind, SourceInfo},
    ty::{TyCtxt, TyKind},
};

use crate::wpa::{FullPlaceRef, StateTransitor};

const STAKE_FUN: &'static str = "anchor_lang::solana_program::stake::instruction::delegate_stake";

const CMP_NE: &'static str = "std::cmp::PartialEq::ne";
const CMP_EQ: &'static str = "std::cmp::PartialEq::eq";

const PUBKEY: &'static str = "anchor_lang::prelude::Pubkey";

#[derive(Clone, PartialEq, Eq)]
pub struct StakingValidator {
    pub is_key: bool,
    pub checked: Rc<Cell<bool>>,
}

impl Default for StakingValidator {
    fn default() -> Self {
        Self {
            is_key: false,
            checked: Rc::new(Cell::new(false)),
        }
    }
}

impl core::fmt::Debug for StakingValidator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StakingValidator")
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
        if ty_str == PUBKEY {
            return true;
        }
    }
    false
}

impl<'tcx> StateTransitor<'tcx> for StakingValidator {
    fn compute_rvalue(
        rvalue: &rustc_middle::mir::Rvalue<'tcx>,
        body: &rustc_middle::mir::Body<'tcx>,
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, StakingValidator>,
        src_info: rustc_middle::mir::SourceInfo,
    ) -> Self {
        match rvalue {
            Rvalue::Use(op) => return Self::compute_operand(op, body, tcx, sm, src_info),
            Rvalue::Ref(_, _, place) => return Self::compute_place(place.as_ref(), body, tcx, sm, Some(src_info)),
            Rvalue::Cast(_, op, _) => return Self::compute_operand(op, body, tcx, sm, src_info),
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
                    };
                    update_op_st_fn(lhs);
                    update_op_st_fn(rhs);
                }
                _ => {}
            },
            _ => {}
        }
        Default::default()
    }

    fn compute_operand(
        op: &rustc_middle::mir::Operand<'tcx>,
        body: &rustc_middle::mir::Body<'tcx>,
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, StakingValidator>,
        src_info: SourceInfo,
    ) -> Self {
        match op {
            Operand::Move(from) | Operand::Copy(from) => {
                Self::compute_place(from.as_ref(), body, tcx, sm, Some(src_info))
            }
            Operand::Constant(_) => Default::default(),
        }
    }

    fn compute_place(
        place: rustc_middle::mir::PlaceRef<'tcx>,
        body: &rustc_middle::mir::Body<'tcx>,
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, StakingValidator>,
        src_info: Option<SourceInfo>,
    ) -> Self {
        let fp = FullPlaceRef::new(body.source.def_id(), place);
        if let Some(st) = sm.get_state(fp) {
            // debug!("Existing place: {:?}, st: {:?}", place, st);
            return st.clone();
        }

        if is_pubkey(place, body, tcx) {
            let mut st = Self::default();
            st.is_key = true;
            // debug!("Place: {:?}, st: {:?}", place, st);
            // We need to preserve their states, otherwise next time
            // we will create a new state for them.
            sm.update(place, st.clone(), body, tcx);
            return st;
        }

        let default = Default::default();
        // debug!("Place: {:?}, st: {:?}", place, default);
        default
    }

    fn init_place(
        place: rustc_middle::mir::PlaceRef<'tcx>,
        body: &rustc_middle::mir::Body<'tcx>,
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
        _sm: &mut StateMachine<'tcx, StakingValidator>,
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
        _body: &rustc_middle::mir::Body<'tcx>,
        _tcx: rustc_middle::ty::TyCtxt<'tcx>,
        _sm: &mut StateMachine<'tcx, StakingValidator>,
    ) -> Self {
        let is_key = self.is_key | other.is_key;
        // Choose the larger set
        // TODO: is this correct?
        // let checked = if self.checked.borrow().len() > other.checked.borrow().len() {
        //     self.checked.clone()
        // } else {
        //     other.checked.clone()
        // };
        let checked = Rc::new(Cell::new(self.checked.get() || other.checked.get()));
        Self { is_key, checked }
    }

    fn compute_terminator(
        terminator: &rustc_middle::mir::Terminator<'tcx>,
        body: &rustc_middle::mir::Body<'tcx>,
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, StakingValidator>,
        src_info: rustc_middle::mir::SourceInfo,
    ) -> Self {
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
                    // debug!("Func name: {}", func_name);
                    if func_name == STAKE_FUN {
                        assert_eq!(args.len(), 3);
                        let vote_key = &args[2];
                        let st = Self::compute_operand(vote_key, body, tcx, sm, src_info);
                        // debug!("delagate stake st: {:?}", st);
                        if st.is_key {
                            if let Operand::Move(vote_place) | Operand::Copy(vote_place) = vote_key
                            {
                                let vote_fp =
                                    FullPlaceRef::new(body.source.def_id(), vote_place.as_ref());
                                // if !st.checked.borrow().contains(&vote_fp) {
                                if !st.checked.get() {
                                    warn!("Stating vote account not checked!");
                                    let call_stack = Report::call_stack_formatter(tcx, sm.get_call_stack());
                                    Report::new_bug(
                                        tcx,
                                        VulnerabilityType::StakingValidator,
                                        "Critical".to_string(),
                                        tcx.def_path_str(body.source.def_id()),
                                        source_info::get_source_file(tcx, src_info.span)
                                            .unwrap_or("".to_string()),
                                        source_info::get_source_lines(tcx, body.span)
                                            .unwrap_or("".to_string()),
                                        call_stack,
                                        "GitHub Link to be added.".to_string(),
                                        None,
                                        None,
                                        None,
                                    );
                                }
                            }
                        }
                    } else if func_name == CMP_EQ || func_name == CMP_NE {
                        // A comparison statement
                        assert_eq!(args.len(), 2);
                        // debug!("Opnd: {:?}, {:?}, {:?}", func_name, args[0], args[1]);
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
                            // debug!("Update op: {:?}, st: {:?}", op, st);
                        };
                        update_op_st_fn(&args[0]);
                        update_op_st_fn(&args[1]);
                    }
                }
            }
        }
        Default::default()
    }
}
