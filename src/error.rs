// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/13
//  @date 2025/03/01

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::error::Error as StdError;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive(Debug)]
pub enum Error {
    /// `EnvVar`
    EnvVar(std::env::VarError),
    /// IO
    IO(std::io::Error),
    /// `TOMLSer`
    TOMLSer(toml::ser::Error),
    /// `TOMLDe`
    TOMLDe(toml::de::Error),
    /// `ParseConfig`
    ParseConfig(String, toml::de::Error),
    /// Column79
    Column79(String),
    /// `InvalidConfig`
    InvalidConfig(String),
    /// Inspect
    Inspect(String),
}
// ============================================================================
impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Self::EnvVar(e)
    }
}
// ----------------------------------------------------------------------------
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}
// ----------------------------------------------------------------------------
impl From<toml::ser::Error> for Error {
    fn from(e: toml::ser::Error) -> Self {
        Self::TOMLSer(e)
    }
}
// ----------------------------------------------------------------------------
impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Self::TOMLDe(e)
    }
}
// ============================================================================
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}
// ============================================================================
impl StdError for Error {
    // ========================================================================
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Self::EnvVar(ref e) => Some(e),
            Self::IO(ref e) => Some(e),
            Self::TOMLSer(ref e) => Some(e),

            Self::TOMLDe(ref e) | Self::ParseConfig(_, ref e) => Some(e),

            Self::Column79(_) | Self::InvalidConfig(_) | Self::Inspect(_) => {
                None
            }
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
    const fn test_send() {
        const fn assert_send<T: Send>() {}
        assert_send::<Error>();
    }
    // ------------------------------------------------------------------------
    #[test]
    const fn test_sync() {
        const fn assert_sync<T: Sync>() {}
        assert_sync::<Error>();
    }
}
