#![feature(rustc_private, bool_to_option, stmt_expr_attributes, box_patterns, once_cell)]

extern crate rustc_borrowck;
extern crate rustc_data_structures;
extern crate rustc_demangle;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_index;
extern crate rustc_interface;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_mir_transform;
extern crate rustc_serialize;
extern crate rustc_session;
extern crate rustc_span;

extern crate env_logger;
extern crate cargo_lock;
extern crate semver;

use rustc_driver::{Callbacks, Compilation};

use rustc_interface::{interface, Queries};
use rustc_middle::ty::TyCtxt;

use std::env;

mod assertions;
mod config;
mod dd;
mod ty_confusion;
mod utils;
mod visit;
mod accounts;
mod reporter;
// mod spl_token;
// mod spl_token_v3;
mod version;
mod source_info;
mod callgraph;
mod wpa;
mod checkers;
use checkers::{
    bump_seed,
    cross_program_invocation,
    instruction_id,
    is_signer_checker,
    overflow,
    owner,
    precision,
    staking_validator,
};


mod conf{
    pub mod is_signer_var;
}

// Determines whether a `--flag` is present.
fn has_arg_flag(name: &str) -> bool {
    let mut args = std::env::args().take_while(|val| val != "--");
    args.any(|val| val == name)
}

struct MiravCompilerCalls {
    _mirav_config: config::MiravConfig,
}

fn run_checkers(tcx: TyCtxt) {
    let mut traverser = wpa::WholeProgramTraverser::<overflow::ExternalNumeric>::new(tcx);
    traverser.start();
    
    // let mut traverser = wpa::WholeProgramTraverser::<is_signer_checker::is_signer_checker>::new(tcx);
    // traverser.start();

    let mut traverser = wpa::WholeProgramTraverser::<precision::RoundChecker>::new(tcx);
    traverser.start();
    let mut traverser = wpa::WholeProgramTraverser::<bump_seed::BumpSeed>::new(tcx);
    traverser.start();
    let mut traverser = wpa::WholeProgramTraverser::<staking_validator::StakingValidator>::new(tcx);
    traverser.start();
    // let mut traverser = wpa::WholeProgramTraverser::<owner::OwnerCheck>::new(tcx);
    // traverser.start();
    let mut traverser = wpa::WholeProgramTraverser::<instruction_id::InstructionIdChecker>::new(tcx);
    traverser.start();

    if !version::check_version("spl-token", "3.1.1") {
        let mut traverser = wpa::WholeProgramTraverser::<cross_program_invocation::ProgramId>::new(tcx);
        traverser.start();
    }
    
    println!("Finished traversal");
}

impl rustc_driver::Callbacks for MiravCompilerCalls {
    fn after_analysis<'tcx>(
        &mut self,
        compiler: &interface::Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        compiler.session().abort_if_errors();

        // debug!("Manifest path: {:?}", env::var("CARGO_MANIFEST_DIR"));
        // debug!("Current dir: {:?}", env::current_dir());

        queries.global_ctxt().unwrap().peek_mut().enter(|tcx| {
            // Set the crate name
            let current_crate = env::var("CARGO_CRATE_NAME").unwrap_or("No crate name".to_string());

            reporter::REPORT.lock().unwrap().set_proj(current_crate.clone());

            run_checkers(tcx);

            // assertions::collect_all_assertions(tcx);
            ty_confusion::check_confusion_types(tcx);
            // spl_token_v3::check_spl_tokens(tcx);

            let mut json_file = std::env::var("PWD").unwrap();
            json_file.push_str("/report.json");

            reporter::REPORT.lock().unwrap().export_json(json_file).unwrap();

            std::process::exit(i32::try_from(0).expect(""));
        });

        compiler.session().abort_if_errors();
        Compilation::Stop
    }
}

struct MiravBeRustCompilerCalls;

impl rustc_driver::Callbacks for MiravBeRustCompilerCalls {}

fn run_compiler(
    mut args: Vec<String>,
    callbacks: &mut (dyn Callbacks + Send),
    insert_default_args: bool,
) {
    if insert_default_args {
        // Some options have different defaults in Miri than in plain rustc; apply those by making
        // them the first arguments after the binary name (but later arguments can overwrite them).
        args.splice(
            1..1,
            config::MIRAV_DEFAULT_ARGS.iter().map(ToString::to_string),
        );
    }

    let exit_code = rustc_driver::catch_with_exit_code(move || {
        rustc_driver::RunCompiler::new(&args, callbacks).run()
    });
    std::process::exit(exit_code);
}

fn main() {
    env_logger::init();
    rustc_driver::install_ice_hook();

    let _mirav_config = config::MiravConfig::default();
    let mut rustc_args = vec![];
    for arg in env::args() {
        rustc_args.push(arg);
    }

    let is_target = env::var("CARGO_PRIMARY_PACKAGE").is_ok();
    // println!("is_target: {}",is_target); // False
    if !is_target {
        rustc_driver::init_rustc_env_logger();

        // When compiling build_scripts, no sysroot is provided
        // and it reports failing to find crate std, so we have to
        // explicity provide one from xargo (always the HOST arch).
        if !has_arg_flag("--sysroot") {
            rustc_args.push("--sysroot".to_string());
            let host_sysroot_path = format!("{}/.xargo/HOST", env::var("HOME").unwrap());
            rustc_args.push(host_sysroot_path);
        }

        // Build it as usual.
        return run_compiler(rustc_args, &mut MiravBeRustCompilerCalls {}, true);
    }

    // Run the analysis on the executable
    run_compiler(rustc_args, &mut MiravCompilerCalls { _mirav_config }, true);
}
