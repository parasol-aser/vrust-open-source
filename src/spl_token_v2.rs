use either::Either;
use hashbrown::HashMap;
use log::{debug, error, warn};
use rustc_hir::def::DefKind;
use rustc_middle::{
    mir::{visit::Visitor, Body, Constant, Operand, PlaceRef, VarDebugInfoContents, Place, BasicBlock, Terminator, TerminatorKind},
    ty::{TyCtxt, TyKind},
};
use rustc_span::Symbol;
use rustc_span::def_id::LocalDefId;
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

#[derive(Debug, std::clone::Clone)]
pub enum CallOperands {
    Unary(u32),
    Binary(u32, u32),
}

#[derive(Debug, std::clone::Clone)]
pub enum KeyStmt<'tcx> {
    AssertionStmt {
        assertion_call_name: &'tcx str,
        op_arg_index: u32,
        elements: CallOperands,
        args: Vec<Operand<'tcx>>,
    },

    SplCallStmt {
        spl_call_name: &'tcx str,
        elements: CallOperands,
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
                    &CallOperands::Unary(e1) => {
                        res.push(&args[e1 as usize]);
                    }
                    &CallOperands::Binary(e1, e2) => {
                        res.push(&args[e1 as usize]);
                        res.push(&args[e2 as usize]);
                    }
                }
            }

            KeyStmt::SplCallStmt {elements, args, ..} => {
                match elements {
                    &CallOperands::Unary(e1) => {
                        res.push(&args[e1 as usize]);
                    }
                    &CallOperands::Binary(e1, e2) => {
                        res.push(&args[e1 as usize]);
                        res.push(&args[e2 as usize]);
                    }
                }
            }
        }
        res
    }
}


fn check_spl_version() -> bool {
    let args: Vec<String> = env::args().collect();
    let key = "PWD";
    match env::var(key) {
        Ok(val) => {
            // val is String, splited by ";"
            // debug!("pwd val =>{}", val);
            let mut pwd = val.clone();
            let mut levels = 0;
            for c in pwd.chars() {
                if c == '/' {
                    levels += 1;
                }
            }
            for i in 0..levels {
                let last = "/..";
                let cargo_dir = pwd.clone() + &last.repeat(i) + "/Cargo.lock";
                //debug!("trying cargo dir: {}", cargo_dir);
                if std::path::Path::new(&cargo_dir).exists() {
                    let lockfile = Lockfile::load(cargo_dir).unwrap();
                    for package in lockfile.packages {
                        if package.name.as_str().contains("spl-token")  {
                            debug!("spl_token crate version: {:?}", package.version);
                            let version = package.version.clone();
                            if version.major > 3 || (version.major == 3 && version.minor > 1) || (version.major == 3 && version.minor == 1 && version.patch >=1) {
                                return true 
                            }
                            else {
                                warn!("Unsafe version of spl token crate version used, which may cause security flaws if not handled carefully.");
                                return false 
                            }
                        }
                    }
                }
                else {
                    continue;
                }
            }
        },
        Err(e) => println!("couldn't interpret {}: {}", key, e),
    }
    true
}

fn check_key_calls<'tcx, 'a>(
    tcx: TyCtxt<'tcx>,
    func: &Operand<'tcx>,
    args: &Vec<Operand<'tcx>>,
    _dest: &Option<(Place<'tcx>, BasicBlock)>,
    key_stmts: &mut Vec<KeyStmt<'tcx>>,
) {
    // debug!("spl defid: {:?}, kind", func);
    match func {
        Operand::Copy(_) | Operand::Move(_) => {
            error!("Indirect call to assertions are not supported!");
        }
        Operand::Constant(box c) => {
            //debug!("spl defid: {:?}, kind: {:?}", c, c.ty().kind());
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
                                elements: CallOperands::Binary(1, 2),
                                args: args.clone(),
                            });
                        }
                        _ => {
                            // add a function to collect all calls and check if there are spl_token instruction
                            if call_name.as_str().contains("spl_token::instruction::transfer") {
                                // debug!("spl defid: {:?}", func);
                                // debug!("call: {}", call_name.as_str());
                                key_stmts.push(KeyStmt::SplCallStmt {
                                    spl_call_name: "spl_token::instruction::transfer",
                                    elements: CallOperands::Unary(0),
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

fn intra_spl_check<'tcx>(tcx: TyCtxt<'tcx>, item_id: &LocalDefId, stmts: Vec<KeyStmt<'tcx>>) -> bool {
    let def_id = item_id.to_def_id();
    let body = tcx.optimized_mir(def_id);
    let heuristic = ASSERT_MACRO_FILTER.clone();
    let ddg = dd::DataDepsGraph::new(body, tcx, heuristic);
    let mut sensitive_vars = Vec::new();
    let mut assertions = Vec::new();
    let mut visitor = visit::BodyVisitor {
        tcx,
        f: Some(check_key_calls),
        defs: HashMap::default(),
        focus: Vec::new(),
        switchint: Vec::new(),
    };
    visitor.visit_body(body);
    let dbg_info = get_local_vars(tcx, body);
    for stmt in stmts.iter() {
        let op = stmt.get_op();
        let args = stmt.get_elements();
        match stmt {
            KeyStmt::SplCallStmt{..} => {
                let arg = args[0];
                // debug!("arg: {:?}", arg);
                let spl_token_field = ddg.origin_op_source(arg, false);
                let spl_token_source = ddg.origin_op_source(arg, true);
                // debug!("spl_token: {:?}; {:?}", spl_token_field, spl_token_source);
                let mut posi = 100000;
                if !spl_token_field.is_none() {
                    if let Some(dd::DdNode::PartialFieldRef{base}) = spl_token_field {
                        if base.projection.len() > 0 {
                            if let rustc_middle::mir::ProjectionElem::Field(Field, T) = base.projection[0] {
                                posi = Field.as_usize();
                            }
                            else if let rustc_middle::mir::ProjectionElem::Deref = base.projection[0] {
                                // debug!("pointer: {:?}", pointer);
                                let pointer_field =  ddg.origin_source(base, false);
                                // debug!("pointer field: {:?}", pointer_field);
                                if !pointer_field.is_none() {
                                    if let Some(dd::DdNode::PartialFieldRef{base}) = pointer_field {
                                        if base.projection.len() > 1 {
                                            if let rustc_middle::mir::ProjectionElem::Field(Field, T) = base.projection[1] {
                                                posi = Field.as_usize();
                                            }
                                        }
                                    }
                                }
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

            KeyStmt::AssertionStmt{..} => {
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
                }
            }

            _ => {}
        }
    }
    debug!("sensitive vars: {:?}", sensitive_vars);
    for assertion in assertions {
        let op_value = assertion.0.clone();
        if op_value.to_string() == "const core::panicking::AssertKind::Eq" {
            let checked = assertion.1.clone()[0].clone().left().unwrap();
            let checked_var = checked.0.clone();
            let checked_field = checked.1.clone()[0];
            if checked_var.to_ident_string() == sensitive_vars[0] {
                if checked_field.to_ident_string() == "key" {
                    return true;
                }
            }
        }
    }
    if sensitive_vars.len() == 0 {
        return true;
    }
    false
}

fn locate_arg<'tcx>(tcx: TyCtxt<'tcx>, item_id: &LocalDefId, stmts: Vec<KeyStmt<'tcx>>) -> u32 {
    let def_id = item_id.to_def_id();
    let body = tcx.optimized_mir(def_id);
    let heuristic = ASSERT_MACRO_FILTER.clone();
    let ddg = dd::DataDepsGraph::new(body, tcx, heuristic);
    for stmt in stmts.iter() {
        let args = stmt.get_elements();
        match stmt {
            KeyStmt::SplCallStmt{..} => {
                let arg = args[0];
                let spl_token_source = ddg.origin_op_source(arg, true);
                debug!("source: {:?}", spl_token_source);
            }

            _ => {}
        }
    }
    0
}

fn find_caller<'tcx>(tcx: TyCtxt<'tcx>, callee: &LocalDefId) -> bool { //&mut Vec<&LocalDefId> {
    //let mut callers = Vec::new();
    let callee_id_u32 = callee.to_def_id().clone().index.as_u32();
    for item_id in tcx.mir_keys(()) {
        let fn_def_id = item_id.to_def_id();
        match tcx.def_kind(fn_def_id) {
            DefKind::Fn | DefKind::AssocFn => {
                if !tcx.is_mir_available(fn_def_id) {
                    continue;
                }
                else {
                    let body = tcx.optimized_mir(fn_def_id);
                    let blocks = body.basic_blocks();
                    for block in blocks.iter() {
                        if let Some(terminator) = &block.terminator {
                            if let TerminatorKind::Call { 
                                ref func, ref args, ..
                            } = terminator.kind {
                                match func {
                                    Operand::Constant(box c) => {
                                        //debug!("spl defid: {:?}, kind: {:?}", c, c.ty().kind());
                                        match c.ty().kind() {
                                            // Only consider explicit assertion call
                                            TyKind::FnDef(def_id, _) => {
                                                let cur_id_u32 = def_id.index.as_u32();
                                                if cur_id_u32 == callee_id_u32 {
                                                    debug!("adding caller: {:?}", item_id);
                                                }
                                                
                                            }

                                            _ => {}
                                        }
                                    }

                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }

            _ => {}
        }
    }
    true
}

fn inter_spl_check<'tcx>(tcx: TyCtxt<'tcx>, item_id: &LocalDefId, stmts: Vec<KeyStmt<'tcx>>) -> bool {
    let def_id = item_id.to_def_id();
    let body = tcx.optimized_mir(def_id);
    let heuristic = ASSERT_MACRO_FILTER.clone();
    let ddg = dd::DataDepsGraph::new(body, tcx, heuristic);

    let blocks = body.basic_blocks();
    for block in blocks.iter() {
        if let Some(terminator) = &block.terminator {
            if let TerminatorKind::Call { 
                ref func, ref args, ..
            } = terminator.kind {
                //debug!("call: {:?}, args: {:?}", func, args);
            }
        }
    }
    true
}

pub fn check_spl_tokens<'tcx>(tcx: TyCtxt<'tcx>) {
    let mut safe_version = false;
    safe_version = check_spl_version();
    if !safe_version {
        // collecting all functions that include an spl_token transfer instruction
        let mut funcs_to_intra_check = Vec::new();
        for item_id in tcx.mir_keys(()) {
            let def_id = item_id.to_def_id();
            match tcx.def_kind(def_id) {
                DefKind::Fn | DefKind::AssocFn => {
                    if !tcx.is_mir_available(def_id) {
                        continue;
                    }
                    else {
                        //debug!("def_id: {:?}", def_id);
                        let body = tcx.optimized_mir(def_id);
                        let mut visitor = visit::BodyVisitor {
                            tcx,
                            f: Some(check_key_calls),
                            defs: HashMap::default(),
                            focus: Vec::new(),
                            switchint: Vec::new(),
                        };
                        visitor.visit_body(body);
                        for stmt in visitor.focus.iter() {
                            match stmt {
                                KeyStmt::SplCallStmt{..} => {
                                    debug!("adding: {:?}", item_id);
                                    funcs_to_intra_check.push((item_id, visitor.focus.clone()));
                                }
                                _ => {}
                            }
                        }
                    }
                }

                _ => {}
            }
        }
        // for func in funcs_to_intra_check.iter() {
        //     debug!("func to intra check: {:?}", func);
        // }

        // do intra-procedural check on all funcs
        let mut funcs_to_inter_check = Vec::new();
        for func in funcs_to_intra_check.iter() {
            if !intra_spl_check(tcx, func.0, func.1.clone()) {
                funcs_to_inter_check.push(func);
            }
        }

        for func in funcs_to_inter_check.iter() {
            debug!("func to inter check: {:?}", func);
        }
        
        // do inter-procedural check on rest funcs
        // let mut funcs_to_report = Vec::new();
        for func in funcs_to_inter_check.iter() {
            find_caller(tcx, func.0);
            let posi = locate_arg(tcx, func.0, func.1.clone());
            //debug!("position: {:}", posi);
            // if !inter_spl_check(tcx, func.0, func.1.clone()) {
            //     funcs_to_report.push(func);
            // }
        }
    }
}