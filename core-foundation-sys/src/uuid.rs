// Copyright 2013-2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use libc::c_void;

use base::CFAllocatorRef;

#[repr(C)]
pub struct __CFUUID(c_void);

pub type CFUUIDRef = *const __CFUUID;

#[repr(C)]
pub struct CFUUIDBytes {
    byte0:  u8,
    byte1:  u8,
    byte2:  u8,
    byte3:  u8,
    byte4:  u8,
    byte5:  u8,
    byte6:  u8,
    byte7:  u8,
    byte8:  u8,
    byte9:  u8,
    byte10: u8,
    byte11: u8,
    byte12: u8,
    byte13: u8,
    byte14: u8,
    byte15: u8
}

extern {
    /*
     * CFUUID.h
     */
    pub fn CFUUIDCreateFromUUIDBytes(allocator: CFAllocatorRef, bytes: CFUUIDBytes) -> CFUUIDRef;
}
