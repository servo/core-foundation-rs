// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// this file defines CGFloat, as well as stubbed data types.

use libc;
use base::CGError;

pub type CGDirectDisplayID = libc::uint32_t;

#[link(name = "ApplicationServices", kind = "framework")]
extern {
    pub fn CGMainDisplayID() -> CGDirectDisplayID;
    pub fn CGGetActiveDisplayList(max_displays: libc::uint32_t,
                                  active_displays: *mut CGDirectDisplayID,
                                  display_count: *mut libc::uint32_t) -> CGError;
    pub fn CGDisplayModelNumber(display: CGDirectDisplayID) -> libc::uint32_t;
    pub fn CGDisplayPixelsHigh(display: CGDirectDisplayID) -> libc::size_t;
    pub fn CGDisplayPixelsWide(display: CGDirectDisplayID) -> libc::size_t;
}
