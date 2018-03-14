// -*- mode:rust; coding:utf-8-unix; -*-

//! config.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/13
//  @date 2018/03/14

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::collections::BTreeMap;
// ----------------------------------------------------------------------------
use error::Error;
use flags::Flags;
use language::{Language, LanguageSrc};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct ConfigSrc
#[derive(Debug, Deserialize)]
pub struct ConfigSrc {
    /// column
    pub column: Option<usize>,
    /// separator_threshold
    pub separator_threshold: Option<usize>,
    /// ask
    pub ask: Option<bool>,
    /// language
    pub language: Option<String>,
    /// languages
    pub languages: Option<Vec<LanguageSrc>>,
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Config
#[derive(Debug, Clone)]
pub struct Config {
    /// column
    pub column: usize,
    /// separator_threshold
    pub separator_threshold: usize,
    /// flags
    pub flags: Flags,
    /// language
    pub language: String,
    /// languages
    pub languages: BTreeMap<String, Language>,
}
// ============================================================================
impl Default for Config {
    fn default() -> Self {
        Config {
            column: 79,
            separator_threshold: 12,
            flags: Flags::empty(),
            language: String::from("cargo"),
            languages: BTreeMap::new(),
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
    /// import
    pub fn import(&mut self, path: &OsString) -> Result<(), Error> {
        let mut source = String::new();
        let _ = File::open(path.clone())
            .and_then(|mut f| f.read_to_string(&mut source))?;
        let src: ConfigSrc = ::toml::from_str(&source)?;
        if let Some(x) = src.column {
            self.column = x;
        }
        if let Some(x) = src.separator_threshold {
            self.separator_threshold = x;
        }
        if let Some(x) = src.ask {
            if x {
                self.flags.remove(Flags::NOASK);
            } else {
                self.flags.insert(Flags::NOASK);
            }
        } else {
            self.flags.remove(Flags::NOASK);
        }
        if let Some(x) = src.language {
            self.language = x;
        }
        if let Some(xs) = src.languages {
            for x in xs {
                let l = Language::from_src(x, &self.languages)?;
                if self.languages.insert(l.peek_name().clone(), l).is_some() {
                    return Err(Error::InvalidConfig(
                        "::column79::language::Config::import(...): \
                         languages base: insert failed"
                            .to_string(),
                    ));
                }
            }
        }
        Ok(())
    }
    // ========================================================================
    /// validation
    pub fn validation(&self) -> Result<(), Error> {
        if self.languages.get(&self.language).is_none() {
            Err(Error::InvalidConfig(format!(
                "::column79::config::Config::validation(&self): \
                 language not found {}",
                self.language
            )))
        } else {
            Ok(())
        }
    }
    // ========================================================================
    /// check_path
    pub fn check_path(
        &self,
        path: &::std::path::PathBuf,
    ) -> Option<&Language> {
        if let Some(lang) = self.languages.get(&self.language) {
            lang.check_path(path, &self.languages)
        } else {
            None
        }
    }
}
