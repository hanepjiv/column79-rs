// -*- mode:rust; coding:utf-8-unix; -*-

//! line_type.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/21
//  @date 2018/05/11

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use config::Config;
use language::Language;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum LineType
#[derive(Debug, Clone)]
pub(crate) enum LineType {
    /// LineComment
    LineComment(String, String),
    /// LineSeparator
    LineSeparator(String, String),
    /// BlockComment
    BlockComment(String, String, String),
    /// BlockSeparator
    BlockSeparator(String, String, String),
    /// Other
    Other,
}
// ============================================================================
impl LineType {
    // ========================================================================
    pub(crate) fn head(&self) -> Option<&String> {
        match *self {
            LineType::LineComment(ref head, _) => Some(head),
            LineType::LineSeparator(ref head, _) => Some(head),
            LineType::BlockComment(ref head, _, _) => Some(head),
            LineType::BlockSeparator(ref head, _, _) => Some(head),
            LineType::Other => None,
        }
    }
    // ------------------------------------------------------------------------
    pub(crate) fn body(&self) -> Option<&String> {
        match *self {
            LineType::LineComment(_, ref body) => Some(body),
            LineType::LineSeparator(_, ref body) => Some(body),
            LineType::BlockComment(_, ref body, _) => Some(body),
            LineType::BlockSeparator(_, ref body, _) => Some(body),
            LineType::Other => None,
        }
    }
    // ------------------------------------------------------------------------
    pub(crate) fn foot(&self) -> Option<&String> {
        match *self {
            LineType::LineComment(_, _) => None,
            LineType::LineSeparator(_, _) => None,
            LineType::BlockComment(_, _, ref foot) => Some(foot),
            LineType::BlockSeparator(_, _, ref foot) => Some(foot),
            LineType::Other => None,
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
    ) -> Option<LineType> {
        if !lang.has_line_comment() {
            return None;
        }
        lang.re_line_captures(line)
            .map(|c| -> LineType {
                let head = String::from(c.get(1).unwrap().as_str());
                let body = String::from(c.get(2).unwrap().as_str());
                if LineType::is_separator(conf, &body) {
                    LineType::LineSeparator(head, body)
                } else {
                    LineType::LineComment(head, body)
                }
            })
    }
    // ------------------------------------------------------------------------
    pub(crate) fn is_block_comment(
        conf: &Config,
        lang: &Language,
        line: &str,
    ) -> Option<LineType> {
        if !lang.has_block_comment() {
            return None;
        }
        lang.re_block_captures(line)
            .map(|c| -> LineType {
                let head = String::from(c.get(1).unwrap().as_str());
                let body = String::from(c.get(2).unwrap().as_str());
                let foot = String::from(c.get(3).unwrap().as_str());
                if LineType::is_separator(conf, &body) {
                    LineType::BlockSeparator(head, body, foot)
                } else {
                    LineType::BlockComment(head, body, foot)
                }
            })
    }
    // ========================================================================
    pub(crate) fn new(conf: &Config, lang: &Language, line: &str) -> LineType {
        match LineType::is_block_comment(conf, lang, line) {
            Some(l) => l,
            None => match LineType::is_line_comment(conf, lang, line) {
                Some(b) => b,
                None => LineType::Other,
            },
        }
    }
}
