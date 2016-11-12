// -*- mode:rust; coding:utf-8-unix; -*-

//! config.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/13
//  @date 2016/11/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use                     ::std::ffi::OsString;
use                     ::std::fs::File;
use                     ::std::io::Read;
use                     ::std::collections::BTreeMap;
// ----------------------------------------------------------------------------
use                     error::Error;
use                     error::Error::{ IOError,
                                        ParseConfigError,
                                        InvalidConfigError };
use                     flags;
use                     language::{ Language, parse_languages, };
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Config
#[derive( Debug, Clone, )]
pub struct Config {
    /// column
    pub column:         usize,
    /// septhr
    pub septhr:         usize,
    /// language
    pub language:       String,
    /// languages
    pub languages:      BTreeMap<String, Language>,
    /// flags
    pub flags:          flags::Flags,
}
// ============================================================================
impl Default for Config {
    fn default() -> Self {
        Config {
            column:     79,
            septhr:     12,
            language:   String::from("cargo"),
            languages:  BTreeMap::new(),
            flags:      flags::Flags::empty(),
        }
    }
}
// ============================================================================
impl Config {
    // ========================================================================
    /// new
    pub fn new(path: &OsString) -> Result<Self, Error> {
        let mut config = Config::default();
        config.import(path)?;
        Ok(config)
    }
    // ========================================================================
    pub fn import(&mut self, path: &OsString) -> Result<(), Error> {
        let table;
        {
            let mut src = String::new();
            let mut parser;
            {
                let _ = File::open(path.clone())
                    .and_then(|mut f| { f.read_to_string(&mut src) })
                    .map_err(|e| IOError(
                        format!("::column79::config::Config::import({:?}): \
                                 open", path),
                        e))?;
                parser = ::toml::Parser::new(&src);
            }
            table = parser.parse()
                .ok_or(ParseConfigError(
                    format!("::column79::config::Config::import({:?}): \
                             parse", path),
                    parser.errors))?;
        }
        {
            let column = match table.get("column") {
                None            => None,
                Some(c)         => match c.as_integer() {
                    None        => {
                        return Err(InvalidConfigError(format!(
                            "::column79::config::Config::import({:?}): \
                             column is not integer", path)));
                    },
                    Some(v)     => Some(v as usize)
                }
            };
            if column.is_some() { self.column = column.unwrap(); }
        }
        {
            let septhr = match table.get("separator_threshold") {
                None            => None,
                Some(c)         => match c.as_integer() {
                    None        => {
                        return Err(InvalidConfigError(format!(
                            "::column79::config::Config::import({:?}): \
                             separator_threshold is not integer", path)));
                    },
                    Some(v)     => Some(v as usize)
                }
            };
            if septhr.is_some() { self.septhr = septhr.unwrap(); }
        }
        {
            let language = match table.get("language") {
                None            => None,
                Some(c)         => match c.as_str() {
                    None        => {
                        return Err(InvalidConfigError(format!(
                            "::column79::config::Config::import({:?}): \
                             language is not integer", path)));
                    },
                    Some(v)     => Some(String::from(v))
                }
            };
            if language.is_some() { self.language = language.unwrap(); }
        }
        {
            let values = match table.get("languages") {
                None            => None,
                Some(c)         => match c.as_slice() {
                    None        => {
                        return Err(InvalidConfigError(format!(
                            "::column79::config::Config::import({:?}): \
                             languages is not slice", path)));
                    },
                    Some(v)     => Some(v)
                }
            };
            if values.is_some() {
                parse_languages(&values.unwrap(), &mut self.languages)?;
            }
        }
        Ok(())
    }
    // ========================================================================
    pub fn validation(&self) -> Result<(), Error> {
        match self.languages.get(&self.language) {
            Some(_)     => Ok(()),
            None        => Err(InvalidConfigError(format!(
                "::column79::config::Config::validation(&self): \
                 language not found {}", self.language))),
        }
    }
    // ========================================================================
    pub fn check_path(&self, path: &::std::path::PathBuf)
                      -> Option<&Language> {
        self.languages.get(&self.language).unwrap().
            check_path(path, &self.languages)
    }
}
