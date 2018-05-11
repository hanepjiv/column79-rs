// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/05/11
//  @date 2018/05/11

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::error::Error as StdErr;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive(Debug)]
pub(crate) enum Error {
    /// OptionNone
    OptionNone(String),
    /// GetOpts
    GetOpts(::getopts::Fail),
    /// StdIO
    StdIO(::std::io::Error),
    /// StdNumParseInt
    StdNumParseInt(::std::num::ParseIntError),
    /// Column79
    Column79(::column79::Error),
}
// ============================================================================
impl From<::getopts::Fail> for Error {
    fn from(e: ::getopts::Fail) -> Self {
        Error::GetOpts(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Self {
        Error::StdIO(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::std::num::ParseIntError> for Error {
    fn from(e: ::std::num::ParseIntError) -> Self {
        Error::StdNumParseInt(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::column79::Error> for Error {
    fn from(e: ::column79::Error) -> Self {
        Error::Column79(e)
    }
}
// ============================================================================
impl ::std::fmt::Display for Error {
    // ========================================================================
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
// ============================================================================
impl ::std::error::Error for Error {
    // ========================================================================
    fn description(&self) -> &str {
        match *self {
            Error::OptionNone(_) => "::column79::Error::OptionNone",
            Error::GetOpts(ref e) => e.description(),
            Error::StdIO(ref e) => e.description(),
            Error::StdNumParseInt(ref e) => e.description(),
            Error::Column79(ref e) => e.description(),
        }
    }
    // ========================================================================
    fn cause(&self) -> Option<&dyn StdErr> {
        match *self {
            Error::OptionNone(_) => None,
            Error::GetOpts(ref e) => Some(e),
            Error::StdIO(ref e) => Some(e),
            Error::StdNumParseInt(ref e) => Some(e),
            Error::Column79(ref e) => Some(e),
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type Result
pub(crate) type Result<T> = ::std::result::Result<T, Error>;
