// -*- mode:rust; coding:utf-8-unix; -*-

//! `line_type.rs`

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/21
//  @date 2025/03/01

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use crate::{config::Config, language::Language};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum `LineType`
#[derive(Debug, Clone)]
pub(crate) enum LineType {
    /// `LineComment`
    LineComment(String, String),
    /// `LineSeparator`
    LineSeparator(String, String),
    /// `BlockComment`
    BlockComment(String, String, String),
    /// `BlockSeparator`
    BlockSeparator(String, String, String),
    /// Other
    Other,
}
// ============================================================================
impl LineType {
    // ========================================================================
    pub(crate) const fn head(&self) -> Option<&String> {
        match *self {
            Self::LineComment(ref head, _)
            | Self::LineSeparator(ref head, _)
            | Self::BlockComment(ref head, _, _)
            | Self::BlockSeparator(ref head, _, _) => Some(head),

            Self::Other => None,
        }
    }
    // ------------------------------------------------------------------------
    pub(crate) const fn body(&self) -> Option<&String> {
        match *self {
            Self::LineComment(_, ref body)
            | Self::LineSeparator(_, ref body)
            | Self::BlockComment(_, ref body, _)
            | Self::BlockSeparator(_, ref body, _) => Some(body),

            Self::Other => None,
        }
    }
    // ------------------------------------------------------------------------
    pub(crate) const fn foot(&self) -> Option<&String> {
        match *self {
            Self::BlockComment(_, _, ref foot)
            | Self::BlockSeparator(_, _, ref foot) => Some(foot),

            Self::LineComment(_, _)
            | Self::LineSeparator(_, _)
            | Self::Other => None,
        }
    }
    // ========================================================================
    pub(crate) fn is_separator(conf: &Config, body: &str) -> bool {
        let t = conf.separator_threshold;
        if body.len() < t {
            return false;
        }
        let mut s = body.chars().rev();
        let b = s.nth(0).unwrap();
        for (c, i) in s.enumerate() {
            if b != i {
                return false;
            }
            if c >= t {
                break;
            }
        }
        true
    }
    // ========================================================================
    pub(crate) fn is_line_comment(
        conf: &Config,
        lang: &Language,
        line: &str,
    ) -> Option<Self> {
        if !lang.has_line_comment() {
            return None;
        }
        lang.re_line_captures(line).map(|c| -> Self {
            let head = String::from(c.get(1).unwrap().as_str());
            let body = String::from(c.get(2).unwrap().as_str());
            if Self::is_separator(conf, &body) {
                Self::LineSeparator(head, body)
            } else {
                Self::LineComment(head, body)
            }
        })
    }
    // ------------------------------------------------------------------------
    pub(crate) fn is_block_comment(
        conf: &Config,
        lang: &Language,
        line: &str,
    ) -> Option<Self> {
        if !lang.has_block_comment() {
            return None;
        }
        lang.re_block_captures(line).map(|c| -> Self {
            let head = String::from(c.get(1).unwrap().as_str());
            let body = String::from(c.get(2).unwrap().as_str());
            let foot = String::from(c.get(3).unwrap().as_str());
            if Self::is_separator(conf, &body) {
                Self::BlockSeparator(head, body, foot)
            } else {
                Self::BlockComment(head, body, foot)
            }
        })
    }
    // ========================================================================
    pub(crate) fn new(conf: &Config, lang: &Language, line: &str) -> Self {
        Self::is_block_comment(conf, lang, line).map_or_else(
            || {
                Self::is_line_comment(conf, lang, line)
                    .map_or(Self::Other, |b| b)
            },
            |l| l,
        )
    }
}
