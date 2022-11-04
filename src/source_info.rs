//! Utility for getting source code

use rustc_middle::ty::TyCtxt;
use rustc_span::{Span, FileName};

use crate::ty_confusion::StructDefLayout;

/// Get the source code for a given Span.
pub fn get_source_lines<'tcx>(tcx: TyCtxt<'tcx>, span: Span) -> Option<String> {
    let source_map = tcx.sess.source_map();
    let loc_info = source_map.span_to_diagnostic_string(span);
    Some(loc_info + " \n\t" + source_map.span_to_snippet(span).ok().as_deref().unwrap_or(""))
    // let lo_source_file = source_map.lookup_source_file(span.lo());
    // let hi_source_file = source_map.lookup_source_file(span.hi());
    // // If within the same file
    // if lo_source_file.name == hi_source_file.name {
    //     if let Some(full_code) = &lo_source_file.src {
    //         let code = &full_code[span.lo().0 as usize..span.hi().0 as usize];
    //         return Some(code.to_string())
    //     }
    //     return None
    // } 
    // if let (Some(lo_full_code), Some(hi_full_code)) = (&lo_source_file.src, &hi_source_file.src) {
    //     let lo_code = &lo_full_code[span.lo().0 as usize..];
    //     let hi_code = &hi_full_code[span.hi().0 as usize..];
    //     return Some(lo_code.to_string() + hi_code);
    // }
    // None
}

/// Get the source code for a given span. The context code is also provided specified
/// by margin.
pub fn get_context_source_lines<'tcx>(tcx: TyCtxt<'tcx>, span: Span, margin: usize) -> Option<String> {
    let source_map = tcx.sess.source_map();
    let mut lo_line = source_map.lookup_line(span.lo()).ok()?;
    let mut hi_line = source_map.lookup_line(span.hi()).ok()?;
    lo_line.line = if lo_line.line >= margin {
        lo_line.line - margin
    } else {
        0
    };
    hi_line.line = if hi_line.line < hi_line.sf.count_lines() {
        hi_line.line + margin
    } else {
        hi_line.sf.count_lines()
    };
    None
}

/// Get the source file path for the given span. If the span.lo and span.hi
/// correspond to different files, return them using ":" as delimeter.
pub fn get_source_file<'tcx>(tcx: TyCtxt<'tcx>, span: Span) -> Option<String> {
    let source_map = tcx.sess.source_map();
    let lo_source_file = source_map.lookup_source_file(span.lo()); 
    let hi_source_file = source_map.lookup_source_file(span.hi());
    let lo_filepath = get_file_path_str(&lo_source_file.name);
    if lo_source_file.name == hi_source_file.name {
        return lo_filepath;
    }
    let hi_filepath = get_file_path_str(&hi_source_file.name);
    lo_filepath.map(|p| p + ";" + hi_filepath.as_deref().unwrap_or("")).or(hi_filepath)
}

fn get_file_path_str(filename: &FileName) -> Option<String> {
    if let FileName::Real(real_file_name) = filename {
        if let Some(path) = real_file_name.local_path() {
            return path.to_str().map(|s| s.to_owned());
        }
    }
    None
}

pub fn get_type_defs_src<'tcx>(tcx: TyCtxt<'tcx>, defs: &[&StructDefLayout]) -> String {
    let mut defs_code = Vec::new();
    for def in defs {
        if let Some(def_code) = get_source_lines(tcx, def.span) {
            defs_code.push(def_code);
        }
    }
    defs_code.join("\n")
}

