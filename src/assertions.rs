//! Collect assertions.

use either::Either;
use hashbrown::HashMap;
use log::{debug, error, warn};
use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
// use tracing::Subscriber;

use crate::reporter;

use crate::callgraph;
use crate::source_info;
use rustc_span::Span;

use rustc_middle::{
    mir::{
        visit::Visitor, BasicBlock, Body, Constant, Operand, Place, PlaceRef, VarDebugInfoContents,
    },
    ty::{TyCtxt, TyKind},
};
use rustc_span::Symbol;

// use crate::dd;

use crate::{
    accounts::get_accounts,
    dd::{self, HeuristicFilter, ASSERT_MACRO_FILTER},
    reporter::Report,
    visit::{self, DefUseChainTrait},
};


use crate::conf::is_signer_var::{
    check_is_signer_name,
    in_blacklist,
    is_signer_var_filter,
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
        arg: Operand<'tcx>,
    },

    AssertIFStmt {
        assertion_call_name: &'static str,
        op_arg_index: u32,
        elements: AssertionOperands,
        args: Vec<Operand<'tcx>>,
    },
    // PanicStmt {
    //     panic_call_name: &'static str,
    //     args: Vec<Operand<'tcx>>,
    // },
}

impl<'tcx> KeyStmt<'tcx> {
    pub fn get_op(&self) -> Option<&Operand<'tcx>> {
        match self {
            KeyStmt::AssertionStmt {
                op_arg_index: idx,
                args,
                ..
            } => Some(&args[*idx as usize]),
            _ => None,
        }
    }

    pub fn get_elements(&self) -> Vec<&Operand<'tcx>> {
        let mut res = Vec::new();
        match self {
            KeyStmt::AssertionStmt { elements, args, .. } => match elements {
                &AssertionOperands::Unary(e1) => {
                    res.push(&args[e1 as usize]);
                }
                &AssertionOperands::Binary(e1, e2) => {
                    res.push(&args[e1 as usize]);
                    res.push(&args[e2 as usize]);
                }
            },
            KeyStmt::BorrowMutStmt { arg } => {
                res.push(arg);
            }

            KeyStmt::AssertIFStmt { elements, args, .. } => match elements {
                &AssertionOperands::Unary(e1) => {
                    res.push(&args[e1 as usize]);
                }
                &AssertionOperands::Binary(e1, e2) => {
                    res.push(&args[e1 as usize]);
                    res.push(&args[e2 as usize]);
                }
            },
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
                    // println!("Call name = {}, Kind: {:?} arg = {:?}",call_name, c.ty().kind(), args);
                    match call_name.as_str() {
                        "core::panicking::assert_failed" => {
                            key_stmts.push(KeyStmt::AssertionStmt {
                                assertion_call_name: "core::panicking::assert_failed",
                                op_arg_index: 0,
                                elements: AssertionOperands::Binary(1, 2),
                                args: args.clone(),
                            });
                        }

                        "std::cmp::PartialEq::eq" => {
                            // debug!("PartialEq: {:?}, {:?}", func, args);
                            // debug!("******** Len of arg = {}", args.len());
                            // // debug!("******** Arg[0] = {:?}", args[0].clone());
                            // if args.len()==2 {
                            //     debug!("******** Arg[0] = {:?}", args[0].clone());
                            //     debug!("******** Arg[0] = {:?}", args[1].clone());

                            // } else {
                            //     debug!("******** !! Args = {:?}", args.clone());
                            // }

                            key_stmts.push(KeyStmt::AssertIFStmt {
                                assertion_call_name: "std::cmp::PartialEq::eq",
                                op_arg_index: 0,
                                elements: AssertionOperands::Binary(0, 1),
                                args: args.clone(),
                            });
                        }

                        "core::panicking::panic" => {
                            // debug!("Panic: {:?}, {:?}", func, args);
                            // debug!("******** Len of arg = {}", args.len());
                            // debug!("******** Arg[0] = {:?}", args[0].clone());

                            key_stmts.push(KeyStmt::AssertionStmt {
                                assertion_call_name: "core::panicking::assert_failed",
                                op_arg_index: 0,
                                elements: AssertionOperands::Unary(0),
                                args: args.clone(),
                            });
                        }
                        "std::cell::RefCell::<T>::borrow_mut" => {
                            assert!(args.len() == 1);
                            key_stmts.push(KeyStmt::BorrowMutStmt {
                                arg: args[0].clone(),
                            });
                        }
                        _ => {}
                    }
                }
                TyKind::Foreign(def_id) => {
                    let call_name = tcx.def_path_str(*def_id);
                    match call_name.as_str() {
                        _ => {}
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

fn check_account_ownership<'tcx>(
    account: &Symbol,
    assertions: &[(
        Constant<'tcx>,
        Vec<Either<(Symbol, Vec<Symbol>), Constant<'tcx>>>,
    )],
) -> bool {
    for (_assert_op, args) in assertions {
        for arg in args {
            if let Some((var, fields)) = arg.as_ref().left() {
                if var == account && fields.len() == 1 && fields[0].as_str() == "owner" {
                    return true;
                }
            }
        }
    }
    false
}

fn check_is_signer_check<'tcx>(
    assertions: &[(
        Constant<'tcx>,
        Vec<Either<(Symbol, Vec<Symbol>), Constant<'tcx>>>,
    )],
) -> bool {
    for (_assert_op, args) in assertions {
        for arg in args {
            if let Some((var, fields)) = arg.as_ref().left() {
                if !check_is_signer_name(var.to_string()){
                    continue;
                }
                if fields.len() > 0 {
                    // debug!(
                    //     "===fileds: {:?}, bool = {}",
                    //     fields,
                    //     fields[0].as_str() == "is_signer"
                    // );
                }
                if fields.len() == 1 && fields[0].as_str() == "is_signer" {
                    // println!("=== Pass is_signer check.");
                    return true;
                }
            }
        }
    }
    false
}


fn trim_newline(s: &mut String) {
    while s.ends_with('\n') {
        s.pop();
        while s.ends_with('\r') {
            s.pop();
        }
    }
    while s.ends_with('\r') {
        s.pop();
        while s.ends_with('\n') {
            s.pop();
        }
    }
}

pub fn collect_all_assertions<'tcx>(tcx: TyCtxt<'tcx>) {
    let mut func_to_ddg: HashMap<String, (DefId, dd::DataDepsGraph)> = HashMap::default();

    for item_id in tcx.mir_keys(()) {
        let def_id = item_id.to_def_id();
        match tcx.def_kind(def_id) {
            // https://doc.rust-lang.org/beta/nightly-rustc/rustc_hir/def/enum.DefKind.html
            DefKind::Fn | DefKind::AssocFn => {
                if !tcx.is_mir_available(def_id) {
                    continue;
                }
                // debug!("Visit item: {}", tcx.def_path_str(def_id));

                
                let mut is_signer_checked = false;
                let mut has_signer_variable = false;
                let mut is_signer_var = String::from("Unknown");
                let mut is_signer_span: Option<Span> = None;
                // let mut signer_variable = Vec::new(); // to do: add debugging info for signer_variable

                let body = tcx.optimized_mir(def_id);

                let mut visitor = visit::BodyVisitor {
                    tcx,
                    f: Some(check_key_calls),
                    defs: HashMap::default(),
                    switchint: Vec::new(),
                    focus: Vec::new(),
                };
                // debug!("Function: {}", tcx.def_path_str(def_id));
                // let heuristic: HeuristicFilter = HeuristicFilter::NoFilter;
                let heuristic = ASSERT_MACRO_FILTER.clone();
                let ddg = dd::DataDepsGraph::new(body, tcx, heuristic);

                let do_cg = true;
                if do_cg {
                    let cg = callgraph::CallGraph::new(
                        body,
                        tcx,
                        ddg.clone(),
                        callgraph::ASSERT_MACRO_FILTER.clone(),
                    );
                    // debug!(" ====  cg.is_signer = {}", cg.is_signer);
                    if cg.is_signer {
                        has_signer_variable = true;
                        is_signer_var = cg.is_signer_var.clone();
                        is_signer_span = cg.is_signer_span;
                        is_signer_checked = true;
                    }
                    if cg.has_signer {
                        has_signer_variable = true;
                        is_signer_var = cg.is_signer_var.clone();
                        is_signer_span = cg.is_signer_span;
                    }
                }

        

                visitor.visit_body(body);

                get_accounts(tcx, body);

                // let dump_path = format!("{}.dot", tcx.def_path_str(def_id));
                // debug!("dump to {}", dump_path);
                // ddg.dump(dump_path);

                func_to_ddg.insert(tcx.def_path_str(def_id).to_string(), (def_id, ddg.clone()));

                let dbg_info = get_local_vars(tcx, body);
                let mut sensitive_vars = Vec::new();
                let mut assertions = Vec::new();
                let debug_switchint = false;
                if debug_switchint {
                    // debug!("++ visitor.switchint = {:?}", visitor.switchint);
                }
                // visitor.switchint
                for op in visitor.switchint.iter() {
                    let origin_op_field = ddg.origin_op_source(op, false);
                    let origin_op_type = ddg.origin_op_source(op, true);

                    let mut posi = 9999999;
                    if debug_switchint {
                        // debug!(
                        //     "++ visitor.switchint original source HERE = {:#?}",
                        //     origin_op_field
                        // );
                    }
                    if !origin_op_field.is_none() {
                        if debug_switchint {
                            // debug!("================ origin_op_field ========");
                            // debug!("++ visitor.switchint op = {:?}", op);
                            // debug!(
                            //     "++ visitor.switchint original source = {:#?}",
                            //     origin_op_field
                            // );
                        }
                        if let Some(dd::DdNode::PartialFieldRef { base }) = origin_op_field {
                            // if debug_switchint {
                            //     debug!("++ visitor.switchint TYPE = {:?}", base.projection);
                            // }
                            if base.projection.len() > 1 {
                                // if debug_switchint {
                                //     debug!("++ visitor.switchint TYPE = {:?}", base.projection[0]);
                                //     debug!("++ visitor.switchint TYPE = {:?}", base.projection[1]);
                                // }

                                if let rustc_middle::mir::ProjectionElem::Field(Field, T) =
                                    base.projection[1]
                                {
                                    if debug_switchint {
                                        // debug!(
                                        //     "++ visitor.switchint Field = {:?}",
                                        //     Field.as_usize()
                                        // );
                                    }
                                    posi = Field.as_usize();
                                } else {
                                    // if debug_switchint {
                                    //     debug!("++ visitor.switchint Field = None");
                                    // }
                                }

                                // Handle anchor projections
                                // Impl tested on https://github.com/project-serum/sealevel-attacks/tree/master/programs/0-signer-authorization/secure
                                let mut i = 0;
                                while i < base.projection.len() {
                                    if let rustc_middle::mir::ProjectionElem::Field(Field, T) =
                                        base.projection[i]
                                    {
                                        if debug_switchint {
                                            // debug!(
                                            //     "++ visitor.switchint Field = {:?}",
                                            //     Field.as_usize()
                                            // );
                                        }
                                        // let posi = Field.as_usize();
                                        if debug_switchint {
                                            // debug!("++++++++++++++++++++++++++");
                                            // debug!("++ visitor.switchint Field = {:?}", Field);
                                            // debug!("++ visitor.switchint T = {:?}", T);
                                        }
                                        let type_of_var = format!("{:#?}",T);
                                        if type_of_var.contains("anchor_lang::prelude::AccountInfo") {
                                            // cannot judge based on type only. Currently use name-based matching
                                            // has_signer_variable = true;
                                            if i + 1 < base.projection.len(){
                                                if let rustc_middle::mir::ProjectionElem::Field(Field, T) =
                                                base.projection[i+1]
                                                {
                                                    if debug_switchint {
                                                        // debug!(
                                                        //     "++ visitor.switchint Field = {:?}",
                                                        //     Field.as_usize()
                                                        // );
                                                    }
                                                    let curposi = Field.as_usize();
                                                    if curposi == 1 {
                                                        if debug_switchint {
                                                            debug!("[*] Successfully captured anchor is_signer check");
                                                        }
                                                        // has_signer_variable = true;
                                                        is_signer_checked = true;
                                                    } 
                                                }
                                            }
                                            
                                        }
                                    }

                                    i = i + 1;
                                }
                            }
                        } else {
                            // if debug_switchint {
                            //     debug!("++ visitor.switchint TYPE = None");
                            // }
                        }
                    }

                    if !origin_op_type.is_none() {
                        if debug_switchint {
                            // debug!("================ origin_op_type ========");
                            // debug!(
                                // "++ visitor.switchint original source = {:#?}",
                                // origin_op_type
                            // );
                        }
                        if let Some(dd::DdNode::LocalDecl {
                            local,
                            ty,
                            dbg_info,
                            ..
                        }) = origin_op_type
                        {
                            if debug_switchint {
                                // debug!(
                                //     "++ visitor.switchint LOCAL = {:?}, TY = {:?}, DBG = {:?}",
                                //     local, ty, dbg_info
                                // );
                            }
                            if ty
                                .to_string()
                                .contains("solana_program::account_info::AccountInfo")
                            {
                                // if debug_switchint {
                                //     debug!("++ Type matches solana_program::account_info::AccountInfo.");
                                // }
                                if dbg_info - 1 > 0 && ddg.var_dbg_info.len() > dbg_info - 1 {
                                    let mut VarDeg = &ddg.var_dbg_info[dbg_info - 1].clone();
                                    // let mut VarDeg2 = VarDeg.clone();
                                    if debug_switchint {
                                        // debug!("++ ddg.var_dbg_info = {:?}", VarDeg);
                                        // debug!("++ ddg.var_dbg_info name = {:?}", VarDeg.name);
                                    }
                                    if check_is_signer_name(VarDeg.name.to_string()){
                                        has_signer_variable = true;
                                        is_signer_var = VarDeg.name.to_string();
                                        is_signer_span = Some(VarDeg.source_info.span);
                                        if posi == 1
                                        && tcx.def_path_str(def_id).to_string().contains("withdraw")
                                        {
                                            //  field == "is_signer"
                                            is_signer_checked = true;
                                            // debug!("++ pass is_signer_checked.")
                                        }   
                                    }
                                   
                                    // VarDeg.name
                                }
                            }
                        }
                    }
                }

                // let mut panic_assert = Vec::new();
                for assert_stmt in visitor.focus.iter() {
                    let op = assert_stmt.get_op();
                    let args = assert_stmt.get_elements();
                    match assert_stmt {
                        KeyStmt::AssertionStmt { .. } => {
                            // debug!(" === AssertionStmt Function: {}, kind: {:?}, args: {:?}", tcx.def_path_str(def_id), op, args);
                            let op = op.unwrap();
                            if let Operand::Copy(op_place) | Operand::Move(op_place) = op {
                                let mut op_from =
                                    visitor.get_def(op_place.as_ref(), body, &dbg_info);
                                // debug!("Function: {}, kind: {:?}, args: {:?}", tcx.def_path_str(def_id), op_from, args);
                                let mut args_from = Vec::new();
                                for arg in args.iter() {
                                    if let Operand::Copy(arg_place) | Operand::Move(arg_place) = arg
                                    {
                                        let mut arg_from =
                                            visitor.get_def(arg_place.as_ref(), body, &dbg_info);
                                        // println!("+++++++++ arg_from = {:?}", arg_from);
                                        if arg_from.len() > 0 {
                                            args_from.push(arg_from.remove(0));
                                        }
                                    } else {
                                        // debug!("Place 2");
                                    }
                                }

                                // debug!(
                                //     "Function {}, kind: {:?}, args: {:?}",
                                //     tcx.def_path_str(def_id),
                                //     op_from,
                                //     args_from
                                // );
                                // debug!(
                                //     "Dd result: {:#?}, {:#?}", ddg.origin_op_source(op, true),
                                //     ddg.origin_ops_source(args.as_slice(), true),
                                // );

                                assert_eq!(op_from.len(), 1);
                                assertions.push((op_from.remove(0).right().unwrap(), args_from));
                            } else {
                                // debug!("Place 1");
                                let mut stmt = args[0].constant().unwrap().to_string().clone();
                                let v: Vec<&str> = stmt.split("assertion failed: ").collect();
                                if v.len() > 1 {
                                    let v2: Vec<&str> = v[1].split(".").collect();
                                    if v2.len() > 1 {
                                        let var_name = v2[0];
                                        let field_vec: Vec<&str> = v2[1].split('"').collect();
                                        let field = field_vec[0];
                                        // debug!("Place 1 {:?} {:?}", var_name, field);
                                        // panic_assert.push((var_name, field));
                                        if !check_is_signer_name(var_name.clone().to_string()) {
                                            continue;
                                        }
                                        
                                        has_signer_variable = true;
                                        is_signer_var = var_name.to_string();
                                        is_signer_span = None; //Some(assert_stmt.get_span());
                                        // signer_variable.push(&&var_name.clone());
                                        if field == "is_signer" {
                                            // println!("=== Pass is_signer check.");
                                            is_signer_checked = true;
                                        }
                                    }
                                }
                            }
                        }
                        KeyStmt::BorrowMutStmt { .. } => {
                            let arg = args[0];
                            if let Operand::Copy(arg_place) | Operand::Move(arg_place) = arg {
                                let backtracked =
                                    visitor.get_def(arg_place.as_ref(), body, &dbg_info);
                                // debug!(
                                //     "BorrowMutStmt---Function {}, Borrowmut: {:?}, from: {:?}",
                                //     tcx.def_path_str(def_id),
                                //     arg,
                                //     backtracked
                                // );
                                if let Some(var_fields) = backtracked.get(0) {
                                    if let Some((var, fields)) = var_fields.as_ref().left() {
                                        if fields.len() == 1 && fields[0].as_str() == "data" {
                                            if !sensitive_vars.contains(&var.clone()) {
                                                sensitive_vars.push(var.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        KeyStmt::AssertIFStmt { .. } => {
                            // debug!(" ===+=+= AssertIFStmt: {}, kind: {:?}, args: {:?}", tcx.def_path_str(def_id), op, args);

                            // let arg0 = args[0];
                            // if let Operand::Copy(arg_place) | Operand::Move(arg_place) = arg0 {
                            //     let backtracked =
                            //         visitor.get_def(arg_place.as_ref(), body, &dbg_info);
                            //         debug!("===+=+= AssertIFStmt: backtracked: Arg0  {:?}", backtracked);
                            // }

                            // let arg1 = args[1];
                            // if let Operand::Copy(arg_place) | Operand::Move(arg_place) = arg1 {
                            //     let backtracked =
                            //         visitor.get_def(arg_place.as_ref(), body, &dbg_info);
                            //         debug!("===+=+= AssertIFStmt: backtracked: Arg1 {:?}", backtracked);
                            // }

                            // let op = op.unwrap();
                            // if let Operand::Copy(op_place) | Operand::Move(op_place) = op {
                            //     let mut op_from =
                            //         visitor.get_def(op_place.as_ref(), body, &dbg_info);
                            //     debug!("===+=+= AssertIFStmt: Function: {}, kind: {:?}, args: {:?}", tcx.def_path_str(def_id), op_from, args);
                            // let mut args_from = Vec::new();
                            // for arg in args {
                            //     if let Operand::Copy(arg_place) | Operand::Move(arg_place) = arg
                            //     {
                            //         let mut arg_from =
                            //             visitor.get_def(arg_place.as_ref(), body, &dbg_info);
                            //         println!("+++++++++ arg_from = {:?}", arg_from);
                            //         if arg_from.len() > 0 {
                            //             args_from.push(arg_from.remove(0));
                            //         }
                            //     }else{
                            //         debug!("Place 2");
                            //     }
                            // }
                            // debug!(
                            //     "AssertionStmt---Function {}, kind: {:?}, args: {:?}",
                            //     tcx.def_path_str(def_id),
                            //     op_from,
                            //     args_from
                            // );
                            // assert_eq!(op_from.len(), 1);
                            // assertions.push((op_from.remove(0).right().unwrap(), args_from));
                            // }else {
                            //     debug!("Place 1");
                            //     let mut stmt = args[0].constant().unwrap().to_string().clone();
                            //     let v: Vec<&str> = stmt.split("assertion failed: ").collect();
                            //     let v2: Vec<&str> = v[1].split(".").collect();
                            //     let var_name = v2[0];
                            //     let field_vec: Vec<&str> = v2[1].split('"').collect();
                            //     let field = field_vec[0];
                            //     debug!("Place 1 {:?} {:?}", var_name, field);
                            //     // panic_assert.push((var_name, field));
                            //     if !auth_names.contains(&var_name){
                            //         continue;
                            //     }
                            //     if field == "is_signer" {
                            //         // println!("=== Pass is_signer check.");
                            //         is_signer_checked = true;
                            //     }
                            // }
                        } // KeyStmt::PanicStmt { .. } => {
                          //     let arg = args[0];
                          //     if let Operand::Copy(arg_place) | Operand::Move(arg_place) = arg {
                          //         let backtracked =
                          //             visitor.get_def(arg_place.as_ref(), body, &dbg_info);
                          //         debug!(
                          //             "Panic---Function {}, Borrowmut: {:?}, from: {:?}",
                          //             tcx.def_path_str(def_id),
                          //             arg,
                          //             backtracked
                          //         );
                          //         if let Some(var_fields) = backtracked.get(0) {
                          //             if let Some((var, fields)) = var_fields.as_ref().left() {
                          //                 if fields.len() == 1 && fields[0].as_str() == "data" {
                          //                     if !sensitive_vars.contains(&var.clone()){
                          //                         sensitive_vars.push(var.clone());
                          //                     }

                          //                 }
                          //             }
                          //         }
                          //     }
                          // }
                          // println!("Function {}, kind: {:?}, args: {:?}", tcx.def_path_str(def_id), op_from, args_from);
                          // debug!("Function {}, kind: {:?}, args: {:?}", tcx.def_path_str(def_id), op_from, args_from);
                    }
                }

                // debug!("Sensitive vars: {:?}", sensitive_vars);
                // debug!("Assertions: {:?}", assertions);
                // debug!("Panic Assertions: {:?}", panic_assert);
                is_signer_checked =
                    is_signer_checked | check_is_signer_check(assertions.as_slice());
                // is_signer_checked = is_signer_checked | check_is_signer_check_panic(panic_assert.as_slice());
                // println!("is_signer_checked returns {}", is_signer_checked);

                for sensitive_var in sensitive_vars {
                    if !check_account_ownership(&sensitive_var, assertions.as_slice()) {
                        warn!(
                            "The ownership of {} is not checked in function {}",
                            sensitive_var,
                            tcx.def_path_str(def_id)
                        );
                        // Report::new_mssing_owner(tcx, tcx.def_path_str(def_id), body.span, None);

                        // Report::new_missing_owner(
                        //     tcx,
                        //     "Missing Owner Check".to_string(),
                        //     "Critical".to_string(),
                        //     source_info::get_source_file(tcx, body.span).unwrap_or("".to_string()),
                        //     source_info::get_source_lines(tcx, body.span).unwrap_or("".to_string()),
                        //     tcx.def_path_str(def_id),
                        //     "Todo: Add owner variable".to_string(),
                        //     "UnResolved".to_string(),
                        //     "GitHub Link to be added.".to_string(),
                        //     body.span,
                        //     Some("Description of the bug here."),
                        //     "Some alleviation steps here.".to_string(),
                        // );
                    }
                }
                // debug!(
                //     "====== end of function {:#?}: {}, {}",
                //     tcx.def_path_str(def_id).to_string(),
                //     has_signer_variable,
                //     is_signer_checked
                // );
                // if has_signer_variable || true {

                let print_no_signer_var = false;
                let print_signer_checked = false;

                let mut span = body.span;
                if let Some(sp) = is_signer_span{
                    // span = sp;
                }
                if has_signer_variable {
                    if !is_signer_checked {
                        // debug!(
                        //     "====== There is no is_signer check for this contract in {} function!",
                        //     tcx.def_path_str(def_id).to_string()
                        // );
                        // reporter::Report::new_missing_signer(tcx, tcx.def_path_str(def_id).to_string(), body.span,  None);
                        let mut code = source_info::get_source_lines(tcx, body.span).unwrap_or("".to_string());
                        let mut code_lines = code.split("\n\t").collect::<Vec<&str>>();
                        let mut anchor_filter = false;
                        if code_lines.len() == 2{
                            let real_code = code_lines[1];
                            let mut trimed_code = real_code.trim().to_string();
                            trim_newline(&mut trimed_code);
                            if trimed_code.trim() == "Accounts"{
                                anchor_filter = true;
                            }
                        }
                        
                        
                        // debug!("!!!!!! in_blacklist for function {} returns {}", tcx.def_path_str(def_id), in_blacklist(tcx.def_path_str(def_id)));
                        if !in_blacklist(tcx.def_path_str(def_id)) && !code.contains("\n\t#[") && !anchor_filter && !is_signer_var_filter(is_signer_var.clone()) {
                            
                            // Report::new_missing_signer(
                            //     tcx,
                            //     "Missing Signer Check".to_string(),
                            //     "Major".to_string(),
                            //     source_info::get_source_file(tcx, body.span).unwrap_or("".to_string()),
                            //     source_info::get_source_lines(tcx, span).unwrap_or("".to_string()),
                            //     tcx.def_path_str(def_id),
                            //     is_signer_var.clone(),
                            //     "UnResolved".to_string(),
                            //     "https://github.com/parasol-aser/vrust/blob/yifei/patterns/01/README.md".to_string(),
                            //     body.span,
                            //     Some(
                            //         String::from("")
                            //             + "Missing is_signer check for function: "
                            //             + &tcx.def_path_str(def_id)
                            //             + "\nWe should add an is_signer check for variable: "
                            //             + is_signer_var.as_str(),
                            //     ),
                            //     "The contract should add an is_signer check in this function.".to_string(),
                            // );
                        }
                    } else if print_signer_checked {
                        // debug!(
                        //     "====== Pass is_signer check for fucntion {}.",
                        //     tcx.def_path_str(def_id).to_string()
                        // );

                        // Report::new_missing_signer(
                        //     tcx,
                        //     "Captured Signer Check".to_string(),
                        //     "Informational".to_string(),
                        //     source_info::get_source_file(tcx, body.span).unwrap_or("".to_string()),
                        //     source_info::get_source_lines(tcx, body.span).unwrap_or("".to_string()),
                        //     tcx.def_path_str(def_id),
                        //     is_signer_var.clone(),
                        //     "Resolved".to_string(),
                        //     "https://github.com/parasol-aser/vrust/blob/yifei/patterns/01/README.md".to_string(),
                        //     body.span,
                        //     Some(
                        //         String::from("")
                        //             + "Captured is_signer check for function: "
                        //             + &tcx.def_path_str(def_id)
                        //             + "\nWe captured an is_signer check for variable: "
                        //             + is_signer_var.as_str(),
                        //     ),
                        //     "Nothing needs to be done.".to_string(),
                        // );
                    }
                } else if print_no_signer_var {
                    // Report::new_missing_signer(
                    //     tcx,
                    //     "No Signer Variable".to_string(),
                    //     "Discussion".to_string(),
                    //     source_info::get_source_file(tcx, body.span).unwrap_or("".to_string()),
                    //     source_info::get_source_lines(tcx, span).unwrap_or("".to_string()),
                    //     tcx.def_path_str(def_id),
                    //     is_signer_var.clone(),
                    //     "UnResolved".to_string(),
                    //     "https://github.com/parasol-aser/vrust/blob/yifei/patterns/01/README.md"
                    //         .to_string(),
                    //     body.span,
                    //     Some(
                    //         String::from("")
                    //             + "No is_signer variable to check for function: "
                    //             + &tcx.def_path_str(def_id),
                    //     ),
                    //     "Nothing needs to be done for now.".to_string(),
                    // );
                }
            }

            _ => {}
        }
    }
    let debug_printing = false;
    if debug_printing {
        // debug!("--------- func_to_ddg:");
        for (k, v) in func_to_ddg {
            debug!("Key: {}", k);
        }
    }
}
