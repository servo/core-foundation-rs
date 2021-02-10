// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core_foundation::base::{CFRelease, CFRetain, CFType, CFTypeID, TCFType};
use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::data::{CFData, CFDataRef};
use core_foundation::number::CFNumber;
use core_foundation::string::{CFString, CFStringRef};
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use data_provider::CGDataProvider;
use geometry::CGRect;

use foreign_types::ForeignType;

use libc::{c_int, size_t};

pub use core_graphics_types::base::CGGlyph;

foreign_type! {
    #[doc(hidden)]
    type CType = ::sys::CGFont;
    fn drop = |p| CFRelease(p as *mut _);
    fn clone = |p| CFRetain(p as *const _) as *mut _;
    pub struct CGFont;
    pub struct CGFontRef;
}

unsafe impl Send for CGFont {}
unsafe impl Sync for CGFont {}

impl CGFont {
    pub fn type_id() -> CFTypeID {
        unsafe {
            CGFontGetTypeID()
        }
    }

    pub fn from_data_provider(provider: CGDataProvider) -> Result<CGFont, ()> {
        unsafe {
            let font_ref = CGFontCreateWithDataProvider(provider.as_ptr());
            if !font_ref.is_null() {
                Ok(CGFont::from_ptr(font_ref))
            } else {
                Err(())
            }
        }
    }

    pub fn from_name(name: &CFString) -> Result<CGFont, ()> {
        unsafe {
            let font_ref = CGFontCreateWithFontName(name.as_concrete_TypeRef());
            if !font_ref.is_null() {
                Ok(CGFont::from_ptr(font_ref))
            } else {
                Err(())
            }
        }
    }

    pub fn create_copy_from_variations(&self, vars: &CFDictionary<CFString, CFNumber>) -> Result<CGFont, ()> {
        unsafe {
            let font_ref = CGFontCreateCopyWithVariations(self.as_ptr(),
                                                          vars.as_concrete_TypeRef());
            if !font_ref.is_null() {
                Ok(CGFont::from_ptr(font_ref))
            } else {
                Err(())
            }
        }
    }

    pub fn postscript_name(&self) -> CFString {
        unsafe {
            let string_ref = CGFontCopyPostScriptName(self.as_ptr());
            TCFType::wrap_under_create_rule(string_ref)
        }
    }

    pub fn get_glyph_b_boxes(&self, glyphs: &[CGGlyph], bboxes: &mut [CGRect]) -> bool {
        unsafe {
            assert!(bboxes.len() >= glyphs.len());
            CGFontGetGlyphBBoxes(self.as_ptr(),
                                 glyphs.as_ptr(),
                                 glyphs.len(),
                                 bboxes.as_mut_ptr())
        }
    }

    pub fn get_glyph_advances(&self, glyphs: &[CGGlyph], advances: &mut [c_int]) -> bool {
        unsafe {
            assert!(advances.len() >= glyphs.len());
            CGFontGetGlyphAdvances(self.as_ptr(),
                                   glyphs.as_ptr(),
                                   glyphs.len(),
                                   advances.as_mut_ptr())
        }
    }

    pub fn get_units_per_em(&self) -> c_int {
        unsafe {
            CGFontGetUnitsPerEm(self.as_ptr())
        }
    }

    pub fn copy_table_tags(&self) -> CFArray<u32> {
        unsafe {
            TCFType::wrap_under_create_rule(CGFontCopyTableTags(self.as_ptr()))
        }
    }

    pub fn copy_table_for_tag(&self, tag: u32) -> Option<CFData> {
        let data_ref = unsafe { CGFontCopyTableForTag(self.as_ptr(), tag) };
        if !data_ref.is_null() {
            Some(unsafe { TCFType::wrap_under_create_rule(data_ref) })
        } else {
            None
        }
    }

    pub fn copy_variations(&self) -> Option<CFDictionary<CFString, CFNumber>> {
        let variations = unsafe { CGFontCopyVariations(self.as_ptr()) };
        if !variations.is_null() {
            Some(unsafe { TCFType::wrap_under_create_rule(variations) })
        } else {
            None
        }
    }

    pub fn copy_variation_axes(&self) -> Option<CFArray<CFDictionary<CFString, CFType>>> {
        let axes = unsafe { CGFontCopyVariationAxes(self.as_ptr()) };
        if !axes.is_null() {
            Some(unsafe { TCFType::wrap_under_create_rule(axes) })
        } else {
            None
        }
    }

    /// Will construct a Vec<u8> containing a font that has the tables and table contents of the
    /// CGFont. This will not necessarily be the exact same bytes as the original font but should
    /// be functionally equivalent.
    ///
    /// Manually reconstructing a font is necessary because CoreGraphics does not provide a method
    /// to retrieve the actual underlying data.
    pub fn construct_font_data(&self) -> Vec<u8> {
        construct_font_data(self)
    }
}

fn calc_table_checksum(table: &[u8], skip_checksum_adjust: bool) -> u32 {
    use std::convert::TryInto;
    let mut sum = std::num::Wrapping(0);
    let mut i = 0;
    let mut chunks = table.chunks_exact(4);
    for chunk in &mut chunks {
        if skip_checksum_adjust && i == 2 {

        } else {
            let val = u32::from_be_bytes(chunk.try_into().unwrap());
            sum += std::num::Wrapping(val)
        }
        i += 1;
    }

    // The table will be zero padded to be 4 byte aligned when written out
    // so compute the checksum as if that were the case.
    let mut val = [0; 4];
    val[0..chunks.remainder().len()].copy_from_slice(chunks.remainder());
    let val = u32::from_be_bytes(val);
    sum += std::num::Wrapping(val);

    sum.0
}

fn max_pow2_less_than_equal(a: i32) -> i32 {
    let x = 1;
    let mut shift = 0;
    while (x << (shift + 1)) <= a {
      shift+=1;
    }
    shift
}

// This code is inspired by the code in mozilla-central/gfx/2d/ScaledFontMac.cpp
fn construct_font_data(font: &CGFont) -> Vec<u8> {
    struct TableRecord {
        tag: u32,
        checksum: u32,
        offset: u32,
        length: u32,
        data: CFData,
    }

    let tags = font.copy_table_tags();
    let count = tags.len();
    let mut records = Vec::with_capacity(tags.len() as usize);
    let mut offset: u32 = 0;
    offset += std::mem::size_of::<u32>() as u32 * 3;
    offset += std::mem::size_of::<u32>() as u32 * 4 * count as u32;
    let mut cff = false;
    for tag in tags.iter() {
        let data = font.copy_table_for_tag(*tag).unwrap();
        let skip_checksum_adjust = *tag == 0x68656164;  // 'head'

        if *tag == 0x43464620 {  // 'CFF '
            cff = true;
        }
        let checksum = calc_table_checksum(data.bytes(), skip_checksum_adjust);
        records.push(TableRecord { tag: *tag, offset, length: data.len() as u32, data: data.clone(), checksum});
        offset += data.len() as u32;
        // 32 bit align the tables
        offset = (offset + 3) & !3;
    }

    let mut buf: Vec<u8> = Vec::new();
    if cff {
        buf.extend_from_slice(&0x4f54544fu32.to_be_bytes());
    } else {
        buf.extend_from_slice(&0x00010000u32.to_be_bytes());
    }

    buf.extend_from_slice(&(count as u16).to_be_bytes());
    let max_pow2_count = max_pow2_less_than_equal(count as i32);
    buf.extend_from_slice(&((1u16 << max_pow2_count) * 16).to_be_bytes());
    buf.extend_from_slice(&(max_pow2_count as u16).to_be_bytes());
    buf.extend_from_slice(&((count as u16 - (1 << max_pow2_count)) * 16).to_be_bytes());

  // write table record entries
  for rec in &records {
    buf.extend_from_slice(&rec.tag.to_be_bytes());
    buf.extend_from_slice(&rec.checksum.to_be_bytes());
    buf.extend_from_slice(&rec.offset.to_be_bytes());
    buf.extend_from_slice(&rec.length.to_be_bytes());
  }

  // write tables
  let mut checksum_adjustment_offset = 0;
  for rec in &records {
    if rec.tag == 0x68656164 { // 'head'
        checksum_adjustment_offset = buf.len() + 2 * 4;
    }
    assert!(buf.len() == rec.offset as usize);
    buf.extend_from_slice(rec.data.bytes());
    // align
    let extra = ((buf.len() + 3) & !3) - buf.len();
    buf.extend_from_slice(&[0;4][0..extra]);
  }

  // clear the checksumAdjust field before checksumming the whole font
  for b in &mut buf[checksum_adjustment_offset..checksum_adjustment_offset+4] {
    *b = 0;
  }
  let font_check_sum = (0xb1b0afba_u32.wrapping_sub(
    calc_table_checksum(&buf, false))).to_be_bytes();
  (&mut buf[checksum_adjustment_offset..checksum_adjustment_offset+4]).copy_from_slice(&font_check_sum);

  buf
}

#[link(name = "CoreGraphics", kind = "framework")]
extern {
    // TODO: basically nothing has bindings (even commented-out) besides what we use.
    fn CGFontCreateWithDataProvider(provider: ::sys::CGDataProviderRef) -> ::sys::CGFontRef;
    fn CGFontCreateWithFontName(name: CFStringRef) -> ::sys::CGFontRef;
    fn CGFontCreateCopyWithVariations(font: ::sys::CGFontRef, vars: CFDictionaryRef) -> ::sys::CGFontRef;
    fn CGFontGetTypeID() -> CFTypeID;

    fn CGFontCopyPostScriptName(font: ::sys::CGFontRef) -> CFStringRef;

    // These do the same thing as CFRetain/CFRelease, except
    // gracefully handle a NULL argument. We don't use them.
    //fn CGFontRetain(font: ::sys::CGFontRef);
    //fn CGFontRelease(font: ::sys::CGFontRef);

    fn CGFontGetGlyphBBoxes(font: ::sys::CGFontRef,
                            glyphs: *const CGGlyph,
                            count: size_t,
                            bboxes: *mut CGRect)
                            -> bool;
    fn CGFontGetGlyphAdvances(font: ::sys::CGFontRef,
                              glyphs: *const CGGlyph,
                              count: size_t,
                              advances: *mut c_int)
                              -> bool;
    fn CGFontGetUnitsPerEm(font: ::sys::CGFontRef) -> c_int;

    fn CGFontCopyTableTags(font: ::sys::CGFontRef) -> CFArrayRef;
    fn CGFontCopyTableForTag(font: ::sys::CGFontRef, tag: u32) -> CFDataRef;
    fn CGFontCopyVariations(font: ::sys::CGFontRef) -> CFDictionaryRef;
    fn CGFontCopyVariationAxes(font: ::sys::CGFontRef) -> CFArrayRef;
}

#[cfg(test)]
mod test {
    use core_foundation::string::CFString;
    use crate::font::*;
    #[test]
    fn construct_font_data() {
        use std::sync::Arc;

        let font = CGFont::from_name(&CFString::from_static_string("Helvetica")).unwrap();
        let data = font.construct_font_data();
        let data_provider = crate::data_provider::CGDataProvider::from_buffer(Arc::new(data));
        let font = CGFont::from_data_provider(data_provider).unwrap();
        assert_eq!(font.postscript_name(), "Helvetica");
    }
}
