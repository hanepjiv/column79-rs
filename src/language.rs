// -*- mode:rust; coding:utf-8-unix; -*-

//! language.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/13
//  @date 2017/10/16

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::cell::RefCell;
use std::collections::BTreeMap;
// ----------------------------------------------------------------------------
use regex::{Regex, Captures};
// ----------------------------------------------------------------------------
use super::error::Error;
use super::error::Error::InvalidConfigError;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct LanguageSrc
#[derive(Debug, Deserialize)]
pub struct LanguageSrc {
    /// name
    pub name: Option<String>,
    /// base
    pub base: Option<String>,
    /// extensions
    pub extensions: Option<Vec<String>>,
    /// line_comment_begin
    pub line_comment_begin: Option<String>,
    /// block_comment_begin
    pub block_comment_begin: Option<String>,
    /// block_comment_end
    pub block_comment_end: Option<String>,
    /// sublanguages
    pub sublanguages: Option<Vec<String>>,
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Language
#[derive(Debug, Clone, Default)]
pub struct Language {
    /// name
    name: String,
    /// base
    base: Option<String>,
    /// extensions
    extensions: Vec<String>,
    /// line_comment_begin
    line_comment_begin: Option<String>,
    /// block_comment_begin
    block_comment_begin: Option<String>,
    /// block_comment_end
    block_comment_end: Option<String>,
    /// sublanguages
    sublanguages: Vec<String>,
    /// re_line
    re_line: RefCell<Option<Regex>>,
    /// re_block
    re_block: RefCell<Option<Regex>>,
}
// ============================================================================
impl Language {
    // ========================================================================
    pub fn peek_name(&self) -> &String {
        &self.name
    }
    pub fn peek_lcb(&self) -> &Option<String> {
        &self.line_comment_begin
    }
    pub fn peek_bcb(&self) -> &Option<String> {
        &self.block_comment_begin
    }
    // pub fn peek_bce(&self)  -> &Option<String>  { &self.block_comment_end }
    // ========================================================================
    pub fn has_line_comment(&self) -> bool {
        self.line_comment_begin.is_some()
    }
    // ------------------------------------------------------------------------
    pub fn has_block_comment(&self) -> bool {
        self.block_comment_begin.is_some() && self.block_comment_end.is_some()
    }
    // ========================================================================
    /// extend
    pub fn extend(&mut self, base: &Language) {
        if self.line_comment_begin.is_none() &&
            base.line_comment_begin.is_some()
        {
            self.line_comment_begin = base.line_comment_begin.clone();
        }
        if self.block_comment_begin.is_none() &&
            base.block_comment_begin.is_some()
        {
            self.block_comment_begin = base.block_comment_begin.clone();
        }
        if self.block_comment_end.is_none() &&
            base.block_comment_end.is_some()
        {
            self.block_comment_end = base.block_comment_end.clone();
        }
    }
    // ========================================================================
    fn check_descent(
        &self,
        ls: &BTreeMap<String, Language>,
        descent: &mut Vec<String>,
    ) -> Result<(), Error> {
        if descent.contains(&self.name) {
            return Err(InvalidConfigError(format!(
                "::column79::language::Language::check_descent(...): \
                 name = \"{}\": cyclic dependencies",
                self.name
            )));
        }
        descent.push(self.name.clone());
        if let Some(ref base) = self.base {
            if !ls.contains_key(base) {
                return Err(InvalidConfigError(format!(
                    "::column79::language::Language::check_descent(...): \
                     name = \"{}\", base = \"{}\": invalid base",
                    self.name,
                    base
                )));
            }
            let _ = ls.get(base).unwrap().check_descent(ls, descent)?;
        }
        Ok(())
    }
    // ------------------------------------------------------------------------
    pub fn from_src(
        src: LanguageSrc,
        languages: &BTreeMap<String, Language>,
    ) -> Result<Self, Error> {
        let mut ret = Language::default();
        if let Some(x) = src.name {
            ret.name = x;
        }
        ret.base = src.base;
        if let Some(x) = src.extensions {
            ret.extensions = x;
        }
        ret.line_comment_begin = src.line_comment_begin;
        ret.block_comment_begin = src.block_comment_begin;
        ret.block_comment_end = src.block_comment_end;
        if let Some(x) = src.sublanguages {
            ret.sublanguages = x;
        }

        ret.check_descent(languages, &mut Vec::default())?;

        if ret.base.is_some() {
            let base = ret.base.clone().unwrap();
            ret.extend(languages.get(&base).unwrap());
        }

        Ok(ret)
    }
    // ========================================================================
    pub fn re_line_captures<'t>(&self, line: &'t str) -> Option<Captures<'t>> {
        if let Some(ref lcb) = self.line_comment_begin {
            if let Some(ref mut re) = *self.re_line.borrow_mut() {
                return re.captures(line);
            }
            let re = Regex::new(&format!(r##"^(.*?{}\s*)(.*)$"##, lcb))
                .expect("re_line_captures");
            let ret = re.captures(line);
            *self.re_line.borrow_mut() = Some(re);
            ret
        } else {
            None
        }
    }
    // ------------------------------------------------------------------------
    pub fn re_block_captures<'t>(
        &self,
        line: &'t str,
    ) -> Option<Captures<'t>> {
        if let Some(ref bcb) = self.block_comment_begin {
            if let Some(ref bce) = self.block_comment_end {
                if let Some(ref mut re) = *self.re_block.borrow_mut() {
                    return re.captures(line);
                }
                let re = Regex::new(
                    &format!(r##"^(.*?{}\s*)(.*?)(\s*{})$"##, bcb, bce),
                ).expect("re_block_captures");
                let ret = re.captures(line);
                *self.re_block.borrow_mut() = Some(re);
                ret
            } else {
                None
            }
        } else {
            None
        }
    }
    // ========================================================================
    pub fn check_path_<'a>(
        &'a self,
        p: &::std::path::PathBuf,
        ls: &'a BTreeMap<String, Language>,
    ) -> Option<&'a Language> {
        for i in self.sublanguages.iter() {
            if let x @ Some(_) = ls.get(i).unwrap().check_path(p, ls) {
                return x;
            }
        }
        None
    }
    // ------------------------------------------------------------------------
    pub fn check_path<'a>(
        &'a self,
        path: &::std::path::PathBuf,
        languages: &'a BTreeMap<String, Language>,
    ) -> Option<&'a Language> {
        if let Some(ext) = path.extension() {
            if let Ok(ref s) = ext.to_os_string().into_string() {
                if self.extensions.contains(s) {
                    Some(self)
                } else {
                    self.check_path_(path, languages)
                }
            } else {
                self.check_path_(path, languages)
            }
        } else {
            self.check_path_(path, languages)
        }
    }
}
