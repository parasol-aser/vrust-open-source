use std::{fmt::Display, ops::Index};

// use hashbrown::{HashMap, HashSet};
use log::{debug, warn};

use std::{cell::RefCell, collections::HashSet, rc::Rc};

use regex::Regex;
use rustc_hir::{def::DefKind, def_id::DefId};
use rustc_middle::{
    mir::{
        traversal::reverse_postorder, Body, Operand, Place, PlaceRef, Rvalue, SourceInfo,
        Statement, StatementKind, Terminator, TerminatorKind,
    },
    ty::{Ty, TyCtxt, TyKind},
};

use crate::dd::{self, DataDepsGraph, ASSERT_MACRO_FILTER};

use crate::wpa::{self, FullPlaceRef, StateMachine, StateTransitor};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct is_signer_checker<'tcx> {
    // pub ty: Ty<'tcx>,
    // pub external: bool,
    pub has_signer_check: bool,
    pub has_signer_var: bool,
    pub signer_var_name: String,
    pub checked_funcs: Rc<RefCell<HashSet<FullPlaceRef<'tcx>>>>,
}

impl<'tcx> Default for is_signer_checker<'tcx> {
    fn default() -> Self {
        Self {
            has_signer_check: false,
            has_signer_var: false,
            signer_var_name: String::new(),
            checked_funcs: Rc::new(RefCell::new(HashSet::new())),
        }
    }
}

impl<'tcx> StateTransitor<'tcx> for is_signer_checker<'tcx> {
    fn compute_rvalue(
        rvalue: &Rvalue<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
        src_info: SourceInfo,
    ) -> Self {
        debug!("IsSignerChecker--compute_rvalue: {:?}", rvalue);
        let dummy = Default::default();
        match rvalue {
            Rvalue::Use(op) => Self::compute_operand(op, body, tcx, sm, src_info),
            Rvalue::Ref(_, _, place) => Self::compute_place(place.as_ref(), body, tcx, sm, Some(src_info)),
            Rvalue::Cast(_, op, _ty) => Self::compute_operand(op, body, tcx, sm, src_info),
            Rvalue::BinaryOp(_op, box (op1, op2)) => {
                let st1 = Self::compute_operand(op1, body, tcx, sm, src_info);
                let st2 = Self::compute_operand(op2, body, tcx, sm, src_info);
                Default::default()
            }
            Rvalue::UnaryOp(_, op) => Self::compute_operand(op, body, tcx, sm, src_info),
            _ => dummy,
        }
    }

    fn compute_operand(
        op: &Operand<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
        src_info: SourceInfo,
    ) -> Self {
        debug!("IsSignerChecker--compute_operand: {:?}", op);
        match op {
            Operand::Copy(from) | Operand::Move(from) => {
                Self::compute_place(from.as_ref(), body, tcx, sm, Some(src_info))
            }
            Operand::Constant(_from) => Default::default(),
        }
    }

    fn compute_place(
        place: PlaceRef<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
        src_info: Option<SourceInfo>,
    ) -> Self {
        debug!("IsSignerChecker--compute_place: {:?}", place);
        if let Some(st) = sm.get_state(FullPlaceRef::new(body.source.def_id(), place)) {
            return Default::default();
        }
        let local_index = place.local.index();
        // If the place base is a parameter, it's from external
        // TODO: the parameter could be coming from an actual argument that is certainly
        // not external value, so we need to query the state of the parameter first if
        // we already update it for the argument-parameter binding.
        let external = if 1usize <= local_index && local_index <= body.arg_count + 1 {
            true
        } else {
            false
        };
        return Default::default();
    }

    fn compute_terminator(
        terminator: &rustc_middle::mir::Terminator<'tcx>,
        body: &rustc_middle::mir::Body<'tcx>,
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
        _src_info: rustc_middle::mir::SourceInfo,
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
                    debug!("Func name: {}", func_name);
                }
            }
        }
        Default::default()
    }

    fn init_place(
        place: PlaceRef<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
    ) -> Self {
        todo!()
    }

    fn merge(
        &self,
        other: Self,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
    ) -> Self {
        // todo!()
        let checked_funcs =
            if self.checked_funcs.borrow().len() > other.checked_funcs.borrow().len() {
                self.checked_funcs.clone()
            } else {
                other.checked_funcs.clone()
            };
        Self {
            has_signer_check: self.has_signer_check || other.has_signer_check,
            has_signer_var: self.has_signer_var || other.has_signer_var,
            signer_var_name: if self.signer_var_name.len() > other.signer_var_name.len() {
                self.signer_var_name.clone()
            } else {
                other.signer_var_name.clone()
            },
            checked_funcs,
        }
    }

    fn is_interesting_fn(
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
    ) -> bool {
        true
    }

    fn compute_func(
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
    ) -> Option<Self> {
        None
    }

    fn report_func(body: &Body<'tcx>, tcx: TyCtxt<'tcx>, state: Self, sm: &mut StateMachine<'tcx, Self>) {}
}
