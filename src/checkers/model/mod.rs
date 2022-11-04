use rustc_hir::def_id::DefId;
use rustc_middle::{mir::Body, ty::TyCtxt};

use crate::wpa::{StateMachine, StateTransitor};

pub mod solana_wrapper;
pub mod std_wrapper;

/// Functions to be intercepted.
/// (function name, the indices of interesting arguments)
pub struct InterceptedFn(&'static str, &'static [usize]);

pub fn intercept_call<'tcx, T: StateTransitor<'tcx>>(
    callee: DefId,
    tcx: TyCtxt<'tcx>,
    arg_states: &[T],
    body: &Body<'tcx>,
    sm: &mut StateMachine<'tcx, T>,
) -> Option<T> {
    std_wrapper::intercept_call(callee, tcx, arg_states, body, sm).or(
        solana_wrapper::intercept_call(callee, tcx, arg_states, body, sm),
    )
}
