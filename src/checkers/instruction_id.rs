//! Check if the instruction account key is checked against solana program instruction ID

use std::{cell::Cell, rc::Rc};

use log::{debug, warn};
use rustc_middle::{
    mir::{
        coverage::Op, BinOp, Body, Operand, PlaceElem, PlaceRef, ProjectionElem, Rvalue,
        Terminator, TerminatorKind, VarDebugInfoContents, SourceInfo,
    },
    ty::{FieldDef, Ty, TyCtxt, TyKind},
};
use rustc_span::Symbol;

use crate::{
    reporter::{Report, VulnerabilityType},
    source_info,
    wpa::{FullPlaceRef, StateMachine, StateTransitor},
};

use super::model;

const INST_ID: &'static str = "solana_program::sysvar::instructions::id";

const CMP_NE: &'static str = "std::cmp::PartialEq::ne";
const CMP_EQ: &'static str = "std::cmp::PartialEq::eq";
// spl_token uses this function to check the equality of two pubkeys.
// This function will call sol_memcmp instead of `==` for the checking.
const CMP_KEY_FUN: &'static str = "cmp_pubkeys";

const PUBKEY: &'static str = "solana_program::pubkey::Pubkey";
const ANCHOR_PUBKEY: &'static str = "anchor_lang::prelude::Pubkey";

const ACC_INFO: &'static str = "solana_program::account_info::AccountInfo";
const ANCHOR_ACC_INFO: &'static str = "anchor_lang::prelude::AccountInfo";

#[derive(Clone, PartialEq, Eq)]
pub struct InstructionIdChecker {
    pub is_key: bool,
    pub is_from_param: bool,
    pub is_inst_id: bool,
    pub access_data: Rc<Cell<bool>>,
    // This is not function state, since we want to
    // distinguish checks on one AccountInfo from others
    pub checked: Rc<Cell<bool>>,
}

impl InstructionIdChecker {
    pub fn get_default<'tcx>(sm: &mut StateMachine<'tcx, Self>) -> Self {
        // let func_st = sm.get_cur_func_state().unwrap();
        Self {
            is_key: false,
            is_from_param: false,
            is_inst_id: false,
            access_data: Rc::new(Cell::new(false)),
            checked: Rc::new(Cell::new(false)),
        }
    }
}

impl core::fmt::Debug for InstructionIdChecker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InstructionIdChecker")
            .field("is_key", &self.is_key)
            .field("is_from_param", &self.is_from_param)
            .field("is_inst_id", &self.is_inst_id)
            .field("access_data", &self.access_data.get())
            .field("checked", &self.checked.get())
            .field("ptr", &self.checked.as_ptr())
            .finish()
    }
}

fn is_pubkey<'tcx>(ty: Ty<'tcx>, tcx: TyCtxt<'tcx>) -> bool {
    let ty = ty.peel_refs();
    if let TyKind::Adt(adt_def, _substs) = ty.kind() {
        let ty_str = tcx.def_path_str(adt_def.did);
        if ty_str == PUBKEY || ty_str == ANCHOR_PUBKEY {
            return true;
        }
    }
    false
}

fn has_downcast_continue<'tcx>(place: PlaceRef<'tcx>) -> bool {
    if place.projection.len() > 0 {
        let first = place.projection.first().unwrap();
        match first {
            ProjectionElem::Downcast(sym, _variant) => {
                if let Some(sym) = sym {
                    if sym.as_str() == "Continue" {
                        return true;
                    }
                }
            }
            _ => {}
        }
    }
    false
}

fn parse_field_names<'tcx>(
    place: PlaceRef<'tcx>,
    body: &Body<'tcx>,
    tcx: TyCtxt<'tcx>,
) -> Vec<(PlaceRef<'tcx>, String, Symbol)> {
    let mut cur = place;
    let mut res = Vec::new();
    while let Some((base, proj)) = cur.last_projection() {
        match proj {
            PlaceElem::Field(field, _field_ty) => {
                let ty = base.ty(body, tcx).ty.peel_refs();
                if let Some(adt_def) = ty.ty_adt_def() {
                    let ty_str = tcx.def_path_str(adt_def.did);
                    let all_fields: Vec<&FieldDef> = adt_def.all_fields().collect();
                    res.push((
                        peels_deref(base),
                        ty_str,
                        all_fields[field.index()].name,
                    ));
                }
            }
            _ => {}
        }
        cur = base;
    }
    res
}

fn peels_deref<'tcx>(place: PlaceRef<'tcx>) -> PlaceRef<'tcx> {
    let mut cur = place;
    while let Some((base, field)) = cur.last_projection() {
        match field {
            ProjectionElem::Deref => {
                cur = base;
                continue;
            }
            _ => {}
        }
        break;
    }
    cur
}

fn get_base_account<'tcx>(
    place: PlaceRef<'tcx>,
    body: &Body<'tcx>,
    tcx: TyCtxt<'tcx>,
) -> Option<PlaceRef<'tcx>> {
    if let Some((base, _field)) = place.last_projection() {
        let base = peels_deref(base);
        let base_ty = base.ty(body, tcx).ty.peel_refs();
        if let Some(adt_def) = base_ty.ty_adt_def() {
            let ty_str = tcx.def_path_str(adt_def.did);
            if ty_str == ACC_INFO || ty_str == ANCHOR_ACC_INFO {
                return Some(base);
            }
        }
    }
    None
}

impl<'tcx> StateTransitor<'tcx> for InstructionIdChecker {
    fn compute_rvalue(
        rvalue: &Rvalue<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
        src_info: rustc_middle::mir::SourceInfo,
    ) -> Self {
        match rvalue {
            Rvalue::Use(op) => return Self::compute_operand(op, body, tcx, sm, src_info),
            Rvalue::Ref(_, _, place) => return Self::compute_place(place.as_ref(), body, tcx, sm, Some(src_info)),
            Rvalue::BinaryOp(opnd, box (lhs, rhs)) => match opnd {
                BinOp::Ne | BinOp::Eq => {
                    // let st1 = Self::compute_operand(lhs, body, tcx, sm);
                    // let st2 = Self::compute_operand(rhs, body, tcx, sm);
                    // if st1.is_key && st1.is_from_param && st2.is_inst_id {
                    //     st1.checked.set(true);
                    // } else if st1.is_inst_id && st2.is_key && st2.is_from_param {
                    //     st2.checked.set(true);
                    // }
                }
                _ => {}
            },
            _ => {}
        }
        Self::get_default(sm)
    }

    fn compute_operand(
        op: &Operand<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
        src_info: SourceInfo,
    ) -> Self {
        match op {
            Operand::Copy(place) | Operand::Move(place) => {
                Self::compute_place(place.as_ref(), body, tcx, sm, Some(src_info))
            }
            Operand::Constant(box c) => {
                let mut st = Self::get_default(sm);
                if is_pubkey(c.ty(), tcx) {
                    st.is_key = true;
                    st.is_inst_id = true;
                }
                st
            }
        }
    }

    fn compute_place(
        place: rustc_middle::mir::PlaceRef<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
        src_info: Option<SourceInfo>,
    ) -> Self {
        let mut st = Self::get_default(sm);
        let fp = FullPlaceRef::new(body.source.def_id(), place);
        if let Some(existing_st) = sm.get_state(fp) {
            st = existing_st.clone();
        }

        // Keep the checked state of the base is AccountInfo
        if let Some(base) = get_base_account(place, body, tcx) {
            if let Some(base_st) = sm.get_state(FullPlaceRef::new(body.source.def_id(), base)) {
                if base_st.checked.get() {
                    st.checked.set(true);
                }
            }
        }

        let local_index = place.local.index();
        // If it's parameter
        if 1usize <= local_index && local_index <= body.arg_count {
            st.is_from_param = true;
        }

        // Check if the place comes from downcast through next_account_info or similar things
        // let mut is_from_param = has_downcast_continue(place);
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
        if let Some(existing_st) = sm.get_state(full_place) {
            // is_from_param |= existing_st.is_from_param;
            st.is_from_param |= existing_st.is_from_param;
        }
        // if is_from_param {
        //     st.is_from_param = true;
        // }

        // If it's Pubkey
        if is_pubkey(place.ty(body, tcx).ty, tcx) {
            st.is_key = true;
            // Link the key's checked state with its parent account
            if let Some(base) = get_base_account(place, body, tcx) {
                let mut base_st = Self::get_default(sm);
                if let Some(existing_st) =
                    sm.get_state(FullPlaceRef::new(body.source.def_id(), base))
                {
                    base_st = existing_st.clone();
                }
                base_st.checked = st.checked.clone();
                debug!("Link base account: {:?}, st: {:?}", base, base_st);
                sm.update(base, base_st, body, tcx);
                let stored_st = sm.get_state(FullPlaceRef::new(body.source.def_id(), base));
                debug!("Stored st: {:?}", stored_st);
            }
        }

        // Check if the place is a FieldRef that accesses data field of AccountInfo object
        let accesses = parse_field_names(place, body, tcx);
        if let Some((account_place, ty_str, field_name)) = accesses.first() {
            if (field_name.as_str() == "data" || field_name.as_str() == "lamports") && (ty_str == ACC_INFO || ty_str == ANCHOR_ACC_INFO) {
                // debug!("Base account: {:?}", account_place);
                // debug!("State: {:?}", sm
                //     .get_state(FullPlaceRef::new(body.source.def_id(), *account_place)));
                let account_st =
                    sm.get_state(FullPlaceRef::new(body.source.def_id(), *account_place));
                let account_checked = account_st.map(|st| st.checked.get()).unwrap_or(false);
                let account_from_param = account_st.map(|st| st.is_from_param).unwrap_or(false);
                if !account_checked && account_from_param {
                    warn!(
                        "Instruction id not checked in function {}",
                        tcx.def_path_str(body.source.def_id())
                    );
                    let call_stack = Report::call_stack_formatter(tcx, sm.get_call_stack());
                    debug!("%%%%%% Call stack: {:?}\n", call_stack);
                    debug!("%%%%%% Reporter: {} {}\n", Report::get_filtered(), Report::get_blacklist());

                    if Report::get_filtered() == false && Report::get_blacklist() == false { 
                        let call_stack = Report::call_stack_formatter(tcx, sm.get_call_stack());
                        let span = if let Some(src_info) = src_info {
                            src_info.span
                        } else {
                            body.span
                        };
                        Report::new_bug(
                            tcx,
                            VulnerabilityType::MissingKeyCheck,
                            "Critical".to_string(),
                            tcx.def_path_str(body.source.def_id()),
                            source_info::get_source_lines(tcx, span).unwrap_or("".to_string()),
                            source_info::get_source_lines(tcx, body.span).unwrap_or("".to_string()),
                            call_stack,
                            "UnResolved".to_string(),
                            None,
                            None,
                        None,
                        );
                    }
                }
            }
        }

        debug!(
            "Function: {}, Computing place: {:?}, st: {:?}",
            tcx.def_path_str(body.source.def_id()),
            place,
            st
        );
        st
    }

    fn init_place(
        place: PlaceRef<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
    ) -> Self {
        Self::get_default(sm)
    }

    fn merge(
        &self,
        other: Self,
        _body: &Body<'tcx>,
        _tcx: TyCtxt<'tcx>,
        _sm: &mut StateMachine<'tcx, Self>,
    ) -> Self {
        let is_key = self.is_key | other.is_key;
        let is_from_param = self.is_from_param | other.is_from_param;
        let is_inst_id = self.is_inst_id | other.is_inst_id;
        let access_data = other.access_data.clone();
        if self.access_data.get() {
            access_data.set(true);
        }
        let checked = other.checked.clone();
        if self.checked.get() {
            checked.set(true);
        }
        Self {
            is_key,
            is_from_param,
            is_inst_id,
            access_data,
            checked,
        }
    }

    fn compute_terminator(
        terminator: &rustc_middle::mir::Terminator<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
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
                    let arg_states: Vec<Self> = args
                        .iter()
                        .map(|op| Self::compute_operand(&args[0], body, tcx, sm, src_info))
                        .collect();
                    if func_name == CMP_EQ || func_name == CMP_NE || func_name.contains(CMP_KEY_FUN) {
                        // A comparison statement
                        assert_eq!(args.len(), 2);
                        let st1 = &arg_states[0];
                        let st2 = &arg_states[1];
                        debug!("Op {:?}, st: {:?}", args[0], st1);
                        debug!("Op {:?}, st: {:?}", args[1], st2);
                        // if st1.is_key && st1.is_from_param && st2.is_inst_id {
                        //     st1.checked.set(true);
                        // } else if st1.is_inst_id && st2.is_key && st2.is_from_param {
                        //     st2.checked.set(true);
                        // }
                        let mut update_checked_fn = |op: &Operand<'tcx>| {
                            debug!("Check op: {:?}", op);
                            // if let Some(base) = get_base_account(op, body, tcx) {
                            //     let base_st = Self::compute_place(base, body, tcx, sm);
                            //     base_st.checked.set(true);
                            //     // Must update immediately because this place may not
                            //     // appear before
                            //     debug!("Update place {:?}, state: {:?}", base, base_st);
                            //     sm.update(base, base_st, body, tcx);
                            // }
                            if let Some(from) = op.place() {
                                let from_st = Self::compute_place(from.as_ref(), body, tcx, sm, Some(src_info));
                                from_st.checked.set(true);
                                sm.update(from.as_ref(), from_st, body, tcx);
                            }
                        };
                        if st1.is_key && st1.is_from_param {
                            update_checked_fn(&args[0]);
                        }
                        if st2.is_key && st2.is_from_param {
                            update_checked_fn(&args[1]);
                        }
                    } else if func_name == INST_ID {
                        if let Some((dest, _)) = destination {
                            let mut st = Self::get_default(sm);
                            st.is_inst_id = true;
                            return st;
                        }
                    }
                    // If the callee is an intercepted target
                    if let Some(st) =
                        model::intercept_call(*f, tcx, arg_states.as_slice(), body, sm)
                    {
                        debug!("Intercepted: {}", func_name);
                        return st;
                    }
                }
            }
        }
        Self::get_default(sm)
    }

    fn is_interesting_fn(
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        _sm: &mut StateMachine<'tcx, Self>,
    ) -> bool {
        let mut args = Vec::new();
        for arg in body.args_iter() {
            args.push(arg);
        }
        for var_dbg in body.var_debug_info.iter() {
            if var_dbg.name.as_str() != "accs" && var_dbg.name.as_str() != "accounts" {
                continue;
            }
            return true;
            // if let VarDebugInfoContents::Place(place) = var_dbg.value {
            //     for arg in args.iter() {
            //         if *arg == place.local {
            //             let local_decl = &body.local_decls[*arg];
            //             if let Some(adt_def) = local_decl.ty.peel_refs().ty_adt_def() {
            //                 let ty_str = tcx.def_path_str(adt_def.did);
            //                 // debug!("Function: {}, arg: {:?}, ty: {}", tcx.def_path_str(body.source.def_id()), arg, ty_str);
            //                 if ty_str == "api::verify_signature::VerifySignatures"
            //                 {
            //                     return true;
            //                 }
            //             }
            //         }
            //     }
            // }
        }
        false
    }

    fn compute_func(
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
    ) -> Option<Self> {
        // We only need to set an initial state for each function,
        // the later traversing will update the shared state
        Some(Self {
            is_key: false,
            is_from_param: false,
            is_inst_id: false,
            access_data: Rc::new(Cell::new(false)),
            checked: Rc::new(Cell::new(false)),
        })
    }

    fn report_func(
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        state: Self,
        sm: &mut StateMachine<'tcx, Self>,
    ) {
        // if Self::is_interesting_fn(body, tcx, sm) {
        //     // debug!("Func state: {:?}", state);
        //     if state.checked.get() {
        //         return;
        //     }
        //     warn!(
        //         "Instruction id is not checked in function {}!",
        //         tcx.def_path_str(body.source.def_id())
        //     );
        //     Report::new_instruction_id_issue(
        //         tcx,
        //         "Instruction id issue".to_string(),
        //         "Critical".to_string(),
        //         source_info::get_source_file(tcx, body.span)
        //             .unwrap_or("".to_string()),
        //         source_info::get_source_lines(tcx, body.span)
        //             .unwrap_or("".to_string()),
        //         tcx.def_path_str(body.source.def_id()),
        //         "UnResolved".to_string(),
        //         "GitHub Link to be added.".to_string(),
        //         Some("message"),
        //         "Description of the bug here.".to_string(),
        //         "Some alleviation steps here.".to_string(),
        //     );
        // }
    }
}
