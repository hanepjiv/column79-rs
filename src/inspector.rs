// -*- mode:rust; coding:utf-8-unix; -*-

//! inspector.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/14
//  @date 2017/02/16

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use                     ::std::path::PathBuf;
use                     ::std::fs::File;
use                     ::std::io::{ Seek, SeekFrom,
                                     BufRead, BufReader,
                                     BufWriter, Write, };
// ----------------------------------------------------------------------------
use                     ::tempfile::tempfile;
use                     ::regex::Regex;
// ----------------------------------------------------------------------------
use                     error::Error;
use                     error::Error::{ IOError, InspectError, };
use                     config::Config;
use                     flags;
use                     line_type::LineType;
use                     language::Language;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// trait Inspector
pub trait Inspector: ::std::fmt::Debug {
    // ========================================================================
    /// inspect
    fn inspect(&self, lang: &Language, path: &PathBuf) -> Result<(), Error>;
    // ========================================================================
    /// inspect_impl
    fn inspect_impl<F>(&self, conf: &Config, lang: &Language,
                       path: &PathBuf, func: &mut F)
                       -> Result<(), Error>
        where F: FnMut(usize, &LineType, &str) -> Result<(), Error>, {
        let file_in = File::open(path).map_err(|e| IOError(format!(
            "::column79::inspector::Inspecrot::inspect_impl(..., \"{:?}\"): \
             open ", path),e))?;
        let fin = BufReader::new(&file_in);
        let mut row = 0usize;
        for line in fin.lines() {
            row += 1usize;
            let l = &String::from(line.unwrap().trim_right());
            let line_type = LineType::new(conf, lang, l);
            func(row, &line_type, l)?
        }
        Ok(())
    }
    // ========================================================================
    /// println_line
    fn println_line(&self, path: &PathBuf, row: usize, line: &str)
                    -> Result<(), Error> {
        println!("{}({}): {} : {}",
                 path.clone().into_os_string().into_string().unwrap(),
                 row, line.len(), line);
        Ok(())
    }
    // ========================================================================
    /// ask
    fn ask(&self, config : &Config, msg: &str, default: bool)
           -> Result<bool, Error> {
        if config.flags.contains(flags::NOASK) { return Ok(default); }
        ::ask::ask(msg, default)
    }
    // ========================================================================
    /// check_type
    fn check_type(&self, lang: &Language,
                  column: usize, line_type: &LineType, line: &str) -> bool {
        match *line_type {
            LineType::LineComment(_, _)         =>
                column >= line.len(),
            LineType::LineSeparator(_, _)       =>
                column == line.len(),
            LineType::BlockComment(_, _, _)     =>
                column >= line.len() && !lang.has_line_comment(),
            LineType::BlockSeparator(_, _, _)   =>
                column == line.len() && !lang.has_line_comment(),
            LineType::Other                     =>
                column >= line.len(),
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Checker
#[derive( Debug, Clone, )]
pub struct Checker<'a> {
    /// config
    config:             &'a Config,
}
// ============================================================================
impl <'a> Checker<'a> {
    // ========================================================================
    /// new
    pub fn new(config: &'a Config) -> Self { Checker {
        config:         config,
    } }
}
// ============================================================================
impl <'a> Inspector for Checker<'a> {
    // ========================================================================
    /// inspect
    fn inspect(&self, lang: &Language, path: &PathBuf) -> Result<(), Error> {
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
#[derive( Debug, Clone, )]
pub struct Replacer<'a> {
    /// config
    config:             &'a Config,
}
// ============================================================================
impl <'a> Replacer<'a> {
    // ========================================================================
    /// new
    pub fn new(config: &'a Config) -> Self { Replacer {
        config:         config,
    } }
    // ========================================================================
    /// line_separator
    fn line_separator(&self, _lang: &Language, path: &PathBuf,
                      row: usize, line_type: &LineType, line: &str)
                      -> Result<(bool, String), Error> {
        let l = line.len();
        let c = self.config.column;
        let body = line_type.body().unwrap();

        if c < l {
            if self.ask(self.config, "* shrink?", true)? {
                let mut s = String::from(line);
                for _ in 0..(l - c) {
                    let _ = s.pop().ok_or(InspectError(format!(
                        "::column79::inspector::Replacer::line_separator : \
                         path = \"{:?}\", row = {}: \
                         pop", path, row)))?;
                }
                Ok((true, s))
            } else {
                Ok((false, String::from(line)))
            }
        } else {
            if self.ask(self.config, "* expand?", true)? {
                let mut s = String::from(line);
                let b = body.chars().rev().nth(0).unwrap();
                for _ in 0..(c - l) { s.push(b) }
                Ok((true, s))
            } else {
                Ok((false, String::from(line)))
            }
        }
    }
    // ========================================================================
    /// make_line
    fn make_line(&self, lang: &Language, line_type: &LineType) -> String {
        let mut s =
            Regex::new(&format!(r"(.*){}(.*)",
                                lang.peek_bcb().clone().unwrap())).unwrap().
            replace(&line_type.head().unwrap(),
                     format!(r"$1{}$2",
                             lang.peek_lcb().clone().unwrap()).as_str()).
            into_owned();
        s.push_str(line_type.body().unwrap());
        s
    }
    // ========================================================================
    /// make_line_separator
    fn make_line_separator(&self, lang: &Language, line_type: &LineType)
                           -> String {
        let c = self.config.column;
        let mut s = self.make_line(lang, line_type);
        let d = s.len() as isize - c as isize;
        if 0 < d {
            for _ in 0..d { let _ = s.pop(); }
        } else if 0 > d {
            let b = line_type.body().unwrap().chars().rev().nth(0).unwrap();
            for _ in 0..-d { s.push(b) }
        }
        s
    }
    // ========================================================================
    /// block_comment
    fn block_comment(&self, lang: &Language, _path: &PathBuf,
                     _row: usize, line_type: &LineType, line: &str)
                     -> Result<(bool, String), Error> {
        if !self.ask(self.config, "* convert to line comment?", true)? {
            Ok((false, String::from(line)))
        } else {
            let s = self.make_line(lang, line_type);
            Ok((true, s))
        }
    }
    // ========================================================================
    /// block_separator
    fn block_separator(&self, lang: &Language, path: &PathBuf,
                       row: usize, line_type: &LineType, line: &str)
                       -> Result<(bool, String), Error> {
        let l = line.len();
        let c = self.config.column;
        let has_line = lang.has_line_comment();
        let body = line_type.body().unwrap();
        if c == l {
            if has_line && self.ask(self.config,
                                    "* convert to line comment?", true)? {
                let s = self.make_line_separator(lang, line_type);
                Ok((true, s))
            } else {
                Ok((false, String::from(line)))
            }
        } else if c < l {
            if has_line && self.ask(self.config,
                                    "* convert to line comment with shrink?",
                                    true)? {
                let s = self.make_line_separator(lang, line_type);
                Ok((true, s))
            } else if self.ask(self.config, "* shrink?", true)? {
                let mut s = line_type.head().unwrap().clone();
                s.push_str(body);
                for _ in 0..(l - c) {
                    let _ = s.pop().ok_or(InspectError(format!(
                        "::column79::inspector::Replacer::block_separator : \
                         path = \"{:?}\", row = {}: \
                         pop", path, row)))?;
                }
                s.push_str(line_type.foot().unwrap());
                Ok((true, s))
            } else {
                Ok((false, String::from(line)))
            }
        } else {
            if has_line && self.ask(self.config,
                                    "* convert to line comment with expand?",
                                    true)? {
                let s = self.make_line_separator(lang, line_type);
                Ok((true, s))
            } else if self.ask(self.config, "* expand?", true)? {
                let mut s = line_type.head().unwrap().clone();
                s.push_str(body);
                let b = body.chars().rev().nth(0).unwrap();
                for _ in 0..(c - l) { s.push(b) }
                s.push_str(line_type.foot().unwrap());
                Ok((true, s))
            } else {
                Ok((false, String::from(line)))
            }
        }
    }
}
// ============================================================================
impl <'a> Inspector for Replacer<'a> {
    // ========================================================================
    /// inspect
    fn inspect(&self, lang: &Language, path: &PathBuf)
               -> Result<(), Error> {
        let c = self.config.column;
        let mut file_tmp = tempfile().map_err(|e| IOError(format!(
            "::column79::inspector::Replacer::inspect : \
             path = \"{:?}\" \
             tempfile", path), e))?;
        let mut ftmp = BufWriter::new(&mut file_tmp);
        let mut fixes = false;
        let _ =
            self.inspect_impl(self.config, lang, path, &mut |row, l_type, l| {
                let (f, mut s) = if self.check_type(lang, c, l_type, l) {
                    (false, String::from(l))
                } else {
                    let _ = self.println_line(path, row, l);
                    match *l_type {
                        LineType::LineSeparator(_, _)           => {
                            self.line_separator(lang, path, row, l_type, l)
                        },
                        LineType::BlockComment(_, _, _)         => {
                            self.block_comment(lang, path, row, l_type, l)
                        },
                        LineType::BlockSeparator(_, _, _)       => {
                            self.block_separator(lang, path, row, l_type, l)
                        },
                        LineType::LineComment(_, _)             |
                        LineType::Other                         =>
                            Ok((false, String::from(l))),
                    }?
                };
                s.push('\n');
                let _ = ftmp.write(s.as_ref()).map_err(|e| IOError(format!(
                    "::column79::inspector::Replacer::inspect : \
                     path = \"{:?}\" \
                     tempfile.write(\"{}\")", path, s), e))?;
                fixes |= f;
                Ok(())
            })?;
        if fixes {
            let file_tmp = ftmp.into_inner().unwrap();
            let _ = file_tmp.seek(SeekFrom::Start(0))
                .map_err(|e| IOError(format!(
                    "::column79::inspector::Replacer::inspect : \
                     path = \"{:?}\" \
                     tempfile.seek", path), e))?;
            let mut ftmp = BufReader::new(file_tmp);
            {  // backup
                let mut extension = path.extension().unwrap().to_os_string();
                extension.push(".backup");
                let mut path_back = path.clone();
                path_back.set_extension(extension);
                println!("* backup: {:?}", path_back.clone().into_os_string());
                let _ = ::std::fs::rename(path, path_back)
                    .map_err(|e| IOError(format!(
                        "::column79::inspector::Replacer::inspect : \
                         ::std::fs::rename(\"{:?}\", ...)", path), e))?;
            }
            let mut file_new = File::create(path).map_err(|e| IOError(format!(
                "::column79::inspector::Replacer::inspect : \
                 File::create(\"{:?}\")", path), e))?;
            let mut fnew = BufWriter::new(&mut file_new);
            let _ = ::std::io::copy(&mut ftmp, &mut fnew)
                .map_err(|e| IOError(format!(
                    "::column79::inspector::Replacer::inspect : \
                     ::std::io::copy(...)"), e))?;
            println!("* replace: {:?}", path.clone().into_os_string());
        }
        Ok(())
    }
}
