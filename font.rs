// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core_foundation::base::{AbstractCFTypeRef, CFTypeID, CFTypeRef, CFWrapper};

use data_provider::{CGDataProvider, CGDataProviderRef};
use std::libc;

pub type CGGlyph = libc::c_ushort;

struct __CGFont { private: () }
pub type CGFontRef = *__CGFont;

impl AbstractCFTypeRef for CGFontRef {
    fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }

    fn type_id() -> CFTypeID {
        unsafe {
            CGFontGetTypeID()
        }
    }
}

pub struct CGFont {
    contents: CFWrapper<CGFontRef, (), ()>
}

impl CGFont {
    /// Convenience method to make it easier to wrap external `CGFont` instances.
    pub fn wrap_owned(font: CGFontRef) -> CGFont {
        CGFont {
            contents: CFWrapper::wrap_owned(font)
        }
    }

    /// Convenience method to make it easier to wrap external `CGFont` instances.
    pub fn wrap_shared(font: CGFontRef) -> CGFont {
        CGFont {
            contents: CFWrapper::wrap_shared(font)
        }
    }
}

impl Clone for CGFont {
    fn clone(&self) -> CGFont {
        CGFont::wrap_shared(*self.contents.borrow_ref())
    }
}

pub fn create_with_data_provider(provider: &CGDataProvider) -> CGFont {
    // TODO: error handling
    unsafe {
        let value = CGFontCreateWithDataProvider(*provider.borrow_ref());
        CGFont {
            contents: CFWrapper::wrap_owned(value)
        }
    }
}

#[nolink]
#[link_args="-framework ApplicationServices"]
extern {

    // TODO: basically nothing has bindings (even commented-out)  besides what we use.
    fn CGFontCreateWithDataProvider(provider: CGDataProviderRef) -> CGFontRef;
    fn CGFontGetTypeID() -> CFTypeID;

    // These do the same thing as CFRetain/CFRelease, except
    // gracefully handle a NULL argument. We don't use them.
    fn CGFontRetain(font: CGFontRef);
    fn CGFontRelease(font: CGFontRef);
}
