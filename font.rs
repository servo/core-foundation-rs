// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_uppercase_statics)]

use font_descriptor::{CTFontDescriptor, CTFontDescriptorRef, CTFontOrientation};
use font_descriptor::{CTFontSymbolicTraits, CTFontTraits, SymbolicTraitAccessors, TraitAccessors};

use core_foundation::base::{CFIndex, CFOptionFlags, CFTypeID, CFRelease, TCFType};
use core_foundation::data::{CFData, CFDataRef};
use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::string::{CFString, CFStringRef, UniChar};
use core_graphics::base::{CGAffineTransform, CGFloat};
use core_graphics::font::{CGGlyph, CGFont, CGFontRef};
use core_graphics::geometry::{CGRect, CGSize};

use libc;
use std::ptr;

pub type CTFontUIFontType = u32;
// kCTFontNoFontType: CTFontUIFontType = -1;
pub static kCTFontUserFontType: CTFontUIFontType = 0;
pub static kCTFontUserFixedPitchFontType: CTFontUIFontType = 1;
pub static kCTFontSystemFontType: CTFontUIFontType = 2;
pub static kCTFontEmphasizedSystemFontType: CTFontUIFontType = 3;
pub static kCTFontSmallSystemFontType: CTFontUIFontType = 4;
pub static kCTFontSmallEmphasizedSystemFontType: CTFontUIFontType = 5;
pub static kCTFontMiniSystemFontType: CTFontUIFontType = 6;
pub static kCTFontMiniEmphasizedSystemFontType: CTFontUIFontType = 7;
pub static kCTFontViewsFontType: CTFontUIFontType = 8;
pub static kCTFontApplicationFontType: CTFontUIFontType = 9;
pub static kCTFontLabelFontType: CTFontUIFontType = 10;
pub static kCTFontMenuTitleFontType: CTFontUIFontType = 11;
pub static kCTFontMenuItemFontType: CTFontUIFontType = 12;
pub static kCTFontMenuItemMarkFontType: CTFontUIFontType = 13;
pub static kCTFontMenuItemCmdKeyFontType: CTFontUIFontType = 14;
pub static kCTFontWindowTitleFontType: CTFontUIFontType = 15;
pub static kCTFontPushButtonFontType: CTFontUIFontType = 16;
pub static kCTFontUtilityWindowTitleFontType: CTFontUIFontType = 17;
pub static kCTFontAlertHeaderFontType: CTFontUIFontType = 18;
pub static kCTFontSystemDetailFontType: CTFontUIFontType = 19;
pub static kCTFontEmphasizedSystemDetailFontType: CTFontUIFontType = 20;
pub static kCTFontToolbarFontType: CTFontUIFontType = 21;
pub static kCTFontSmallToolbarFontType: CTFontUIFontType = 22;
pub static kCTFontMessageFontType: CTFontUIFontType = 23;
pub static kCTFontPaletteFontType: CTFontUIFontType = 24;
pub static kCTFontToolTipFontType: CTFontUIFontType = 25;
pub static kCTFontControlContentFontType: CTFontUIFontType = 26;

pub type CTFontTableTag = u32;
// TODO: create bindings for enum with 'chars' values

pub type CTFontTableOptions = u32;
pub static kCTFontTableOptionsNoOptions: CTFontTableOptions = 0;
pub static kCTFontTableOptionsExcludeSynthetic: CTFontTableOptions = (1 << 0);

pub type CTFontOptions = CFOptionFlags;
pub static kCTFontOptionsDefault: CTFontOptions = 0;
pub static kCTFontOptionsPreventAutoActivation: CTFontOptions = (1 << 0);
pub static kCTFontOptionsPreferSystemFont: CTFontOptions = (1 << 2);

struct __CTFont;

pub type CTFontRef = *__CTFont;

pub struct CTFont {
    obj: CTFontRef,
}

impl Drop for CTFont {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl TCFType<CTFontRef> for CTFont {
    fn as_concrete_TypeRef(&self) -> CTFontRef {
        self.obj
    }

    unsafe fn wrap_under_create_rule(obj: CTFontRef) -> CTFont {
        CTFont {
            obj: obj,
        }
    }

    #[inline]
    fn type_id(_: Option<CTFont>) -> CFTypeID {
        unsafe {
            CTFontGetTypeID()
        }
    }
}

pub fn new_from_CGFont(cgfont: &CGFont, pt_size: f64) -> CTFont {
    unsafe {
        let font_ref = CTFontCreateWithGraphicsFont(cgfont.as_concrete_TypeRef(),
                                                    pt_size as CGFloat,
                                                    ptr::null(),
                                                    ptr::null());
        TCFType::wrap_under_create_rule(font_ref)
    }
}

pub fn new_from_descriptor(desc: &CTFontDescriptor, pt_size: f64) -> CTFont {
    unsafe {
        let font_ref = CTFontCreateWithFontDescriptor(desc.as_concrete_TypeRef(),
                                                      pt_size as CGFloat,
                                                      ptr::null());
        TCFType::wrap_under_create_rule(font_ref)
    }
}

pub fn new_from_name(name: ~str, pt_size: f64) -> Result<CTFont, ()> {
    unsafe {
        let name: CFString = from_str(name).unwrap();
        let font_ref = CTFontCreateWithName(name.as_concrete_TypeRef(),
                                            pt_size as CGFloat,
                                            ptr::null());
        if font_ref.is_null() {
            Err(())
        } else {
            Ok(TCFType::wrap_under_create_rule(font_ref))
        }
    }
}

impl CTFont {
    // Properties
    pub fn symbolic_traits(&self) -> CTFontSymbolicTraits {
        unsafe {
            CTFontGetSymbolicTraits(self.obj)
        }
    }
}

impl CTFont {
    // Creation methods
    pub fn copy_to_CGFont(&self) -> CGFont {
        unsafe {
            let CGFont_ref = CTFontCopyGraphicsFont(self.obj, ptr::null());
            TCFType::wrap_under_create_rule(CGFont_ref)
        }
    }

    pub fn clone_with_font_size(&self, size: f64) -> CTFont {
        unsafe {
            let font_ref = CTFontCreateCopyWithAttributes(self.obj,
                                                          size as CGFloat,
                                                          ptr::null(),
                                                          ptr::null());
            TCFType::wrap_under_create_rule(font_ref)
        }
    }

    // Names
    pub fn family_name(&self) -> ~str {
        let value = get_string_by_name_key(self, kCTFontFamilyNameKey);
        value.expect("Fonts should always have a family name.")
    }

    pub fn face_name(&self) -> ~str {
        let value = get_string_by_name_key(self, kCTFontSubFamilyNameKey);
        value.expect("Fonts should always have a face name.")
    }

    pub fn unique_name(&self) -> ~str {
        let value = get_string_by_name_key(self, kCTFontUniqueNameKey);
        value.expect("Fonts should always have a unique name.")
    }

    pub fn postscript_name(&self) -> ~str {
        let value = get_string_by_name_key(self, kCTFontPostScriptNameKey);
        value.expect("Fonts should always have a PostScript name.")
    }

    pub fn all_traits(&self) -> CTFontTraits {
        unsafe {
            TCFType::wrap_under_create_rule(CTFontCopyTraits(self.obj))
        }
    }

    // Font metrics
    pub fn ascent(&self) -> CGFloat {
        unsafe {
            CTFontGetAscent(self.obj)
        }
    }

    pub fn descent(&self) -> CGFloat {
        unsafe {
            CTFontGetDescent(self.obj)
        }
    }

    pub fn underline_thickness(&self) -> CGFloat {
        unsafe {
            CTFontGetUnderlineThickness(self.obj)
        }
    }

    pub fn underline_position(&self) -> CGFloat {
        unsafe {
            CTFontGetUnderlinePosition(self.obj)
        }
    }

    pub fn bounding_box(&self) -> CGRect {
        unsafe {
            CTFontGetBoundingBox(self.obj)
        }
    }

    pub fn leading(&self) -> CGFloat {
        unsafe {
            CTFontGetLeading(self.obj)
        }
    }

    pub fn x_height(&self) -> CGFloat {
        unsafe {
            CTFontGetXHeight(self.obj)
        }
    }

    pub fn pt_size(&self) -> CGFloat {
        unsafe {
            CTFontGetSize(self.obj)
        }
    }

    pub fn get_glyphs_for_characters(&self, characters: *UniChar, glyphs: *CGGlyph, count: CFIndex)
                                     -> bool {
        unsafe {
            CTFontGetGlyphsForCharacters(self.obj, characters, glyphs, count)
        }
    }

    pub fn get_advances_for_glyphs(&self,
                                   orientation: CTFontOrientation,
                                   glyphs: *CGGlyph,
                                   advances: *CGSize,
                                   count: CFIndex)
                                   -> f64 {
        unsafe {
            CTFontGetAdvancesForGlyphs(self.obj, orientation, glyphs, advances, count) as f64
        }
    }

    pub fn get_font_table(&self, tag: u32) -> Option<CFData> {
        unsafe {
            let result = CTFontCopyTable(self.obj,
                                         tag as CTFontTableTag,
                                         kCTFontTableOptionsExcludeSynthetic);
            if result.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(result))
            }
        }
    }
}

// Helper methods
fn get_string_by_name_key(font: &CTFont, name_key: CFStringRef) -> Option<~str> {
    unsafe {
        let result = CTFontCopyName(font.as_concrete_TypeRef(), name_key);
        if result.is_null() {
            None
        } else {
            let string: CFString = TCFType::wrap_under_create_rule(result);
            Some(string.to_str())
        }
    }
}

pub fn debug_font_names(font: &CTFont) {
    fn get_key(font: &CTFont, key: CFStringRef) -> ~str {
        get_string_by_name_key(font, key).unwrap()
    }

    println!("kCTFontFamilyNameKey: {:s}", get_key(font, kCTFontFamilyNameKey));
    println!("kCTFontSubFamilyNameKey: {:s}", get_key(font, kCTFontSubFamilyNameKey));
    println!("kCTFontStyleNameKey: {:s}", get_key(font, kCTFontStyleNameKey));
    println!("kCTFontUniqueNameKey: {:s}", get_key(font, kCTFontUniqueNameKey));
    println!("kCTFontFullNameKey: {:s}", get_key(font, kCTFontFullNameKey));
    println!("kCTFontPostScriptNameKey: {:s}", get_key(font, kCTFontPostScriptNameKey));
}

pub fn debug_font_traits(font: &CTFont) {
    let sym = font.symbolic_traits();
    println!("kCTFontItalicTrait: {:b}", sym.is_italic());
    println!("kCTFontBoldTrait: {:b}", sym.is_bold());
    println!("kCTFontExpandedTrait: {:b}", sym.is_expanded());
    println!("kCTFontCondensedTrait: {:b}", sym.is_condensed());
    println!("kCTFontMonoSpaceTrait: {:b}", sym.is_monospace());

    let traits = font.all_traits();
    println!("kCTFontWeightTrait: {:f}", traits.normalized_weight());
//    println!("kCTFontWidthTrait: {:f}", traits.normalized_width());
//    println!("kCTFontSlantTrait: {:f}", traits.normalized_slant());
}

#[link(name = "ApplicationServices", kind = "framework")]
extern {
    /*
     * CTFont.h
     */

    /* Name Specifier Constants */
    //static kCTFontCopyrightNameKey: CFStringRef;
    static kCTFontFamilyNameKey: CFStringRef;
    static kCTFontSubFamilyNameKey: CFStringRef;
    static kCTFontStyleNameKey: CFStringRef;
    static kCTFontUniqueNameKey: CFStringRef;
    static kCTFontFullNameKey: CFStringRef;
    //static kCTFontVersionNameKey: CFStringRef;
    static kCTFontPostScriptNameKey: CFStringRef;
    //static kCTFontTrademarkNameKey: CFStringRef;
    //static kCTFontManufacturerNameKey: CFStringRef;
    //static kCTFontDesignerNameKey: CFStringRef;
    //static kCTFontDescriptionNameKey: CFStringRef;
    //static kCTFontVendorURLNameKey: CFStringRef;
    //static kCTFontDesignerURLNameKey: CFStringRef;
    //static kCTFontLicenseNameKey: CFStringRef;
    //static kCTFontLicenseURLNameKey: CFStringRef;
    //static kCTFontSampleTextNameKey: CFStringRef;
    //static kCTFontPostScriptCIDNameKey: CFStringRef;

    //static kCTFontVariationAxisIdentifierKey: CFStringRef;
    //static kCTFontVariationAxisMinimumValueKey: CFStringRef;
    //static kCTFontVariationAxisMaximumValueKey: CFStringRef;
    //static kCTFontVariationAxisDefaultValueKey: CFStringRef;
    //static kCTFontVariationAxisNameKey: CFStringRef;

    //static kCTFontFeatureTypeIdentifierKey: CFStringRef;
    //static kCTFontFeatureTypeNameKey: CFStringRef;
    //static kCTFontFeatureTypeExclusiveKey: CFStringRef;
    //static kCTFontFeatureTypeSelectorsKey: CFStringRef;
    //static kCTFontFeatureSelectorIdentifierKey: CFStringRef;
    //static kCTFontFeatureSelectorNameKey: CFStringRef;
    //static kCTFontFeatureSelectorDefaultKey: CFStringRef;
    //static kCTFontFeatureSelectorSettingKey: CFStringRef;

    // N.B. Unlike most Cocoa bindings, this extern block is organized according
    // to the documentation's Functions By Task listing, because there so many functions.

    /* Creating Fonts */
    fn CTFontCreateWithName(name: CFStringRef, size: CGFloat, matrix: *CGAffineTransform) -> CTFontRef;
    //fn CTFontCreateWithNameAndOptions
    fn CTFontCreateWithFontDescriptor(descriptor: CTFontDescriptorRef, size: CGFloat,
                                      matrix: *CGAffineTransform) -> CTFontRef;
    //fn CTFontCreateWithFontDescriptorAndOptions
    //fn CTFontCreateUIFontForLanguage
    fn CTFontCreateCopyWithAttributes(font: CTFontRef, size: CGFloat, matrix: *CGAffineTransform, 
                                      attributes: CTFontDescriptorRef) -> CTFontRef;
    //fn CTFontCreateCopyWithSymbolicTraits
    //fn CTFontCreateCopyWithFamily
    //fn CTFontCreateForString

    /* Getting Font Data */
    //fn CTFontCopyFontDescriptor(font: CTFontRef) -> CTFontDescriptorRef;
    //fn CTFontCopyAttribute(font: CTFontRef) -> CFTypeRef;
    fn CTFontGetSize(font: CTFontRef) -> CGFloat;
    //fn CTFontGetMatrix
    fn CTFontGetSymbolicTraits(font: CTFontRef) -> CTFontSymbolicTraits;
    fn CTFontCopyTraits(font: CTFontRef) -> CFDictionaryRef;

    /* Getting Font Names */
    //fn CTFontCopyPostScriptName(font: CTFontRef) -> CFStringRef;
    //fn CTFontCopyFamilyName(font: CTFontRef) -> CFStringRef;
    //fn CTFontCopyFullName(font: CTFontRef) -> CFStringRef;
    //fn CTFontCopyDisplayName(font: CTFontRef) -> CFStringRef;
    fn CTFontCopyName(font: CTFontRef, nameKey: CFStringRef) -> CFStringRef;
    //fn CTFontCopyLocalizedName(font: CTFontRef, nameKey: CFStringRef, 
    //                           language: *CFStringRef) -> CFStringRef;

    /* Working With Encoding */
    //fn CTFontCopyCharacterSet
    //fn CTFontGetStringEncoding
    //fn CTFontCopySupportedLanguages

    /* Getting Font Metrics */
    fn CTFontGetAscent(font: CTFontRef) -> CGFloat;
    fn CTFontGetDescent(font: CTFontRef) -> CGFloat;
    fn CTFontGetLeading(font: CTFontRef) -> CGFloat;
    //fn CTFontGetUnitsPerEm(font: CTFontRef) -> libc::c_uint;
    //fn CTFontGetGlyphCount
    fn CTFontGetBoundingBox(font: CTFontRef) -> CGRect;
    fn CTFontGetUnderlinePosition(font: CTFontRef) -> CGFloat;
    fn CTFontGetUnderlineThickness(font: CTFontRef) -> CGFloat;
    //fn CTFontGetSlantAngle
    //fn CTFontGetCapHeight
    fn CTFontGetXHeight(font: CTFontRef) -> CGFloat;

    /* Getting Glyph Data */
    //fn CTFontCreatePathForGlyph
    //fn CTFontGetGlyphWithName
    //fn CTFontGetBoundingRectsForGlyphs
    fn CTFontGetAdvancesForGlyphs(font: CTFontRef, orientation: CTFontOrientation, glyphs: *CGGlyph, advances: *CGSize, count: CFIndex) -> libc::c_double;
    //fn CTFontGetVerticalTranslationsForGlyphs

    /* Working With Font Variations */
    //fn CTFontCopyVariationAxes
    //fn CTFontCopyVariation

    /* Getting Font Features */
    //fn CTFontCopyFeatures
    //fn CTFontCopyFeatureSettings

    /* Working with Glyphs */
    fn CTFontGetGlyphsForCharacters(font: CTFontRef, characters: *UniChar, glyphs: *CGGlyph, count: CFIndex) -> bool;
    //fn CTFontDrawGlyphs
    //fn CTFontGetLigatureCaretPositions

    /* Converting Fonts */
    fn CTFontCopyGraphicsFont(font: CTFontRef, attributes: *CTFontDescriptorRef) -> CGFontRef;
    fn CTFontCreateWithGraphicsFont(graphicsFont: CGFontRef, size: CGFloat, 
                                    matrix: *CGAffineTransform, 
                                    attributes: CTFontDescriptorRef) -> CTFontRef;
    //fn CTFontGetPlatformFont
    //fn CTFontCreateWithPlatformFont
    //fn CTFontCreateWithQuickdrawInstance

    /* Getting Font Table Data */
    //fn CTFontCopyAvailableTables(font: CTFontRef, options: CTFontTableOptions) -> CFArrayRef;
    fn CTFontCopyTable(font: CTFontRef, table: CTFontTableTag, options: CTFontTableOptions) -> CFDataRef;

    fn CTFontGetTypeID() -> CFTypeID;
    
}
