// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/12
//  @date 2024/09/23

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
// rustc 1.81.0 (eeb90cda1 2024-09-04)
#![forbid(
    clippy::all,
    absolute_paths_not_starting_with_crate,
    deprecated_in_future,
    deprecated_safe,
    edition_2024_expr_fragment_specifier,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    ffi_unwind_calls,
    keyword_idents_2018,
    keyword_idents_2024,
    let_underscore_drop,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    missing_unsafe_on_extern,
    non_ascii_idents,
    non_local_definitions,
    redundant_lifetimes,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unit_bindings,
    unnameable_types,
    unreachable_pub,
    unsafe_attr_outside_unsafe,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unused_crate_dependencies,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
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
    boxed_slice_into_iter,
    break_with_label_and_loop,
    byte_slice_in_packed_struct_with_derive,
    clashing_extern_declarations,
    coherence_leak_check,
    confusable_idents,
    const_evaluatable_unchecked,
    const_eval_mutable_ptr_in_final_value,
    const_item_mutation,
    dependency_on_unit_never_type_fallback,
    deprecated_where_clause_location,
    deref_into_dyn_supertrait,
    deref_nullptr,
    dropping_copy_types,
    dropping_references,
    drop_bounds,
    duplicate_macro_attributes,
    dyn_drop,
    ellipsis_inclusive_range_patterns,
    exported_private_dependencies,
    forbidden_lint_groups,
    forgetting_copy_types,
    forgetting_references,
    for_loops_over_fallibles,
    function_item_references,
    hidden_glob_reexports,
    improper_ctypes,
    improper_ctypes_definitions,
    incomplete_features,
    inline_no_sanitize,
    internal_features,
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
    never_type_fallback_flowing_into_unsafe,
    non_contiguous_range_endpoints,
    non_fmt_panics,
    non_shorthand_field_patterns,
    non_snake_case,
    noop_method_call,
    no_mangle_generic_items,
    opaque_hidden_inferred_bound,
    out_of_scope_macro_calls,
    overlapping_range_endpoints,
    path_statements,
    private_bounds,
    private_interfaces,
    ptr_cast_add_auto_to_object,
    redundant_semicolons,
    refining_impl_trait_internal,
    refining_impl_trait_reachable,
    renamed_and_removed_lints,
    repr_transparent_external_private_fields,
    self_constructor_from_outer_item,
    semicolon_in_expressions_from_macros,
    special_module_name,
    stable_features,
    static_mut_refs,
    suspicious_double_ref_op,
    temporary_cstring_as_ptr,
    trivial_bounds,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    uncommon_codepoints,
    unconditional_recursion,
    uncovered_param_in_projection,
    undefined_naked_function_abi,
    unexpected_cfgs,
    unfulfilled_lint_expectations,
    ungated_async_fn_track_caller,
    uninhabited_static,
    unknown_lints,
    unknown_or_malformed_diagnostic_attributes,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unstable_name_collisions,
    unstable_syntax_pre_expansion,
    unsupported_calling_conventions,
    unused_allocation,
    unused_assignments,
    unused_associated_type_bounds,
    unused_braces,
    unused_comparisons,
    unused_features,
    unused_labels,
    unused_macros,
    unused_must_use,
    unused_parens,
    unused_unsafe,
    unused_variables,
    useless_ptr_null_checks,
    wasm_c_abi,
    while_true,
    writes_through_immutable_pointer,
    ambiguous_associated_items,
    arithmetic_overflow,
    binary_asm_labels,
    bindings_with_variant_name,
    cenum_impl_drop_cast,
    conflicting_repr_hints,
    deprecated_cfg_attr_crate_type_name,
    elided_lifetimes_in_associated_constant,
    enum_intrinsics_non_enums,
    ill_formed_attribute_input,
    incomplete_include,
    ineffective_unstable_trait_impl,
    invalid_atomic_ordering,
    invalid_doc_attributes,
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
#![deny(
    clippy::all,
    non_camel_case_types,
    non_upper_case_globals,
    unused_attributes,
    unused_doc_comments,
    unused_imports,
    unused_mut,
    unused_extern_crates,
    unused_qualifications,
    dead_code,
    deprecated
)]
// mod  =======================================================================
mod ask;
mod config;
mod error;
mod flags;
mod inspector;
mod language;
mod line_type;
// use  =======================================================================
use std::{fs::File, io::Write, path::PathBuf};
// ----------------------------------------------------------------------------
use env_logger as _;
use getopts as _;
use log::info;
// ----------------------------------------------------------------------------
use self::config::Config;
pub use self::error::Error;
pub use self::flags::Flags;
use self::inspector::{Checker, Inspector, Replacer};
// define  ====================================================================
const CONFIG_DIRNAME: &str = ".config";
const CONFIG_DEFAULT_PATH: &str = "default.toml";
const CONFIG_USER_PATH: &str = "user.toml";
const CONFIG_DEFAULT: &str = include_str!("config/default.toml");
const CONFIG_USER: &str = include_str!("config/user.toml");
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Command
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command {
    ///  Unknown
    Unknown,
    ///  Init
    Init,
    ///  Check
    Check,
    ///  Replace
    Replace,
}
// ============================================================================
impl<'a> From<&'a str> for Command {
    // ========================================================================
    fn from(src: &'a str) -> Self {
        match src.to_lowercase().as_str() {
            "init" => Command::Init,
            "check" => Command::Check,
            "replace" => Command::Replace,
            _ => Command::Unknown,
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Column79
#[derive(Debug, Clone)]
pub struct Column79 {
    /// command
    command: Command,
    /// input
    input: PathBuf,
    /// config_dir
    config_dir: PathBuf,
    /// config_default_path
    config_default_path: PathBuf,
    /// config_user_path
    config_user_path: PathBuf,
    /// Config
    config: Config,
}
// ============================================================================
impl Column79 {
    // ========================================================================
    /// as_config_dir
    pub fn as_config_dir(&self) -> &PathBuf {
        &self.config_dir
    }
    // ========================================================================
    /// create_config_default
    fn create_config(
        path: &PathBuf,
        config: &'static str,
    ) -> Result<(), Error> {
        let mut f = File::create(path)?;
        f.write_all(config.as_ref())?;
        Ok(())
    }
    // ========================================================================
    /// run
    pub fn run(
        command: Command,
        input: PathBuf,
        language: Option<String>,
        column: Option<usize>,
        septhr: Option<usize>,
        flags: Flags,
    ) -> Result<(), Error> {
        // config_dir  --------------------------------------------------------
        let mut config_dir = dirs::home_dir().ok_or_else(|| {
            Error::Column79(format!(
                "::column79::lib::Column79::run(\"{:?}\"): \
                 ::std::env::home_dir(): not found",
                input
            ))
        })?;
        config_dir.push(CONFIG_DIRNAME);
        config_dir.push(std::env::current_exe()?.file_name().ok_or_else(
            || {
                Error::Column79(format!(
                    "::column79::lib::Column79::run(\"{:?}\"): \
                     ::std::env::current_exe().file_name(): \
                     not found",
                    input
                ))
            },
        )?);
        if !config_dir.exists() {
            std::fs::create_dir_all(config_dir.clone())?
        }
        // config_default_path  -----------------------------------------------
        let mut config_default_path = config_dir.clone();
        config_default_path.push(CONFIG_DEFAULT_PATH);
        if !config_default_path.exists() {
            Column79::create_config(&config_default_path, CONFIG_DEFAULT)?
        }
        // config_user_path  --------------------------------------------------
        let mut config_user_path = config_dir.clone();
        config_user_path.push(CONFIG_USER_PATH);
        if !config_user_path.exists() {
            Column79::create_config(&config_user_path, CONFIG_USER)?
        }

        let mut config =
            Config::new(&config_default_path.clone().into_os_string())?;
        config.import(&config_user_path.clone().into_os_string())?;

        if column.is_some() {
            config.column = column.unwrap()
        };
        if septhr.is_some() {
            config.separator_threshold = septhr.unwrap()
        };
        if language.is_some() {
            config.language = language.unwrap()
        };
        config.flags.insert(flags);

        config.validation()?;

        let c79 = Column79 {
            command,
            input,
            config_dir,
            config_default_path,
            config_user_path,
            config,
        };
        match c79.command {
            Command::Unknown => Err(Error::Column79(format!(
                "::column79::lib::Column79::run: \
                 invalid command {:?}",
                c79.command
            ))),
            Command::Init => c79.init(),
            Command::Check => c79.check(),
            Command::Replace => c79.replace(),
        }
    }
    // ========================================================================
    /// walk
    fn walk(
        &self,
        path: &PathBuf,
        inspector: &impl Inspector,
    ) -> Result<(), Error> {
        for i in std::fs::read_dir(path)? {
            let entry = i?;
            let ftype = entry.file_type()?;
            if ftype.is_dir() {
                self.walk(&entry.path(), inspector)?;
                continue;
            }
            let entry_path = &entry.path();
            if let Some(language) = self.config.check_path(entry_path) {
                info!(
                    "Column79::walk {} {:?}",
                    language.peek_name(),
                    entry_path
                );
                inspector.inspect(language, entry_path)?
            }
        }
        Ok(())
    }
    // ========================================================================
    /// init
    fn init(&self) -> Result<(), Error> {
        Column79::create_config(&self.config_default_path, CONFIG_DEFAULT)?;
        if !self.config_user_path.exists() {
            return Column79::create_config(
                &self.config_user_path,
                CONFIG_USER,
            );
        }
        if self.config.flags.contains(Flags::NOASK) {
            return Column79::create_config(
                &self.config_user_path,
                CONFIG_USER,
            );
        }
        if ask::ask("Do you want to overwrite your user config?", false)? {
            return Column79::create_config(
                &self.config_user_path,
                CONFIG_USER,
            );
        }
        Ok(())
    }
    // ========================================================================
    /// check
    fn check(&self) -> Result<(), Error> {
        self.walk(&self.input, &Checker::new(&self.config))
    }
    // ========================================================================
    /// replace
    fn replace(&self) -> Result<(), Error> {
        self.walk(&self.input, &Replacer::new(&self.config))
    }
}
