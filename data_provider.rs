// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core_foundation::base::{AbstractCFTypeRef, CFTypeID, CFTypeRef, CFWrapper};

use std::cast;
use std::libc::{c_void, c_char, size_t};
use std::ptr;

pub type CGDataProviderGetBytesCallback = *u8;
pub type CGDataProviderReleaseInfoCallback = *u8;
pub type CGDataProviderRewindCallback = *u8;
pub type CGDataProviderSkipBytesCallback = *u8;
pub type CGDataProviderSkipForwardCallback = *u8;

pub type CGDataProviderGetBytePointerCallback = *u8;
pub type CGDataProviderGetBytesAtOffsetCallback = *u8;
pub type CGDataProviderReleaseBytePointerCallback = *u8;
pub type CGDataProviderReleaseDataCallback = *u8;
pub type CGDataProviderGetBytesAtPositionCallback = *u8;

struct __CGDataProvider { private: () }
pub type CGDataProviderRef = *__CGDataProvider;

impl AbstractCFTypeRef for CGDataProviderRef {
    fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }

    fn type_id() -> CFTypeID {
        unsafe {
            CGDataProviderGetTypeID()
        }
    }
}

pub type CGDataProvider = CFWrapper<CGDataProviderRef, (), ()>;

pub fn new_from_buffer(buf: *u8, len: uint) -> CGDataProvider {
    unsafe {
        let result = CGDataProviderCreateWithData(
            ptr::null(),
            cast::transmute(buf),
            len as size_t,
            ptr::null());

        CFWrapper::wrap_owned(result)
    }
}

#[nolink]
#[link_args="-framework ApplicationServices"]
pub extern {
    //fn CGDataProviderCopyData
    //fn CGDataProviderCreateDirect
    //fn CGDataProviderCreateSequential
    //fn CGDataProviderCreateWithCFData
    fn CGDataProviderCreateWithData(info: *c_void,
                                    data: *c_void,
                                    size: size_t,
                                    releaseData: CGDataProviderReleaseDataCallback
                                   ) -> CGDataProviderRef;
    fn CGDataProviderCreateWithFilename(filename: *c_char) -> CGDataProviderRef;
    //fn CGDataProviderCreateWithURL
    fn CGDataProviderGetTypeID() -> CFTypeID;
    fn CGDataProviderRelease(provider: CGDataProviderRef);
    fn CGDataProviderRetain(provider: CGDataProviderRef) -> CGDataProviderRef;
}
