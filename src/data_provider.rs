// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core_foundation::base::{CFRelease, CFRetain, CFTypeID, TCFType};
use core_foundation::data::{CFData, CFDataRef};

use libc::{c_void, size_t, off_t};
use std::ptr;

use foreign_types::{ForeignType, ForeignTypeRef};

pub type CGDataProviderGetBytesCallback = Option<unsafe extern fn (*mut c_void, *mut c_void, size_t) -> size_t>;
pub type CGDataProviderReleaseInfoCallback = Option<unsafe extern fn (*mut c_void)>;
pub type CGDataProviderRewindCallback = Option<unsafe extern fn (*mut c_void)>;
pub type CGDataProviderSkipBytesCallback = Option<unsafe extern fn (*mut c_void, size_t)>;
pub type CGDataProviderSkipForwardCallback = Option<unsafe extern fn (*mut c_void, off_t) -> off_t>;

pub type CGDataProviderGetBytePointerCallback = Option<unsafe extern fn (*mut c_void) -> *mut c_void>;
pub type CGDataProviderGetBytesAtOffsetCallback = Option<unsafe extern fn (*mut c_void, *mut c_void, size_t, size_t)>;
pub type CGDataProviderReleaseBytePointerCallback = Option<unsafe extern fn (*mut c_void, *const c_void)>;
pub type CGDataProviderReleaseDataCallback = Option<unsafe extern fn (*mut c_void, *const c_void, size_t)>;
pub type CGDataProviderGetBytesAtPositionCallback = Option<unsafe extern fn (*mut c_void, *mut c_void, off_t, size_t)>;

foreign_type! {
    #[doc(hidden)]
    type CType = ::sys::CGDataProvider;
    fn drop = |cs| CFRelease(cs as *mut _);
    fn clone = |p| CFRetain(p as *const _) as *mut _;
    pub struct CGDataProvider;
    pub struct CGDataProviderRef;
}

impl CGDataProvider {
    pub fn type_id() -> CFTypeID {
        unsafe {
            CGDataProviderGetTypeID()
        }
    }

    pub fn from_buffer(buffer: &[u8]) -> Self {
        unsafe {
            let result = CGDataProviderCreateWithData(ptr::null_mut(),
                                                      buffer.as_ptr() as *const c_void,
                                                      buffer.len() as size_t,
                                                      None);
            CGDataProvider::from_ptr(result)
        }
    }
}

impl CGDataProviderRef {
    /// Creates a copy of the data from the underlying `CFDataProviderRef`.
    pub fn copy_data(&self) -> CFData {
        unsafe { CFData::wrap_under_create_rule(CGDataProviderCopyData(self.as_ptr())) }
    }
}

#[link(name = "ApplicationServices", kind = "framework")]
extern {
    fn CGDataProviderCopyData(provider: ::sys::CGDataProviderRef) -> CFDataRef;
    //fn CGDataProviderCreateDirect
    //fn CGDataProviderCreateSequential
    //fn CGDataProviderCreateWithCFData
    fn CGDataProviderCreateWithData(info: *mut c_void,
                                    data: *const c_void,
                                    size: size_t,
                                    releaseData: CGDataProviderReleaseDataCallback
                                   ) -> ::sys::CGDataProviderRef;
    //fn CGDataProviderCreateWithFilename(filename: *c_char) -> CGDataProviderRef;
    //fn CGDataProviderCreateWithURL
    fn CGDataProviderGetTypeID() -> CFTypeID;
    //fn CGDataProviderRelease(provider: CGDataProviderRef);
    //fn CGDataProviderRetain(provider: CGDataProviderRef) -> CGDataProviderRef;
}
