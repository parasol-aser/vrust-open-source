//! Generating a JSON report for all checks.

use std::fmt::Display;
use std::{fs::File, lazy::SyncLazy, path::Path, sync::Mutex};

use chrono::prelude::*;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::Body;
use rustc_middle::ty::TyCtxt;
use rustc_span::Span;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet}; 

use std::time::{SystemTime, UNIX_EPOCH};

use log::{debug, warn};

use crate::source_info;
use crate::ty_confusion::StructDefLayout;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum IntegerCveType {
    Underflow,
    Overflow,
    DivByZero,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum MissingCheckerCveType {
    is_signer,
    is_owner,
    wormhole,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum SplVersionType {
    DecrepitedVersion,
}

// #[serde(tag = "type")]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
// #[serde(untagged)]
pub enum VulnerabilityType {
    IntegerFlow,
    MissingSignerCheck,
    MissingOwnerCheck,
    TypeConfusion,
    CrossProgramInvocation,
    Precision,
    BumpSeed,
    StakingValidator,
    MissingKeyCheck,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vulnerability {
    #[serde(rename = "id")]
    id: String,

    
    #[serde(rename = "category")]
    category: VulnerabilityType,

    #[serde(rename = "severity")]
    severity: String,

    /// The function that contains the vulnerable code
    #[serde(rename = "location")]
    location: String, // func

    #[serde(rename = "code")]
    code: String,

    #[serde(rename = "context")]
    context: String,

    /// Source code of site of the cve
    #[serde(rename = "callstack")]
    callstack: String, // code

    #[serde(rename = "status")]
    status: String,

    /// Error message
    #[serde(rename = "description")]
    msg: String,

    #[serde(rename = "link")]
    link: String,

    #[serde(rename = "alleviation")]
    alleviation: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Report {
    #[serde(rename = "id")]
    pub id: Option<String>,

    #[serde(rename = "user")]
    pub user: Option<String>,

    /// Target project being analyzed
    #[serde(rename = "crate")]
    pub proj: Option<String>,
    /// Target project being analyzed
    #[serde(rename = "git-loc")]
    pub gitloc: Option<String>,

    /// Time of the report
    #[serde(rename = "timestamp")]
    pub time: Option<String>,

    // Statistics
    // #[serde(skip_serializing)]
    pub int_cnt: usize,
    // #[serde(skip_serializing)]
    pub chk_cnt: usize,
    // #[serde(skip_serializing)]
    pub typ_cnt: usize,
    // #[serde(skip_serializing)]
    pub oth_cnt: usize,
    pub total: usize,
    #[serde(skip_serializing)]
    pub filtered: Vec<String>,
    #[serde(skip_serializing)]
    pub filter_result: bool,
    #[serde(skip_serializing)]
    pub blacklist: bool,

    /// Map of diagnostics for each source code file.
    /// NOTE: serde does not work for hashbrow HashMap by default.
    #[serde(rename = "errors")]
    pub vulns_map: Vec<Vulnerability>,
    #[serde(skip_serializing)]
    pub vulns_code: HashSet<String>,
}

impl Default for Report {
    fn default() -> Self {
        Report {
            id: Some(String::from("VRust")),
            user: Some(String::from("O2Lab VRust Team")),
            proj: Some(String::from("VRust")),
            gitloc: Some(String::from("https://github.com/parasol-aser/vrust")),
            filtered: Vec::new(),
            filter_result: false,
            blacklist: false,
            vulns_map: Vec::new(),
            time: {
                // Local::now()
                match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(n) => Some(n.as_secs().to_string()),
                    Err(_) => panic!("SystemTime before UNIX EPOCH!"),
                }
            },
            int_cnt: 0,
            chk_cnt: 0,
            typ_cnt: 0,
            oth_cnt: 0,
            total: 0,
            vulns_code: HashSet::new(),
        }
    }
}

impl Report {
    pub fn new_bug<'tcx>(
        tcx: TyCtxt<'tcx>,
        category: VulnerabilityType,
        severity: String,
        func: String,
        code: String,    // the statement of the Vulnerability
        context: String, // the function context of the Vulnerability (function body)
        callstack: String,
        status: String,
        link: Option<String>,
        msg: Option<String>,
        alleviation: Option<String>,
    ) {
        let vuln = Vulnerability {
            id: (REPORT.lock().unwrap().total).to_string(),
            category,
            severity,
            location: func,
            code: code.clone(),
            context,
            callstack,
            status,
            msg: msg.unwrap_or_default(),
            link: link.unwrap_or_default(),
            alleviation: alleviation.unwrap_or_default(),
        };
        if !REPORT.lock().unwrap().vulns_code.insert(code) {
            match category {
                VulnerabilityType::IntegerFlow | VulnerabilityType::MissingKeyCheck => return,
                _ => {}
            }
        }
        match category {
            VulnerabilityType::IntegerFlow => {
                REPORT.lock().unwrap().int_cnt += 1;
            }
            VulnerabilityType::TypeConfusion => {
                REPORT.lock().unwrap().typ_cnt += 1;
            }
            VulnerabilityType::MissingKeyCheck => {
                REPORT.lock().unwrap().chk_cnt += 1;
            }
            _ => {
                REPORT.lock().unwrap().oth_cnt += 1;
            }
        }
        REPORT.lock().unwrap().vulns_map.push(vuln);
        REPORT.lock().unwrap().total += 1;
    }

    pub fn set_proj<S: Into<String>>(&mut self, proj: S) {
        self.proj = Some(proj.into());
    }

    /// Export all diagnostic messages to a JSON file
    pub fn export_json<P: AsRef<Path>>(&self, out: P) -> std::io::Result<()> {
        let writer = File::create(out)?;
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }

    pub fn reset_filter<'tcx>() {
        REPORT.lock().unwrap().filtered = Vec::new();
    }

    pub fn filter_callstack<'tcx>(s: String) {
        REPORT.lock().unwrap().filter_result = false;

        if REPORT.lock().unwrap().filtered.contains(&s) {
            REPORT.lock().unwrap().filter_result = true;
        } else {
            REPORT.lock().unwrap().filtered.push(s.clone());
        }
        debug!(
            "filter_callstack: {}, {}",
            &s,
            REPORT.lock().unwrap().filter_result
        );
    }

    pub fn get_filtered() -> bool {
        REPORT.lock().unwrap().filter_result
    }

    pub fn get_blacklist() -> bool {
        REPORT.lock().unwrap().blacklist
    }

    pub fn get_blacklist_func() -> &'static [&'static str] {
        const BLACKLIST_FUNC: &'static [&'static str] = &[
            "clone",
            "from_account_info",
            "invoke_signed",
            "data_len",
            "data_is_empty",
            "__idl_create_account",
            // Ignore all integer overflow caused by this function
            "solana_program::sysvar::instructions::deserialize_instruction",
            // TODO: This is from uint crate, probably we
            // should remove all errors reported in it
            "overflowing_mul",
        ];
        BLACKLIST_FUNC
    }

    pub fn in_blacklist(var_name: String) -> bool {
        let auth_names = Report::get_blacklist_func();
        for auth_name in auth_names {
            if var_name.contains(auth_name) {
                return true;
            }
        }
        false
    }

    pub fn call_stack_formatter<'tcx>(tcx: TyCtxt<'tcx>, all_def_id: &[DefId]) -> String {
        // we can also: call call_stack_formatter_span
        let mut callstack = String::new();
        let mut incident = 0;

        let mut call_stack_filter = String::new();
        REPORT.lock().unwrap().blacklist = false;
        for def_id in all_def_id {
            for i in 0..incident {
                callstack.push_str("\t");
            }
            incident += 1;
            if incident > 6 {
                incident = 0;
            }
            let body = tcx.optimized_mir(*def_id);
            let span = body.span;
            let source_file = source_info::get_source_file(tcx, span).unwrap_or("".to_string());

            let source_map = tcx.sess.source_map();
            let loc_info = source_map.span_to_diagnostic_string(span);

            let mut func_name = &format!("{:#?}", tcx.def_path_str(*def_id));
            let mut function_name = func_name.replace("\"", "");

            // debug!("========= func_name: {}", func_name);

            callstack.push_str(&format!("fn {}(){{", function_name));
            callstack.push_str(&format!("// {} }}\n", loc_info));
            // let vec: Vec<&str> = source_file.split("{").collect();
            // let file = vec[0];
            // callstack.push_str(vec[0]);
            // callstack.push_str("\n");

            // callstack.push_str(source_info::get_source_lines(tcx, span).unwrap_or("".to_string()).split("{").collect()[0].as_str() );
            // // callstack.push_str(&format!("{:#?}", tcx.def_path_str(*def_id)));
            // // callstack.push_str(&format!("{:#?}", def_id.as_local() ));
            // callstack.push_str("\n");

            // callstack.push_str(&format!("{:#?}", def_id));
            
            if Report::in_blacklist(function_name.clone()) {
                REPORT.lock().unwrap().blacklist = true;
            }

            // filter out generic types
            if function_name.contains("::") {
                function_name = function_name.split("::").collect::<Vec<&str>>()
                    [function_name.split("::").collect::<Vec<&str>>().len() - 1]
                    .to_string();
            }

            // remove duplicated try_borrow_mut_data and try_borrow_data
            if function_name.contains("try_borrow_mut_data")
                || function_name.contains("try_borrow_data")
            {
                // function_name = "try_borrow_mut_data".to_string();
                // debug!("{}", function_name);
            } else {
                call_stack_filter.push_str("--");
                call_stack_filter.push_str(&function_name);
            }
        }
        Report::filter_callstack(call_stack_filter);

        // tcx.def_path_str(body.source.def_id())
        callstack
    }

    pub fn call_stack_formatter_span<'tcx>(tcx: TyCtxt<'tcx>, all_span: &[Span]) -> String {
        let mut callstack = String::new();
        let mut incident = 0;

        for span in all_span {
            for i in 0..incident {
                callstack.push_str("\t");
            }
            incident += 1;
            callstack.push_str(
                source_info::get_source_lines(tcx, *span)
                    .unwrap_or("".to_string())
                    .as_str(),
            );
            // callstack.push_str(&format!("{:#?}", tcx.def_path_str(*def_id)));
            // callstack.push_str(&format!("{:#?}", def_id.as_local() ));
            callstack.push_str("\n");

            // callstack.push_str(&format!("{:#?}", def_id));
        }

        // tcx.def_path_str(body.source.def_id())
        callstack
    }
}

/// Singleton to store all diagnostics.
pub static REPORT: SyncLazy<Mutex<Report>> = SyncLazy::new(|| Mutex::new(Default::default()));
