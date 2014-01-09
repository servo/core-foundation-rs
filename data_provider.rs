// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core_foundation::base::{CFRelease, CFTypeID, TCFType};

use std::cast;
use std::libc::{c_void, size_t};
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

struct __CGDataProvider;

pub type CGDataProviderRef = *__CGDataProvider;

pub struct CGDataProvider {
    obj: CGDataProviderRef,
}

impl Drop for CGDataProvider {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl TCFType<CGDataProviderRef> for CGDataProvider {
    fn as_concrete_TypeRef(&self) -> CGDataProviderRef {
        self.obj
    }

    unsafe fn wrap_under_create_rule(obj: CGDataProviderRef) -> CGDataProvider {
        CGDataProvider {
            obj: obj,
        }
    }

    #[inline]
    fn type_id(_: Option<CGDataProvider>) -> CFTypeID {
        unsafe {
            CGDataProviderGetTypeID()
        }
    }
}

impl CGDataProvider {
    pub fn from_buffer(buffer: &[u8]) -> CGDataProvider {
        unsafe {
            let result = CGDataProviderCreateWithData(ptr::null(),
                                                      cast::transmute(buffer.as_ptr()),
                                                      buffer.len() as u64,
                                                      ptr::null());
            TCFType::wrap_under_create_rule(result)
        }
    }
}

#[link(name = "ApplicationServices", kind = "framework")]
extern {
    //fn CGDataProviderCopyData
    //fn CGDataProviderCreateDirect
    //fn CGDataProviderCreateSequential
    //fn CGDataProviderCreateWithCFData
    fn CGDataProviderCreateWithData(info: *c_void,
                                    data: *c_void,
                                    size: size_t,
                                    releaseData: CGDataProviderReleaseDataCallback
                                   ) -> CGDataProviderRef;
    //fn CGDataProviderCreateWithFilename(filename: *c_char) -> CGDataProviderRef;
    //fn CGDataProviderCreateWithURL
    fn CGDataProviderGetTypeID() -> CFTypeID;
    //fn CGDataProviderRelease(provider: CGDataProviderRef);
    //fn CGDataProviderRetain(provider: CGDataProviderRef) -> CGDataProviderRef;
}
