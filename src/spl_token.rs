use either::Either;
use hashbrown::HashMap;
use log::{debug, error, warn};
use rustc_hir::def::DefKind;
use rustc_middle::{
    mir::{visit::Visitor, Body, Constant, Operand, PlaceRef, VarDebugInfoContents, Place, BasicBlock},
    ty::{TyCtxt, TyKind},
};
use rustc_span::Symbol;
use std::env;
use cargo_lock::Lockfile;
use cargo_lock::Version;
use semver::Prerelease;
use semver::BuildMetadata;

use crate::reporter;

use crate::{
    dd::{self, HeuristicFilter, ASSERT_MACRO_FILTER},
    visit::{self, DefUseChainTrait}, accounts::get_accounts,
    
};
//use crate::visit::{self, DefUseChainTrait};

static mut cur_version: cargo_lock::Version = cargo_lock::Version{
    major: 0,
    minor: 0,
    patch: 0,
    pre: Prerelease::EMPTY,
    build: BuildMetadata::EMPTY
};

pub enum AssertionOperands {
    Unary(u32),
    Binary(u32, u32),
}


pub enum KeyStmt<'tcx> {
    AssertionStmt {
        assertion_call_name: &'static str,
        op_arg_index: u32,
        elements: AssertionOperands,
        args: Vec<Operand<'tcx>>,
    },

    BorrowMutStmt {
        arg: Operand<'tcx>
    },

    SplCallStmt {
        spl_call_name: &'static str,
        elements: AssertionOperands,
        args: Vec<Operand<'tcx>>,
    }
}

impl<'tcx> KeyStmt<'tcx> {
    pub fn get_op(&self) -> Option<&Operand<'tcx>> {
        match self {
            KeyStmt::AssertionStmt {op_arg_index: idx, args, ..} => {
                Some(&args[*idx as usize])
            }
            _ => None
        }
    }

    pub fn get_elements(&self) -> Vec<&Operand<'tcx>> {
        let mut res = Vec::new();
        match self {
            KeyStmt::AssertionStmt {elements, args, ..} => {
                match elements {
                    &AssertionOperands::Unary(e1) => {
                        res.push(&args[e1 as usize]);
                    }
                    &AssertionOperands::Binary(e1, e2) => {
                        res.push(&args[e1 as usize]);
                        res.push(&args[e2 as usize]);
                    }
                }
            }
            KeyStmt::BorrowMutStmt { arg } => {
                res.push(arg);
            }
            KeyStmt::SplCallStmt {elements, args, ..} => {
                match elements {
                    &AssertionOperands::Unary(e1) => {
                        res.push(&args[e1 as usize]);
                    }
                    &AssertionOperands::Binary(e1, e2) => {
                        res.push(&args[e1 as usize]);
                        res.push(&args[e2 as usize]);
                    }
                }
            }
        }
        res
    }
}


fn check_key_calls<'tcx, 'a>(
    tcx: TyCtxt<'tcx>,
    func: &Operand<'tcx>,
    args: &Vec<Operand<'tcx>>,
    _dest: &Option<(Place<'tcx>, BasicBlock)>,
    key_stmts: &mut Vec<KeyStmt<'tcx>>,
) {
    match func {
        Operand::Copy(_) | Operand::Move(_) => {
            error!("Indirect call to assertions are not supported!");
        }
        Operand::Constant(box c) => {
            match c.ty().kind() {
                // Only consider explicit assertion call
                TyKind::FnDef(def_id, _) => {
                    let call_name = tcx.def_path_str(*def_id);
                    // if call_name.contains("deref") {
                    //     debug!("Deref: {}, {:?}", call_name, func);
                    // }
                    // debug!("call: {}, {:?}", call_name, func);
                    match call_name.as_str() {
                        "core::panicking::assert_failed" => {
                            key_stmts.push(KeyStmt::AssertionStmt {
                                assertion_call_name: "core::panicking::assert_failed",
                                op_arg_index: 0,
                                elements: AssertionOperands::Binary(1, 2),
                                args: args.clone(),
                            });
                        }
                        "std::cell::RefCell::<T>::borrow_mut" => {
                            assert!(args.len() == 1);
                            key_stmts.push(KeyStmt::BorrowMutStmt {
                                arg: args[0].clone()
                            });
                        }
                        "vendored_spl_token::instruction::transfer_checked" => {
                            key_stmts.push(KeyStmt::SplCallStmt {
                                spl_call_name: "vendored_spl_token::instruction::transfer_checked",
                                elements: AssertionOperands::Unary(0),
                                args: args.clone(),
                            });
                        }
                        _ => {
                            //debug!("call: {}", call_name.as_str());
                            if call_name.as_str().contains("spl_token::instruction::transfer") {
                                key_stmts.push(KeyStmt::SplCallStmt {
                                    spl_call_name: "spl_token::instruction::transfer",
                                    elements: AssertionOperands::Unary(0),
                                    args: args.clone(),
                                });
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn get_local_vars<'tcx>(_tcx: TyCtxt<'tcx>, body: &Body<'tcx>) -> HashMap<PlaceRef<'tcx>, usize> {
    let mut vars_map = HashMap::new();
    for (idx, var_dbg_info) in body.var_debug_info.iter().enumerate() {
        match var_dbg_info.value {
            VarDebugInfoContents::Place(place) => {
                vars_map.insert(place.as_ref(), idx);
            }
            _ => {}
        }
    }
    vars_map
}

fn check_spl_version() -> bool {
    let args: Vec<String> = env::args().collect();
    let key = "PWD";
    match env::var(key) {
        Ok(val) => {
            // val is String, splited by ";"
            debug!("pwd val =>{}",val);
            let mut pwd = val.clone();
            let cargo_dir = pwd + "/../../Cargo.lock";
            if std::path::Path::new(&cargo_dir).exists() {
                let lockfile = Lockfile::load(cargo_dir).unwrap();
                debug!("number of dependencies: {}", lockfile.packages.len());
                for package in lockfile.packages {
                    if package.name.as_str().contains("spl-token")  {
                        debug!("spl_token crate version: {:?}", package.version);
                        let version = package.version.clone();
                        unsafe {cur_version = version.clone();}
                        if version.major > 3 || (version.major == 3 && version.minor > 1) || (version.major == 3 && version.minor == 1 && version.patch >=1)
                            {
                                return true 
                            }
                        else
                            {
                                warn!("Unsafe version of spl token crate version used, which may cause security flaws if not handled carefully.");
                                return false 
                            }
                    }
                }
            }
            else {
                //debug!("no such directory");
                return true
            }
        },
        Err(e) => println!("couldn't interpret {}: {}", key, e),
    }
    true
}

pub fn check_spl_tokens<'tcx>(tcx: TyCtxt<'tcx>) {
    let mut safe_version = false;
    safe_version = check_spl_version();
    if !safe_version {
        let mut spl_token_checked = true;
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
                        f: Some(check_key_calls),
                        defs: HashMap::default(),
                        focus: Vec::new(),
                        switchint: Vec::new(),
                    };
                    let heuristic = ASSERT_MACRO_FILTER.clone();
                    let ddg = dd::DataDepsGraph::new(body, tcx, heuristic);
                    visitor.visit_body(body);

                    let dbg_info = get_local_vars(tcx, body);
                    let mut sensitive_vars = Vec::new();
                    let mut assertions = Vec::new();
                    for assert_stmt in visitor.focus.iter() {
                        let op = assert_stmt.get_op();
                        let args = assert_stmt.get_elements();
                        match assert_stmt {
                            KeyStmt::AssertionStmt{..} => {
                                //debug!("Function: {}, kind: {:?}, args: {:?}", tcx.def_path_str(def_id), op, args);
                                let op = op.unwrap();
                                if let Operand::Copy(op_place) | Operand::Move(op_place) = op {
                                    let mut op_from = visitor.get_def(op_place.as_ref(), body, &dbg_info);
                                    // debug!("Function: {}, kind: {:?}, args: {:?}", tcx.def_path_str(def_id), op_from, args);
                                    let mut args_from = Vec::new();
                                    for arg in args {
                                        if let Operand::Copy(arg_place) | Operand::Move(arg_place) = arg {
                                            let mut arg_from =
                                                visitor.get_def(arg_place.as_ref(), body, &dbg_info);
                                            if arg_from.len() > 0 {
                                                args_from.push(arg_from.remove(0));
                                            }
                                        }
                                    }
                                    debug!(
                                        "Function {}, kind: {:?}, args: {:?}",
                                        tcx.def_path_str(def_id),
                                        op_from,
                                        args_from
                                    );
                                    assert_eq!(op_from.len(), 1);
                                    let var_path = tcx.def_path_str(def_id);
                                    assertions.push((op_from.remove(0).right().unwrap(), args_from, var_path));
                                    // let op_value = op_from.clone();
                                    // if op_value[0].is_right() {
                                    //     let kind = op_value[0].clone().right().unwrap();
                                    //     if kind.to_string() == "const core::panicking::AssertKind::Eq" {
                                    //         let checked = args_from[0].clone().left().unwrap();
                                    //         let checked_var = checked.0.clone();
                                    //         let checked_field = checked.1.clone()[0];
                                    //         if checked_var.to_ident_string() == "spl_token" && checked_field.to_ident_string() == "key" {
                                    //             let assert_def = tcx.def_path_str(def_id).clone();
                                    //             if assert_def == "processor::withdraw" {
                                    //                 spl_token_checked = true;
                                    //             }
                                    //         }
                                    //     }
                                    // }
                                }
                            }
                            KeyStmt::BorrowMutStmt{..} => {
                                let arg = args[0];
                                if let Operand::Copy(arg_place) | Operand::Move(arg_place) = arg {
                                    let backtracked = visitor.get_def(arg_place.as_ref(), body, &dbg_info);
                                    //debug!("Borrowmut: {:?}, from: {:?}", arg, backtracked);
                                }
                            }
                            KeyStmt::SplCallStmt{..} => {
                                let arg = args[0];
                                if let Operand::Copy(arg_place) | Operand::Move(arg_place) = arg {
                                    let backtracked = visitor.get_def(arg_place.as_ref(), body, &dbg_info);
                                    debug!("Splcall: {:?}, from: {:?}", arg, backtracked);
                                    if let Some(var_fields) = backtracked.get(0) {
                                        if let Some((var, fields)) = var_fields.as_ref().left() {
                                            if fields.len() == 1 && fields[0].as_str() == "key" {
                                                let var_path = tcx.def_path_str(def_id);
                                                if var_path == "processor::withdraw" {
                                                    sensitive_vars.push(var.clone().to_ident_string());
                                                }
                                            }
                                        }
                                    }
                                    else {
                                        let spl_token_field = ddg.origin_op_source(arg, false);
                                        let spl_token_source = ddg.origin_op_source(arg, true);
                                        //debug!("++ visitor.switchint original source = {:#?}", spl_token_field);
                                        let mut posi = 100000;
                                        if !spl_token_field.is_none() {
                                            if let Some(dd::DdNode::PartialFieldRef{base}) = spl_token_field {
                                                if base.projection.len() > 0 {
                                                    if let rustc_middle::mir::ProjectionElem::Field(Field, T) = base.projection[0] {
                                                        posi = Field.as_usize();
                                                    }
                                                    else {
                                                        debug!("++ visitor.switchint Field = None");
                                                    }
                                                }
                                            }
                                        }
                                        if let Some(dd::DdNode::LocalDecl{local, ty, dbg_info, ..}) =  spl_token_source {
                                            if ty.to_string().contains("solana_program::account_info::AccountInfo") {
                                                if dbg_info-1>0 && ddg.var_dbg_info.len()> dbg_info-1 {
                                                    let mut VarDeg = &ddg.var_dbg_info[dbg_info-1].clone();
                                                    if posi == 0 {
                                                        sensitive_vars.push(VarDeg.name.to_string().clone());
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // debug!("sensitive var: {:?}", sensitive_vars);
                    // debug!("assertions: {:?}", assertions);
                    //if tcx.def_path_str(def_id) == "processor::withdraw" {
                        spl_token_checked = false;
                        for assertion in assertions {
                            let op_value = assertion.0.clone();
                            if op_value.to_string() == "const core::panicking::AssertKind::Eq" && assertion.1.clone().len() > 0 {
                                let checked = assertion.1.clone()[0].clone().left().unwrap();
                                let checked_var = checked.0.clone();
                                let checked_field = checked.1.clone()[0];
                                if sensitive_vars.len() > 0 {
                                    if checked_var.to_ident_string() == sensitive_vars[0].clone().to_string() {
                                        if checked_field.to_ident_string() == "key" {
                                            if assertion.2 == "processor::withdraw" {
                                                spl_token_checked = true;
                                            }
                                        }
                                    }
                                }
                                
                            }
                        }
                    //}
                }
                _ => {}
            }
        }
        if !spl_token_checked {
            //error!("spl_token program id not checked and an old version of spl token checked, may cause arbitary program invocation.");
            unsafe {
                let cur_version_str = cur_version.major.to_string() + "." + &cur_version.minor.to_string() + "." + &cur_version.patch.to_string();
                // reporter::Report::new_vuln_crate_dep("spl-token", &cur_version_str, "3.1.1", None);
                reporter::Report::new_vuln_crate_dep(
                    tcx,
                    "spl-token error".to_string(),
                    "Critical".to_string(), 
                    "SPL token version 3.1.1".to_string(), 
                    "".to_string(),
                    "Callstack".to_string(),
                    "UnResolved".to_string(),
                    "GitHub Link to be added.".to_string(),
                    // body.span, 
                    Some("Description of the bug here."), 
                    "Some alleviation steps here.".to_string(),
                    "spl-token",
                    &cur_version_str, 
                    "3.1.1", 

                );
            }
        }
    }
}