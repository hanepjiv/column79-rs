// -*- mode:rust; coding:utf-8-unix; -*-

//! config.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/13
//  @date 2025/07/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use alloc::collections::{BTreeMap, btree_map::Entry};
use std::ffi::OsString;
// ----------------------------------------------------------------------------
use serde::Deserialize;
// ----------------------------------------------------------------------------
use crate::{
    error::Error,
    flags::Flags,
    language::{Language, LanguageSrc},
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct `ConfigSrc`
#[derive(Debug, Deserialize)]
pub(crate) struct ConfigSrc {
    /// column
    pub column: Option<usize>,
    /// `separator_threshold`
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
pub(crate) struct Config {
    /// column
    pub column: usize,
    /// `separator_threshold`
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
        Self {
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
    pub(crate) fn new(path: &OsString) -> Result<Self, Error> {
        let mut config = Self::default();
        config.import(path)?;
        Ok(config)
    }
    // ========================================================================
    /// import
    pub(crate) fn import(&mut self, path: &OsString) -> Result<(), Error> {
        let src: ConfigSrc = toml::from_str(&std::fs::read_to_string(path)?)?;
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
                            .to_owned(),
                    ));
                }
            }
        }
        Ok(())
    }
    // ========================================================================
    /// validation
    pub(crate) fn validation(&mut self) -> Result<(), Error> {
        match self.languages.entry(self.language.clone()) {
            Entry::<'_, _, _, _>::Vacant(_) => {
                Err(Error::InvalidConfig(format!(
                    "::column79::config::Config::validation(&self): \
                     language not found {}",
                    self.language
                )))
            }
            Entry::<'_, _, _, _>::Occupied(_) => Ok(()),
        }
    }
    // ========================================================================
    /// `check_path`
    pub(crate) fn check_path(
        &self,
        path: &std::path::PathBuf,
    ) -> Option<&Language> {
        self.languages
            .get(&self.language)?
            .check_path(path, &self.languages)
    }
}
