// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/13
//  @date 2020/04/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::error::Error as StdError;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive(Debug)]
pub enum Error {
    /// EnvVar
    EnvVar(::std::env::VarError),
    /// IO
    IO(::std::io::Error),
    /// TOMLSer
    TOMLSer(::toml::ser::Error),
    /// TOMLDe
    TOMLDe(::toml::de::Error),
    /// Column79
    Column79(String),
    /// ParseConfig
    ParseConfig(String, ::toml::de::Error),
    /// InvalidConfig
    InvalidConfig(String),
    /// Inspect
    Inspect(String),
}
// ============================================================================
impl From<::std::env::VarError> for Error {
    fn from(e: ::std::env::VarError) -> Self {
        Error::EnvVar(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Self {
        Error::IO(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::toml::ser::Error> for Error {
    fn from(e: ::toml::ser::Error) -> Self {
        Error::TOMLSer(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::toml::de::Error> for Error {
    fn from(e: ::toml::de::Error) -> Self {
        Error::TOMLDe(e)
    }
}
// ============================================================================
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        <Self as ::std::fmt::Debug>::fmt(self, f)
    }
}
// ============================================================================
impl StdError for Error {
    // ========================================================================
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Error::EnvVar(ref e) => Some(e),
            Error::IO(ref e) => Some(e),
            Error::TOMLSer(ref e) => Some(e),
            Error::TOMLDe(ref e) => Some(e),
            Error::Column79(_) => None,
            Error::ParseConfig(_, ref e) => Some(e),
            Error::InvalidConfig(_) => None,
            Error::Inspect(_) => None,
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[cfg(test)]
mod tests {
    // use  ===================================================================
    use crate::Error;
    // ========================================================================
    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Error>();
    }
    // ------------------------------------------------------------------------
    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Error>();
    }
}
