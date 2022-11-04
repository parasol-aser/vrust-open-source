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
    reporter::{Report},
};

pub fn check_version(crate_name: &str, target_version: &str) -> bool { //(bool, String) {
    let args: Vec<String> = env::args().collect();
    let key = "PWD";
    let mut version_string = String::new();
    let v: Vec<&str> = target_version.split('.').collect();
    assert_eq!(v.len(), 3);
    let major = v[0].to_string().parse::<u64>().unwrap();
    let minor = v[1].to_string().parse::<u64>().unwrap();
    let patch = v[2].to_string().parse::<u64>().unwrap();

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
                        if package.name.as_str().contains(crate_name)  {
                            debug!("crate version: {:?}", package.version);
                            let version = package.version.clone();
                            version_string.push_str(&version.major.to_string());
                            version_string.push_str(".");
                            version_string.push_str(&version.minor.to_string());
                            version_string.push_str(".");
                            version_string.push_str(&version.patch.to_string());
                            if version.major > major {
                                //return (true, version_string);
                                return true;
                            }
                            else if (version.major == major && version.minor > minor) {
                                return true;
                            }
                            else if (version.major == major && version.minor == minor && version.patch >= patch) {
                                return true;
                            }
                            else {
                                //warn!("Unsafe version of {:?} used, which may cause security flaws if not handled carefully.", crate_name);
                                //return (false, version_string);
                                // Report::new_vuln_crate_dep(
                                //     //tcx,
                                //     "Vulnerable crate dependency".to_string(),
                                //     "Informational".to_string(),
                                //     "Cargo.lock".to_string(),
                                //     "Cargo.lock".to_string(),
                                //     "Cargo.lock".to_string(),
                                //     "UnResolved".to_string(),
                                //     "GitHub Link to be added.".to_string(),
                                //     Some("message"),
                                //     "Update to newer version.".to_string(),
                                //     &crate_name.to_string(),
                                //     &version_string,
                                //     &target_version.to_string(),
                                // );
                                return false;
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
    // version_string.push_str("unknown");
    // (true, version_string)
    true
}
