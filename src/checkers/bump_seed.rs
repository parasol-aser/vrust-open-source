//! Bump seed canonicalization.

use std::ops::BitOr;

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

const CPA: &'static str = "anchor_lang::prelude::Pubkey::create_program_address";
const DESERIALIZE: &'static str = "BorshDeserialize::deserialize";

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct BumpSeed {
    pub external: bool,
    pub is_bump: bool,
}

impl Default for BumpSeed {
    fn default() -> Self {
        Self {
            external: false,
            is_bump: false,
        }
    }
}

fn is_bump_var<'tcx>(place: PlaceRef<'tcx>, body: &Body<'tcx>) -> bool {
    for (idx, var_dbg_info) in body.var_debug_info.iter().enumerate() {
        match var_dbg_info.value {
            VarDebugInfoContents::Place(var_place) => {
                if var_place.as_ref() == place {
                    if var_dbg_info.name.as_str() == "bump" {
                        return true;
                    }
                }
            }
            _ => {}
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

impl<'tcx> StateTransitor<'tcx> for BumpSeed {
    fn compute_rvalue(
        rvalue: &Rvalue<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, BumpSeed>,
        src_info: SourceInfo,
    ) -> Self {
        let dummy = Default::default();
        let st = match rvalue {
            Rvalue::Use(op) => Self::compute_operand(op, body, tcx, sm, src_info),
            Rvalue::Ref(_, _, place) => Self::compute_place(place.as_ref(), body, tcx, sm, Some(src_info)),
            Rvalue::Cast(_, op, _ty) => Self::compute_operand(op, body, tcx, sm, src_info),
            Rvalue::Aggregate(box _kind, elements) => {
                // debug!("Process aggregate: {:?}", elements);
                let mut external = false;
                // If one element is bump from external, the entire array is marked.
                for element in elements {
                    let st = Self::compute_operand(element, body, tcx, sm, src_info);
                    // debug!("Aggregate element: {:?}, state: {:?}", element, st);
                    if st.external && st.is_bump {
                        return st;
                    }
                    external |= st.external;
                }
                return Self {
                    external,
                    is_bump: false,
                };
            }
            _ => dummy,
        };
        // debug!("Rvalue: {:?}, state: {:?}", rvalue, st);
        st
    }

    fn compute_operand(
        op: &Operand<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, BumpSeed>,
        src_info: SourceInfo,
    ) -> Self {
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
        sm: &mut StateMachine<'tcx, BumpSeed>,
        src_info: Option<SourceInfo>,
    ) -> Self {
        let ty = place.ty(body, tcx);
        let external_from_downcast = has_downcast_continue(place);
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
            // debug!("Fetched place: {:?}, state: {:?}", place, st);
            let is_bump = is_bump_var(place, body);
            let st = Self {
                external: st.external || external_from_downcast,
                is_bump: is_bump | st.is_bump,
            };
            // debug!("Place: {:?}, state: {:?}", place, st);
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
            external_from_downcast
        };
        let is_bump = is_bump_var(place, body);
        let st = Self { external, is_bump };
        // debug!("Place: {:?}, state: {:?}", place, st);
        st
    }

    fn init_place(
        place: PlaceRef<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, BumpSeed>,
    ) -> Self {
        Self::compute_place(place, body, tcx, sm, None)
    }

    fn merge(
        &self,
        other: Self,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, BumpSeed>,
    ) -> Self {
        Self {
            external: other.external,
            is_bump: other.is_bump,
        }
    }

    fn compute_terminator(
        terminator: &Terminator<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, BumpSeed>,
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
                    if func_name == CPA {
                        assert_eq!(args.len(), 2);
                        let seeds = &args[0];
                        let st = Self::compute_operand(seeds, body, tcx, sm, src_info);
                        debug!("State in CPA: {:?}", st);
                        if st.is_bump && st.external {
                            warn!(
                                "Bump seed canonicalization issue found in function: {}!",
                                tcx.def_path_str(body.source.def_id())
                            );
                            let call_stack = Report::call_stack_formatter(tcx, sm.get_call_stack());
                            Report::new_bug(
                                tcx,
                                VulnerabilityType::BumpSeed,
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
                    } else if func_name == DESERIALIZE {
                        // Any deserialized value is not safe
                        if destination.is_some() {
                            return Self {
                                external: true,
                                is_bump: false,
                            };
                        }
                    }
                }
            }
        }
        Default::default()
    }
}
