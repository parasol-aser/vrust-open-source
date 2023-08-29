//! Vairous visitors.


use crate::dd;

use either::Either;
use hashbrown::HashMap;
use log::debug;
use rustc_middle::{
    mir::{
        visit::Visitor, BasicBlock, Body, Constant, LocalKind, Location, Operand, Place, PlaceElem,
        PlaceRef, Rvalue, Statement, StatementKind, Terminator, TerminatorKind, BinOp,
    },
    ty::{AdtDef, FieldDef, TyCtxt, TyKind},
};
use rustc_span::Symbol;

pub trait DefUseChainTrait<'tcx> {
    fn get_def(
        &self,
        place: PlaceRef<'tcx>,
        body: &Body<'tcx>,
        dbg_info: &HashMap<PlaceRef<'tcx>, usize>,
    ) -> Vec<Either<(Symbol, Vec<Symbol>), Constant>>;

    fn get_use(
        &self,
        place: PlaceRef<'tcx>,
        body: &Body<'tcx>,
        dbg_info: &HashMap<PlaceRef<'tcx>, usize>,
    ) -> &[Location];
}

// existential type DefUseChain<'tcx> : DefUseChainTrait<'tcx>;

pub struct BodyVisitor<'tcx, F, D>
where
    F: Copy
        + Fn(
            TyCtxt<'tcx>,
            &Operand<'tcx>,
            &Vec<Operand<'tcx>>,
            &Option<(Place<'tcx>, BasicBlock)>,
            &mut Vec<D>,
        ) -> (),
{
    pub tcx: TyCtxt<'tcx>,
    pub f: Option<F>,
    pub defs: HashMap<PlaceRef<'tcx>, Vec<Location>>,
    pub switchint: Vec<Operand<'tcx>>,
    pub focus: Vec<D>,
}

fn get_all_fields<'tcx>(
    adt_def: &'tcx AdtDef,
    projections: &'tcx [PlaceElem<'tcx>],
) -> Vec<Symbol> {
    // debug!("Adt: {:?}, Fields: {:?}", adt_def, projections);
    let all_fields: Vec<&FieldDef> = adt_def.all_fields().collect();
    let mut field_indices = Vec::new();
    for proj in projections {
        match proj {
            PlaceElem::Field(field, _field_ty) => {
                field_indices.push(field);
            }
            _ => {}
        }
    }
    field_indices
        .iter()
        .map(|idx| all_fields[idx.index()].name)
        .collect()
}

/// Check if the symbol is from assert_eq, either
/// "left_val" or "right_val".
fn is_from_assert_eq_macro(var: &str) -> bool {
    if var == "left_val" || var == "right_val" {
        return true;
    }
    false
}

impl<'tcx, F, D> BodyVisitor<'tcx, F, D>
where
    F: Copy
        + Fn(
            TyCtxt<'tcx>,
            &Operand<'tcx>,
            &Vec<Operand<'tcx>>,
            &Option<(Place<'tcx>, BasicBlock)>,
            &mut Vec<D>,
        ) -> (),
{
    fn backtrack(
        &self,
        place: PlaceRef<'tcx>,
        body: &Body<'tcx>,
        dbg_info: &HashMap<PlaceRef<'tcx>, usize>,
    ) -> Option<Either<(Symbol, Vec<Symbol>), Constant<'tcx>>> {
        // debug!("backtrack {:?}", place);
        let base_place = Place::from(place.local).as_ref();
        if let LocalKind::Var = body.local_kind(place.local) {
            let ty = base_place.ty(body, self.tcx).ty.peel_refs();
            // debug!("Local: {:?}, ty: {:?}", place.local, ty.kind());
            if let TyKind::Adt(adt_def, _) = ty.kind() {
                // debug!("=========== Adt: Type: {}, place: {:?}", self.tcx.def_path_str(adt_def.did), base_place);
                // let ty_name = self.tcx.def_path_str(adt_def.did);
                // if ty_name == "solana_program::account_info::AccountInfo"
                // || ty_name == "Wallet"
                // {
                // TODO: "left_val"
                let src_var_idx = dbg_info.get(&base_place).unwrap();
                let src_var = &body.var_debug_info[*src_var_idx];
                let src_var_name = src_var.name;
                // TODO: Erfan
                if !is_from_assert_eq_macro(src_var_name.as_str()) {
                    let fields = get_all_fields(adt_def, place.projection);
                    return Some(Either::Left((src_var.name, fields)));
                }
                // }
            }
        }
        let mut base = place;
        if !self.defs.contains_key(&base) {
            base = base_place;
        }
        if let Some(locs) = self.defs.get(&base) {
            let loc = locs[0];
            let stmt = body.stmt_at(loc);
            if let Some(stmt) = stmt.left() {
                match &stmt.kind {
                    StatementKind::Assign(box (_left, right)) => match right {
                        Rvalue::Use(r_op) => match r_op {
                            Operand::Constant(box con) => {
                                return Some(Either::Right(*con));
                            }
                            Operand::Copy(r_place) | Operand::Move(r_place) => {
                                return self.backtrack(r_place.as_ref(), body, dbg_info);
                            }
                        },
                        Rvalue::Ref(_, _, from) => {
                            return self.backtrack(from.as_ref(), body, dbg_info);
                        },
                        _ => {}
                    },
                    _ => {}
                }
            } else {
                let terminator = stmt.right().unwrap();
                // debug!("------ Visitor terminator -- {:?}", terminator);
                if let TerminatorKind::Call {
                    ref func, ref args, ..
                } = terminator.kind
                {
                    if let Operand::Constant(box c) = func {
                        if let TyKind::FnDef(def_id, _) = c.ty().kind() {
                            let call_name = self.tcx.def_path_str(*def_id);
                            match call_name.as_str() {
                                "std::ops::Deref::deref" => {
                                    assert_eq!(
                                        args.len(),
                                        1,
                                        "Deref() must have a single operand."
                                    );
                                    if let Operand::Copy(arg_place) | Operand::Move(arg_place) =
                                        args[0]
                                    {
                                        return self.backtrack(arg_place.as_ref(), body, dbg_info);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                } else if let TerminatorKind::SwitchInt {
                    ref discr, ref targets, ..
                } = terminator.kind{
                    // debug!("------ Visitor SwitchInt -- discr = {:?} ", discr);
                    // debug!("------ Visitor SwitchInt -- targets = {:?} ", targets);

                }

            }
        }
        None
    }
}

impl<'tcx, F, D> DefUseChainTrait<'tcx> for BodyVisitor<'tcx, F, D>
where
    F: Copy
        + Fn(
            TyCtxt<'tcx>,
            &Operand<'tcx>,
            &Vec<Operand<'tcx>>,
            &Option<(Place<'tcx>, BasicBlock)>,
            &mut Vec<D>,
        ) -> (),
{
    fn get_def(
        &self,
        place: PlaceRef<'tcx>,
        body: &Body<'tcx>,
        dbg_info: &HashMap<PlaceRef<'tcx>, usize>,
    ) -> Vec<Either<(Symbol, Vec<Symbol>), Constant>> {
        // TODO: we must do backtrack w.r.t. to CFG
        let mut res = Vec::new();
        if let Some(from) = self.backtrack(place, body, dbg_info) {
            res.push(from);
        }
        res
    }

    fn get_use(
        &self,
        _place: PlaceRef<'tcx>,
        _body: &Body<'tcx>,
        _dbg_info: &HashMap<PlaceRef<'tcx>, usize>,
    ) -> &[Location] {
        todo!()
    }
}

impl<'tcx, F, D> Visitor<'tcx> for BodyVisitor<'tcx, F, D>
where
    F: Copy
        + Fn(
            TyCtxt<'tcx>,
            &Operand<'tcx>,
            &Vec<Operand<'tcx>>,
            &Option<(Place<'tcx>, BasicBlock)>,
            &mut Vec<D>,
        ) -> (),
{
    fn visit_body(&mut self, body: &Body<'tcx>) {
        // debug!(
        //     "Visit body: {}",
        //     self.tcx.def_path_str(body.source.def_id())
        // );
        self.super_body(body);
    }

    fn visit_rvalue(&mut self, rvalue: &Rvalue<'tcx>, location: Location) {
        self.super_rvalue(rvalue, location);
    }

    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        if let TerminatorKind::Call {
            ref func,
            ref args,
            ref destination,
            ..
        } = terminator.kind
        {
            // debug!("------ Visitor TerminatorKind::Call ===-- func = {:?} ", func);
            // debug!("------ Visitor TerminatorKind::Call -- args = {:?} ", args);
            // debug!("------ Visitor TerminatorKind::Call -- destination = {:?} ", destination);
            // Function call with return is also a def.
            if let Some((destination, _bb)) = destination {
                self.defs
                    .entry(destination.as_ref())
                    .or_default()
                    .push(location);
            }

            if let Some(f) = self.f {
                f(self.tcx, func, args, destination, &mut self.focus);
            }
        }
        // else if let TerminatorKind::Assert {
        //     ref cond, ref msg, ..
        // } = terminator.kind {
        //     debug!("Cond: {:?}, msg: {:?}", cond, msg);
        // }

        else if let TerminatorKind::SwitchInt { ref discr, ref switch_ty, ref targets } = terminator.kind {

            // debug!("------ Visitor TerminatorKind::SwitchInt ===-- discr = {:?} ", discr);
            // debug!("------ Visitor TerminatorKind::SwitchInt -- switch_ty = {:?} ", switch_ty);
            // debug!("------ Visitor TerminatorKind::SwitchInt -- targets = {:?} ", targets);

            self.switchint.push(discr.clone());
            
//             if let Operand::Copy(arg_place) | Operand::Move(arg_place) =
//             discr
//             {
//                 debug!("------ Visitor TerminatorKind::SwitchInt -- arg_place = {:?} ",  arg_place.as_local());  //  (arg_place.as_ref(), body, dbg_info);
//                 // self.switchint.push(arg_place.as_ref());


//             }
            
            

        }
        self.super_terminator(terminator, location);
    }

    fn visit_statement(&mut self, statement: &Statement<'tcx>, location: Location) {
        self.super_statement(statement, location);
    }

    fn visit_assign(&mut self, place: &Place<'tcx>, _rvalue: &Rvalue<'tcx>, location: Location) {
        self.defs.entry(place.as_ref()).or_default().push(location);
        self.super_assign(place, _rvalue, location);
    }
}  
