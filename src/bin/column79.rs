// -*- mode:rust; coding:utf-8-unix; -*-

//! main.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/12
//  @date 2016/11/09

//! # Examples
//!
//! ```
//! ```

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![deny(fat_ptr_transmutes, missing_copy_implementations,
        missing_debug_implementations, missing_docs, unstable_features,
        unused_results, unused_import_braces, variant_size_differences)]
#![warn(unused_qualifications, unused_extern_crates, warnings)]
#![allow(box_pointers, trivial_casts, trivial_numeric_casts, unsafe_code)]
// extern  ====================================================================
#[macro_use] extern     crate env_logger;
// ----------------------------------------------------------------------------
extern                  crate getopts;
#[macro_use] extern     crate column79;
// use  =======================================================================
use                     ::std::path::PathBuf;
// ----------------------------------------------------------------------------
use                     ::column79::{ flags, Command, Column79, };
// mod  =======================================================================
#[macro_use] mod        unwrap;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
fn print_usage(opts: &::getopts::Options, program: &str) {
    print!("{}", opts.usage(&format!(r##"Usage:
    {0} Command [Input] [Options]

Command:
    init        initialize the configure
    check       to check the column overflow
    replace     replace the result of the checked

Input:
    ./          current directory (default)"##, program)));
}
// ============================================================================
fn main() {
    let _ = env_logger::init().unwrap();

    let args: Vec<String> = ::std::env::args().collect();
    let mut opts = ::getopts::Options::new();
    let _ = opts.optflag("h", "help", "print this help menu")
        .optopt("c", "column", "set column number", "NUM")
        .optopt("t", "threshold", "set separator threshold number", "NUM")
        .optopt("l", "language",
                "set language LANG=('rust'|'c'|'c++'|...)", "LANG")
        .optflag("", "no-ask", "will not be asked to allow");

    let matches = unwrap!(opts.parse(&args[1..]));

    if matches.free.is_empty() || matches.opt_present("h") {
        return print_usage(&opts, args[0].as_ref());
    }

    let command = Command::from(matches.free[0].as_ref());
    if Command::Unknown == command {
        return print_usage(&opts, args[0].as_ref());
    }

    let input = if 1usize == matches.free.len() {
        unwrap!(::std::env::current_dir())
    } else {
        PathBuf::from(matches.free[1].clone())
    };

    let column = if matches.opt_present("c") {
        Some(unwrap!(unwrap!(matches.opt_str("c")).parse::<usize>()))
    } else {
        None
    };

    let septhr = if matches.opt_present("t") {
        Some(unwrap!(unwrap!(matches.opt_str("t")).parse::<usize>()))
    } else {
        None
    };

    let language = if matches.opt_present("l") {
        Some(unwrap!(matches.opt_str("l")))
    } else {
        None
    };

    let mut fs = flags::Flags::empty();
    if matches.opt_present("no-ask") { fs.insert(flags::NOASK); }

    unwrap!(Column79::run(command, input, language, column, septhr, fs));
}
