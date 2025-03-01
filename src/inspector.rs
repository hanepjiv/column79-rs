// -*- mode:rust; coding:utf-8-unix; -*-

//! inspector.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/14
//  @date 2024/12/02

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    fs::File,
    io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write},
    path::Path,
};
// ----------------------------------------------------------------------------
use regex::Regex;
use tempfile::tempfile;
// ----------------------------------------------------------------------------
use crate::{
    config::Config, error::Error, flags::Flags, language::Language,
    line_type::LineType,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// trait Inspector
pub(crate) trait Inspector: std::fmt::Debug {
    // ========================================================================
    /// inspect
    fn inspect(&self, lang: &Language, path: &Path) -> Result<(), Error>;
    // ========================================================================
    /// `inspect_impl`
    fn inspect_impl(
        &self,
        conf: &Config,
        lang: &Language,
        path: &Path,
        func: &mut impl FnMut(usize, &LineType, &str) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let file_in = File::open(path)?;
        let fin = BufReader::new(&file_in);
        for (row, line) in fin.lines().enumerate() {
            let l = &line.unwrap();
            let line_type = LineType::new(conf, lang, l);
            func(row + 1, &line_type, l)?;
        }
        Ok(())
    }
    // ========================================================================
    /// `println_line`
    fn println_line(
        &self,
        path: &Path,
        row: usize,
        line: &str,
    ) -> Result<(), Error> {
        println!(
            "{}({}): {} : {}",
            path.as_os_str().to_str().unwrap(),
            row,
            line.len(),
            line
        );
        Ok(())
    }
    // ========================================================================
    /// ask
    fn ask(
        &self,
        config: &Config,
        msg: &str,
        default: bool,
    ) -> Result<bool, Error> {
        if config.flags.contains(Flags::NOASK) {
            return Ok(default);
        }
        crate::ask::ask(msg, default)
    }
    // ========================================================================
    /// `check_type`
    fn check_type(
        &self,
        lang: &Language,
        column: usize,
        line_type: &LineType,
        line: &str,
    ) -> bool {
        match *line_type {
            LineType::LineComment(_, _)
            | LineType::LineSeparator(_, _)
            | LineType::Other => column >= line.len(),

            LineType::BlockComment(_, _, _) => {
                column >= line.len() && !lang.has_line_comment()
            }
            LineType::BlockSeparator(_, _, _) => {
                column == line.len() && !lang.has_line_comment()
            }
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Checker
#[derive(Debug, Clone)]
pub(crate) struct Checker<'a> {
    /// config
    config: &'a Config,
}
// ============================================================================
impl<'a> Checker<'a> {
    // ========================================================================
    /// new
    pub(crate) const fn new(config: &'a Config) -> Self {
        Checker { config }
    }
}
// ============================================================================
impl Inspector for Checker<'_> {
    // ========================================================================
    /// inspect
    fn inspect(&self, lang: &Language, path: &Path) -> Result<(), Error> {
        let c = self.config.column;
        self.inspect_impl(self.config, lang, path, &mut |row, line_type, l| {
            if self.check_type(lang, c, line_type, l) {
                Ok(())
            } else {
                self.println_line(path, row, l)
            }
        })
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Replacer
#[derive(Debug, Clone)]
pub(crate) struct Replacer<'a> {
    /// config
    config: &'a Config,
}
// ============================================================================
impl<'a> Replacer<'a> {
    // ========================================================================
    /// new
    pub(crate) const fn new(config: &'a Config) -> Self {
        Replacer { config }
    }
    // ========================================================================
    /// `line_separator`
    fn line_separator(
        &self,
        _lang: &Language,
        path: &Path,
        row: usize,
        line_type: &LineType,
        line: &str,
    ) -> Result<(bool, String), Error> {
        let l = line.len();
        let c = self.config.column;
        let body = line_type.body().unwrap();

        if c < l {
            if self.ask(self.config, "* shrink?", true)? {
                let mut s = String::from(line);
                for _ in 0..(l - c) {
                    let _ = s.pop().ok_or_else(|| {
                        Error::Inspect(format!(
                            "::column79::inspector::Replacer::line_separator: \
                         path = \"{path:?}\", row = {row}: \
                         pop"
                        ))
                    })?;
                }
                Ok((true, s))
            } else {
                Ok((false, String::from(line)))
            }
        } else if self.ask(self.config, "* expand?", true)? {
            let mut s = String::from(line);
            let b = body.chars().rev().nth(0).unwrap();
            for _ in 0..(c - l) {
                s.push(b);
            }
            Ok((true, s))
        } else {
            Ok((false, String::from(line)))
        }
    }
    // ========================================================================
    /// `make_line`
    fn make_line(lang: &Language, line_type: &LineType) -> String {
        let mut s =
            Regex::new(&format!(r"(.*){}(.*)", lang.peek_bcb().unwrap()))
                .unwrap()
                .replace(
                    line_type.head().unwrap(),
                    format!(r"$1{}$2", lang.peek_lcb().unwrap()).as_str(),
                )
                .into_owned();
        s.push_str(line_type.body().unwrap());
        s
    }
    // ========================================================================
    /// `make_line_separator`
    #[allow(clippy::cast_possible_wrap)]
    fn make_line_separator(
        &self,
        lang: &Language,
        line_type: &LineType,
    ) -> String {
        let c = self.config.column;
        let mut s = Self::make_line(lang, line_type);
        let d = s.len() as isize - c as isize;
        match d.cmp(&0) {
            Less => {
                let b =
                    line_type.body().unwrap().chars().rev().nth(0).unwrap();
                for _ in 0..-d {
                    s.push(b);
                }
            }
            Greater => {
                for _ in 0..d {
                    let _ = s.pop();
                }
            }
            Equal => {}
        }
        s
    }
    // ========================================================================
    /// `block_comment`
    fn block_comment(
        &self,
        lang: &Language,
        _path: &Path,
        _row: usize,
        line_type: &LineType,
        line: &str,
    ) -> Result<(bool, String), Error> {
        if self.ask(self.config, "* convert to line comment?", true)? {
            let s = Self::make_line(lang, line_type);
            Ok((true, s))
        } else {
            Ok((false, String::from(line)))
        }
    }
    // ========================================================================
    /// `block_separator`
    fn block_separator(
        &self,
        lang: &Language,
        path: &Path,
        row: usize,
        line_type: &LineType,
        line: &str,
    ) -> Result<(bool, String), Error> {
        let l = line.len();
        let c = self.config.column;
        let has_line = lang.has_line_comment();
        let body = line_type.body().unwrap();
        if c == l {
            if has_line
                && self.ask(self.config, "* convert to line comment?", true)?
            {
                let s = self.make_line_separator(lang, line_type);
                Ok((true, s))
            } else {
                Ok((false, String::from(line)))
            }
        } else if c < l {
            if has_line
                && self.ask(
                    self.config,
                    "* convert to line comment with shrink?",
                    true,
                )?
            {
                let s = self.make_line_separator(lang, line_type);
                Ok((true, s))
            } else if self.ask(self.config, "* shrink?", true)? {
                let mut s = line_type.head().unwrap().clone();
                s.push_str(body);
                for _ in 0..(l - c) {
                    let _ = s.pop().ok_or_else(|| {
                        Error::Inspect(format!(
                        "::column79::inspector::Replacer::block_separator : \
                         path = \"{path:?}\", row = {row}: \
                         pop"
                    ))
                    })?;
                }
                s.push_str(line_type.foot().unwrap());
                Ok((true, s))
            } else {
                Ok((false, String::from(line)))
            }
        } else if has_line
            && self.ask(
                self.config,
                "* convert to line comment with expand?",
                true,
            )?
        {
            let s = self.make_line_separator(lang, line_type);
            Ok((true, s))
        } else if self.ask(self.config, "* expand?", true)? {
            let mut s = line_type.head().unwrap().clone();
            s.push_str(body);
            let b = body.chars().rev().nth(0).unwrap();
            for _ in 0..(c - l) {
                s.push(b);
            }
            s.push_str(line_type.foot().unwrap());
            Ok((true, s))
        } else {
            Ok((false, String::from(line)))
        }
    }
}
// ============================================================================
impl Inspector for Replacer<'_> {
    // ========================================================================
    /// inspect
    fn inspect(&self, lang: &Language, path: &Path) -> Result<(), Error> {
        let c = self.config.column;
        let mut file_tmp = tempfile()?;
        let mut ftmp = BufWriter::new(&mut file_tmp);
        let mut fixes = false;
        self.inspect_impl(self.config, lang, path, &mut |row, l_type, l| {
            let (f, mut s) = if self.check_type(lang, c, l_type, l) {
                (false, String::from(l))
            } else {
                drop(self.println_line(path, row, l));
                match *l_type {
                    LineType::LineSeparator(_, _) => {
                        self.line_separator(lang, path, row, l_type, l)
                    }
                    LineType::BlockComment(_, _, _) => {
                        self.block_comment(lang, path, row, l_type, l)
                    }
                    LineType::BlockSeparator(_, _, _) => {
                        self.block_separator(lang, path, row, l_type, l)
                    }
                    LineType::LineComment(_, _) | LineType::Other => {
                        Ok((false, String::from(l)))
                    }
                }?
            };
            s.push('\n');
            let _ = ftmp.write(s.as_ref())?;
            fixes |= f;
            Ok(())
        })?;
        if fixes {
            let file_tmp = ftmp.into_inner().unwrap();
            let _ = file_tmp.seek(SeekFrom::Start(0))?;
            let mut ftmp = BufReader::new(file_tmp);
            {
                // backup
                let mut extension = path.extension().unwrap().to_os_string();
                extension.push(".backup");
                let path_back = path.with_extension(extension);
                println!("* backup: {:?}", path_back.as_os_str());
                std::fs::rename(path, path_back)?;
            }
            let mut file_new = File::create(path)?;
            let mut fnew = BufWriter::new(&mut file_new);
            let _ = std::io::copy(&mut ftmp, &mut fnew)?;
            println!("* replace: {:?}", path.as_os_str());
        }
        Ok(())
    }
}
