// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/13
//  @date 2016/11/11

// ////////////////////////////////////////////////////////////////////////////
// extern  ====================================================================
extern                  crate toml;
// use  =======================================================================
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive( Debug, )]
pub enum Error {
    /// Column79Error
    Column79Error(String),
    /// IOError
    IOError(String, ::std::io::Error),
    /// ParseConfigError
    ParseConfigError(String, Vec<::toml::ParserError>),
    /// InvalidConfigError
    InvalidConfigError(String),
    /// InspectError
    InspectError(String),
}
// ============================================================================
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
// ============================================================================
impl ::std::error::Error for Error {
    fn description(&self) -> &str { match self {
        &Error::Column79Error(ref m)            => m.as_str(),
        &Error::IOError(ref m, _)               => m.as_str(),
        &Error::ParseConfigError(ref m, _)      => m.as_str(),
        &Error::InvalidConfigError(ref m)       => m.as_str(),
        &Error::InspectError(ref m)             => m.as_str(),
    } }
}
