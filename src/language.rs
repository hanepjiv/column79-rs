// -*- mode:rust; coding:utf-8-unix; -*-

//! language.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/13
//  @date 2025/04/06

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use alloc::collections::BTreeMap;
use core::cell::RefCell;
// ----------------------------------------------------------------------------
use regex::{Captures, Regex};
use serde_derive::Deserialize;
// ----------------------------------------------------------------------------
use crate::error::Error;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct `LanguageSrc`
#[derive(Debug, Deserialize)]
pub(crate) struct LanguageSrc {
    /// name
    pub name: Option<String>,
    /// base
    pub base: Option<String>,
    /// extensions
    pub extensions: Option<Vec<String>>,
    /// `line_comment_begin`
    pub line_comment_begin: Option<String>,
    /// `block_comment_begin`
    pub block_comment_begin: Option<String>,
    /// `block_comment_end`
    pub block_comment_end: Option<String>,
    /// sublanguages
    pub sublanguages: Option<Vec<String>>,
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Language
#[derive(Debug, Clone, Default)]
pub(crate) struct Language {
    /// name
    name: String,
    /// base
    base: Option<String>,
    /// extensions
    extensions: Vec<String>,
    /// `line_comment_begin`
    line_comment_begin: Option<String>,
    /// `block_comment_begin`
    block_comment_begin: Option<String>,
    /// `block_comment_end`
    block_comment_end: Option<String>,
    /// sublanguages
    sublanguages: Vec<String>,
    /// `re_line`
    re_line: RefCell<Option<Regex>>,
    /// `re_block`
    re_block: RefCell<Option<Regex>>,
}
// ============================================================================
impl Language {
    // ========================================================================
    pub(crate) const fn peek_name(&self) -> &String {
        &self.name
    }
    pub(crate) const fn peek_lcb(&self) -> Option<&String> {
        self.line_comment_begin.as_ref()
    }
    pub(crate) const fn peek_bcb(&self) -> Option<&String> {
        self.block_comment_begin.as_ref()
    }
    /*
    pub(crate) fn peek_bce(&self)  -> &Option<String>  {
    &self.block_comment_end
    }
     */
    // ========================================================================
    pub(crate) const fn has_line_comment(&self) -> bool {
        self.line_comment_begin.is_some()
    }
    // ------------------------------------------------------------------------
    pub(crate) const fn has_block_comment(&self) -> bool {
        self.block_comment_begin.is_some() && self.block_comment_end.is_some()
    }
    // ========================================================================
    /// extend
    pub(crate) fn extend(&mut self, base: &Self) {
        if self.line_comment_begin.is_none()
            && base.line_comment_begin.is_some()
        {
            self.line_comment_begin.clone_from(&base.line_comment_begin);
        }
        if self.block_comment_begin.is_none()
            && base.block_comment_begin.is_some()
        {
            self.block_comment_begin
                .clone_from(&base.block_comment_begin);
        }
        if self.block_comment_end.is_none() && base.block_comment_end.is_some()
        {
            self.block_comment_end.clone_from(&base.block_comment_end);
        }
    }
    // ========================================================================
    #[expect(
        clippy::unwrap_used,
        clippy::unwrap_in_result,
        reason = "checked"
    )]
    fn check_descent(
        &self,
        ls: &BTreeMap<String, Self>,
        descent: &mut Vec<String>,
    ) -> Result<(), Error> {
        if descent.contains(&self.name) {
            return Err(Error::InvalidConfig(format!(
                "::column79::language::Language::check_descent(...): \
                 name = \"{}\": cyclic dependencies",
                self.name
            )));
        }
        descent.push(self.name.clone());
        if let Some(ref base) = self.base {
            if !ls.contains_key(base) {
                return Err(Error::InvalidConfig(format!(
                    "::column79::language::Language::check_descent(...): \
                     name = \"{}\", base = \"{}\": invalid base",
                    self.name, base
                )));
            }
            ls.get(base).unwrap().check_descent(ls, descent)?;
        }
        Ok(())
    }
    // ------------------------------------------------------------------------
    #[expect(
        clippy::unwrap_used,
        clippy::unwrap_in_result,
        reason = "checked"
    )]
    pub(crate) fn from_src(
        src: LanguageSrc,
        languages: &BTreeMap<String, Self>,
    ) -> Result<Self, Error> {
        let mut ret = Self::default();
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
    #[expect(
        clippy::expect_used,
        clippy::unwrap_in_result,
        reason = "checked"
    )]
    pub(crate) fn re_line_captures<'t>(
        &self,
        line: &'t str,
    ) -> Option<Captures<'t>> {
        if let Some(ref lcb) = self.line_comment_begin {
            if let Some(ref mut re) = *self.re_line.borrow_mut() {
                return re.captures(line);
            }
            let re = Regex::new(&format!(r"^(.*?{lcb}\s*)(.*)$"))
                .expect("re_line_captures");
            let ret = re.captures(line);
            *self.re_line.borrow_mut() = Some(re);
            ret
        } else {
            None
        }
    }
    // ------------------------------------------------------------------------
    #[expect(
        clippy::expect_used,
        clippy::unwrap_in_result,
        reason = "checked"
    )]
    pub(crate) fn re_block_captures<'t>(
        &self,
        line: &'t str,
    ) -> Option<Captures<'t>> {
        if let Some(ref bcb) = self.block_comment_begin {
            if let Some(ref bce) = self.block_comment_end {
                if let Some(ref mut re) = *self.re_block.borrow_mut() {
                    return re.captures(line);
                }
                let re =
                    Regex::new(&format!(r"^(.*?{bcb}\s*)(.*?)(\s*{bce})$"))
                        .expect("re_block_captures");
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
    #[expect(
        clippy::unwrap_used,
        clippy::unwrap_in_result,
        reason = "checked"
    )]
    pub(crate) fn check_path_<'a>(
        &'a self,
        p: &std::path::PathBuf,
        ls: &'a BTreeMap<String, Self>,
    ) -> Option<&'a Self> {
        for i in &self.sublanguages {
            if let x @ Some(_) = ls.get(i).unwrap().check_path(p, ls) {
                return x;
            }
        }
        None
    }
    // ------------------------------------------------------------------------
    pub(crate) fn check_path<'a>(
        &'a self,
        path: &std::path::PathBuf,
        languages: &'a BTreeMap<String, Self>,
    ) -> Option<&'a Self> {
        path.extension().map_or_else(
            || self.check_path_(path, languages),
            |ext| {
                ext.to_os_string().into_string().as_ref().map_or_else(
                    |_| self.check_path_(path, languages),
                    |s| {
                        if self.extensions.contains(s) {
                            Some(self)
                        } else {
                            self.check_path_(path, languages)
                        }
                    },
                )
            },
        )
    }
}
