// -*- mode:rust; coding:utf-8-unix; -*-

//! main.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/12
//  @date 2020/04/12

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
// rustc 1.42.0 (b8cedc004 2020-03-09)
#![deny(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    box_pointers,
    deprecated_in_future,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    indirect_structural_match,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    missing_doc_code_examples,
    non_ascii_idents,
    private_doc_tests,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences,
    array_into_iter,
    bare_trait_objects,
    bindings_with_variant_name,
    deprecated,
    ellipsis_inclusive_range_patterns,
    exported_private_dependencies,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    incomplete_features,
    intra_doc_link_resolution_failure,
    invalid_value,
    irrefutable_let_patterns,
    late_bound_lifetime_arguments,
    mutable_borrow_reservation_conflict,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    no_mangle_generic_items,
    overlapping_patterns,
    path_statements,
    private_in_public,
    proc_macro_derive_resolution_fallback,
    redundant_semicolon,
    safe_packed_borrows,
    stable_features,
    trivial_bounds,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    uncommon_codepoints,
    unconditional_recursion,
    unknown_lints,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unstable_name_collisions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_comparisons,
    unused_doc_comments,
    unused_features,
    unused_imports,
    unused_labels,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_unsafe,
    unused_variables,
    where_clauses_object_safety,
    while_true,
    ambiguous_associated_items,
    conflicting_repr_hints,
    const_err,
    exceeding_bitshifts,
    ill_formed_attribute_input,
    invalid_type_param_default,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    missing_fragment_specifier,
    mutable_transmutes,
    no_mangle_const_items,
    order_dependent_trait_objects,
    overflowing_literals,
    patterns_in_fns_without_body,
    pub_use_of_private_extern_crate,
    soft_unstable,
    unknown_crate_types
)]
#![warn(dead_code, renamed_and_removed_lints)]
// mod  =======================================================================
mod error;
// use  =======================================================================
use std::{env, path::PathBuf};
// ----------------------------------------------------------------------------
use self::error::{Error, Result};
use column79::{Column79, Command, Flags};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
fn print_usage(opts: &::getopts::Options) {
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
            module_path!()
        ))
    );
}
// ============================================================================
fn main() -> Result<()> {
    env_logger::init();

    let mut opts = ::getopts::Options::new();
    let _ = opts
        .optflag("v", "version", "print version")
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

    let matches = opts.parse(env::args().skip(1))?;
    if matches.opt_present("v") {
        println!(concat!(module_path!(), " v", env!("CARGO_PKG_VERSION")));
        return Ok(());
    }
    if matches.free.is_empty() || matches.opt_present("h") {
        print_usage(&opts);
        return Ok(());
    }
    let command = Command::from(matches.free[0].as_ref());
    if Command::Unknown == command {
        print_usage(&opts);
        return Ok(());
    }
    let input = if 1usize == matches.free.len() {
        ::std::env::current_dir()?
    } else {
        PathBuf::from(matches.free[1].clone())
    };
    let column = if matches.opt_present("c") {
        Some(
            matches
                .opt_str("c")
                .ok_or_else(|| {
                    Error::OptionNone(
                        "column79: matches.opt_str(\"c\").".to_string(),
                    )
                })?
                .parse::<usize>()?,
        )
    } else {
        None
    };
    let septhr = if matches.opt_present("t") {
        Some(
            matches
                .opt_str("t")
                .ok_or_else(|| {
                    Error::OptionNone(
                        "column79: matches.opt_str(\"t\").".to_string(),
                    )
                })?
                .parse::<usize>()?,
        )
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
