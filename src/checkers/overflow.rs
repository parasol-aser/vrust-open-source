//! Checker for integer overflow/underflow

use log::{warn, debug};
use rustc_middle::{
    mir::{
        BinOp, Body, Operand, PlaceRef, ProjectionElem, Rvalue, SourceInfo, Terminator,
        TerminatorKind,
    },
    ty::{Ty, TyCtxt, TyKind},
};

use crate::{
    reporter::{IntegerCveType, Report, VulnerabilityType},
    source_info,
    wpa::{FullPlaceRef, StateMachine, StateTransitor},
};

const DESERIALIZE: &'static str = "BorshDeserialize::deserialize";

const WHITELIST: &'static [&'static str] = &[
    "minimum_balance"
];

fn should_filter(fname:  &str) -> bool {
    for f in WHITELIST {
        if fname.contains(f) {
            return true;
        }
    }
    false
}

// TODO: for some expressions, e.g., 
// b = a.checked_rem(8), we can infer that 0 <= b < 8,
// we should have a abstract state to track the possible
// value range of a place (only when we are confident),
// this will help reduce some false positives.
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct ExternalNumeric {
    pub external: bool,
}

impl Default for ExternalNumeric {
    fn default() -> Self {
        Self { external: false }
    }
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

impl<'tcx> StateTransitor<'tcx> for ExternalNumeric {
    fn compute_rvalue(
        rvalue: &Rvalue<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
        src_info: SourceInfo,
    ) -> Self {
        let default = Default::default();
        match rvalue {
            Rvalue::Use(op) => Self::compute_operand(op, body, tcx, sm, src_info),
            Rvalue::Ref(_, _, place) => Self::compute_place(place.as_ref(), body, tcx, sm, Some(src_info)),
            Rvalue::Cast(_, op, _ty) => Self::compute_operand(op, body, tcx, sm, src_info),
            Rvalue::BinaryOp(op, box (op1, op2))
            | Rvalue::CheckedBinaryOp(op, box (op1, op2)) => {
                match op {
                    BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {}
                    _ => {
                        // Ignore all other binary operations.
                        return default;
                    }
                }
                let st1 = Self::compute_operand(op1, body, tcx, sm, src_info);
                let st2 = Self::compute_operand(op2, body, tcx, sm, src_info);
                // debug!("Op1: {:?}, st: {:?}", op1, st1);
                // debug!("Op2: {:?}, st: {:?}", op2, st2);
                if ((op1.ty(body, tcx).is_numeric() && st1.external)
                    || (op2.ty(body, tcx).is_numeric() && st2.external))
                    && !should_filter(tcx.def_path_str(body.source.def_id()).as_str())
                {
                    warn!("Potential integer overflow: {:?}", rvalue);
                    let call_stack = Report::call_stack_formatter(tcx, sm.get_call_stack());
                    if !Report::get_filtered() && !Report::get_blacklist() {
                        Report::new_bug(
                            tcx,
                            VulnerabilityType::IntegerFlow,
                            "Critical".to_string(),
                            tcx.def_path_str(body.source.def_id()),
                            source_info::get_source_lines(tcx, src_info.span).unwrap_or("".to_string()),
                            source_info::get_source_lines(tcx, body.span).unwrap_or("".to_string()),
                            call_stack,
                            "UnResolved".to_string(),
                            None,
                            None,
                            None,
                        );
                    }
                }
                Self {
                    external: st1.external || st2.external,
                }
            }
            Rvalue::UnaryOp(_, op) => Self::compute_operand(op, body, tcx, sm, src_info),
            _ => default,
        }
    }

    fn compute_operand(
        op: &Operand<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
        src_info: SourceInfo,
    ) -> Self {
        match op {
            Operand::Copy(from) | Operand::Move(from) => {
                Self::compute_place(from.as_ref(), body, tcx, sm, Some(src_info))
            }
            Operand::Constant(_from) => ExternalNumeric { external: false },
        }
    }

    fn compute_place(
        place: PlaceRef<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
        src_info: Option<SourceInfo>,
    ) -> Self {
        let ty = place.ty(body, tcx);
        let external_from_downcast = has_downcast_continue(place);
        let mut cur = place;
        let mut full_place = FullPlaceRef::new(body.source.def_id(), cur);
        // If place is external, then all its projects should be external too
        while sm.get_state(full_place).is_none() {
            if let Some((base, _projs)) = cur.last_projection() {
                full_place = FullPlaceRef::new(body.source.def_id(), base);
                cur = base;
            } else {
                break;
            }
        }
        if let Some(st) = sm.get_state(full_place) {
            let st = Self {
                external: st.external || external_from_downcast,
            };
            // debug!("Exist place {:?} st: {:?}", place, st);
            return st;
        }

        let local_index = place.local.index();
        // If the place base is a parameter, it's from external
        // TODO: the parameter could be coming from an actual argument that is certainly
        // not external value, so we need to query the state of the parameter first if
        // we already update it for the argument-parameter binding.
        let external = if 1usize <= local_index && local_index <= body.arg_count {
            true
        } else {
            false
        };
        // debug!("New place {:?} st: {}", place, external);
        Self { external }
    }

    fn init_place(
        place: PlaceRef<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
    ) -> Self {
        Self::compute_place(place, body, tcx, sm, None)
    }

    fn merge(
        &self,
        other: Self,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
    ) -> Self {
        Self {
            external: self.external || other.external,
        }
    }

    fn compute_terminator(
        terminator: &Terminator<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
        src_info: SourceInfo,
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
                    if func_name == DESERIALIZE {
                        // Any deserialized value is not safe
                        if destination.is_some() {
                            return Self { external: true };
                        }
                    }
                }
            }
        }
        Default::default()
    }
}
