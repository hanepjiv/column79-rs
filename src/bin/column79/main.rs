// -*- mode:rust; coding:utf-8-unix; -*-

//! main.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/12
//  @date 2024/11/27

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================

// mod  =======================================================================
mod error;
// use  =======================================================================
use std::{env, path::PathBuf};
// ----------------------------------------------------------------------------
use self::error::{Error, Result};
use column79::{Column79, Command, Flags};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
fn print_usage(opts: &::getopts::Options) {
    print!(
        "{}",
        opts.usage(&format!(
            r"Usage:
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
    let command = Command::from(matches.free[0].as_ref());
    if Command::Unknown == command {
        print_usage(&opts);
        return Ok(());
    }
    let input = if 1usize == matches.free.len() {
        ::std::env::current_dir()?
    } else {
        PathBuf::from(matches.free[1].clone())
    };
    let column = if matches.opt_present("c") {
        Some(
            matches
                .opt_str("c")
                .ok_or_else(|| {
                    Error::OptionNone(
                        "column79: matches.opt_str(\"c\").".to_string(),
                    )
                })?
                .parse::<usize>()?,
        )
    } else {
        None
    };
    let septhr = if matches.opt_present("t") {
        Some(
            matches
                .opt_str("t")
                .ok_or_else(|| {
                    Error::OptionNone(
                        "column79: matches.opt_str(\"t\").".to_string(),
                    )
                })?
                .parse::<usize>()?,
        )
    } else {
        None
    };
    let language = if matches.opt_present("l") {
        Some(matches.opt_str("l").ok_or_else(|| {
            Error::OptionNone("column79: matches.opt_str(\"l\").".to_string())
        })?)
    } else {
        None
    };
    let mut fs = Flags::empty();
    if matches.opt_present("no-ask") {
        fs.insert(Flags::NOASK);
    }

    Column79::run(command, input, language, column, septhr, fs)?;

    Ok(())
}
