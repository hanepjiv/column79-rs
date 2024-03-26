// -*- mode:rust; coding:utf-8-unix; -*-

//! flags.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/10/15
//  @date 2024/03/26

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use bitflags::bitflags;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
bitflags! {
    #[allow(missing_docs)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Flags: u32 {
    #[allow(missing_docs)] const NOASK  = 0b0000_0001u32;
    }
}
