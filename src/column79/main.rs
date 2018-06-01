// -*- mode:rust; coding:utf-8-unix; -*-

//! main.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/12
//  @date 2018/06/01

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
// rustc 1.26.1 (827013a31 2018-05-25)
#![deny(
    anonymous_parameters, missing_copy_implementations,
    missing_debug_implementations, missing_docs, unstable_features,
    unused_extern_crates, unused_import_braces, unused_qualifications,
    unused_results, variant_size_differences, const_err, deprecated,
    illegal_floating_point_literal_pattern, improper_ctypes,
    incoherent_fundamental_impls, late_bound_lifetime_arguments,
    non_camel_case_types, non_shorthand_field_patterns, non_snake_case,
    non_upper_case_globals, no_mangle_generic_items, overflowing_literals,
    path_statements, patterns_in_fns_without_body, plugin_as_library,
    private_in_public, private_no_mangle_fns, private_no_mangle_statics,
    safe_packed_borrows, stable_features, type_alias_bounds,
    tyvar_behind_raw_pointer, unconditional_recursion, unions_with_drop_fields,
    unknown_lints, unreachable_code, unreachable_patterns,
    unstable_name_collision, unused_allocation, unused_assignments,
    unused_attributes, unused_comparisons, unused_doc_comment, unused_features,
    unused_imports, unused_macros, unused_must_use, unused_mut, unused_parens,
    unused_unsafe, unused_variables, while_true, exceeding_bitshifts,
    invalid_type_param_default, legacy_constructor_visibility,
    legacy_directory_ownership, legacy_imports, missing_fragment_specifier,
    mutable_transmutes, no_mangle_const_items,
    parenthesized_params_in_types_and_modules, pub_use_of_private_extern_crate,
    safe_extern_statics, unknown_crate_types
)]
#![warn(
    bare_trait_object, dead_code, renamed_and_removed_lints, unreachable_pub
)]
#![allow(
    box_pointers, elided_lifetime_in_path, single_use_lifetime, trivial_casts,
    trivial_numeric_casts, unsafe_code
)]
// extern  ====================================================================
extern crate env_logger;
// ----------------------------------------------------------------------------
extern crate column79;
extern crate getopts;
// use  =======================================================================
use std::path::PathBuf;
// ----------------------------------------------------------------------------
use self::error::{Error, Result};
use column79::{Column79, Command, Flags};
// mod  =======================================================================
mod error;
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
fn main() -> Result<()> {
    env_logger::init();

    let args: Vec<String> = ::std::env::args().collect();
    let mut opts = ::getopts::Options::new();
    let _ = opts.optflag("v", "version", "print version")
        .optflag("h", "help", "print this help menu")
        .optopt("c", "column", "set column number", "NUM")
        .optopt(
            "t",
            "threshold",
            "set separator threshold number",
            "NUM",
        )
        .optopt(
            "l",
            "language",
            "set language LANG=('cargo'|'rust'|'c'|'c++'|...)",
            "LANG",
        )
        .optflag("", "no-ask", "will not be asked to allow");
    let matches = opts.parse(&args[1..])?;
    if matches.opt_present("v") {
        println!(concat!(
            module_path!(),
            " v",
            env!("CARGO_PKG_VERSION")
        ));
    }
    if matches.free.is_empty() || matches.opt_present("h") {
        print_usage(&opts, args[0].as_ref());
        return Ok(());
    }
    let command = Command::from(matches.free[0].as_ref());
    if Command::Unknown == command {
        print_usage(&opts, args[0].as_ref());
        return Ok(());
    }
    let input = if 1usize == matches.free.len() {
        ::std::env::current_dir()?
    } else {
        PathBuf::from(matches.free[1].clone())
    };
    let column = if matches.opt_present("c") {
        Some(matches
            .opt_str("c")
            .ok_or_else(|| {
                Error::OptionNone(
                    "column79: matches.opt_str(\"c\").".to_string(),
                )
            })?
            .parse::<usize>()?)
    } else {
        None
    };
    let septhr = if matches.opt_present("t") {
        Some(matches
            .opt_str("t")
            .ok_or_else(|| {
                Error::OptionNone(
                    "column79: matches.opt_str(\"t\").".to_string(),
                )
            })?
            .parse::<usize>()?)
    } else {
        None
    };
    let language = if matches.opt_present("l") {
        Some(matches.opt_str("l").ok_or_else(|| {
            Error::OptionNone("column79: matches.opt_str(\"l\").".to_string())
        })?)
    } else {
        None
    };
    let mut fs = Flags::empty();
    if matches.opt_present("no-ask") {
        fs.insert(Flags::NOASK);
    }
    Column79::run(command, input, language, column, septhr, fs)?;
    Ok(())
}
