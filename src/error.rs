// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/13
//  @date 2017/10/16

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
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
// ============================================================================
impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::EnvVar(ref e) => e.description(),
            Error::IO(ref e) => e.description(),
            Error::TOMLSer(ref e) => e.description(),
            Error::TOMLDe(ref e) => e.description(),
            Error::Column79(ref m) => m.as_str(),
            Error::ParseConfig(ref m, _) => m.as_str(),
            Error::InvalidConfig(ref m) => m.as_str(),
            Error::Inspect(ref m) => m.as_str(),
        }
    }
    // ========================================================================
    fn cause(&self) -> Option<&::std::error::Error> {
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
