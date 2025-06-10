// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// 20250606 added by jwhur

use super::line::{CTLine, CTLineRef};
use core_foundation::attributed_string::CFAttributedStringRef;
use core_foundation::base::{CFIndex, CFRange, CFTypeID, TCFType};
use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::{declare_TCFType, impl_CFTypeDescription, impl_TCFType};
use core_graphics::geometry::CGSize;
use core_graphics::path::{CGPath, CGPathRef};
use foreign_types::{ForeignType, ForeignTypeRef};
use std::ptr::null;

#[repr(C)]
pub struct __CTTypesetter(core::ffi::c_void);

pub type CTTypesetterRef = *const __CTTypesetter;

declare_TCFType! {
    CTTypesetter, CTTypesetterRef
}
impl_TCFType!(CTTypesetter, CTTypesetterRef, CTTypesetterGetTypeID);
impl_CFTypeDescription!(CTTypesetter);

impl CTTypesetter {
    pub fn new_with_attributed_string(string: CFAttributedStringRef) -> Self {
        unsafe {
            let ptr = CTTypesetterCreateWithAttributedString(string);
            CTTypesetter::wrap_under_create_rule(ptr)
        }
    }

    pub fn create_line(&self, string_range: CFRange) -> CTLine {
        unsafe {
            let ptr = CTTypesetterCreateLine(
                self.as_concrete_TypeRef(),
                string_range
            );

            CTLine::wrap_under_create_rule(ptr)
        }
    }
}

#[cfg_attr(feature = "link", link(name = "CoreText", kind = "framework"))]
extern "C" {
    fn CTTypesetterGetTypeID() -> CFTypeID;
    fn CTTypesetterCreateWithAttributedString(string: CFAttributedStringRef) -> CTTypesetterRef;
    fn CTTypesetterCreateLine(
        typesetter: CTTypesetterRef,
        string_range: CFRange,
    ) -> CTLineRef;
    fn CTTypesetterSuggestLineBreak(
        typesetter: CTTypesetterRef,
        startIndex: CFIndex,
        width: f64,
    ) -> CFIndex;
    fn CTTypesetterCreateLineWithOffset(
        typesetter: CTTypesetterRef,
        startIndex: CFIndex,
        offset: f64,
    ) -> CFIndex;
    fn CTTypesetterSuggestClusterBreak(
        typesetter: CTTypesetterRef,
        startIndex: CFIndex,
        width: f64,
    ) -> CFIndex;
    fn CTTypesetterSuggestClusterBreakWithOffset(
        typesetter: CTTypesetterRef,
        startIndex: CFIndex,
        width: f64,
        offset: f64,
    ) -> CFIndex;
}
