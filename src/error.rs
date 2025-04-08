// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/13
//  @date 2025/04/06

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use core::error::Error as CoreError;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive(Debug)]
#[non_exhaustive]
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
    #[inline]
    fn from(e: std::env::VarError) -> Self {
        Self::EnvVar(e)
    }
}
// ----------------------------------------------------------------------------
impl From<std::io::Error> for Error {
    #[inline]
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}
// ----------------------------------------------------------------------------
impl From<toml::ser::Error> for Error {
    #[inline]
    fn from(e: toml::ser::Error) -> Self {
        Self::TOMLSer(e)
    }
}
// ----------------------------------------------------------------------------
impl From<toml::de::Error> for Error {
    #[inline]
    fn from(e: toml::de::Error) -> Self {
        Self::TOMLDe(e)
    }
}
// ============================================================================
impl core::fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as core::fmt::Debug>::fmt(self, f)
    }
}
// ============================================================================
impl CoreError for Error {
    // ========================================================================
    #[inline]
    fn source(&self) -> Option<&(dyn CoreError + 'static)> {
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
