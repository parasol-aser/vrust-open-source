//! Wrappers for soem solana programs

use std::lazy::SyncLazy;

use hashbrown::HashMap;
use rustc_hir::def_id::DefId;
use rustc_middle::{ty::TyCtxt, mir::Body};

use crate::wpa::{StateTransitor, StateMachine};

use super::InterceptedFn;

static SOLANA_IDENTITY_FNS: SyncLazy<HashMap<&'static str, InterceptedFn>> = SyncLazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "solana_program::account_info::next_account_info",
        InterceptedFn("solana_program::account_info::next_account_info", &[0]),
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
    if let Some(val) = SOLANA_IDENTITY_FNS.get(callee_name.as_str()) {
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
