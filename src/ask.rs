// -*- mode:rust; coding:utf-8-unix; -*-

//! ask.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/21
//  @date 2016/10/21

// use  =======================================================================
use                             ::std::io::Write;
// ----------------------------------------------------------------------------
use                             error::Error;
use                             error::Error::IOError;
// ////////////////////////////////////////////////////////////////////////////
pub fn ask(msg: &str, default: bool) -> Result<bool, Error> {
    let _ = try!(::std::io::stdout().write_all(msg.as_ref())
                 .map_err(|e| IOError(e)));
    let _ = try!(::std::io::stdout()
                 .write_all(if default { b" [Y/n]: " } else { b" [y/N]: " })
                 .map_err(|e| IOError(e)));
    let _ = try!(::std::io::stdout().flush()
                 .map_err(|e| IOError(e)));

    let mut line = String::new();
    let _ = try!(::std::io::stdin().read_line(&mut line)
                 .map_err(|e| IOError(e)));

    match line.to_lowercase().trim() {
        ""          => Ok(default),
        "y"         => Ok(true),
        "n"         => Ok(false),
        _           => ask(msg, default),
    }
}
