// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Core Foundation Bundle Type

use base::{CFRelease, CFRetain, CFTypeID, CFTypeRef, TCFType};
use std::mem;

use string::CFStringRef;
use libc::c_void;

#[repr(C)]
struct __CFBundle;

pub type CFBundleRef = *const __CFBundle;

/// A Bundle type.
pub struct CFBundle(CFBundleRef);

impl Drop for CFBundle {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl TCFType<CFBundleRef> for CFBundle {
    #[inline]
    fn as_concrete_TypeRef(&self) -> CFBundleRef {
        self.0
    }

    #[inline]
    unsafe fn wrap_under_get_rule(reference: CFBundleRef) -> CFBundle {
        let reference: CFBundleRef = mem::transmute(CFRetain(mem::transmute(reference)));
        TCFType::wrap_under_create_rule(reference)
    }

    #[inline]
    fn as_CFTypeRef(&self) -> CFTypeRef {
        unsafe {
            mem::transmute(self.as_concrete_TypeRef())
        }
    }

    unsafe fn wrap_under_create_rule(obj: CFBundleRef) -> CFBundle {
        CFBundle(obj)
    }

    #[inline]
    fn type_id() -> CFTypeID {
        unsafe {
            CFBundleGetTypeID()
        }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern {
    /*
     * CFBundle.h
     */


    pub fn CFBundleGetBundleWithIdentifier(bundleID: CFStringRef) -> CFBundleRef;
    pub fn CFBundleGetFunctionPointerForName(bundle: CFBundleRef, function_name: CFStringRef) -> *const c_void;

    fn CFBundleGetTypeID() -> CFTypeID;
}

