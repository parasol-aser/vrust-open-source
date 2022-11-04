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

use crate::{
    dd::{self, HeuristicFilter, ASSERT_MACRO_FILTER},
    visit::{self, DefUseChainTrait}, accounts::get_accounts,
    wpa::{self, StateTransitor, StateMachine, FullPlaceRef},
    reporter::{Report},
};


fn check_spl_version() -> (bool, String) {
    let args: Vec<String> = env::args().collect();
    let key = "PWD";
    let mut version_string = String::new();
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
                            version_string.push_str(&version.major.to_string());
                            version_string.push_str(".");
                            version_string.push_str(&version.minor.to_string());
                            version_string.push_str(".");
                            version_string.push_str(&version.patch.to_string());
                            if version.major > 3 || (version.major == 3 && version.minor > 1) || (version.major == 3 && version.minor == 1 && version.patch >=1) {
                                return (true, version_string);
                            }
                            else {
                                warn!("Unsafe version of spl token crate version used, which may cause security flaws if not handled carefully.");
                                return (false, version_string);
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
    version_string.push_str("unknown");
    (true, version_string)
}

pub fn check_spl_tokens<'tcx>(tcx: TyCtxt<'tcx>) {
    let mut safe_version = (false, String::new());
    safe_version = check_spl_version();
    if !safe_version.0 {

        // let mut traverser = wpa::WholeProgramTraverser::<wpa::ExternalNumric>::new(tcx);
        // traverser.start();

        // Report::new_vuln_crate_dep("spl-token", &safe_version.1, "3.1.1", None);
        Report::new_vuln_crate_dep(
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
            &safe_version.1, 
            "3.1.1", 

        );
    }
}