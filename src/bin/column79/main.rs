// -*- mode:rust; coding:utf-8-unix; -*-

//! main.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/12
//  @date 2024/03/26

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
// rustc 1.77.0 (aedd173a2 2024-03-17)
#![deny(
    absolute_paths_not_starting_with_crate,
    box_pointers,
    deprecated_in_future,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    ffi_unwind_calls,
    keyword_idents,
    let_underscore_drop,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    non_ascii_idents,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unit_bindings,
    unreachable_pub,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    unused_results,
    variant_size_differences,
    ambiguous_glob_imports,
    ambiguous_glob_reexports,
    ambiguous_wide_pointer_comparisons,
    anonymous_parameters,
    array_into_iter,
    asm_sub_register,
    async_fn_in_trait,
    bad_asm_style,
    bare_trait_objects,
    break_with_label_and_loop,
    byte_slice_in_packed_struct_with_derive,
    clashing_extern_declarations,
    coherence_leak_check,
    confusable_idents,
    const_evaluatable_unchecked,
    const_eval_mutable_ptr_in_final_value,
    const_item_mutation,
    const_patterns_without_partial_eq,
    deprecated,
    deprecated_where_clause_location,
    deref_into_dyn_supertrait,
    deref_nullptr,
    dropping_copy_types,
    dropping_references,
    drop_bounds,
    duplicate_macro_attributes,
    dyn_drop,
    elided_lifetimes_in_associated_constant,
    ellipsis_inclusive_range_patterns,
    exported_private_dependencies,
    forbidden_lint_groups,
    forgetting_copy_types,
    forgetting_references,
    for_loops_over_fallibles,
    function_item_references,
    hidden_glob_reexports,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    improper_ctypes_definitions,
    incomplete_features,
    indirect_structural_match,
    inline_no_sanitize,
    internal_features,
    invalid_doc_attributes,
    invalid_from_utf8,
    invalid_macro_export_arguments,
    invalid_nan_comparisons,
    invalid_value,
    irrefutable_let_patterns,
    large_assignments,
    late_bound_lifetime_arguments,
    legacy_derive_helpers,
    map_unit_fn,
    mixed_script_confusables,
    named_arguments_used_positionally,
    nontrivial_structural_match,
    non_camel_case_types,
    non_fmt_panics,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    noop_method_call,
    no_mangle_generic_items,
    opaque_hidden_inferred_bound,
    overlapping_range_endpoints,
    path_statements,
    pointer_structural_match,
    private_bounds,
    private_interfaces,
    redundant_semicolons,
    refining_impl_trait,
    renamed_and_removed_lints,
    repr_transparent_external_private_fields,
    semicolon_in_expressions_from_macros,
    special_module_name,
    stable_features,
    static_mut_refs,
    suspicious_auto_trait_impls,
    suspicious_double_ref_op,
    temporary_cstring_as_ptr,
    trivial_bounds,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    uncommon_codepoints,
    unconditional_recursion,
    undefined_naked_function_abi,
    unexpected_cfgs,
    ungated_async_fn_track_caller,
    uninhabited_static,
    unknown_lints,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unstable_name_collisions,
    unstable_syntax_pre_expansion,
    unsupported_calling_conventions,
    unused_allocation,
    unused_assignments,
    unused_associated_type_bounds,
    unused_attributes,
    unused_braces,
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
    useless_ptr_null_checks,
    where_clauses_object_safety,
    while_true,
    writes_through_immutable_pointer,
    ambiguous_associated_items,
    arithmetic_overflow,
    bindings_with_variant_name,
    cenum_impl_drop_cast,
    conflicting_repr_hints,
    deprecated_cfg_attr_crate_type_name,
    enum_intrinsics_non_enums,
    ill_formed_attribute_input,
    incomplete_include,
    ineffective_unstable_trait_impl,
    invalid_atomic_ordering,
    invalid_from_utf8_unchecked,
    invalid_reference_casting,
    invalid_type_param_default,
    let_underscore_lock,
    long_running_const_eval,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    missing_fragment_specifier,
    mutable_transmutes,
    named_asm_labels,
    no_mangle_const_items,
    order_dependent_trait_objects,
    overflowing_literals,
    patterns_in_fns_without_body,
    proc_macro_back_compat,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    soft_unstable,
    text_direction_codepoint_in_comment,
    text_direction_codepoint_in_literal,
    unconditional_panic,
    undropped_manually_drops,
    unknown_crate_types,
    useless_deprecated
)]
#![warn(unsafe_code, dead_code)]
// mod  =======================================================================
mod error;
// use  =======================================================================
use std::{env, path::PathBuf};
// ----------------------------------------------------------------------------
use bitflags as _;
use dirs as _;
use log as _;
use regex as _;
use serde as _;
use serde_derive as _;
use tempfile as _;
use toml as _;
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
