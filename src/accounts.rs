//! Get accounts list required by each function and build
//! a graph recording their relationship, e.g., a.authority == b.

use hashbrown::HashMap;
use petgraph::graph::{DiGraph, NodeIndex};
use rustc_hir::{def::DefKind, ItemId, ItemKind};
use rustc_middle::{ty::{TyCtxt, TyKind, subst::{GenericArgKind, SubstsRef}, Ty, AdtDef}, mir::{Body, traversal::reverse_postorder, TerminatorKind}};
use log::debug;
use rustc_span::{symbol::Ident, Symbol};

pub const ACCOUNT_TY: &'static str = "anchor_lang::Account";

#[derive(Debug)]
pub struct CtxWrappedAccounts<'tcx> {
    pub adt_def: &'tcx AdtDef,
    pub fields: Vec<(Ty<'tcx>, Ident, Option<&'tcx AdtDef>)>
}

fn get_last_generic_subst<'tcx>(substs: SubstsRef<'tcx>) -> Option<(&'tcx AdtDef, SubstsRef<'tcx>)> {
    if let GenericArgKind::Type(ty) = substs.last().unwrap().unpack() {
        if let TyKind::Adt(adt_def, substs) = ty.kind() {
            return Some((*adt_def, *substs));
        }
    }
    None
}

pub fn get_accounts<'tcx>(tcx: TyCtxt<'tcx>, body: &Body<'tcx>) -> Vec<CtxWrappedAccounts<'tcx>> {
    let mut accounts = Vec::new();
    if body.arg_count >= 1 {
        let first_arg = &body.local_decls[1u32.into()];
        if let TyKind::Adt(adt_def, substs) = first_arg.ty.kind() {
            if tcx.def_path_str(adt_def.did).contains("Context") {
                // debug!("Arg type: {:?}, subst: {:?}", tcx.def_path_str(adt_def.did), substs);
                // Context has 4 lifetime generic parameter and one normal generic parameter
                assert_eq!(substs.len(), 5);
                if let Some((user_adt_def, user_ty_substs)) = get_last_generic_subst(substs) {
                    let mut fields = Vec::new();
                    for field in user_adt_def.all_fields() {
                        let field_ty = field.ty(tcx, user_ty_substs);
                        let data_ty = if let TyKind::Adt(field_adt_def, field_substs) = field_ty.kind() {
                            if tcx.def_path_str(field_adt_def.did) == ACCOUNT_TY {
                                Some(get_last_generic_subst(field_substs).unwrap().0)
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        fields.push((field_ty, field.ident(tcx), data_ty));
                    }
                    accounts.push(
                        CtxWrappedAccounts {
                            adt_def: user_adt_def,
                            fields,
                        }
                    );
                }
            }
        }
    }
    print_all_wrapped_accounts(tcx, &accounts);
    accounts
}

/// This is for debug purpose.
#[allow(unused)]
pub fn print_all_wrapped_accounts<'tcx>(tcx: TyCtxt<'tcx>, account_adts: &[CtxWrappedAccounts<'tcx>]) {
    for account_adt in account_adts {
        let adt_name = tcx.def_path_str(account_adt.adt_def.did);
        debug!("Def: {}", adt_name);
        for (_, field_ident, field_adt_def) in account_adt.fields.iter() {
            debug!("\t Field: {}, account data (if some): {:?}", field_ident.as_str(), field_adt_def);
        }
        
        find_impls_for(tcx, account_adt.adt_def);
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum AccountRole<'tcx> {
    Onwer(Ident, Option<&'tcx AdtDef>),
    Signer(Ident, Option<&'tcx AdtDef>),
    Sender(Vec<(Ident, Option<&'tcx AdtDef>)>),
    Receiver(Vec<(Ident, Option<&'tcx AdtDef>)>),
}

/// In anchor Context<T>, each T corresponds to a function, e.g., deposit, withdraw, etc.
/// Each field in T has some kind of role in the function, e.g., owner, signer, receiver, etc.
/// For each function, we can built a graph to model the relationship between different accounts.
/// If the account names are reliable, we can connect the graphs for every function together and
/// check if constraints in other functions are repsected in a function.
pub struct AccountsModel<'tcx> {
    pub g: DiGraph<AccountRole<'tcx>, ()>,
    nodes_map: HashMap<AccountRole<'tcx>, NodeIndex>
}

// pub const OWNER_LIE: &'static str = "owner_account";

const ACCOUNTS_TRAIT: &'static str = "anchor_lang::Accounts";
/// The constraints specified in T with derive syntax can be obtained in 
/// anchor_lang::Accounts::try_accounts (as well as try_accounts called in it).
/// Therefore, we need to find all impls of this trait for each T.
pub fn find_impls_for<'tcx>(tcx: TyCtxt<'tcx>, adt_def: &'tcx AdtDef) {
    for (trait_id, impls) in tcx.all_local_trait_impls(()) {
        if tcx.def_path_str(*trait_id) == ACCOUNTS_TRAIT {
            // debug!("Trait {}", tcx.def_path_str(*trait_id));
            for local_def_id in impls {
                let def_id = local_def_id.to_def_id();
                if tcx.type_of(def_id).ty_adt_def().unwrap() == adt_def {
                    for item in tcx.associated_items(def_id).filter_by_name_unhygienic(Symbol::intern("try_accounts")) {
                        let body = tcx.optimized_mir(item.def_id);
                        collect_constraints(tcx, body);
                    }
                }
                // if let DefKind::Impl = tcx.def_kind(def_id) {
                //     debug!("{:?}", tcx.def_path_str(def_id));
                //     let item = tcx.hir().item(ItemId{def_id: *local_def_id});
                //     // debug!("Item {:#?}", item);
                //     if let ItemKind::Impl(imp) = &item.kind {
                //         // debug!("Target: {:?}", tcx.def_path_str(imp.self_ty.hir_id.owner.to_def_id()));
                //         if tcx.type_of(def_id).ty_adt_def().unwrap() == adt_def {

                //         }
                //     }
                // }
            }
        }
    }
}

/// For constraints expressed in "constraints = xx op val ...",
/// we need to check SwitchInt terminator and the corresponding
/// binary expressions that lead to the operand in SwithInt.
/// For other constraints in anchor Accounts, such as "seeds = ...",
/// we may have to handle calls like "anchor_lang::solana_program::program::invoke_signed"
fn collect_constraints<'tcx>(tcx: TyCtxt<'tcx>, body: &Body<'tcx>) {
    for (_, bb_data) in reverse_postorder(body) {
        let terminator = bb_data.terminator();
        match &terminator.kind {
            TerminatorKind::SwitchInt {discr, ..} => {
                // debug!("{:?}", terminator);
            }
            _ => {}
        }
    }
}

impl<'tcx> AccountsModel<'tcx> {
    pub fn build(tcx: TyCtxt<'tcx>, account_def: CtxWrappedAccounts<'tcx>) -> Self {
        let g = DiGraph::new();
        let mut nodes_map = HashMap::new();
        for field in account_def.fields {
        }
        
        Self {
            g,
            nodes_map
        }
    }
}
