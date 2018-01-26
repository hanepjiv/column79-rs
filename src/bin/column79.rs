// -*- mode:rust; coding:utf-8-unix; -*-

//! column79.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/12
//  @date 2018/01/26

//! # Examples
//!
//! ```
//! ```

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![deny(anonymous_parameters, box_pointers, missing_copy_implementations,
        missing_debug_implementations, missing_docs, trivial_casts,
        trivial_numeric_casts, unsafe_code, unstable_features,
        unused_extern_crates, unused_import_braces, unused_qualifications,
        unused_results, variant_size_differences, const_err, dead_code,
        deprecated, illegal_floating_point_literal_pattern, improper_ctypes,
        late_bound_lifetime_arguments, non_camel_case_types,
        non_shorthand_field_patterns, non_snake_case, non_upper_case_globals,
        no_mangle_generic_items, overflowing_literals, path_statements,
        patterns_in_fns_without_body, plugin_as_library, private_in_public,
        private_no_mangle_fns, private_no_mangle_statics,
        renamed_and_removed_lints, stable_features, unconditional_recursion,
        unions_with_drop_fields, unknown_lints, unreachable_code,
        unreachable_patterns, unused_allocation, unused_assignments,
        unused_attributes, unused_comparisons, unused_doc_comment,
        unused_features, unused_imports, unused_macros, unused_must_use,
        unused_mut, unused_parens, unused_unsafe, unused_variables,
        while_true)]
#![warn(dead_code)]
#![allow(box_pointers, unsafe_code, trivial_casts, trivial_numeric_casts)]
// extern  ====================================================================
extern crate env_logger;
// ----------------------------------------------------------------------------
extern crate column79;
extern crate getopts;
// use  =======================================================================
use std::path::PathBuf;
// ----------------------------------------------------------------------------
use column79::{Column79, Command, Flags};
// mod  =======================================================================
#[macro_use]
mod unwrap;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
fn print_usage(opts: &::getopts::Options, program: &str) {
    print!(
        "{}",
        opts.usage(&format!(
            r##"Usage:
    {0} Command [Input] [Options]

Command:
    init        initialize the configure
    check       to check the column overflow
    replace     replace the result of the checked

Input:
    ./          current directory (default)"##,
            program
        ))
    );
}
// ============================================================================
fn main() {
    env_logger::init();

    let args: Vec<String> = ::std::env::args().collect();
    let mut opts = ::getopts::Options::new();
    let _ = opts.optflag("v", "version", "print version")
        .optflag("h", "help", "print this help menu")
        .optopt("c", "column", "set column number", "NUM")
        .optopt("t", "threshold", "set separator threshold number", "NUM")
        .optopt(
            "l",
            "language",
            "set language LANG=('cargo'|'rust'|'c'|'c++'|...)",
            "LANG",
        )
        .optflag("", "no-ask", "will not be asked to allow");
    let matches = unwrap!(opts.parse(&args[1..]));
    if matches.opt_present("v") {
        return println!(
            "{}",
            concat!(module_path!(), " v", env!("CARGO_PKG_VERSION"))
        );
    }
    if matches.free.is_empty() || matches.opt_present("h") {
        return print_usage(&opts, args[0].as_ref());
    }
    let command = Command::from(matches.free[0].as_ref());
    if Command::Unknown == command {
        return print_usage(&opts, args[0].as_ref());
    }
    let input = if 1usize == matches.free.len() {
        unwrap!(::std::env::current_dir())
    } else {
        PathBuf::from(matches.free[1].clone())
    };
    let column = if matches.opt_present("c") {
        Some(unwrap!(unwrap!(matches.opt_str("c")).parse::<usize>()))
    } else {
        None
    };
    let septhr = if matches.opt_present("t") {
        Some(unwrap!(unwrap!(matches.opt_str("t")).parse::<usize>()))
    } else {
        None
    };
    let language = if matches.opt_present("l") {
        Some(unwrap!(matches.opt_str("l")))
    } else {
        None
    };
    let mut fs = Flags::empty();
    if matches.opt_present("no-ask") {
        fs.insert(Flags::NOASK);
    }
    unwrap!(Column79::run(command, input, language, column, septhr, fs));
}
