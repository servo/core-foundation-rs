// Copyright 2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Evil private APIs.
//!
//! These are liable to change at any time. Use with caution!

use geometry::CGRect;
use std::ptr;

pub struct CGSRegion {
    region: ffi::CGSRegionRef,
}

impl Drop for CGSRegion {
    fn drop(&mut self) {
        unsafe {
            ffi::CGSRegionRelease(self.region)
        }
    }
}

impl CGSRegion {
    #[inline]
    pub fn from_rect(rect: &CGRect) -> CGSRegion {
        unsafe {
            let mut region = ptr::null_mut();
            assert!(ffi::CGSNewRegionWithRect(rect, &mut region) == 0);
            CGSRegion {
                region: region,
            }
        }
    }
}

mod ffi {
    use geometry::CGRect;

    // This is an enum so that we can't easily make instances of this opaque type.
    enum CGSRegionObject {}

    pub type CGError = OSStatus;
    pub type CGSRegionRef = *mut CGSRegionObject;
    pub type OSStatus = i32;

    #[link(name = "ApplicationServices", kind = "framework")]
    extern {
        pub fn CGSRegionRelease(region: CGSRegionRef);
        pub fn CGSNewRegionWithRect(rect: *const CGRect, outRegion: *mut CGSRegionRef) -> CGError;
    }
}

