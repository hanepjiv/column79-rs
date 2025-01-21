// -*- mode:rust; coding:utf-8-unix; -*-

//! flags.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/15
//  @date 2025/01/21

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use bitflags::bitflags;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
bitflags! {
    /// struct Flags
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Flags: u32 {
    /// const NOASK
    const NOASK = 0b0000_0001u32;
    }
}
