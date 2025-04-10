// -*- mode:rust; coding:utf-8-unix; -*-

//! main.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/12
//  @date 2025/04/10

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),
                                            "/README.md")))]
// mod  =======================================================================
mod error;
// use  =======================================================================
use std::{env, path::PathBuf};
// ----------------------------------------------------------------------------
use bitflags as _;
use dirs as _;
use log as _;
use regex as _;
use serde as _;
use serde_derive as _;
use tempfile as _;
use toml as _;
// ----------------------------------------------------------------------------
use self::error::{Error, Result};
use column79::{Column79, Command, Flags};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
fn print_usage(opts: &::getopts::Options) {
    print!(
        "{}",
        opts.usage(&format!(
            "Usage:
    {0} Command [Input] [Options]

Command:
    init        initialize the configure
    check       to check the column overflow
    replace     replace the result of the checked

Input:
    ./          current directory (default)",
            module_path!()
        ))
    );
}
// ============================================================================
fn main() -> Result<()> {
    env_logger::init();

    let mut opts = ::getopts::Options::new();
    let _ = opts
        .optflag("v", "version", "print version")
        .optflag("h", "help", "print this help menu")
        .optopt("c", "column", "set column number", "NUM")
        .optopt("t", "threshold", "set separator threshold number", "NUM")
        .optopt(
            "l",
            "language",
            "set language LANG=('cargo'|'rust'|'c'|'c++'|...)",
            "LANG",
        )
        .optflag("", "no-ask", "will not be asked to allow");

    let matches = opts.parse(env::args().skip(1))?;
    if matches.opt_present("v") {
        println!(concat!(module_path!(), " v", env!("CARGO_PKG_VERSION")));
        return Ok(());
    }
    if matches.free.is_empty() || matches.opt_present("h") {
        print_usage(&opts);
        return Ok(());
    }

    let command = if let Some(cmd) = matches.free.first() {
        Command::from(cmd.as_ref())
    } else {
        print_usage(&opts);
        return Ok(());
    };

    if Command::Unknown == command {
        print_usage(&opts);
        return Ok(());
    }

    let input = if 1_usize == matches.free.len() {
        ::std::env::current_dir()?
    } else {
        PathBuf::from(matches.free.get(1).ok_or_else(|| {
            Error::OptionNone("column79: empty input".to_owned())
        })?)
    };

    let column = match matches.opt_str("c") {
        Some(x) => Some(x.parse::<usize>().map_err(|_e| {
            Error::OptionNone("column79: opt_str('c').".to_owned())
        })?),
        None => None,
    };

    let septhr = match matches.opt_str("t") {
        Some(x) => Some(x.parse::<usize>().map_err(|_e| {
            Error::OptionNone("column79: opt_str('t').".to_owned())
        })?),
        None => None,
    };

    let language = matches.opt_str("l");

    let mut fs = Flags::empty();

    if matches.opt_present("no-ask") {
        fs.insert(Flags::NOASK);
    }

    Column79::run(command, input, language, column, septhr, fs)?;

    Ok(())
}
