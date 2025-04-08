// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/05/11
//  @date 2025/04/06

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use core::error::Error as StdError;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive(Debug)]
pub(crate) enum Error {
    /// `OptionNone`
    OptionNone(String),
    /// `GetOpts`
    GetOpts(::getopts::Fail),
    /// `StdIO`
    StdIO(::std::io::Error),
    /// `StdNumParseInt`
    StdNumParseInt(::core::num::ParseIntError),
    /// Column79
    Column79(::column79::Error),
}
// ============================================================================
impl From<::getopts::Fail> for Error {
    fn from(e: ::getopts::Fail) -> Self {
        Self::GetOpts(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Self {
        Self::StdIO(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::core::num::ParseIntError> for Error {
    fn from(e: ::core::num::ParseIntError) -> Self {
        Self::StdNumParseInt(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::column79::Error> for Error {
    fn from(e: ::column79::Error) -> Self {
        Self::Column79(e)
    }
}
// ============================================================================
impl ::core::fmt::Display for Error {
    // ========================================================================
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match *self {
            Self::OptionNone(ref txt) => txt.fmt(f),
            Self::GetOpts(_)
            | Self::StdIO(_)
            | Self::StdNumParseInt(_)
            | Self::Column79(_) => <Self as ::core::fmt::Debug>::fmt(self, f),
        }
    }
}
// ============================================================================
impl StdError for Error {
    // ========================================================================
    fn cause(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Self::OptionNone(_) => None,
            Self::GetOpts(ref e) => Some(e),
            Self::StdIO(ref e) => Some(e),
            Self::StdNumParseInt(ref e) => Some(e),
            Self::Column79(ref e) => Some(e),
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type Result
pub(crate) type Result<T> = ::core::result::Result<T, Error>;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[cfg(test)]
mod tests {
    // use  ===================================================================
    use crate::{Error, Result};
    // ========================================================================
    #[test]
    const fn test_send() {
        const fn assert_send<T: Send>() {}
        assert_send::<Error>();
        assert_send::<Result<()>>();
    }
    // ------------------------------------------------------------------------
    #[test]
    const fn test_sync() {
        const fn assert_sync<T: Sync>() {}
        assert_sync::<Error>();
        assert_sync::<Result<()>>();
    }
}
