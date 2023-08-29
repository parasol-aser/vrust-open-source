use std::{fmt::Display, ops::Index};

use hashbrown::{HashMap, HashSet};
use log::{debug, warn};
use regex::Regex;
use rustc_hir::{def::DefKind, def_id::DefId};
use rustc_middle::{
    mir::{
        traversal::reverse_postorder, Body, Operand, Place, PlaceRef, Rvalue, SourceInfo,
        Statement, StatementKind, Terminator, TerminatorKind,
    },
    ty::{Ty, TyCtxt, TyKind},
};

use crate::{
    dd::{self, DataDepsGraph, ASSERT_MACRO_FILTER},
    reporter::VulnerabilityType,
};

use crate::wpa::{self, FullPlaceRef, StateMachine, StateTransitor};

use crate::reporter::Report;
use crate::source_info;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct RoundChecker<'tcx> {
    pub ty: Ty<'tcx>,
    pub round: bool,
}

impl<'tcx> StateTransitor<'tcx> for RoundChecker<'tcx> {
    fn compute_rvalue(
        rvalue: &Rvalue<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
        src_info: SourceInfo,
    ) -> Self {
        let dummy = RoundChecker {
            ty: tcx.types.unit,
            round: false,
        };
        match rvalue {
            Rvalue::Use(op) => Self::compute_operand(op, body, tcx, sm, src_info),
            Rvalue::Ref(_, _, place) => {
                Self::compute_place(place.as_ref(), body, tcx, sm, Some(src_info))
            }
            Rvalue::Cast(_, op, _ty) => Self::compute_operand(op, body, tcx, sm, src_info),
            Rvalue::BinaryOp(_op, box (op1, op2)) => {
                let st1 = Self::compute_operand(op1, body, tcx, sm, src_info);
                let st2 = Self::compute_operand(op2, body, tcx, sm, src_info);
                Self {
                    ty: rvalue.ty(body, tcx),
                    round: st1.round || st2.round,
                }
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
        //debug!("PrecisionChecker--compute_operand: {:?}", op);
        match op {
            Operand::Copy(from) | Operand::Move(from) => {
                Self::compute_place(from.as_ref(), body, tcx, sm, Some(src_info))
            }
            Operand::Constant(_from) => RoundChecker {
                ty: tcx.types.unit,
                round: false,
            },
        }
    }

    fn compute_place(
        place: PlaceRef<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
        src_info: Option<SourceInfo>,
    ) -> Self {
        //debug!("IsSignerChecker--compute_place: {:?}", place);
        if let Some(st) = sm.get_state(FullPlaceRef::new(body.source.def_id(), place)) {
            return *st;
        }

        let dummy = RoundChecker {
            ty: tcx.types.unit,
            round: false,
        };

        return dummy;
    }

    fn compute_terminator(
        terminator: &Terminator<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
        src_info: SourceInfo,
    ) -> Self {
        // todo
        // Self { // todo: now only place holder
        //     ty: tcx.types.unit,
        //     round: true,
        // }
        // debug!("terminator: {:?}, kind: {:?}", terminator, &terminator.kind);
        match &terminator.kind {
            TerminatorKind::Call {
                func,
                args,
                destination,
                ..
            } => {
                // debug!("func: {:?}", func);
                // Self::compute_operand(func, body, tcx, sm);
                match func {
                    Operand::Constant(from) => {
                        if from.to_string() == "std::f64::<impl f64>::round"
                            || from.to_string() == "std::f32::<impl f32>::round" {
                            //report error
                            let call_stack = Report::call_stack_formatter(tcx, sm.get_call_stack());
                            Report::new_bug(
                                tcx,
                                VulnerabilityType::Precision,
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
                            )
                        }
                    }

                    _ => {}
                }
            }

            _ => {
                //debug!("terminator: {:?}, kind: {:?}", terminator, &terminator.kind);
            }
        }

        Self {
            // todo: now only place holder
            ty: tcx.types.unit,
            round: true,
        }
    }

    fn init_place(
        place: PlaceRef<'tcx>,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
    ) -> Self {
        Self {
            // todo: now only place holder
            ty: tcx.types.unit,
            round: false,
        }
    }

    fn merge(
        &self,
        other: Self,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
        sm: &mut StateMachine<'tcx, Self>,
    ) -> Self {
        Self {
            // todo: now only place holder
            ty: tcx.types.unit,
            round: false,
        }
    }
}
