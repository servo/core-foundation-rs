// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Core Foundation Bundle Type

use core_foundation_sys::base::CFRelease;
use core_foundation_sys::bundle::{CFBundleRef, CFBundleGetTypeID};
use std::mem;

use base::{TCFType};

/// A Bundle type.
pub struct CFBundle(CFBundleRef);

impl Drop for CFBundle {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl_TCFType!(CFBundle, CFBundleRef, CFBundleGetTypeID);
