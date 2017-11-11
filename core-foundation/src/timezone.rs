// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Core Foundation time zone objects.

pub use core_foundation_sys::timezone::*;
use core_foundation_sys::base::{CFRelease, kCFAllocatorDefault};

use base::TCFType;
use date::{CFDate, CFTimeInterval};

/// A time zone.
pub struct CFTimeZone(CFTimeZoneRef);

impl Drop for CFTimeZone {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl_TCFType!(CFTimeZone, CFTimeZoneRef, CFTimeZoneGetTypeID);

impl Default for CFTimeZone {
    fn default() -> CFTimeZone {
        unsafe {
            let tz_ref = CFTimeZoneCopyDefault();
            TCFType::wrap_under_create_rule(tz_ref)
        }
    }
}

impl CFTimeZone {
    #[inline]
    pub fn new(offset: CFTimeInterval) -> CFTimeZone {
        unsafe {
            let tz_ref = CFTimeZoneCreateWithTimeIntervalFromGMT(kCFAllocatorDefault, offset);
            TCFType::wrap_under_create_rule(tz_ref)
        }
    }

    #[inline]
    pub fn system() -> CFTimeZone {
        unsafe {
            let tz_ref = CFTimeZoneCopySystem();
            TCFType::wrap_under_create_rule(tz_ref)
        }
    }

    pub fn seconds_from_gmt(&self, date: CFDate) -> CFTimeInterval {
        unsafe {
            CFTimeZoneGetSecondsFromGMT(self.0, date.abs_time())
        }
    }
}

#[cfg(test)]
mod test {
    use super::CFTimeZone;

    #[test]
    fn timezone_comparison() {
        let system = CFTimeZone::system();
        let default = CFTimeZone::default();
        assert!(system == default);
    }
}
