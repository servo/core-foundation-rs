// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Core Foundation byte buffers.

use base::{CFAllocatorRef, CFIndex, CFIndexConvertible, CFRange, CFRelease, CFTypeID, TCFType};
use base::{kCFAllocatorDefault};

use std::cast;
use std::vec;

struct __CFData;

pub type CFDataRef = *__CFData;

/// A byte buffer.
///
/// FIXME(pcwalton): Should be a newtype struct, but that fails due to a Rust compiler bug.
pub struct CFData {
    priv obj: CFDataRef,
}

impl Drop for CFData {
    #[fixed_stack_segment]
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl TCFType<CFDataRef> for CFData {
    fn as_concrete_TypeRef(&self) -> CFDataRef {
        self.obj
    }

    unsafe fn wrap_under_create_rule(obj: CFDataRef) -> CFData {
        CFData {
            obj: obj,
        }
    }

    #[fixed_stack_segment]
    #[inline]
    fn type_id(_: Option<CFData>) -> CFTypeID {
        unsafe {
            CFDataGetTypeID()
        }
    }
}

impl CFData {
    #[fixed_stack_segment]
    pub fn from_buffer(buffer: &[u8]) -> CFData {
        unsafe {
            let data_ref = CFDataCreate(kCFAllocatorDefault, 
                                        vec::raw::to_ptr(buffer),
                                        buffer.len().to_CFIndex());
            TCFType::wrap_under_create_rule(data_ref)
        }
    }

    /// Returns a pointer to the underlying bytes in this data. Note that this byte buffer is
    /// read-only.
    #[fixed_stack_segment]
    #[inline]
    pub fn bytes<'a>(&'a self) -> &'a [u8] {
        unsafe {
            cast::transmute((CFDataGetBytePtr(self.obj), self.len() as uint))
        }
    }

    /// Returns the length of this byte buffer.
    #[fixed_stack_segment]
    #[inline]
    pub fn len(&self) -> CFIndex {
        unsafe {
            CFDataGetLength(self.obj)
        }
    }
}

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    /*
     * CFData.h
     */

    fn CFDataCreate(allocator: CFAllocatorRef, 
                    bytes: *u8, length: CFIndex) -> CFDataRef;
    fn CFDataCreateCopy(allocator: CFAllocatorRef, theData: CFDataRef) -> CFDataRef;
    fn CFDataCreateWithBytesNoCopy(allocator: CFAllocatorRef, 
                                   bytes: *u8, length: CFIndex, 
                                   bytesDeallocator: CFAllocatorRef) -> CFDataRef;
    //fn CFDataFind
    fn CFDataGetBytePtr(theData: CFDataRef) -> *u8;
    fn CFDataGetBytes(theData: CFDataRef, range: CFRange, buffer: *u8);
    fn CFDataGetLength(theData: CFDataRef) -> CFIndex;

    fn CFDataGetTypeID() -> CFTypeID;
}
