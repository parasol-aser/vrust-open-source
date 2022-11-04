//! Utilities for data source/dependency analysis. Note that this is not
//! full data dependency analysis as many statements/expressions are not
//! explicit modelled. For now, it is mostly used to compute the source
//! user variable that can eventually flow to a given PlaceRef, or the
//! source user variables that a specific location depends on through control
//! edges, e.g., assert!(expr), the generated panic call depends on all the operands
//! in expr.

use hashbrown::HashMap;
use lazy_static::lazy_static;
use log::debug;
use log::warn;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::depth_first_search;
use petgraph::visit::EdgeFiltered;
use petgraph::visit::{Control, DfsEvent};
use rustc_data_structures::graph::WithPredecessors;
use rustc_middle::mir::{Constant, ConstantKind, VarDebugInfo};
use rustc_middle::{
    mir::{
        visit::Visitor, Body, Field, Local, Location, Operand, Place, PlaceRef, ProjectionElem,
        Rvalue, Statement, Terminator, TerminatorKind, VarDebugInfoContents, BinOp,
    },
    ty::{Ty, TyCtxt, TyKind},
};
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use rustc_span::Span;

use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;

use crate::dd;


use crate::conf::is_signer_var::{
    check_is_signer_name,

};


#[derive(Debug, Clone, Copy)]
pub enum CallNode<'tcx> {
    /// Implicit nodes that are part of a Rvalue,
    /// e.g., in _2 = ((_1 as Object).0: u32), _2 is
    /// connected by a FieldRef edge to an implicit node
    /// that is connected to _1 by as cast edge
    // Implicit { op: &'tcx Operand<'tcx> },

    /// Part of field ref that is not directly assigned to
    /// a local, e.g., in _2 = _1.0.1, _1.0 can only
    /// be stored in this variant.
    PartialFieldRef { base: PlaceRef<'tcx> },

    /// All locals declared at the beginning of a Body
    LocalDecl {
        local: Local,
        ty: Ty<'tcx>,
        dbg_info: usize,
        arg_count: usize,
    },

    /// Constant. This is useful when handling
    /// constant op (eq, neq) in assert_* macros.
    Constant { val: Constant<'tcx> },

    FunctionNode {
        // func: &'tcx Operand<'tcx>,  // Copy trait not implemented
        // func: String,  // Copy trait not implemented
        arg_count: usize,

    }
}

impl<'tcx> Display for CallNode<'tcx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PartialFieldRef { base } => write!(f, "{:?}", base),
            Self::LocalDecl { local, .. } => write!(f, "{:?}", local),
            Self::Constant { val } => write!(f, "{:?}", val.literal),
            Self::FunctionNode {..} => {Ok(())},
            // Self::FunctionNode { func, ..} => write!(f, "{:?}", func),
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub enum CallEdge {
    /// Either a move or copy, e.g., _1 = _2
    Assign,

    /// Field reference
    FieldRef(Field),

    /// reference, e.g., &x or &mut x
    Ref,

    /// All other projections other than FieldRef, e.g., DeRef
    IgnoredProjection,

    /// Address of, e.g., &raw const x
    // AddrOf,

    /// Dereference, e.g., *x
    // DeRef,

    /// Casting, e.g., x as u32.
    /// Since the two connected node represent the original and casted type,
    /// we do not record it here.
    Cast,
}

impl Display for CallEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assign => write!(f, "="),
            Self::FieldRef(field) => write!(f, ".{}", field.index()),
            Self::Ref => write!(f, "&"),
            Self::Cast => write!(f, "as"),
            Self::IgnoredProjection => write!(f, "proj"),
        }
    }
}

pub type DefaultPredicateFn = fn() -> bool;

#[derive(Debug, Clone)]
pub enum HeuristicFilter<F = DefaultPredicateFn>
where
    F: FnOnce() -> bool,
{
    /// Filter out nodes that are used defined local
    /// variables whose name are included in the vec.
    NameFilter(Vec<&'static str>),
    /// Filter out nodes with types included in the vec.
    TypeFilter(Vec<&'static str>),
    /// Skip nodes that satisfies the predicate
    PredicateFilter(F),
    NoFilter,
}

impl<F> HeuristicFilter<F>
where
    F: FnOnce() -> bool,
{
    pub fn filter<'tcx>(
        &self,
        node: &CallNode<'tcx>,
        var_dbg_info: &Vec<VarDebugInfo<'tcx>>,
    ) -> bool {
        match node {
            CallNode::LocalDecl { dbg_info, .. } => {
                if *dbg_info > 0 {
                    let dbg_info = &var_dbg_info[*dbg_info - 1];
                    match self {
                        HeuristicFilter::NameFilter(names) => {
                            return !names.contains(&&*dbg_info.name.as_str())
                        }
                        _ => return true,
                    }
                }
                true
            }
            _ => true,
        }
    }
}

lazy_static! {
    pub static ref ASSERT_MACRO_FILTER: HeuristicFilter = {
        let names = vec!["left_val", "right_val"];
        HeuristicFilter::NameFilter(names)
    };
}

#[derive(Clone)]
pub struct CallGraph<'tcx, F = DefaultPredicateFn>
where
    F: FnOnce() -> bool,
{
    pub tcx: TyCtxt<'tcx>,
    pub g: DiGraph<CallNode<'tcx>, CallEdge>,
    pub place_dbg_info_map: HashMap<PlaceRef<'tcx>, usize>,
    pub var_dbg_info: Vec<VarDebugInfo<'tcx>>,
    pub is_signer: bool,
    pub is_signer_var: String,
    pub is_signer_span: Option<Span>,
    pub ddg: dd::DataDepsGraph<'tcx>,
    heuristic: HeuristicFilter<F>,
    local_decl_nodes: HashMap<PlaceRef<'tcx>, NodeIndex>,
    /// Place nodes that are not local decl, e.g., projections
    /// on local decl
    other_nodes: HashMap<PlaceRef<'tcx>, NodeIndex>,
    /// Constant nodes
    constant_nodes: HashMap<ConstantKind<'tcx>, NodeIndex>,


    // for is_signer check
    check_function: bool,
    arg_posi: usize,
    pub has_signer: bool,

}

impl<'tcx, F> CallGraph<'tcx, F>
where
    F: FnOnce() -> bool,
{
    pub fn new(body: &Body<'tcx>, tcx: TyCtxt<'tcx>, ddg: dd::DataDepsGraph<'tcx>, heuristic: HeuristicFilter<F>) -> Self {
        let mut cg = Self {
            g: DiGraph::new(),
            var_dbg_info: body.var_debug_info.clone(),
            is_signer: false,
            ddg,
            place_dbg_info_map: get_dbg_info_mapping(body),
            tcx,
            heuristic,
            local_decl_nodes: HashMap::new(),
            other_nodes: HashMap::new(),
            constant_nodes: HashMap::new(),
            check_function: false,
            arg_posi:0,
            has_signer: false,
            is_signer_var: String::new(),
            is_signer_span: None,
        };
        cg.visit_body(body);

        cg
    }

    pub fn dump<P: AsRef<Path>>(&self, out: P) -> std::io::Result<()> {
        let dot = Dot::with_config(&self.g, &[Config::EdgeNoLabel]);
        let mut file = File::create(out)?;
        file.write_all(dot.to_string().as_bytes())?;
        Ok(())
    }

    pub fn origin_op_source(&self, op: &Operand<'tcx>, pass_field: bool) -> Option<CallNode<'tcx>> {
        match op {
            Operand::Move(place) | Operand::Copy(place) => {
                // debug!("++ origin_op_source: Move or Copy");
                self.origin_source(place.as_ref(), pass_field)
                
            }
            Operand::Constant(box c) => self
                .constant_nodes
                .get(&c.literal)
                .map(|ni| *self.g.node_weight(*ni).unwrap()),
           
        }
    }

    pub fn origin_ops_source(
        &self,
        ops: &[&Operand<'tcx>],
        pass_field: bool,
    ) -> Vec<Option<CallNode<'tcx>>> {
        let mut res = Vec::new();
        for op in ops {
            res.push(self.origin_op_source(*op, pass_field));
        }
        res
    }

    /// Track back to the base source variable that place comes from.
    /// It will not stop at field accesses if pass_field is true,
    /// e.g., a.b will result a temporary place that comes from source
    /// variable a, and if we stop at field accesses, it then comes from
    /// source field b of source variable a.
    pub fn origin_source(&self, place: PlaceRef<'tcx>, pass_field: bool) -> Option<CallNode<'tcx>> {
        // debug!("origin source: {:?}", place);
        let start = self
            .local_decl_nodes
            .get(&place)
            .or_else(|| self.other_nodes.get(&place));
        // debug!("++=== start = {:?}", start);
        if let Some(start) = start {
            // debug!("++=== start enter");
            let mut goal: Option<CallNode<'tcx>> = None;
            let visit_fn = |event: DfsEvent<NodeIndex>| {
                if let DfsEvent::TreeEdge(_u, v) = event {
                    let v_node = self.g.node_weight(v).unwrap();
                    // debug!("traverse node: {:?}", v_node);
                    if let CallNode::LocalDecl { dbg_info: 1.., .. } = v_node {
                        if self.heuristic.filter(v_node, &self.var_dbg_info) {
                            goal = Some(*v_node);
                            return Control::Break(v);
                        }
                    } else if let CallNode::Constant { .. } = v_node {
                        goal = Some(*v_node);
                        return Control::Break(v);
                    } 
                    else if let CallNode::PartialFieldRef {..} = v_node { //  && pass_field == false
                        if pass_field == false{
                            goal = Some(*v_node);
                            return Control::Break(v);
                        }
                        
                    }

                }
                Control::Continue
            };
            if !pass_field {
                let g = EdgeFiltered::from_fn(&self.g, |e| match e.weight() {
                    CallEdge::FieldRef(_) => false,
                    _ => true,
                });
                depth_first_search(&g, Some(*start), visit_fn);
            } else {
                depth_first_search(&self.g, Some(*start), visit_fn);
            }
            // debug!("return goal ");
            return goal;
        }

        None
    }

    /// Check all the source variables that location depends on.
    /// There are two cases:
    /// 1. The location does not take non-const parameters, so it only
    /// depends on the branch conditions that precedes the bb of location,
    /// we then need to check the dependency of the the branch conditions.
    /// 2. The location is a stmt taking other locals as parameters, we should also
    /// check their dependent source variables.
    /// In both cases, the location should actually represent a terminator.
    /// e.g., assert!(a) will causes a panic statement that depends on
    /// the value of a.
    #[allow(unused)]
    pub fn depends_source(&self, location: Location, body: &Body<'tcx>) -> Vec<CallNode<'tcx>> {
        let mut nodes = Vec::new();
        let stmt = body.stmt_at(location);
        stmt.map_right(|terminator| {
            nodes.append(&mut self.depends_param_source(terminator, body));

            // Track control dependent sources
            // TODO: should we apply this for all stmts?
            nodes.append(&mut self.depends_branch_source(location, body));
        });

        nodes
    }

    /// Track the source variable for each params.
    fn depends_param_source(
        &self,
        terminator: &Terminator<'tcx>,
        _body: &Body<'tcx>,
    ) -> Vec<CallNode<'tcx>> {
        let mut nodes = Vec::new();
        if let TerminatorKind::Call { ref args, .. } = terminator.kind {
            // Track params sources
            for arg in args {
                if let Operand::Copy(from) | Operand::Move(from) = arg {
                    self.origin_source(from.as_ref(), true).map(|n| {
                        nodes.push(n);
                    });
                }
            }
        }

        nodes
    }

    /// Get all the predecessors of the current basic block, and track the dependent
    /// source variables of the branch conditions of the predecessors.
    /// TODO: if the branch condition is from some expressions, do we want to traverse them?
    fn depends_branch_source(&self, location: Location, body: &Body<'tcx>) -> Vec<CallNode<'tcx>> {
        let mut nodes = Vec::new();
        for pred in WithPredecessors::predecessors(body, location.block) {
            if let Some(terminator) = body.basic_blocks()[pred].terminator.as_ref() {
                match terminator.kind {
                    TerminatorKind::SwitchInt {ref discr, ..} => {
                        self.origin_op_source(discr, true).map(|n| nodes.push(n));
                        // debug!("*********---- dd --  SwitchInt discr {:?}", discr);
                        
                    }
                    _ => {}
                }
            }
        }

        nodes
    }

    fn get_right_place_node(&mut self, place: PlaceRef<'tcx>) -> NodeIndex {
        if let Some(ni) = self.local_decl_nodes.get(&place) {
            *ni
        } else if let Some(ni) = self.other_nodes.get(&place) {
            *ni
        } else {
            let node = CallNode::PartialFieldRef { base: place };
            let ni = self.g.add_node(node);
            self.other_nodes.insert(place, ni);
            ni
        }
    }

    /// Get the node for lvalue, e.g. _1, _1.0, *_1, etc.
    /// For now we only create a single node for it, unlike rvalue, for which
    /// we will handle field ref.
    fn get_left_place_node(&mut self, place: PlaceRef<'tcx>) -> NodeIndex {
        if let Some(ni) = self.local_decl_nodes.get(&place) {
            *ni
        } else if let Some(ni) = self.other_nodes.get(&place) {
            *ni
        } else {
            let node = CallNode::PartialFieldRef { base: place };
            let ni = self.g.add_node(node);
            // Since this is not local decl, store it in other_nodes.
            self.other_nodes.insert(place, ni);
            ni
        }
    }

    fn handle_place(&mut self, left: NodeIndex, place: PlaceRef<'tcx>, edge: Option<CallEdge>) {
        let mut cursor = place;
        let mut cur_node_i = self.get_right_place_node(cursor);
        if let Some(edge) = edge {
            self.g.add_edge(left, cur_node_i, edge);
        }

        loop {
            match cursor.last_projection() {
                Some((base, elem)) => {
                    // Add a edge between each cursor base and the field, the edge
                    // records the field index.
                    // TODO: should we store the type info for both the base and field?
                    match elem {
                        ProjectionElem::Field(field, _ty) => {
                            let base_node_i = self.get_right_place_node(base);
                            self.g
                                .add_edge(cur_node_i, base_node_i, CallEdge::FieldRef(field));
                            cur_node_i = base_node_i;
                            cursor = base;
                        }
                        ProjectionElem::Downcast(..)
                        | ProjectionElem::Subslice { .. }
                        | ProjectionElem::ConstantIndex { .. }
                        | ProjectionElem::Index(_)
                        | ProjectionElem::Deref => {
                            cursor = base;
                        }
                    }
                }
                None => break,
            }
        }
        let last_node_i = self.get_right_place_node(cursor);
        // Due to DeRef or other projections, the last cursor maye not be connected
        // to previous one, so do a final check here.
        if last_node_i != cur_node_i {
            self.g
                            .add_edge(cur_node_i, last_node_i, CallEdge::IgnoredProjection);
        }
    }

    fn handle_constant(&mut self, left: NodeIndex, c: Constant<'tcx>, edge: CallEdge) {
        let node = CallNode::Constant { val: c };
        let ni = self.g.add_node(node);
        self.constant_nodes.insert(c.literal, ni);
        self.g.add_edge(left, ni, edge);
    }

    fn handle_op(&mut self, left: NodeIndex, op: &Operand<'tcx>, edge: CallEdge) {
        match op {
            Operand::Move(from) | Operand::Copy(from) => {
                self.handle_place(left, from.as_ref(), Some(edge))
            }
            Operand::Constant(box c) => {
                self.handle_constant(left, *c, edge);
            }
        }
    }

    fn handle_binary_op(&mut self, operator: &BinOp, operands: &Box<(Operand<'tcx>, Operand<'tcx>)>) {
        let origin = self.origin_op_source(&operands.1, true);
        if !origin.is_none() {
            if let Some(CallNode::LocalDecl{ local, ty, dbg_info, arg_count }) = origin {
                // check if this local variable is a parameter of a function and that it's apart of an add/sub operation
                if local.index() >= 1 && local.index() <= arg_count && (*operator == BinOp::Add || *operator == BinOp::Sub) {
                    warn!("Unchecked {:?} for external variable name {:?} ({:?})", operator, self.var_dbg_info[local.index()-1].name, self.var_dbg_info[local.index()-1].source_info.span);
                }
            }
        }
    }

    fn handle_rvalue(&mut self, left: NodeIndex, rvalue: &Rvalue<'tcx>) {
        match rvalue {
            Rvalue::Use(op) => self.handle_op(left, op, CallEdge::Assign),
            Rvalue::Ref(_, _, op) => self.handle_place(left, op.as_ref(), Some(CallEdge::Ref)),
            Rvalue::Cast(_, op, _ty) => self.handle_op(left, op, CallEdge::Cast),
            Rvalue::BinaryOp(op, ops) => self.handle_binary_op(op, ops),
            Rvalue::UnaryOp(_, op) => {
                // debug!("++===== visit handle_rvalue: UnaryOp: {:?}", op);
                self.handle_op(left, op, CallEdge::Assign);
            },
            _ => {}
        }
    }
}

fn get_dbg_info_mapping<'tcx>(body: &Body<'tcx>) -> HashMap<PlaceRef<'tcx>, usize> {
    let mut vars_map = HashMap::new();
    for (idx, var_dbg_info) in body.var_debug_info.iter().enumerate() {
        match var_dbg_info.value {
            VarDebugInfoContents::Place(place) => {
                vars_map.insert(place.as_ref(), idx + 1);
            }
            _ => {}
        }
    }
    vars_map
}

fn resolve_call_target<'tcx>(func: &Operand<'tcx>) -> Option<DefId> {
    match func {
        Operand::Copy(target) | Operand::Move(target) => {
            // TODO: leverage ddg to get the call targets if func is not constant
            None
        }
        Operand::Constant(box c) => {
            match c.ty().kind() {
                // TODO: Should we handle substs?
                TyKind::FnDef(callee_def_id, substs) => Some(*callee_def_id),
                TyKind::Closure(callee_def_id, substs) => Some(*callee_def_id),
                _ => None,
            }
        }
    }
}



impl<'tcx, F> Visitor<'tcx> for CallGraph<'tcx, F>
where
    F: FnOnce() -> bool,
{
    fn visit_body(&mut self, body: &Body<'tcx>) {
        // In current MIR Visitor implementation, local decl is visited after
        // basic block data, so we have to do this here manually.
        // for local in body.local_decls.indices() {
        //     let local_decl = &body.local_decls[local];
        //     let place = Place::from(local).as_ref();
        //     let node = CallNode::LocalDecl {
        //         local,
        //         ty: local_decl.ty,
        //         dbg_info: *self.place_dbg_info_map.get(&place).unwrap_or(&0),
        //         arg_count: body.arg_count
        //     };
        //     let ni = self.g.add_node(node);
        //     self.local_decl_nodes.insert(place, ni);
        // }


        // debug!("visit_body: {:#?}", body);
        // debug!("visit_body func: {:#?}", body.source.def_id());


        // let node = CallNode::FunctionNode {
        //     body.get_op,
        //     arg_count: body.arg_count
        // };
        // let ni = self.g.add_node(node);
        // self.local_decl_nodes.insert(place, ni);

        self.super_body(body);
    }

    

    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        
        

        // /// The function that’s being called.
        // func: Operand<'tcx>,
        // /// Arguments the function is called with.
        // /// These are owned by the callee, which is free to modify them.
        // /// This allows the memory occupied by "by-value" arguments to be
        // /// reused across function calls without duplicating the contents.
        // args: Vec<Operand<'tcx>>,
        // /// Destination for the return value. If some, the call is converging.
        // destination: Option<(Place<'tcx>, BasicBlock)>,
        // /// Cleanups to be done if the call unwinds.
        // cleanup: Option<BasicBlock>,
        // /// `true` if this is from a call in HIR rather than from an overloaded
        // /// operator. True for overloaded function call.
        // from_hir_call: bool,
        // /// This `Span` is the span of the function, without the dot and receiver
        // /// (e.g. `foo(a, b)` in `x.foo(a, b)`
        // fn_span: Span,
        if let TerminatorKind::Call {
            ref func,
            ref args,
            ref destination,
            ref cleanup,
            ref from_hir_call,
            ref fn_span
        } = terminator.kind
        {
            if let Operand::Copy(op_place) = func {
                // debug!("visit_terminator op_place Operand::Copy: {:#?}", op_place);
            }
            if let Operand::Move(op_place) = func {
                // debug!("visit_terminator op_place Operand::Move: {:#?}", op_place);
            }
            if let Operand::Constant(op_place) = func { // function call
                // debug!("visit_terminator op_place Operand::Constant: {:#?}", func);

                // For anchor:
                // https://github.com/project-serum/sealevel-attacks/tree/master/programs/0-signer-authorization/recommended
                let func_name = format!("{:#?}", func);
                // if func_name.contains("anchor_lang::Accounts") && func_name.contains("try_accounts"){
                //     self.is_signer = true;
                // }
                if func_name.contains("Signer as anchor_lang::Accounts>::try_accounts"){
                    self.is_signer = true;
                    self.is_signer_var = func_name;
                    self.is_signer_span = Some(fn_span.clone());
                }

                // debug!("visit_terminator op_place Operand::Constant: fn_span: {:#?}", fn_span);
                for (i, arg) in args.iter().enumerate(){

                    let mut debug_switchint = false;
                    // check if is a is_signer check 
                    let origin_op_field = self.ddg.origin_op_source(arg, false);
                    let origin_op_type = self.ddg.origin_op_source(arg, true);

                    let mut posi = 10000000000;
                    // if debug_switchint {debug!("++ TerminatorKind::Call.switchint original source HERE = {:#?}", origin_op_field);}
                    if !origin_op_field.is_none() {
                        if debug_switchint {
                            // debug!("================ origin_op_field ========");
                            // debug!("++ TerminatorKind::Call.switchint op = {:?}", arg);
                            // debug!("++ TerminatorKind::Call.switchint original source = {:#?}", origin_op_field);
                        }
                        if let Some(dd::DdNode::PartialFieldRef{base}) = origin_op_field {
                            // if debug_switchint { debug!("++ TerminatorKind::Call.switchint TYPE = {:?}", base.projection);}
                            if base.projection.len() == 2 {
                                if debug_switchint {
                                    // debug!("++ TerminatorKind::Call.switchint TYPE = {:?}", base.projection[0]);
                                    // debug!("++ TerminatorKind::Call.switchint TYPE = {:?}", base.projection[1]);
                                }
                                if let rustc_middle::mir::ProjectionElem::Field(Field, T) = base.projection[1] {

                                    // anchor
                                    // let type_of_var = format!("{:#?}",T);
                                    // if type_of_var.contains("anchor_lang::prelude::AccountInfo") {
                                    //     self.has_signer = true;
                                    // }


                                    // if debug_switchint { debug!("++ TerminatorKind::Call.switchint Field = {:?}", Field.as_usize());}
                                    posi = Field.as_usize();
                                }else {
                                    // if debug_switchint { debug!("++ TerminatorKind::Call.switchint Field = None"); }
                                }
                            } else if base.projection.len() == 3 {
                                if debug_switchint {
                                    // debug!("++ TerminatorKind::Call.switchint TYPE1 = {:?}", base.projection[0]);
                                    // debug!("++ TerminatorKind::Call.switchint TYPE2 = {:?}", base.projection[1]);
                                    // debug!("++ TerminatorKind::Call.switchint TYPE3 = {:?}", base.projection[2]);
                                }
                                if let rustc_middle::mir::ProjectionElem::Field(Field, T) = base.projection[2] {

                                    // anchor   
                                    // to do: design a better way to capture the variable to be checked.
                                    // let type_of_var = format!("{:#?}",T);
                                    // if type_of_var.contains("anchor_lang::prelude::AccountInfo") {
                                    //     self.has_signer = true;
                                    // }

                                    // if debug_switchint { debug!("++ TerminatorKind::Call.switchint Field = {:?}", Field.as_usize());}
                                    posi = Field.as_usize();
                                }else {
                                    // if debug_switchint { debug!("++ TerminatorKind::Call.switchint Field = None"); }
                                }

                            } else if base.projection.len() == 4 {
                                // debug!("############!!!!!!!!!!############## base.projection.len() == 4");
                            } else{
                                // debug!("############!!!!!!!!!!############## Len: {}", base.projection.len());
                            }
                            
                        } else {
                            // if debug_switchint { debug!("++ TerminatorKind::Call.switchint TYPE = None");}
                        }
                        
                    }
                    // if debug_switchint && posi!= 10000000000 {debug!("--------- pos = {}", posi); debug_switchint = true;}

                    if !origin_op_type.is_none() {
                        // if debug_switchint {
                        //     debug!("================ origin_op_type ========");
                        //     debug!("++ TerminatorKind::Call.switchint original source = {:#?}", origin_op_type);
                        // }
                        if let Some(dd::DdNode::LocalDecl{local,ty, dbg_info, ..}) = origin_op_type {
                            // if debug_switchint { debug!("++ TerminatorKind::Call.switchint LOCAL = {:?}, TY = {:?}, DBG = {:?}", local,ty, dbg_info);}

                            if ty.to_string().contains("solana_program::account_info::AccountInfo")  // to do: we might want to put this to a list of func names.
                            || ty.to_string().contains("anchor_lang::prelude::AccountInfo")  // to do: also, if different types have different is_signer field posi, we need to properly handle that.
                            {
                                // if debug_switchint || true{ debug!("++--------- Type matches AccountInfo.");}
                                if dbg_info-1>0 && self.ddg.var_dbg_info.len()> dbg_info-1{
                                    let mut VarDeg = &self.ddg.var_dbg_info[dbg_info-1].clone();
                                    // let mut VarDeg2 = VarDeg.clone();
                                    if debug_switchint {
                                        // debug!("++ ddg.var_dbg_info = {:?}", VarDeg);
                                        // debug!("++ ddg.var_dbg_info name = {:?}", VarDeg.name);
                                    }
                                    if check_is_signer_name(VarDeg.name.to_string()) && posi == 1  {
                                        // debug!("(((((((((( success");
                                        // debug!("visit_terminator args: {:#?}", args);
                                        // debug!("visit_terminator destination: {:#?}", destination);
                                        // debug!("visit_terminator fn_span: {:#?}", fn_span);
                                        // debug!("visit_terminator arg i: {:#?}", i);

                                        // debug!(")))))))))) end of success");

                                        // now, we conclude that this is a potential interprocedural is_signer check
                                        // next, we visit function body to capture switch int

                                        // to do: what if they use assert inside?

                                        self.has_signer = true;
                                        self.is_signer_var = VarDeg.name.to_string();
                                        self.is_signer_span = Some(VarDeg.source_info.span);
                                        if let Some(callee) = resolve_call_target(func) {
                                            
                                            if self.tcx.is_mir_available(callee) {
                                                let callee_body = self.tcx.optimized_mir(callee);
                                                self.check_function = true;
                                                self.arg_posi = i;
                                                self.visit_body(callee_body);

                                                self.check_function = false;
                                                debug!("{:#?}", callee_body.source.def_id());
                                            }
                                        }
                                        // for item_id in self.tcx.mir_keys(()) {  // To do: try to use hash mapping 
                                        //     let def_id = item_id.to_def_id();
                                        //     match self.tcx.def_kind(def_id) { 
                                        //         DefKind::Fn | DefKind::AssocFn => {
                                        //             if !self.tcx.is_mir_available(def_id) {
                                        //                 continue;
                                        //             }
                                        //             if self.tcx.def_path_str(def_id).to_string() == op_place.to_string(){
                                        //                 debug!("Callgraph-- Visit item: {}.", self.tcx.def_path_str(def_id));
                                        //                 let body = self.tcx.optimized_mir(def_id);
                                        //                 self.visit_body(body);
                                        //             }
                                                    
                                        //         },
                    
                                        //         _ => {}
                    
                                        //     }
                                        // }


                                        
                                    }
                                    
                                // VarDeg.name
                                }
                                
                            } else {
                                // if debug_switchint || true {
                                //      debug!("++ Type does not match solana_program::account_info::AccountInfo.");
                                //      debug!("++++++++++ Type = {:?}", ty);
                                // }
                            }

                        } else{
                            // debug!("origin_op_type not dd::DdNode::LocalDecl.");
                        }
                    }
                    if self.check_function {
                        let prev_arg_posi = self.arg_posi;

                        let origin_op_field = self.ddg.origin_op_source(arg, false);
                        let origin_op_type = self.ddg.origin_op_source(arg, true);

                        let mut posi = 10000000000;
                        // if debug_switchint {debug!("++ TerminatorKind::Call.switchint original field HERE = {:#?}", origin_op_field);}
                        // if debug_switchint {debug!("++ TerminatorKind::Call.switchint original type HERE = {:#?}", origin_op_type);}
                        if !origin_op_field.is_none() {
                            // if debug_switchint {
                            //     debug!("================ origin_op_field ========");
                            //     debug!("++ TerminatorKind::Call.switchint op = {:?}", arg);
                            //     debug!("++ TerminatorKind::Call.switchint original source = {:#?}", origin_op_field);
                            // }
                            if let Some(dd::DdNode::PartialFieldRef{base}) = origin_op_field {
                                // if debug_switchint { debug!("++ TerminatorKind::Call.switchint TYPE = {:?}", base.projection);}
                                if base.projection.len() > 1 {
                                    if debug_switchint {
                                        // debug!("++ TerminatorKind::Call.switchint TYPE = {:?}", base.projection[0]);
                                        // debug!("++ TerminatorKind::Call.switchint TYPE = {:?}", base.projection[1]);
                                    }
                                    if let rustc_middle::mir::ProjectionElem::Field(Field, T) = base.projection[1] {

                                        // anchor
                                        // let type_of_var = format!("{:#?}",T);
                                        // if type_of_var.contains("anchor_lang::prelude::AccountInfo") {
                                        //     self.has_signer = true;
                                        //     self.is_signer_var = format!("{:#?}",origin_op_field);
                                        // }
                                        
                                        // if debug_switchint { debug!("++ TerminatorKind::Call.switchint Field = {:?}", Field.as_usize());}
                                        posi = Field.as_usize();
                                    }else {
                                        // if debug_switchint { debug!("++ TerminatorKind::Call.switchint Field = None"); }
                                    }
                                }
                                
                            } else {
                                // if debug_switchint { debug!("++ TerminatorKind::Call.switchint TYPE = None");}
                            }
                            
                        }

                        // debug!("!!!!!!!! TerminatorKind::Call.switchint posi = {:?}", posi);

                        // self.arg_posi = i;
                        // self.visit_body(body);
                        // self.arg_posi = prev_arg_posi;

                    }


                }


                // handle the call itself

                if self.check_function {
                    if let Some(callee) = resolve_call_target(func) {
                        if self.tcx.is_mir_available(callee) {
                            let callee_body = self.tcx.optimized_mir(callee);

                            // matching args

                            for arg in callee_body.args_iter(){
                                // debug!("=== handle the call == arg = {:?}", arg);
                                // let origin_op_field = self.ddg.origin_op_source(&arg, false);
                                // let origin_op_type = self.ddg.origin_op_source(&arg, true);
                                // let debug_switchint = true;
                                // let mut posi = 10000000000;
                                // if debug_switchint {debug!("++ TerminatorKind::Call.switchint original source HERE = {:#?}", origin_op_field);}
                                // if !origin_op_field.is_none() {
                                //     if debug_switchint {
                                //         debug!("================ origin_op_field ========");
                                //         debug!("++ TerminatorKind::Call.switchint op = {:?}", arg);
                                //         debug!("++ TerminatorKind::Call.switchint original source = {:#?}", origin_op_field);
                                //     }
                                //     if let Some(dd::DdNode::PartialFieldRef{base}) = origin_op_field {
                                //         if debug_switchint { debug!("++ TerminatorKind::Call.switchint TYPE = {:?}", base.projection);}
                                //         if base.projection.len() > 1 {
                                //             if debug_switchint {
                                //                 debug!("++ TerminatorKind::Call.switchint TYPE = {:?}", base.projection[0]);
                                //                 debug!("++ TerminatorKind::Call.switchint TYPE = {:?}", base.projection[1]);
                                //             }
                                //             if let rustc_middle::mir::ProjectionElem::Field(Field, T) = base.projection[1] {
                                //                 if debug_switchint { debug!("++ TerminatorKind::Call.switchint Field = {:?}", Field.as_usize());}
                                //                 posi = Field.as_usize();
                                //             }else {
                                //                 if debug_switchint { debug!("++ TerminatorKind::Call.switchint Field = None"); }
                                //             }
                                //         }
                                        
                                //     } else {
                                //         if debug_switchint { debug!("++ TerminatorKind::Call.switchint TYPE = None");}
                                //     }
                                    
                                // }

                                // debug!("!!!!!!!!!! posi = {:?}", posi);

                            }

                            self.visit_body(callee_body);
                        }
                    }
                }


                


                // debug!("visit_terminator op_place Operand::Constant: {:#?}", op_place);
                // if op_place.to_string().contains("check_assert") || op_place.to_string().contains("withdraw"){
                //     debug!("visit_terminator: {:#?}", terminator);
                //     debug!("visit_terminator func: {:#?}", func);
                    
                //     for item_id in self.tcx.mir_keys(()) {  // To do: try to use hash mapping 
                //         let def_id = item_id.to_def_id();
                //         match self.tcx.def_kind(def_id) { 
                //             DefKind::Fn | DefKind::AssocFn => {
                //                 if !self.tcx.is_mir_available(def_id) {
                //                     continue;
                //                 }
                //                 if self.tcx.def_path_str(def_id).to_string() == op_place.to_string(){
                //                     debug!("Callgraph-- Visit item: {}.", self.tcx.def_path_str(def_id));
                //                     let body = self.tcx.optimized_mir(def_id);
                //                     self.visit_body(body);
                //                 }
                                
                //             },

                //             _ => {}

                //         }
                //     }

                //     debug!("visit_terminator args: {:#?}", args);
                //     debug!("visit_terminator destination: {:#?}", destination);
                //     debug!("visit_terminator fn_span: {:#?}", fn_span);

                // }
            }
            
            // if func.contains("withdraw") {
                
            // }
            

            // let node = CallNode::FunctionNode {
            //     func,
            //     arg_count: args.len()
            // };  

            // let ni = self.g.add_node(node);
            // let place = Place::from(location).as_ref();
            // self.local_decl_nodes.insert(place, ni);

        } else if let TerminatorKind::SwitchInt { ref discr, ref switch_ty, ref targets } = terminator.kind {

            let mut debug_switchint = false;
            if self.check_function{
                // debug!("------ Call graph Visitor TerminatorKind::SwitchInt ===-- discr = {:?} ", discr);
                // debug!("------ Call graph Visitor TerminatorKind::SwitchInt -- switch_ty = {:?} ", switch_ty);
                // debug!("------ Call graph Visitor TerminatorKind::SwitchInt -- targets = {:?} ", targets);
                
                // let origin_op_field = self.ddg.origin_op_source(discr, false);
                let origin_op_type = self.ddg.origin_op_source(discr, true);


                // if debug_switchint {debug!("++ TerminatorKind::Call.switchint original source HERE = {:#?}", origin_op_field);}
                if debug_switchint {debug!("++ TerminatorKind::Call.switchint original source HERE = {:#?}", origin_op_type);}
                if !origin_op_type.is_none() {
                    if debug_switchint {
                        // debug!("================ origin_op_type ========");
                        // debug!("++ visitor.switchint original source = {:#?}", origin_op_type);
                    }
                    if let Some(dd::DdNode::LocalDecl{local,ty, dbg_info, ..}) = origin_op_type {
                        if debug_switchint { debug!("++ visitor.switchint LOCAL = {:?}, local idx = {:?}, TY = {:?}, DBG = {:?}", local, local.index(), ty, dbg_info);}
                            if dbg_info-1>0 && self.ddg.var_dbg_info.len()> dbg_info-1{ // no debug info for direct visit_body call
                                let mut VarDeg = &self.ddg.var_dbg_info[dbg_info-1].clone();
                                // let mut VarDeg2 = VarDeg.clone();
                                if debug_switchint {
                                    // debug!("++ ddg.var_dbg_info = {:?}", VarDeg);
                                    // debug!("++ ddg.var_dbg_info name = {:?}", VarDeg.name);
                                }
                                
                            }
                        
                        if local.index() == self.arg_posi+1 {
                            self.is_signer = true;
                            
                            // self.is_signer_var = local.to_string();
                            // debug!(" ====  set cg.is_signer to true");

                        }

                    }
                }


            }
            
            
            

        }

        self.super_terminator(terminator, location);
    }

    fn visit_statement(&mut self, statement: &Statement<'tcx>, location: Location) {
        // debug!("visit_statement: {:#?}", statement);
        self.super_statement(statement, location);
    }

    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {
        // debug!("visit_assign: {:#?}", rvalue);
        let left = self.get_left_place_node(place.as_ref());
        self.handle_rvalue(left, rvalue);

        self.super_assign(place, rvalue, location);
    }

}

