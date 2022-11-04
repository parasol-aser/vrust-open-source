//! For some std structures/functions used in solana programs, it's non trivial
//! to correctly propagate the states through these functions.
//! e.g., core::slice::iter will eventuall call some intrinsics like std::intrinsics::arith_offset.
//! Instead of intercepting all these intrinsics, we directly intercept the functions
//! like core::slice::iter.

use std::lazy::SyncLazy;

use hashbrown::HashMap;
use log::debug;
use rustc_hir::def_id::DefId;
use rustc_middle::{ty::TyCtxt, mir::Body};

use crate::wpa::{StateTransitor, StateMachine};

use super::InterceptedFn;

static STD_IDENTITY_FNS: SyncLazy<HashMap<&'static str, InterceptedFn>> = SyncLazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "core::slice::<impl [T]>::iter",
        InterceptedFn("core::slice::<impl [T]>::iter", &[0]),
    );
    map.insert(
        "std::ops::Try::branch",
        InterceptedFn("std::ops::Try::branch", &[0])); 
    map.insert(
        "std::ops::Deref::deref",
        InterceptedFn("std::ops::Deref::deref", &[0]),
    );
    map
});

pub(crate) fn intercept_call<'tcx, T: StateTransitor<'tcx>>(
    callee: DefId,
    tcx: TyCtxt<'tcx>,
    arg_states: &[T],
    body: &Body<'tcx>,
    sm: &mut StateMachine<'tcx, T>,
) -> Option<T> {
    let callee_name = tcx.def_path_str(callee);
    if let Some(val) = STD_IDENTITY_FNS.get(callee_name.as_str()) {
        let mut res: Option<T> = None;
        for &idx in val.1 {
            if res.is_none() {
                res = Some(arg_states[idx].clone());
            } else {
                res = res.map(|s| s.merge(arg_states[idx].clone(), body, tcx, sm));
            }
        }
        return res;
    }
    None
}