// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;
use core_foundation::attributed_string::CFAttributedStringRef;
use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::{CFIndex, CFTypeID, TCFType};
use core_graphics::base::{CGFloat};
use core_graphics::context::{CGContext};
use core_graphics::geometry::{CGPoint,CGRect};
use foreign_types::{ForeignType};
use run::CTRun;

#[repr(C)]
pub struct __CTLine(c_void);

pub type CTLineRef = *const __CTLine;

declare_TCFType! {
    CTLine, CTLineRef
}
impl_TCFType!(CTLine, CTLineRef, CTLineGetTypeID);
impl_CFTypeDescription!(CTLine);

impl CTLine {
    pub fn new_with_attributed_string(string: CFAttributedStringRef) -> Self {
        unsafe {
            let ptr = CTLineCreateWithAttributedString(string);
            CTLine::wrap_under_create_rule(ptr)
        }
    }

    pub fn glyph_runs(&self) -> CFArray<CTRun> {
        unsafe {
            TCFType::wrap_under_get_rule(CTLineGetGlyphRuns(self.0))
        }
    }

    pub fn draw(&self, context: &CGContext) {
        unsafe {
            CTLineDraw(self.as_concrete_TypeRef(), context.as_ptr())
        }
    }

    pub fn get_image_bounds(&self, context: &CGContext) -> CGRect {
        unsafe {
            CTLineGetImageBounds(self.as_concrete_TypeRef(), context.as_ptr())
        }
    }

    pub fn get_string_index_for_position(&self, position: CGPoint) -> CFIndex {
        unsafe {
            CTLineGetStringIndexForPosition(self.as_concrete_TypeRef(), position)
        }
    }

    pub fn get_string_offset_for_string_index(&self, charIndex: CFIndex) -> CGFloat {
        unsafe {
            CTLineGetOffsetForStringIndex(self.as_concrete_TypeRef(), charIndex, std::ptr::null())
        }
    }
}

#[link(name = "CoreText", kind = "framework")]
extern {
    fn CTLineGetTypeID() -> CFTypeID;
    fn CTLineGetGlyphRuns(line: CTLineRef) -> CFArrayRef;

    // Creating Lines
    fn CTLineCreateWithAttributedString(string: CFAttributedStringRef) -> CTLineRef;

    // Drawing the Line
    fn CTLineDraw(line: CTLineRef, context: * const core_graphics::sys::CGContext);

    // Measuring Lines
    fn CTLineGetImageBounds(line: CTLineRef, context: * const core_graphics::sys::CGContext) -> CGRect;

    // Getting Line Positioning
    fn CTLineGetStringIndexForPosition(line: CTLineRef, position: CGPoint) -> CFIndex;
    fn CTLineGetOffsetForStringIndex(line: CTLineRef, charIndex: CFIndex, secondaryOffset: *const CGFloat) -> CGFloat;
}
