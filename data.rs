// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base::{
    AbstractCFTypeRef,
    CFAllocatorRef,
    CFIndex,
    CFRange,
    CFTypeRef,
    CFTypeID,
    CFWrapper,
    kCFAllocatorDefault,
};

use core::vec;

struct __CFData { private: () }
pub type CFDataRef = *__CFData;

impl AbstractCFTypeRef for CFDataRef {
    fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }

    fn type_id() -> CFTypeID {
        unsafe {
            CFDataGetTypeID()
        }
    }
}

// FIXME: Should be a newtype struct, but that fails due to a Rust compiler
// bug.
pub struct CFData {
    contents: CFWrapper<CFDataRef, (), ()>
}

pub impl CFData {
    fn wrap_owned(data: CFDataRef) -> CFData {
        CFData {
            contents: CFWrapper::wrap_owned(data)
        }
    }

    fn new_from_buf(buf: &[u8]) -> CFData {
        let result;
        unsafe {
            result = CFDataCreate(kCFAllocatorDefault, 
                                  vec::raw::to_ptr(buf),
                                  buf.len() as CFIndex);
        }

        CFData {
            contents: CFWrapper::wrap_owned(result)
        }
    }

    // tread with caution; read-only
    fn bytes(&self) -> *u8 {
        unsafe {
            CFDataGetBytePtr(self.contents.obj)
        }
    }

    fn len(&self) -> uint {
        unsafe {
            CFDataGetLength(self.contents.obj) as uint
        }
    }

    fn copy_to_buf(&self) -> ~[u8] {
        unsafe {
            vec::from_buf(self.bytes(), self.len())
        }
    }

    fn with_buf<U>(&self, blk: &fn(v: &[u8]) -> U) -> U {
        unsafe {
            vec::raw::buf_as_slice(self.bytes(), self.len(), blk)
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
