// Copyright 2013-2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

use base::{CFAllocatorRef, CFTypeID, CFIndex, CFRange};

#[repr(C)]
pub struct __CFData(c_void);

pub type CFDataRef = *const __CFData;
pub type CFMutableDataRef = *mut __CFData;

extern {
    /*
     * CFData.h
     */

    pub fn CFDataCreate(allocator: CFAllocatorRef,
                        bytes: *const u8, length: CFIndex) -> CFDataRef;
    // fn CFDataCreateWithBytesNoCopy
    pub fn CFDataGetBytePtr(theData: CFDataRef) -> *const u8;
    
    pub fn CFDataGetBytes(theData: CFDataRef, range: CFRange, buffer: *mut u8);
    pub fn CFDataGetLength(theData: CFDataRef) -> CFIndex;
    // fn CFDataFind

    pub fn CFDataGetTypeID() -> CFTypeID;

    pub fn CFDataCreateMutable(allocator: CFAllocatorRef,
                               capacity: CFIndex) -> CFMutableDataRef;
    // fn CFDataCreateMutableCopy
    pub fn CFDataGetMutableBytePtr(theData: CFMutableDataRef) -> *mut u8;

    pub fn CFDataSetLength(theData: CFMutableDataRef, length: CFIndex);
    pub fn CFDataIncreaseLength(theData: CFMutableDataRef, extraLength: CFIndex);
    pub fn CFDataAppendBytes(theData: CFMutableDataRef,
                             bytes: *const u8, length: CFIndex);
    pub fn CFDataReplaceBytes(theData: CFMutableDataRef,
                              range: CFRange,
                              newBytes: *const u8,
                              newLength: CFIndex);
    pub fn CFDataDeleteBytes(theData: CFMutableDataRef, range: CFRange);
}
