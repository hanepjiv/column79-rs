// -*- mode:rust; coding:utf-8-unix; -*-

//! language.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/13
//  @date 2016/11/11

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use                     ::std::collections::BTreeMap;
// ----------------------------------------------------------------------------
use                     ::toml::{ Value, Table };
use                     ::regex::{ Regex, Captures, };
// ----------------------------------------------------------------------------
use                     error::Error;
use                     Error::InvalidConfigError;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct LanguageCommon
#[derive( Debug, Clone, )]
pub struct LanguageCommon {
    /// name
    name:               String,
    /// exts
    exts:               Vec<String>,
    /// lcb
    lcb:                Option<String>,
    /// bcb
    bcb:                Option<String>,
    /// bce
    bce:                Option<String>,
    /// sublanguages
    sublanguages:       Vec<String>,
}
// ============================================================================
impl LanguageCommon {
    // ========================================================================
    /// new
    pub fn new(table: &Table) -> Result<Self, Error> {
        let name_value = table.get("name").ok_or(InvalidConfigError(format!(
            "::column79::language::LanguageCommon::new(...): \
             get(\"name\")")))?;
        let name = name_value.as_str().ok_or(InvalidConfigError(format!(
            "::column79::language::LanguageCommon::new(...): \
             name = \"{:?}\": as_str", name_value)))?;
        let exts_src = match table.get("extensions") {
            Some(v)     =>
                v.as_slice().ok_or(InvalidConfigError(format!(
                    "::column79::language::LanguageCommon::new(...): \
                     : name = \"{}\": as_slice", name)))?,
            None        => &[],
        };
        let mut exts = Vec::new();
        for i in exts_src {
            exts.push(String::from(
                i.as_str().ok_or(InvalidConfigError(format!(
                    "::column79::language::LanguageCommon::new(...): \
                     : name = \"{}\", extension = {:?}: \
                     as_str", name, i)))?))
        }
        let lcb = match table.get("line_comment_begin") {
            Some(v)     => Some(String::from(
                v.as_str().ok_or(InvalidConfigError(format!(
                    "::column79::language::LanguageCommon::new(...): \
                     : name = \"{}\", line_comment_begin = \"{:?}\": \
                     as_str", name, v)))?)),
            None        => None,
        };
        let bcb = match table.get("block_comment_begin") {
            Some(v)     => Some(String::from(
                v.as_str().ok_or(InvalidConfigError(format!(
                    "::column79::language::LanguageCommon::new(...): \
                     : name = \"{}\", block_comment_begin = \"{:?}\": \
                     as_str", name, v)))?)),
            None        => None,
        };
        let bce = match table.get("block_comment_end") {
            Some(v)     => Some(String::from(
                v.as_str().ok_or(InvalidConfigError(format!(
                    "::column79::language::LanguageCommon::new(...): \
                     : name = \"{}\", block_comment_end = \"{:?}\": \
                     as_str", name, v)))?)),
            None        => None,
        };
        let mut sublanguages = Vec::new();
        {
            let sl = table.get("sublanguages");
            if sl.is_some() {
                for i in sl.unwrap().as_slice()
                    .ok_or(InvalidConfigError(format!(
                        "::column79::language::LanguageCommon::new(...): \
                         : name = \"{}\", sublanguages = \"{:?}\": \
                         as_slice", name, sl)))? {
                    sublanguages.push(String::from(
                        i.as_str().ok_or(InvalidConfigError(format!(
                            "::column79::language::LanguageCommon::new(...): \
                             : name = \"{}\", sublanguage = \"{:?}\": \
                             as_str", name, i)))?));
                    }
            }
        }
        Ok(LanguageCommon {
            name:               String::from(name),
            exts:               exts,
            lcb:                lcb,
            bcb:                bcb,
            bce:                bce,
            sublanguages:       sublanguages,
        })
    }
    // ========================================================================
    /// extend
    pub fn extend(&mut self, src: &LanguageCommon) {
        if self.lcb.is_none() && src.lcb.is_some() {
            self.lcb = src.lcb.clone();
        }
        if self.bcb.is_none() && src.bcb.is_some() {
            self.bcb = src.bcb.clone();
        }
        if self.bce.is_none() && src.bce.is_some() {
            self.bce = src.bce.clone();
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct LanguageSrc
#[derive( Debug, Clone, )]
pub struct LanguageSrc {
    /// common
    common:             LanguageCommon,
    /// base
    base:               Option<String>,
}
// ============================================================================
impl LanguageSrc {
    // ========================================================================
    /// new
    pub fn new(table: &Table) -> Result<Self, Error> {
        let common = LanguageCommon::new(table)?;
        let base = match table.get("base") {
            Some(v)     => Some(String::from(
                v.as_str().ok_or(InvalidConfigError(format!(
                    "::column79::language::LanguageSrc::new(...): \
                     : name = \"{}\", base = {:?}: \
                     as_str", common.name, v)))?)),
            None        => None,
        };

        Ok(LanguageSrc {
            common:     common,
            base:       base,
        })
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Language
#[derive( Debug, Clone, )]
pub struct Language {
    /// re_line
    re_line:    Regex,
    /// re_block
    re_block:   Regex,
    /// common
    common:     LanguageCommon,
}
// ============================================================================
impl  Language {
    // ========================================================================
    pub fn peek_name(&self) -> &String { &self.common.name }
    pub fn peek_lcb(&self) -> &Option<String> { &self.common.lcb }
    pub fn peek_bcb(&self) -> &Option<String> { &self.common.bcb }
    // pub fn peek_bce(&self) -> &Option<String> { &self.common.bce }
    // ========================================================================
    pub fn has_line_comment(&self) -> bool { self.common.lcb.is_some() }
    pub fn has_block_comment(&self) -> bool { self.common.bcb.is_some() }
    // ========================================================================
    pub fn re_line_captures<'t>(&self, line: &'t str)
                                -> Option<Captures<'t>> {
        self.re_line.captures(line)
    }
    pub fn re_block_captures<'t>(&self, line: &'t str)
                                 -> Option<Captures<'t>> {
        self.re_block.captures(line)
    }
    // ========================================================================
    /// new
    pub fn new(srcs:    &BTreeMap<String, LanguageSrc>,
               name:    &String,
               descent: &mut Vec<String>,
               langs:   &mut BTreeMap<String, Language>)
               -> Result<Self, Error> {
        descent.push(name.clone());
        let src = srcs.get(name).ok_or(InvalidConfigError(format!(
            "::column79::language::Language::new(..., \"{}\", ...): \
             not found", name)))?;

        let mut common = src.common.clone();
        if src.base.is_some() {
            let base_name = src.base.clone().unwrap();
            if descent.contains(&base_name) {
                return Err(InvalidConfigError(format!(
                    "::column79::language::Language::new(...): \
                     name = \"{}\" base = \"{}\": \
                     cyclic dependencies", common.name, base_name)));
            }
            if !langs.contains_key(&base_name) {
                let l = Language::new(srcs, &base_name, descent, langs)?;
                match langs.insert(base_name.clone(), l) {
                    Some(_)     => {
                        return Err(InvalidConfigError(format!(
                            "::column79::language::Language::new(...): \
                             name = \"{}\": \
                             already exists", base_name)))
                    }
                    None        => (),
                }
            }
            let base = langs.get(&base_name).unwrap();
            common.extend(&base.common.clone());
        }

        for ref i in &src.common.sublanguages {
            if !langs.contains_key(i.clone()) {
                let l = Language::new(srcs, i, &mut Vec::new(), langs)?;
                match langs.insert((*i).clone(), l) {
                    Some(_)     => {
                        return Err(InvalidConfigError(format!(
                            "::column79::language::Language::new(...): \
                             name = \"{}\": \
                             already exists", i)))
                    }
                    None        => (),
                }
            }
        }

        Ok(Language {
            re_line:    Regex::new(&format!(
                r##"^(.*?{}\s*)(.*)$"##,
                common.lcb.clone().unwrap_or(String::new())))
                .map_err(|_| {
                    InvalidConfigError(format!(
                        "::column79::language::Language::new(...): \
                         name = \"{}\": \
                         regex line comment", common.name))
                })?,
            re_block:   Regex::new(&format!(
                r##"^(.*?{}\s*)(.*?)(\s*{})$"##,
                common.bcb.clone().unwrap_or(String::new()),
                common.bce.clone().unwrap_or(String::new())))
                .map_err(|_| {
                    InvalidConfigError(format!(
                        "::column79::language::Language::new(...): \
                         name = \"{}\": \
                         regex block comment", common.name))
                })?,
            common:     common,
        })
    }
    // ========================================================================
    pub fn check_path<'a>(&'a self, path: &::std::path::PathBuf,
                           languages: &'a BTreeMap<String, Language>)
                           -> Option<&'a Language> {
        if match path.extension() {
            Some(ext)           => match ext.to_os_string().into_string() {
                Ok(ref s)       => self.common.exts.contains(s),
                Err(_)          => false,
            },
            None                => false,
        } {
            Some(self)
        } else {
            for ref i in self.common.sublanguages.clone() {
                match languages.get(i).unwrap().check_path(path, languages) {
                    x@Some(_)   => return x,
                    None        => (),
                }
            }
            None
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
pub fn parse_languages(slice: &[Value],
                       languages: &mut BTreeMap<String, Language>)
                       -> Result<(), Error> {
    let mut srcs = BTreeMap::new();
    for i in slice {
        let table = i.as_table().ok_or(InvalidConfigError(format!(
            "::column79::language::parse_languages(...): \
             as_table {:?}", i)))?;
        let src = LanguageSrc::new(table)?;
        let name = src.common.name.clone();
        match srcs.insert(name.clone(), src.clone()) {
            Some(_)     => {
                return Err(InvalidConfigError(format!(
                    "::column79::language::parse_languages: \
                     name = \"{}\": \
                     already exists", name)))
            }
            None        => (),
        }
    }

    for k in srcs.keys() {
        if languages.contains_key(k) { continue; }
        let l = Language::new(&srcs, k, &mut Vec::new(), languages)?;
        match languages.insert(k.clone(), l) {
            Some(_)     => {
                return Err(InvalidConfigError(format!(
                    "::column79::language::parse_languages: \
                     name = \"{}\": \
                     already exists", k)))
            }
            None        => (),
        }
    }

    Ok(())
}
