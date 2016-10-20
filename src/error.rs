// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/13
//  @date 2016/10/15

// ////////////////////////////////////////////////////////////////////////////
// extern  ====================================================================
extern                  crate toml;
// use  =======================================================================
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive( Debug, )]
pub enum Error {
    /// IOError
    IOError(::std::io::Error),
    /// ParseConfigError
    ParseConfigError(Vec<::toml::ParserError>),
    /// InvalidConfig
    InvalidConfig(&'static str),
    /// InspectError
    InspectError(&'static str),
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
        &Error::IOError(ref _inner)
            => "::column79::Error::IOError",
        &Error::ParseConfigError(ref _inner)
            => "::column79::Error::ParseConfigError",
        &Error::InvalidConfig(ref _inner)
            => "::column79::Error::InvalidConfig",
        &Error::InspectError(ref _inner)
            => "::column79::Error::InspectError",
    } }
}
