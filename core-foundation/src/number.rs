// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Immutable numbers.

use core_foundation_sys::base::{CFRelease, kCFAllocatorDefault};
pub use core_foundation_sys::number::*;
use std::mem;

use base::{TCFType};

/// An immutable numeric value.
pub struct CFNumber(pub CFNumberRef);

impl Drop for CFNumber {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl_TCFType!(CFNumber, CFNumberRef, CFNumberGetTypeID);

// TODO(pcwalton): Floating point.
impl CFNumber {
    #[inline]
    pub fn from_i32(value: i32) -> CFNumber {
        unsafe {
            let number_ref = CFNumberCreate(kCFAllocatorDefault,
                                            kCFNumberSInt32Type,
                                            mem::transmute(&value));
            TCFType::wrap_under_create_rule(number_ref)
        }
    }
    #[inline]
    pub unsafe fn to_isize(&self, kcf_number_type: CFNumberType) -> Option<isize> {
        let mut value: isize = 0;
        let ok = CFNumberGetValue(self.0, kcf_number_type, mem::transmute(&mut value));
        if ok { Some(value) } else { None }
    }
    #[inline]
    pub fn to_i32(&self) -> Option<i32> {
        unsafe {
            let mut value: i32 = 0;
            let ok = CFNumberGetValue(self.0, kCFNumberSInt32Type, mem::transmute(&mut value));
            if ok { Some(value) } else { None }
        }
    }
    #[inline]
    pub fn to_i64(&self) -> Option<i64> {
        unsafe {
            let mut value: i64 = 0;
            let ok = CFNumberGetValue(self.0, kCFNumberSInt64Type, mem::transmute(&mut value));
            if ok { Some(value) } else { None }
        }
    }
    #[inline]
    pub fn to_f32(&self) -> Option<f32> {
        unsafe {
            let mut value: f32 = 0.0;
            let ok = CFNumberGetValue(self.0, kCFNumberFloatType, mem::transmute(&mut value));
            if ok { Some(value) } else { None }
        }
    }
    #[inline]
    pub fn to_f64(&self) -> Option<f64> {
        unsafe {
            let mut value: f64 = 0.0;
            let ok = CFNumberGetValue(self.0, kCFNumberFloat64Type, mem::transmute(&mut value));
            if ok { Some(value) } else { None }
        }
    }

    #[inline]
    pub fn from_i64(value: i64) -> CFNumber {
        unsafe {
            let number_ref = CFNumberCreate(kCFAllocatorDefault,
                                            kCFNumberSInt64Type,
                                            mem::transmute(&value));
            TCFType::wrap_under_create_rule(number_ref)
        }
    }

    #[inline]
    pub fn from_f64(value: f64) -> CFNumber {
        unsafe {
            let number_ref = CFNumberCreate(kCFAllocatorDefault,
                                            kCFNumberFloat64Type,
                                            mem::transmute(&value));
            TCFType::wrap_under_create_rule(number_ref)
        }
    }
}

/// A convenience function to create CFNumbers.
pub fn number(value: i64) -> CFNumber {
    CFNumber::from_i64(value)
}
