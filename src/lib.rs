// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/12
//  @date 2016/10/22

//! # Examples
//!
//! ```
//! ```

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![deny(fat_ptr_transmutes, missing_copy_implementations,
        missing_debug_implementations, missing_docs, unstable_features,
        unused_extern_crates, unused_qualifications, unused_results,
        unused_import_braces, variant_size_differences, warnings)]
#![allow(box_pointers, trivial_casts, trivial_numeric_casts, unsafe_code)]
// extern  ====================================================================
#[macro_use] extern     crate bitflags;
#[macro_use] extern     crate log;
extern                  crate regex;
extern                  crate tempfile;
extern                  crate toml;
// use  =======================================================================
use                     ::std::env;
use                     ::std::path::PathBuf;
use                     ::std::io;
use                     ::std::io::Write;
use                     ::std::fs::File;
// ----------------------------------------------------------------------------
use                     config::Config;
pub use                 error::Error;
use                     error::Error::IOError;
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
        let mut f = try!(File::create(path).map_err(|e| IOError(e)));
        f.write_all(config.as_ref()).map_err(|e| IOError(e))
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
        let mut config_dir = try!(env::home_dir()
                                  .ok_or(IOError(io::Error::last_os_error())));
        config_dir.push(CONFIG_DIRNAME);
        config_dir.push(try!(try!(env::current_exe().map_err(|e| IOError(e)))
                             .file_name()
                             .ok_or(IOError(io::Error::last_os_error()))));
        if !config_dir.exists() {
            try!(::std::fs::create_dir_all(config_dir.clone())
                 .map_err(|e| IOError(e)));
        }
        // config_default_path  -----------------------------------------------
        let mut config_default_path = config_dir.clone();
        config_default_path.push(CONFIG_DEFAULT_PATH);
        if !config_default_path.exists() {
            try!(Column79::create_config(config_default_path.clone(),
                                         CONFIG_DEFAULT))
        }
        // config_user_path  --------------------------------------------------
        let mut config_user_path = config_dir.clone();
        config_user_path.push(CONFIG_USER_PATH);
        if !config_user_path.exists() {
            try!(Column79::create_config(config_user_path.clone(),
                                         CONFIG_USER))
        }

        let mut config =
            try!(Config::new(&config_default_path.clone().into_os_string()));
        try!(config.import(&config_user_path.clone().into_os_string()));

        if column.is_some()     { config.column         = column.unwrap() };
        if septhr.is_some()     { config.septhr         = septhr.unwrap() };
        if language.is_some()   { config.language       = language.unwrap() };
        config.flags.insert(flags);

        try!(config.validation());

        let c79 = Column79 {
            command:                    command,
            input:                      input,
            config_dir:                 config_dir,
            config_default_path:        config_default_path,
            config_user_path:           config_user_path,
            config:                     config,
        };
        match c79.command {
            Command::Unknown    =>
                panic!("Column79::run: invalid command {:?}", c79.command),
            Command::Init       => c79.init(),
            Command::Check      => c79.check(),
            Command::Replace    => c79.replace(),
        }
    }
    // ========================================================================
    /// walk
    fn walk<T>(&self, path: &PathBuf, inspector: &T) ->  Result<(), Error>
        where T:        Inspector, {
        for i in try!(::std::fs::read_dir(path).map_err(|e| IOError(e))) {
            let entry = try!(i.map_err(|e| IOError(e)));
            let ftype = try!(entry.file_type().map_err(|e| IOError(e)));
            if ftype.is_dir() {
                let _ = try!(self.walk(&entry.path(), inspector));
                continue;
            }
            let entry_path = &entry.path();
            match self.config.check_path(entry_path) {
                Some(language)  => {
                    info!("Column79::walk {} {:?}",
                          language.peek_name(), entry_path);
                    try!(inspector.inspect(language, entry_path))
                },
                None            => (),
            }
        }
        Ok(())
    }
    // ========================================================================
    /// init
    fn init(&self) -> Result<(), Error> {
        try!(Column79::create_config(self.config_default_path.clone(),
                                     CONFIG_DEFAULT));
        if !self.config_user_path.exists() {
            return Column79::create_config(self.config_user_path.clone(),
                                           CONFIG_USER);
        }
        if self.config.flags.contains(flags::NOASK) {
            return Column79::create_config(self.config_user_path.clone(),
                                           CONFIG_USER);
        }
        if try!(::ask::ask("Do you want to overwrite your user config?",
                           false)) {
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
