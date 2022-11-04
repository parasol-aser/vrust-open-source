//! Iterate all struct types in the current smart contracts.
//! If there are two types that can be confused (has similar layout),
//! and can be used in sensitive functions.

use std::collections::BTreeMap;

use crate::{
    reporter::{Report, VulnerabilityType},
    source_info, visit,
};
use hashbrown::HashMap;
use itertools::Itertools;
use log::{debug, warn};
use rustc_hir::{def::DefKind, def_id::DefId, itemlikevisit};
use rustc_middle::{
    mir::{visit::Visitor, BasicBlock, Operand, Place, PlaceRef},
    ty::{AdtDef, List, ParamEnv, ParamEnvAnd, Ty, TyCtxt, TyKind},
};
use rustc_span::Span;

pub struct TypeVisitor<'tcx> {
    pub all_struct_defs: Vec<(&'tcx AdtDef, Span)>,
    pub tcx: TyCtxt<'tcx>,
}

impl<'hir, 'tcx> itemlikevisit::ItemLikeVisitor<'hir> for TypeVisitor<'tcx> {
    fn visit_item(&mut self, item: &'hir rustc_hir::Item<'hir>) {
        let def_id = item.def_id.to_def_id();
        match self.tcx.def_kind(def_id) {
            DefKind::Struct => {
                let adt_def = self.tcx.adt_def(def_id);
                if adt_def.is_struct() {
                    if adt_def.all_fields().count() > 0 {
                        self.all_struct_defs.push((adt_def, item.span));
                        // debug!("Def code: {}", self.tcx.sess.source_map().span_to_snippet(item.span).unwrap());
                    }
                }
            }
            _ => {}
        }
    }

    fn visit_trait_item(&mut self, _trait_item: &'hir rustc_hir::TraitItem<'hir>) {}

    fn visit_impl_item(&mut self, _impl_item: &'hir rustc_hir::ImplItem<'hir>) {}

    fn visit_foreign_item(&mut self, _foreign_item: &'hir rustc_hir::ForeignItem<'hir>) {}
}

/// Get all struct types in the current crate.
fn get_mod_struct_types<'tcx>(tcx: TyCtxt<'tcx>) -> Vec<(&'tcx AdtDef, Span)> {
    let mut ty_visitor = TypeVisitor {
        all_struct_defs: Vec::new(),
        tcx,
    };
    tcx.hir().visit_all_item_likes(&mut ty_visitor);
    ty_visitor.all_struct_defs
}

#[derive(Debug)]
pub struct StructDefLayout<'tcx> {
    pub struct_def: &'tcx AdtDef,
    pub layout: Vec<Ty<'tcx>>,
    pub span: Span,
}

/// Find all possible user defined confusion types.
/// 1. Get all structs' layout (type of each field).
/// 2. For struct with k fields, check all structs with n (n >= k) fields, see
///    if each of the k fields has the same bytes as the first k fields in the n fields.
/// 3. If there are structs that have the same memory layout (or one struct memory layout
///    is identical to part of another one), we check if it's possible for them to be used
///    in sensitive functions (e.g., one of such struct object is obtained from deserialization).
///
/// NOTE: usually memory layout information is not unstable in Rust, but in solana,
/// every data structs are marked as #[repr(C)], so they have determined memory layout as C.
///
/// UPDATE: In Anchor, e.g., Account<'a, Wallet>, the deserialization of Wallet is done by
/// anchor_lang::AccountDeserialize::try_deserialize. But in this function, it will check
/// type mismatch. So we should not report any such issue. In addition, we
fn find_confusion_types<'tcx>(
    tcx: TyCtxt<'tcx>,
    all_struct_defs: &[(&'tcx AdtDef, Span)],
) -> BTreeMap<usize, Vec<StructDefLayout<'tcx>>> {
    // Get all structs field layout
    let mut layouts = BTreeMap::<usize, Vec<StructDefLayout<'tcx>>>::new();
    for &(struct_def, span) in all_struct_defs {
        let mut layout = Vec::new();
        for field in struct_def.all_fields() {
            // let ty = field.ty(tcx, List::empty());
            // FIXME: type_of will not do substs, will this cause problem?
            let ty = tcx.type_of(field.did);
            layout.push(ty);
        }
        layouts
            .entry(layout.len())
            .or_default()
            .push(StructDefLayout {
                struct_def,
                layout,
                span,
            });
    }
    layouts
}

fn is_layout_compatible<'tcx>(
    tcx: TyCtxt<'tcx>,
    s1: &StructDefLayout<'tcx>,
    s2: &StructDefLayout<'tcx>,
) -> bool {
    assert!(s1.layout.len() <= s2.layout.len());
    let k = s1.layout.len();
    for i in 0..k {
        let s1_fi = s1.layout[i];
        let s2_fi = s2.layout[i];
        let s1_fi_layout = tcx.layout_of(ParamEnvAnd {
            param_env: ParamEnv::reveal_all(),
            value: s1_fi,
        });
        let s2_fi_layout = tcx.layout_of(ParamEnvAnd {
            param_env: ParamEnv::reveal_all(),
            value: s2_fi,
        });
        if let (Ok(s1_fi_layout), Ok(s2_fi_layout)) = (s1_fi_layout, s2_fi_layout) {
            if s1_fi_layout.layout.size != s2_fi_layout.layout.size {
                return false;
            }
        }
    }
    true
}

#[derive(Debug)]
struct BorshDeserializeStmt<'tcx> {
    _from: Operand<'tcx>,
    dest: PlaceRef<'tcx>,
}

fn check_deserializations<'tcx>(
    tcx: TyCtxt<'tcx>,
    func: &Operand<'tcx>,
    args: &Vec<Operand<'tcx>>,
    dest: &Option<(Place<'tcx>, BasicBlock)>,
    stmts: &mut Vec<BorshDeserializeStmt<'tcx>>,
) {
    match func {
        Operand::Copy(_) | Operand::Move(_) => {}
        Operand::Constant(box c) => match c.ty().kind() {
            TyKind::FnDef(def_id, _) => {
                let call_name = tcx.def_path_str(*def_id);
                if call_name.as_str() == "borsh::BorshDeserialize::deserialize"
                    || call_name.as_str() == "anchor_lang::AnchorDeserialize::try_from_slice"
                {
                    // || call_name.as_str() == "anchor_lang::AccountDeserialize::try_deserialize_unchecked"
                    // || call_name.as_str() == "anchor_lang::AccountDeserialize::try_deserialize" {
                    assert_eq!(args.len(), 1);
                    stmts.push(BorshDeserializeStmt {
                        _from: args[0].clone(),
                        dest: dest.unwrap().0.as_ref(),
                    });
                }
            }
            _ => {}
        },
    }
}

/// Check if any type in the array of compatible is used in a function of
/// the smart contracts, and that use involves deserialization, in which
/// type confusion is possible.
fn check_uses<'tcx>(tcx: TyCtxt<'tcx>, compatible: &[&StructDefLayout<'tcx>]) -> Vec<DefId> {
    let mut used_in = Vec::new();
    for item_id in tcx.mir_keys(()) {
        let def_id = item_id.to_def_id();
        match tcx.def_kind(def_id) {
            DefKind::Fn | DefKind::AssocFn => {
                if !tcx.is_mir_available(def_id) {
                    continue;
                }
                let body = tcx.optimized_mir(def_id);
                let mut visitor = visit::BodyVisitor {
                    tcx,
                    f: Some(check_deserializations),
                    defs: HashMap::default(),
                    switchint: Vec::new(),
                    focus: Vec::new(),
                };
                visitor.visit_body(body);

                for stmt in visitor.focus.iter() {
                    // debug!("Deserialization stmt: {:?}", stmt);
                    let ty = stmt.dest.ty(body, tcx).ty;
                    match ty.kind() {
                        TyKind::Adt(adt_def, substs) => {
                            // The return of deserialization is always Result.
                            if tcx.def_path_str(adt_def.did) == "std::result::Result" {
                                // debug!("Substs: {:?}, candidates: {:?}", substs, compatible);
                                for subst in substs.iter() {
                                    if let Some(subst_adt_def) = subst.expect_ty().ty_adt_def() {
                                        let used = compatible
                                            .iter()
                                            .any(|&x| x.struct_def == subst_adt_def);
                                        if used {
                                            used_in.push(def_id);
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    used_in
}

pub fn check_confusion_types<'tcx>(tcx: TyCtxt<'tcx>) {
    let all_types = get_mod_struct_types(tcx);
    // debug!("All struct types: {:?}", all_types);
    let confusion_types = find_confusion_types(tcx, all_types.as_slice());
    for (fields_num, structs) in confusion_types.iter() {
        // debug!("All struct types with {} fields: {:?}", fields_num, structs);
        for i in 0..structs.len() {
            let st = &structs[i];
            let mut compatible = Vec::new();
            compatible.push(st);
            for j in i + 1..structs.len() {
                if is_layout_compatible(tcx, st, &structs[j]) {
                    compatible.push(&structs[j]);
                }
            }

            for (_, larger_structs) in confusion_types.range(fields_num + 1..) {
                for larger_st in larger_structs {
                    if is_layout_compatible(tcx, st, larger_st) {
                        compatible.push(larger_st);
                    }
                }
            }

            // At least 2 compatible types.
            if compatible.len() >= 2 {
                // debug!("Check potential confusion types: {:?}", compatible);
                let used_in = check_uses(tcx, compatible.as_slice());
                if !used_in.is_empty() {
                    let func_list = used_in
                        .into_iter()
                        .map(|id| tcx.def_path_str(id))
                        .join(", ");
                    let msg = format!(
                        "Function {} is vulnerable to type confusion attacks: {:?}!",
                        func_list, compatible
                    );
                    warn!("{}", msg);
                    let callstack = "".to_string();
                    // Report::new_type_confusion::<&str>(tcx, compatible.as_slice(), None);
                    let code = source_info::get_type_defs_src(tcx, compatible.as_slice());
                    Report::new_bug(
                        tcx,
                        VulnerabilityType::TypeConfusion,
                        "Critical".to_string(),
                        func_list,
                        code,
                        callstack,
                        "UnResolved".to_string(),
                        "GitHub Link to be added.".to_string(),
                        None,
                        None,
                        None,
                    );
                }
            }
        }
    }
}
