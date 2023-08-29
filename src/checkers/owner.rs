//! Check if the account owner is ever checked.
//! This is a bit complex since in some functions we need
//! to check the owner while in others we don't.
//! And in some functions we might check the owner multiple times,
//! in some functions we only do it once.
//! Currently the checking criteria is:
//! 1. The function must have a parameter that is &[AccountInfo],
//! or Context<...> (in anchor).
//! 2. In the function (or its callees), there must be a place
//! with AccountInfo type and its 'owner' field is involved
//! in a comparison (directly or indirectly). The other field
//! must either be a constant (spl_token::ID) or from the parameter
//! of the function (program_id: Pubkey).

use std::{cell::Cell, rc::Rc, sync::Once};

use log::{debug, warn};
use rustc_hir::def_id::DefId;
use rustc_middle::{
    mir::{BinOp, Body, Operand, Place, PlaceRef, Rvalue, TerminatorKind, PlaceElem, SourceInfo},
    ty::{
        subst::{GenericArgKind, SubstsRef},
        AdtDef, Ty, TyCtxt, TyKind, FieldDef,
    },
};
use rustc_span::Symbol;

use crate::{
    accounts::ACCOUNT_TY,
    wpa::{FullPlaceRef, StateMachine, StateTransitor}, reporter::{Report, VulnerabilityType}, source_info,
};

const PUBKEY: &'static str = "anchor_lang::prelude::Pubkey";
const CONTEXT: &'static str = "anchor_lang::context::Context";
const ACC_INFO: &'static str = "solana_program::account_info::AccountInfo";
const ANCHOR_ACC_INFO: &'static str = "anchor_lang::prelude::AccountInfo";
const ACCOUNTS_TRAIT: &'static str = "anchor_lang::Accounts";
const CMP_NE: &'static str = "std::cmp::PartialEq::ne";
const CMP_EQ: &'static str = "std::cmp::PartialEq::eq";

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OwnerCheck {
    pub from_owner_field: bool,
    // If the place/operand represents a expected
    // owner id, which is either from the parameter
    // or a constant
    pub owner_id: bool,
    // This state is per-function
    pub checked: Rc<Cell<bool>>,
    pub data_used: Rc<Cell<bool>>
}

impl OwnerCheck {
    pub fn get_default<'tcx>(sm: &mut StateMachine<'tcx, Self>) -> Self {
        let func_st = sm.get_cur_func_state().unwrap();
        Self {
            from_owner_field: true,
            owner_id: false,
            checked: func_st.checked.clone(),
            data_used: func_st.data_used.clone(),
        }
    }
}

fn is_pubkey<'tcx>(ty: Ty<'tcx>, tcx: TyCtxt<'tcx>) -> bool {
    let ty = ty.peel_refs();
    if let TyKind::Adt(adt_def, _substs) = ty.kind() {
        let ty_str = tcx.def_path_str(adt_def.did);
        if ty_str == PUBKEY {
            return true;
        }
    }
    false
}

fn get_all_fields<'tcx>(
    adt_def: &'tcx AdtDef,
    projections: &'tcx [PlaceElem<'tcx>],
) -> Vec<Symbol> {
    // debug!("Adt: {:?}, Fields: {:?}", adt_def, projections);
    let all_fields: Vec<&FieldDef> = adt_def.all_fields().collect();
    let mut field_indices = Vec::new();
    for proj in projections {
        match proj {
            PlaceElem::Field(field, _field_ty) => {
                field_indices.push(field);
            }
            _ => {}
        }
    }
    field_indices
        .iter()
        .map(|idx| all_fields[idx.index()].name)
        .collect()
}

fn parse_field_names<'tcx>(
    place: PlaceRef<'tcx>, body: &Body<'tcx>, tcx: TyCtxt<'tcx>
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
                    res.push((base, ty_str, all_fields[field.index()].name));
                }
            }
            _ => {}
        }
        cur = base;
    }
    res
}

fn is_owner_field<'tcx>(place: PlaceRef<'tcx>, body: &Body<'tcx>, tcx: TyCtxt<'tcx>) -> bool {
    let base = Place::from(place.local);
    let ty = base.ty(body, tcx).ty;
    if let TyKind::Adt(adt_def, _substs) = ty.kind() {
        let ty_str = tcx.def_path_str(adt_def.did);
        // if ty_str == ACC_INFO {
        let fields = get_all_fields(adt_def, place.projection);
        // TODO: we need to check if the parent object of the data fields is AccountInfo
        // e.g., acc.data or accs.acc.data
        if fields.len() >= 1 && fields.last().unwrap().as_str() == "data" {
            return true;
        }
        // }
    }
    false
}

impl<'tcx> StateTransitor<'tcx> for OwnerCheck {
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
                    let mut update_op_st_fn = |op| {
                        let st = Self::compute_operand(op, body, tcx, sm, src_info);
                        if st.owner_id {
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
        OwnerCheck::get_default(sm)
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
                let mut st = OwnerCheck::get_default(sm);
                if is_pubkey(c.ty(), tcx) {
                    st.owner_id = true;
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
        let fp = FullPlaceRef::new(body.source.def_id(), place);
        if let Some(st) = sm.get_state(fp) {
            // debug!("Existing place: {:?}, st: {:?}", place, st);
            return st.clone();
        }

        let mut st = OwnerCheck::get_default(sm);
        let local_index = place.local.index();
        // If it's parameter and its type is Pubkey
        if 1usize <= local_index && local_index <= body.arg_count {
            if is_pubkey(place.ty(body, tcx).ty, tcx) {
                st.owner_id = true;
            }
        }
        // debug!("Computing place: {:?}, st: {:?}", place, st);
        st
    }

    fn init_place(
        place: PlaceRef<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
    ) -> Self {
        let mut st = OwnerCheck::get_default(sm);
        // If it's from parameter and its type is Pubkey
        if is_pubkey(place.ty(body, tcx).ty, tcx) {
            if place.local >= 1u32.into() && place.local <= body.arg_count.into() {
                st.owner_id = true;
            }
        }
        st
    }

    fn merge(
        &self,
        other: Self,
        _body: &Body<'tcx>,
        _tcx: TyCtxt<'tcx>,
        _sm: &mut StateMachine<'tcx, Self>,
    ) -> Self {
        let owner_id = self.owner_id | other.owner_id;
        let from_owner_field = self.from_owner_field | other.from_owner_field;
        let checked = self.checked.clone();
        let data_used = self.data_used.clone();
        if other.checked.get() {
            self.checked.set(true);
        }
        if other.data_used.get() {
            self.data_used.set(true);
        }
        Self { from_owner_field, owner_id, checked, data_used}
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
                    if func_name == CMP_EQ || func_name == CMP_NE {
                        // A comparison statement
                        assert_eq!(args.len(), 2);
                        // debug!("Opnd: {:?}, {:?}, {:?}", func_name, args[0], args[1]);
                        let mut update_op_st_fn = |op| {
                            let st = Self::compute_operand(op, body, tcx, sm, src_info);
                            if st.owner_id {
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
        OwnerCheck::get_default(sm)
    }

    /// If this function contains a parameter that is Context,
    /// or is &[AccountInfo] (means this function is for
    /// process Instruction)
    fn is_interesting_fn(
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        _sm: &mut StateMachine<'tcx, Self>,
    ) -> bool {
        if body.arg_count >= 1 {
            let arg = &body.local_decls[1u32.into()];
            if let TyKind::Adt(adt_def, _substs) = arg.ty.kind() {
                let ty_str = tcx.def_path_str(adt_def.did);
                if ty_str == CONTEXT {
                    return true;
                }
            }
        }
        for arg_i in body.args_iter() {
            let arg = &body.local_decls[arg_i];
            if let TyKind::Slice(element_ty) = arg.ty.peel_refs().kind() {
                if let TyKind::Adt(adt_def, _) = element_ty.kind() {
                    let ty_str = tcx.def_path_str(adt_def.did);
                    if ty_str == ACCOUNT_TY {
                        return true;
                    }
                }
            }
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
            from_owner_field: false,
            owner_id: false,
            checked: Rc::new(Cell::new(false)),
            data_used: Rc::new(Cell::new(false)),
        })
    }

    fn report_func(
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        state: Self,
        sm: &mut StateMachine<'tcx, Self>,
    ) {
        if Self::is_interesting_fn(body, tcx, sm) {
            // debug!("Func state: {:?}", state);
            if state.checked.get() {
                return;
            }
            if body.arg_count >= 1 {
                let arg = &body.local_decls[1u32.into()];
                if let TyKind::Adt(adt_def, substs) = arg.ty.kind() {
                    let ty_str = tcx.def_path_str(adt_def.did);
                    if ty_str == CONTEXT {
                        let (account_ty, _) = get_last_generic_subst(substs).unwrap();
                        // debug!("Check account ty: {}", tcx.def_path_str(account_ty.did));
                        let trait_id = get_try_account_trait(tcx).unwrap();
                        let mut checked = false;
                        tcx.for_each_relevant_impl(
                            trait_id,
                            tcx.type_of(account_ty.did),
                            |impl_id| {
                                if let Some(assoc_item) = tcx
                                    .associated_items(impl_id)
                                    .filter_by_name_unhygienic(Symbol::intern("try_accounts"))
                                    .next()
                                {
                                    let st = sm.get_func_state(assoc_item.def_id).unwrap();
                                    // debug!("Try accounts Func state: {:?}", st);
                                    checked = st.checked.get();
                                }
                            },
                        );
                        if checked {
                            return;
                        }
                    }
                }
            }
            warn!(
                "Function {} missed a owner check!",
                tcx.def_path_str(body.source.def_id())
            );
            
            let call_stack = Report::call_stack_formatter(tcx, sm.get_call_stack());
            Report::new_bug(
                tcx,
                VulnerabilityType::MissingOwnerCheck,
                "Critical".to_string(),
                tcx.def_path_str(body.source.def_id()),
                source_info::get_source_file(tcx, body.span).unwrap_or("".to_string()),
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

fn get_try_account_trait<'tcx>(tcx: TyCtxt<'tcx>) -> Option<DefId> {
    static INIT: Once = Once::new();
    static mut ACCOUNTS_TRAIT_ID: Option<DefId> = None;
    INIT.call_once(|| {
        for trait_id in tcx.all_traits() {
            // TODO: Erfan
            if tcx.def_path_str(trait_id) == ACCOUNTS_TRAIT {
                unsafe {
                    ACCOUNTS_TRAIT_ID = Some(trait_id);
                }
            }
        }
    });
    unsafe { ACCOUNTS_TRAIT_ID }
}

fn get_last_generic_subst<'tcx>(
    substs: SubstsRef<'tcx>,
) -> Option<(&'tcx AdtDef, SubstsRef<'tcx>)> {
    if let GenericArgKind::Type(ty) = substs.last().unwrap().unpack() {
        if let TyKind::Adt(adt_def, substs) = ty.kind() {
            return Some((*adt_def, *substs));
        }
    }
    None
}
