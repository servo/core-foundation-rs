// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Immutable numbers.

#![allow(non_uppercase_statics)]

use base::{CFAllocatorRef, CFRelease, CFRetain, CFTypeID, CFTypeRef};
use base::{TCFType, kCFAllocatorDefault};

use libc::c_void;
use std::mem;

pub type CFNumberType = u32;

// members of enum CFNumberType
// static kCFNumberSInt8Type:     CFNumberType = 1;
// static kCFNumberSInt16Type:    CFNumberType = 2;
// static kCFNumberSInt32Type:    CFNumberType = 3;
static kCFNumberSInt64Type:    CFNumberType = 4;
// static kCFNumberFloat32Type:   CFNumberType = 5;
static kCFNumberFloat64Type:   CFNumberType = 6;
// static kCFNumberCharType:      CFNumberType = 7;
// static kCFNumberShortType:     CFNumberType = 8;
// static kCFNumberIntType:       CFNumberType = 9;
// static kCFNumberLongType:      CFNumberType = 10;
// static kCFNumberLongLongType:  CFNumberType = 11;
// static kCFNumberFloatType:     CFNumberType = 12;
// static kCFNumberDoubleType:    CFNumberType = 13;
// static kCFNumberCFIndexType:   CFNumberType = 14;
// static kCFNumberNSIntegerType: CFNumberType = 15;
// static kCFNumberCGFloatType:   CFNumberType = 16;
// static kCFNumberMaxType:       CFNumberType = 16;

struct __CFNumber;

pub type CFNumberRef = *const __CFNumber;

/// An immutable numeric value.
///
/// FIXME(pcwalton): Should be a newtype struct, but that fails due to a Rust compiler bug.
pub struct CFNumber {
    obj: CFNumberRef,
}

impl Drop for CFNumber {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl TCFType<CFNumberRef> for CFNumber {
    #[inline]
    fn as_concrete_TypeRef(&self) -> CFNumberRef {
        self.obj
    }

    #[inline]
    unsafe fn wrap_under_get_rule(reference: CFNumberRef) -> CFNumber {
        let reference: CFNumberRef = mem::transmute(CFRetain(mem::transmute(reference)));
        TCFType::wrap_under_create_rule(reference)
    }

    #[inline]
    fn as_CFTypeRef(&self) -> CFTypeRef {
        unsafe {
            mem::transmute(self.as_concrete_TypeRef())
        }
    }

    unsafe fn wrap_under_create_rule(obj: CFNumberRef) -> CFNumber {
        CFNumber {
            obj: obj,
        }
    }

    #[inline]
    fn type_id(_: Option<CFNumber>) -> CFTypeID {
        unsafe {
            CFNumberGetTypeID()
        }
    }
}

// TODO(pcwalton): Floating point.
impl ToPrimitive for CFNumber {
    #[inline]
    fn to_i64(&self) -> Option<i64> {
        unsafe {
            let mut value: i64 = 0;
            let ok = CFNumberGetValue(self.obj, kCFNumberSInt64Type, mem::transmute(&mut value));
            assert!(ok);
            Some(value)
        }
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        // CFNumber does not support unsigned 64-bit values.
        None
    }

    #[inline]
    fn to_f64(&self) -> Option<f64> {
        unsafe {
            let mut value: f64 = 0.0;
            let ok = CFNumberGetValue(self.obj, kCFNumberFloat64Type, mem::transmute(&mut value));
            assert!(ok);
            Some(value)
        }
    }
}

// TODO(pcwalton): Floating point.
impl FromPrimitive for CFNumber {
    #[inline]
    fn from_i64(value: i64) -> Option<CFNumber> {
        unsafe {
            let number_ref = CFNumberCreate(kCFAllocatorDefault,
                                            kCFNumberSInt64Type,
                                            mem::transmute(&value));
            Some(TCFType::wrap_under_create_rule(number_ref))
        }
    }

    #[inline]
    fn from_u64(_: u64) -> Option<CFNumber> {
        // CFNumber does not support unsigned 64-bit values.
        None
    }

    #[inline]
    fn from_f64(value: f64) -> Option<CFNumber> {
        unsafe {
            let number_ref = CFNumberCreate(kCFAllocatorDefault,
                                            kCFNumberFloat64Type,
                                            mem::transmute(&value));
            Some(TCFType::wrap_under_create_rule(number_ref))
        }
    }
}

/// A convenience function to create CFNumbers.
pub fn number(value: i64) -> CFNumber {
    FromPrimitive::from_i64(value).unwrap()
}

#[link(name = "CoreFoundation", kind = "framework")]
extern {
    /*
     * CFNumber.h
     */


    fn CFNumberCreate(allocator: CFAllocatorRef, theType: CFNumberType, valuePtr: *const c_void)
                   -> CFNumberRef;
    //fn CFNumberGetByteSize
    fn CFNumberGetValue(number: CFNumberRef, theType: CFNumberType, valuePtr: *mut c_void) -> bool;
    //fn CFNumberCompare
    fn CFNumberGetTypeID() -> CFTypeID;
}

