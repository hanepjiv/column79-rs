// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/12
//  @date 2017/02/16

//! # Examples
//!
//! ```
//! ```

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![deny(
    fat_ptr_transmutes,
    missing_docs,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    variant_size_differences,
    const_err,
    deprecated,
    deprecated_attr,
    extra_requirement_in_impl,
    hr_lifetime_in_assoc_type,
    improper_ctypes,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    plugin_as_library,
    private_in_public,
    private_no_mangle_fns,
    private_no_mangle_statics,
    renamed_and_removed_lints,
    safe_extern_statics,
    stable_features,
    unconditional_recursion,
    unions_with_drop_fields,
    unknown_lints,
    unreachable_code,
    while_true,
    exceeding_bitshifts,
    illegal_floating_point_constant_pattern,
    illegal_struct_or_enum_constant_pattern,
    inaccessible_extern_crate,
    invalid_type_param_default,
    lifetime_underscore,
    mutable_transmutes,
    no_mangle_const_items,
    overlapping_inherent_impls,
    super_or_self_in_global_path,
    transmute_from_fn_item_types,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_comparisons,
    unused_features,
    unused_imports,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_unsafe,
    unknown_crate_types,
)]
#![warn(
    dead_code,
    missing_copy_implementations,
    missing_debug_implementations,
    unused_variables,
)]
#![allow(
    box_pointers,
    unsafe_code,
    trivial_casts,
    trivial_numeric_casts,
)]
// extern  ====================================================================
#[macro_use] extern     crate bitflags;
#[macro_use] extern     crate log;
extern                  crate regex;
#[macro_use] extern     crate serde_derive;
extern                  crate tempfile;
extern                  crate toml;
// use  =======================================================================
use                     ::std::path::PathBuf;
use                     ::std::io::Write;
use                     ::std::fs::File;
// ----------------------------------------------------------------------------
use                     config::Config;
pub use                 error::Error;
use                     error::Error::{ IOError, Column79Error, };
use                     inspector::{ Inspector, Checker, Replacer, };
// mod  =======================================================================
mod                     error;
mod                     ask;
pub mod                 flags;
mod                     config;
mod                     line_type;
mod                     language;
mod                     inspector;
// define  ====================================================================
const CONFIG_DIRNAME: &'static str      = ".config";
const CONFIG_DEFAULT_PATH: &'static str = "default.toml";
const CONFIG_USER_PATH: &'static str    = "user.toml";
const CONFIG_DEFAULT: &'static str      = include_str!("config/default.toml");
const CONFIG_USER: &'static str         = include_str!("config/user.toml");
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Command
#[derive( Debug, Clone, Copy, PartialEq)]
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
impl <'a> From<&'a str> for Command {
    // ========================================================================
    fn from(src: &'a str) -> Self { match src.to_lowercase().as_str() {
        "init"          => Command::Init,
        "check"         => Command::Check,
        "replace"       => Command::Replace,
        _               => Command::Unknown,
    } }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Column79
#[derive( Debug, )]
pub struct Column79 {
    /// command
    command:                    Command,
    /// input
    input:                      PathBuf,
    /// config_dir
    config_dir:                 PathBuf,
    /// config_default_path
    config_default_path:        PathBuf,
    /// config_user_path
    config_user_path:           PathBuf,
    /// Config
    config:                     Config,
}
// ============================================================================
impl Column79 {
    // ========================================================================
    /// create_config_default
    fn create_config(path: PathBuf, config: &'static str)
                     -> Result<(), Error> {
        let mut f = File::create(path.clone()).map_err(|e| IOError(format!(
            "::column79::lib::Column79::create_config(\"{:?}\", ...): \
             ::std::fs::File::create(...): \
             failed", path), e))?;
        f.write_all(config.as_ref()).map_err(|e| IOError(format!(
            "::column79::lib::Column79::create_config(\"{:?}\", ...): \
             ::std::fs::File::write_all(...): \
             failed", path), e))
    }
    // ========================================================================
    /// run
    pub fn run(command:         Command,
               input:           PathBuf,
               language:        Option<String>,
               column:          Option<usize>,
               septhr:          Option<usize>,
               flags:           flags::Flags) -> Result<(), Error> {
        // config_dir  --------------------------------------------------------
        let mut config_dir =
            ::std::env::home_dir().ok_or(Column79Error(format!(
                "::column79::lib::Column79::run(\"{:?}\"): \
                 ::std::env::home_dir(): not found", input)))?;
        config_dir.push(CONFIG_DIRNAME);
        config_dir.push(::std::env::current_exe().map_err(|e| IOError(format!(
            "::column79::lib::Column79::run(..., \"{:?}\", ...): \
             ::std::env::current_exe(): failed", input), e))?
                        .file_name()
                        .ok_or(Column79Error(format!(
                            "::column79::lib::Column79::run(\"{:?}\"): \
                             ::std::env::current_exe().file_name(): \
                             not found", input)))?);
        if !config_dir.exists() {
            ::std::fs::create_dir_all(config_dir.clone())
                .map_err(|e| IOError(format!(
                    "::column79::lib::Column79::run(\"{:?}\"): \
                     ::std::fs::current_dir_all(): failed", input), e))?
        }
        // config_default_path  -----------------------------------------------
        let mut config_default_path = config_dir.clone();
        config_default_path.push(CONFIG_DEFAULT_PATH);
        if !config_default_path.exists() {
            Column79::create_config(config_default_path.clone(),
                                    CONFIG_DEFAULT)?
        }
        // config_user_path  --------------------------------------------------
        let mut config_user_path = config_dir.clone();
        config_user_path.push(CONFIG_USER_PATH);
        if !config_user_path.exists() {
            Column79::create_config(config_user_path.clone(),
                                    CONFIG_USER)?
        }

        let mut config =
            Config::new(&config_default_path.clone().into_os_string())?;
        config.import(&config_user_path.clone().into_os_string())?;

        if column.is_some() {
            config.column               = column.unwrap()
        };
        if septhr.is_some() {
            config.separator_threshold  = septhr.unwrap()
        };
        if language.is_some() {
            config.language             = language.unwrap()
        };
        config.flags.insert(flags);

        config.validation()?;

        let c79 = Column79 {
            command:                    command,
            input:                      input,
            config_dir:                 config_dir,
            config_default_path:        config_default_path,
            config_user_path:           config_user_path,
            config:                     config,
        };
        match c79.command {
            Command::Unknown    => Err(Column79Error(format!(
                "::column79::lib::Column79::run: \
                 invalid command {:?}", c79.command))),
            Command::Init       => c79.init(),
            Command::Check      => c79.check(),
            Command::Replace    => c79.replace(),
        }
    }
    // ========================================================================
    /// walk
    fn walk<T>(&self, path: &PathBuf, inspector: &T) ->  Result<(), Error>
        where T:        Inspector, {
        for i in ::std::fs::read_dir(path).map_err(|e| IOError(format!(
            "::column79::lib::Column79::walk(\"{:?}\", ...): \
             ::std::fs::read_dir(...): \
             failed", path), e))? {
            let entry = i.map_err(|e| IOError(format!(
                "::column79::lib::Column79::walk(\"{:?}\", ...): \
                 entry(...): failed", path), e))?;
            let ftype = entry.file_type().map_err(|e| IOError(format!(
                "::column79::lib::Column79::walk(\"{:?}\", ...): \
                 file_type(...): failed", path), e))?;
            if ftype.is_dir() {
                let _ = self.walk(&entry.path(), inspector)?;
                continue;
            }
            let entry_path = &entry.path();
            match self.config.check_path(entry_path) {
                Some(language)  => {
                    info!("Column79::walk {} {:?}",
                          language.peek_name(), entry_path);
                    inspector.inspect(language, entry_path)?
                },
                None            => (),
            }
        }
        Ok(())
    }
    // ========================================================================
    /// init
    fn init(&self) -> Result<(), Error> {
        Column79::create_config(self.config_default_path.clone(),
                                CONFIG_DEFAULT)?;
        if !self.config_user_path.exists() {
            return Column79::create_config(self.config_user_path.clone(),
                                           CONFIG_USER);
        }
        if self.config.flags.contains(flags::NOASK) {
            return Column79::create_config(self.config_user_path.clone(),
                                           CONFIG_USER);
        }
        if ::ask::ask("Do you want to overwrite your user config?", false)? {
            return Column79::create_config(self.config_user_path.clone(),
                                           CONFIG_USER);
        }
        return Ok(());
    }
    // ========================================================================
    /// check
    fn check(&self) ->  Result<(), Error> {
        self.walk(&self.input, &Checker::new(&self.config))
    }
    // ========================================================================
    /// replace
    fn replace(&self) ->  Result<(), Error> {
        self.walk(&self.input, &Replacer::new(&self.config))
    }
}
// ////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
