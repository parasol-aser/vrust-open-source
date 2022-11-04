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
use rustc_hir::def_id::DefId;
use rustc_middle::mir::{Constant, ConstantKind, VarDebugInfo};
use rustc_middle::{
    mir::{
        visit::Visitor, BinOp, Body, Field, Local, Location, Operand, Place, PlaceRef,
        ProjectionElem, Rvalue, Statement, Terminator, TerminatorKind, VarDebugInfoContents,
    },
    ty::{Ty, TyCtxt},
};
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use rustc_hir::def::DefKind;

use crate::reporter;
use crate::reporter::IntegerCveType;
use crate::source_info;

#[derive(Debug, Clone, Copy)]
pub enum DdNode<'tcx> {
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
        def_id: DefId,
    },

    /// Constant. This is useful when handling
    /// constant op (eq, neq) in assert_* macros.
    Constant { val: Constant<'tcx> },
}

impl<'tcx> Display for DdNode<'tcx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PartialFieldRef { base } => write!(f, "{:?}", base),
            Self::LocalDecl { local, .. } => write!(f, "{:?}", local),
            Self::Constant { val } => write!(f, "{:?}", val.literal),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DdEdge {
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

impl Display for DdEdge {
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
        node: &DdNode<'tcx>,
        var_dbg_info: &Vec<VarDebugInfo<'tcx>>,
    ) -> bool {
        match node {
            DdNode::LocalDecl { dbg_info, .. } => {
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
pub struct DataDepsGraph<'tcx, F = DefaultPredicateFn>
where
    F: FnOnce() -> bool,
{
    pub tcx: TyCtxt<'tcx>,
    pub g: DiGraph<DdNode<'tcx>, DdEdge>,
    pub place_dbg_info_map: HashMap<PlaceRef<'tcx>, usize>,
    pub var_dbg_info: Vec<VarDebugInfo<'tcx>>,
    heuristic: HeuristicFilter<F>,
    local_decl_nodes: HashMap<PlaceRef<'tcx>, NodeIndex>,
    /// Place nodes that are not local decl, e.g., projections
    /// on local decl
    other_nodes: HashMap<PlaceRef<'tcx>, NodeIndex>,
    /// Constant nodes
    constant_nodes: HashMap<ConstantKind<'tcx>, NodeIndex>,
}

impl<'tcx, F> DataDepsGraph<'tcx, F>
where
    F: FnOnce() -> bool,
{
    pub fn new(body: &Body<'tcx>, tcx: TyCtxt<'tcx>, heuristic: HeuristicFilter<F>) -> Self {
        let mut ddg = Self {
            g: DiGraph::new(),
            var_dbg_info: body.var_debug_info.clone(),
            place_dbg_info_map: get_dbg_info_mapping(body),
            tcx,
            heuristic,
            local_decl_nodes: HashMap::new(),
            other_nodes: HashMap::new(),
            constant_nodes: HashMap::new(),
        };
        ddg.visit_body(body);

        ddg
    }

    pub fn dump<P: AsRef<Path>>(&self, out: P) -> std::io::Result<()> {
        let dot = Dot::with_config(&self.g, &[Config::EdgeNoLabel]);
        let mut file = File::create(out)?;
        file.write_all(dot.to_string().as_bytes())?;
        Ok(())
    }

    pub fn origin_op_source(&self, op: &Operand<'tcx>, pass_field: bool) -> Option<DdNode<'tcx>> {
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
    ) -> Vec<Option<DdNode<'tcx>>> {
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
    pub fn origin_source(&self, place: PlaceRef<'tcx>, pass_field: bool) -> Option<DdNode<'tcx>> {
        // debug!("origin source: {:?}", place);
        let start = self
            .local_decl_nodes
            .get(&place)
            .or_else(|| self.other_nodes.get(&place));
        // debug!("++=== start = {:?}", start);
        if let Some(start) = start {
            // debug!("++=== start enter");
            let mut goal: Option<DdNode<'tcx>> = None;
            let visit_fn = |event: DfsEvent<NodeIndex>| {
                if let DfsEvent::TreeEdge(_u, v) = event {
                    let v_node = self.g.node_weight(v).unwrap();
                    // debug!("traverse node: {:?}", v_node);
                    if let DdNode::LocalDecl { dbg_info: 1.., .. } = v_node {
                        if self.heuristic.filter(v_node, &self.var_dbg_info) {
                            goal = Some(*v_node);
                            return Control::Break(v);
                        }
                    } else if let DdNode::Constant { .. } = v_node {
                        goal = Some(*v_node);
                        return Control::Break(v);
                    } else if let DdNode::PartialFieldRef { .. } = v_node {
                        //  && pass_field == false
                        if pass_field == false {
                            goal = Some(*v_node);
                            return Control::Break(v);
                        }
                    }
                }
                Control::Continue
            };
            if !pass_field {
                let g = EdgeFiltered::from_fn(&self.g, |e| match e.weight() {
                    DdEdge::FieldRef(_) => false,
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
    pub fn depends_source(&self, location: Location, body: &Body<'tcx>) -> Vec<DdNode<'tcx>> {
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
    ) -> Vec<DdNode<'tcx>> {
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
    fn depends_branch_source(&self, location: Location, body: &Body<'tcx>) -> Vec<DdNode<'tcx>> {
        let mut nodes = Vec::new();
        for pred in WithPredecessors::predecessors(body, location.block) {
            if let Some(terminator) = body.basic_blocks()[pred].terminator.as_ref() {
                match terminator.kind {
                    TerminatorKind::SwitchInt { ref discr, .. } => {
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
            let node = DdNode::PartialFieldRef { base: place };
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
            let node = DdNode::PartialFieldRef { base: place };
            let ni = self.g.add_node(node);
            // Since this is not local decl, store it in other_nodes.
            self.other_nodes.insert(place, ni);
            ni
        }
    }

    fn handle_place(&mut self, left: NodeIndex, place: PlaceRef<'tcx>, edge: Option<DdEdge>) {
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
                                .add_edge(cur_node_i, base_node_i, DdEdge::FieldRef(field));
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
                .add_edge(cur_node_i, last_node_i, DdEdge::IgnoredProjection);
        }
    }

    fn handle_constant(&mut self, left: NodeIndex, c: Constant<'tcx>, edge: DdEdge) {
        let node = DdNode::Constant { val: c };
        let ni = self.g.add_node(node);
        self.constant_nodes.insert(c.literal, ni);
        self.g.add_edge(left, ni, edge);
    }

    fn handle_op(&mut self, left: NodeIndex, op: &Operand<'tcx>, edge: DdEdge) {
        match op {
            Operand::Move(from) | Operand::Copy(from) => {
                self.handle_place(left, from.as_ref(), Some(edge))
            }
            Operand::Constant(box c) => {
                self.handle_constant(left, *c, edge);
            }
        }
    }

    fn handle_binary_op(
        &mut self,
        operator: &BinOp,
        operands: &Box<(Operand<'tcx>, Operand<'tcx>)>,
        location: Location,
    ) {
        let origin = self.origin_op_source(&operands.1, true);
        if !origin.is_none() {
            if let Some(DdNode::LocalDecl {
                local,
                ty,
                dbg_info,
                arg_count,
                def_id,
            }) = origin
            {
                let body = self.tcx.optimized_mir(def_id);

                // check if this local variable is a parameter of a function
                if local.index() >= 1 && local.index() <= body.arg_count {
                    if (*operator == BinOp::Add
                        || *operator == BinOp::Mul
                        || *operator == BinOp::Sub
                        || *operator == BinOp::Div)
                    {
                        // let call_name = self.tcx.def_path_str(def_id);
                        // let span = body.basic_blocks()[location.block].statements
                        //     [location.statement_index]
                        //     .source_info
                        //     .span;
                        // let error_type = if (*operator == BinOp::Add || *operator == BinOp::Mul) {
                        //     reporter::IntegerCveType::Overflow
                        // } else {
                        //     reporter::IntegerCveType::Underflow
                        // };

                        // warn!(
                        //     "Unchecked {:?} for external variable name {:?} ({:?})",
                        //     operator,
                        //     self.var_dbg_info[local.index() - 1].name,
                        //     span
                        // );
                        // // reporter::Report::new_integer_cve(self.tcx, error_type, call_name, span, None);

                        // reporter::Report::new_integer_cve(
                        //     self.tcx,
                        //     IntegerCveType::Overflow,
                        //     "Integer Overflow".to_string(),
                        //     "Critical".to_string(),
                        //     self.tcx.def_path_str(def_id),
                        //     source_info::get_source_lines(self.tcx, span).unwrap_or("".to_string()),
                        //     source_info::get_source_lines(self.tcx, body.span).unwrap_or("".to_string()),
                        //     "UnResolved".to_string(),
                        //     "GitHub Link to be added.".to_string(),
                        //     span,
                        //     Some("Description of the bug here."),
                        //     "Some alleviation steps here.".to_string(),
                        // );
                    }
                }
            }
        }
    }

    fn handle_rvalue(&mut self, left: NodeIndex, rvalue: &Rvalue<'tcx>, location: Location) {
        match rvalue {
            Rvalue::Use(op) => self.handle_op(left, op, DdEdge::Assign),
            Rvalue::Ref(_, _, op) => self.handle_place(left, op.as_ref(), Some(DdEdge::Ref)),
            Rvalue::Cast(_, op, _ty) => self.handle_op(left, op, DdEdge::Cast),
            Rvalue::BinaryOp(op, ops) => self.handle_binary_op(op, ops, location),
            Rvalue::UnaryOp(_, op) => {
                // debug!("++===== visit handle_rvalue: UnaryOp: {:?}", op);
                self.handle_op(left, op, DdEdge::Assign);
            }
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

impl<'tcx, F> Visitor<'tcx> for DataDepsGraph<'tcx, F>
where
    F: FnOnce() -> bool,
{
    fn visit_body(&mut self, body: &Body<'tcx>) {
        // In current MIR Visitor implementation, local decl is visited after
        // basic block data, so we have to do this here manually.

        for local in body.local_decls.indices() {
            let local_decl = &body.local_decls[local];
            let place = Place::from(local).as_ref();
            let node = DdNode::LocalDecl {
                local,
                ty: local_decl.ty,
                dbg_info: *self.place_dbg_info_map.get(&place).unwrap_or(&0),
                arg_count: body.arg_count,
                def_id: body.source.def_id(),
            };

            let ni = self.g.add_node(node);
            self.local_decl_nodes.insert(place, ni);
        }

        self.super_body(body);
    }

    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        if let TerminatorKind::Call {
            ref func,
            ref args,
            ref destination,
            ref cleanup,
            ref from_hir_call,
            ref fn_span,
        } = terminator.kind
        {
            if let Operand::Copy(op_place) = func {
                // debug!("visit_terminator op_place Operand::Copy: {:#?}", op_place);
            }
            if let Operand::Move(op_place) = func {
                // debug!("visit_terminator op_place Operand::Move: {:#?}", op_place);
            }
            if let Operand::Constant(op_place) = func {
                // debug!("visit_terminator op_place Operand::Constant: {:#?}", op_place);
                if op_place.to_string().contains("check_assert")
                    || op_place.to_string().contains("withdraw")
                {
                    // debug!("visit_terminator: {:#?}", terminator);
                    // debug!("visit_terminator func: {:#?}", func);

                    for item_id in self.tcx.mir_keys(()) {
                        // To do: try to use hash mapping
                        let def_id = item_id.to_def_id();
                        match self.tcx.def_kind(def_id) {
                            DefKind::Fn | DefKind::AssocFn => {
                                if !self.tcx.is_mir_available(def_id) {
                                    continue;
                                }
                                if self.tcx.def_path_str(def_id).to_string() == op_place.to_string()
                                {
                                    // debug!("Callgraph-- Visit item: {}.", self.tcx.def_path_str(def_id));
                                    let body = self.tcx.optimized_mir(def_id);
                                    self.visit_body(body);
                                }
                            }

                            _ => {}
                        }
                    }

                    // debug!("visit_terminator args: {:#?}", args);
                    // debug!("visit_terminator destination: {:#?}", destination);
                    // debug!("visit_terminator fn_span: {:#?}", fn_span);
                }
            }
        }

        self.super_terminator(terminator, location);
    }

    fn visit_statement(&mut self, statement: &Statement<'tcx>, location: Location) {
        self.super_statement(statement, location);
    }

    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {
        let left = self.get_left_place_node(place.as_ref());
        self.handle_rvalue(left, rvalue, location);

        self.super_assign(place, rvalue, location);
    }
}
