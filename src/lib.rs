// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/12
//  @date 2024/11/27

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================

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
