// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "core_graphics"]
#![crate_type = "rlib"]

extern crate libc;
extern crate core_foundation;

#[cfg(target_os="macos")]
pub mod base;
#[cfg(target_os="macos")]
pub mod data_provider;
#[cfg(target_os="macos")]
pub mod font;
#[cfg(target_os="macos")]
pub mod geometry;
#[cfg(target_os="macos")]
pub mod quartz_display_services;
