// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_snake_case)]
#![allow(deprecated)] // TODO(madsmtm): Remove this

extern crate block2;
#[macro_use]
extern crate bitflags;
extern crate core_foundation;
extern crate core_graphics_types;
extern crate libc;
#[macro_use]
pub extern crate objc2;

#[macro_export]
macro_rules! impl_Encode {
    ($t:ty, $delegation:ty) => {
        unsafe impl $crate::objc2::encode::Encode for $t {
            const ENCODING: $crate::objc2::encode::Encoding = <$delegation>::ENCODING;
        }
    };
}

pub mod base;
pub mod foundation;
