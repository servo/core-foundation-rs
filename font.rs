// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core_foundation::base::{CFRelease, CFRetain, CFTypeID, CFTypeRef, TCFType};
use data_provider::{CGDataProvider, CGDataProviderRef};

use libc;
use std::mem;

pub type CGGlyph = libc::c_ushort;

struct __CGFont;

pub type CGFontRef = *__CGFont;

pub struct CGFont {
    obj: CGFontRef,
}

impl Clone for CGFont {
    #[inline]
    fn clone(&self) -> CGFont {
        unsafe {
            TCFType::wrap_under_get_rule(self.obj)
        }
    }
}

impl Drop for CGFont {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl TCFType<CGFontRef> for CGFont {
    #[inline]
    fn as_concrete_TypeRef(&self) -> CGFontRef {
        self.obj
    }

    #[inline]
    unsafe fn wrap_under_get_rule(reference: CGFontRef) -> CGFont {
        let reference: CGFontRef = mem::transmute(CFRetain(mem::transmute(reference)));
        TCFType::wrap_under_create_rule(reference)
    }

    #[inline]
    fn as_CFTypeRef(&self) -> CFTypeRef {
        unsafe {
            mem::transmute(self.as_concrete_TypeRef())
        }
    }

    #[inline]
    unsafe fn wrap_under_create_rule(obj: CGFontRef) -> CGFont {
        CGFont {
            obj: obj,
        }
    }

    #[inline]
    fn type_id(_: Option<CGFont>) -> CFTypeID {
        unsafe {
            CGFontGetTypeID()
        }
    }
}

impl CGFont {
    pub fn from_data_provider(provider: CGDataProvider) -> CGFont {
        unsafe {
            let font_ref = CGFontCreateWithDataProvider(provider.as_concrete_TypeRef());
            TCFType::wrap_under_create_rule(font_ref)
        }
    }
}

#[link(name = "ApplicationServices", kind = "framework")]
extern {

    // TODO: basically nothing has bindings (even commented-out)  besides what we use.
    fn CGFontCreateWithDataProvider(provider: CGDataProviderRef) -> CGFontRef;
    fn CGFontGetTypeID() -> CFTypeID;

    // These do the same thing as CFRetain/CFRelease, except
    // gracefully handle a NULL argument. We don't use them.
    //fn CGFontRetain(font: CGFontRef);
    //fn CGFontRelease(font: CGFontRef);
}
