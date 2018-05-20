// -*- mode:rust; coding:utf-8-unix; -*-

//! ask.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/21
//  @date 2018/05/20

// use  =======================================================================
use std::io::Write;
// ----------------------------------------------------------------------------
use error::Error;
// ////////////////////////////////////////////////////////////////////////////
pub(crate) fn ask(msg: &str, default: bool) -> Result<bool, Error> {
    ::std::io::stdout().write_all(msg.as_ref())?;
    ::std::io::stdout().write_all(if default {
        b" [Y/n]: "
    } else {
        b" [y/N]: "
    })?;
    ::std::io::stdout().flush()?;
    let mut line = String::new();
    let _ = ::std::io::stdin().read_line(&mut line)?;
    match line.trim().to_lowercase().as_str() {
        "" => Ok(default),
        "y" | "yes" => Ok(true),
        "n" | "no" => Ok(false),
        _ => ask(msg, default),
    }
}
