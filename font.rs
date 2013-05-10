// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern mod core_foundation;
extern mod core_graphics;

use font_descriptor::{CTFontDescriptor, CTFontDescriptorRef, CTFontOrientation};
use font_descriptor::{CTFontSymbolicTraits, CTFontTraits, SymbolicTraitAccessors, TraitAccessors};

use core_foundation::array::{CFArrayRef};
use core_foundation::base::{AbstractCFTypeRef, CFIndex, CFOptionFlags, CFTypeID, CFTypeRef};
use core_foundation::base::{CFWrapper};
use core_foundation::data::{CFData, CFDataRef};
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::string::{CFString, CFStringRef, UniChar};
use core_graphics::base::{CGAffineTransform, CGFloat};
use core_graphics::font::{CGGlyph, CGFont, CGFontRef};
use core_graphics::geometry::{CGRect, CGSize};

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

struct __CTFont { private: () }
pub type CTFontRef = *__CTFont;

impl AbstractCFTypeRef for CTFontRef {
    fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }

    fn type_id() -> CFTypeID {
        unsafe {
            CTFontGetTypeID()
        }
    }
}

pub type CTFont = CFWrapper<CTFontRef, (), ()>;

pub trait CTFontMethods {
    // Creation methods (statics below)
    fn copy_to_CGFont(&self) -> CGFont;
    fn clone_with_font_size(&self, size: float) -> CTFont;

    // Names
    fn family_name(&self) -> ~str;
    fn face_name(&self) -> ~str;
    fn unique_name(&self) -> ~str;
    fn postscript_name(&self) -> ~str;

    // Properties
    fn all_traits(&self) -> CTFontTraits;

    // Font metrics
    fn ascent(&self) -> CGFloat;
    fn descent(&self) -> CGFloat;
    fn underline_thickness(&self) -> CGFloat;
    fn underline_position(&self) -> CGFloat;
    fn bounding_box(&self) -> CGRect;
    fn leading(&self) -> CGFloat;
    fn x_height(&self) -> CGFloat;
    fn pt_size(&self) -> CGFloat;
    fn get_glyphs_for_characters(&self,
                                 characters: *UniChar,
                                 glyphs: *CGGlyph,
                                 count: CFIndex)
                              -> bool;
    fn get_advances_for_glyphs(&self,
                               orientation: CTFontOrientation,
                               glyphs: *CGGlyph,
                               advances: *CGSize,
                               count: CFIndex)
                            -> float;
    fn get_font_table(&self, tag: u32) -> Option<CFData>;
}

pub fn new_from_CGFont(cgfont: &CGFont, pt_size: float) -> CTFont {
    unsafe {
        let result = CTFontCreateWithGraphicsFont(*cgfont.contents.borrow_ref(),
                                                  pt_size as CGFloat,
                                                  ptr::null(),
                                                  ptr::null());
        CFWrapper::wrap_owned(result)
    }
}

pub fn new_from_descriptor(desc: &CTFontDescriptor, pt_size: float) -> CTFont {
    unsafe {
        let result = CTFontCreateWithFontDescriptor(*desc.borrow_ref(),
                                                    pt_size as CGFloat,
                                                    ptr::null());
        CFWrapper::wrap_owned(result)
    }
}

pub fn new_from_name(name: ~str, pt_size: float) -> Result<CTFont, ()> {
    unsafe {
        let cfname = CFString::new(name);
        let result = CTFontCreateWithName(*cfname.contents.borrow_ref(),
                                          pt_size as CGFloat,
                                          ptr::null());
        if result.is_null() {
            return Err(());
        }

        return Ok(CFWrapper::wrap_owned(result));
    }
}

pub trait CTFontMethodsPrivate {
    fn symbolic_traits(&self) -> CTFontSymbolicTraits;
}

impl CTFontMethodsPrivate for CTFont {
    // Properties
    fn symbolic_traits(&self) -> CTFontSymbolicTraits {
        unsafe {
            CTFontGetSymbolicTraits(self.obj)
        }
    }
}

impl CTFontMethods for CTFont {
    // Creation methods
    fn copy_to_CGFont(&self) -> CGFont {
        unsafe {
            let value = CTFontCopyGraphicsFont(self.obj, ptr::null());
            CGFont::wrap_owned(value)
        }
    }

    fn clone_with_font_size(&self, size: float) -> CTFont {
        unsafe {
            let result = CTFontCreateCopyWithAttributes(self.obj,
                                                        size as CGFloat,
                                                        ptr::null(),
                                                        ptr::null());
            CFWrapper::wrap_owned(result)
        }
    }

    // Names
    fn family_name(&self) -> ~str {
        let value = get_string_by_name_key(self, kCTFontFamilyNameKey);
        value.expect(~"Fonts should always have a family name.")
    }

    fn face_name(&self) -> ~str {
        let value = get_string_by_name_key(self, kCTFontSubFamilyNameKey);
        value.expect(~"Fonts should always have a face name.")
    }

    fn unique_name(&self) -> ~str {
        let value = get_string_by_name_key(self, kCTFontUniqueNameKey);
        value.expect(~"Fonts should always have a unique name.")
    }

    fn postscript_name(&self) -> ~str {
        let value = get_string_by_name_key(self, kCTFontPostScriptNameKey);
        value.expect(~"Fonts should always have a PostScript name.")
    }

    fn all_traits(&self) -> CTFontTraits {
        unsafe {
            let result = CTFontCopyTraits(self.obj);
            CFDictionary::wrap_owned(result)
        }
    }

    // Font metrics
    fn ascent(&self) -> CGFloat {
        unsafe {
            CTFontGetAscent(self.obj)
        }
    }

    fn descent(&self) -> CGFloat {
        unsafe {
            CTFontGetDescent(self.obj)
        }
    }

    fn underline_thickness(&self) -> CGFloat {
        unsafe {
            CTFontGetUnderlineThickness(self.obj)
        }
    }

    fn underline_position(&self) -> CGFloat {
        unsafe {
            CTFontGetUnderlinePosition(self.obj)
        }
    }

    fn bounding_box(&self) -> CGRect {
        unsafe {
            CTFontGetBoundingBox(self.obj)
        }
    }

    fn leading(&self) -> CGFloat {
        unsafe {
            CTFontGetLeading(self.obj)
        }
    }

    fn x_height(&self) -> CGFloat {
        unsafe {
            CTFontGetXHeight(self.obj)
        }
    }

    fn pt_size(&self) -> CGFloat {
        unsafe {
            CTFontGetSize(self.obj)
        }
    }

    fn get_glyphs_for_characters(&self,
                                 characters: *UniChar,
                                 glyphs: *CGGlyph,
                                 count: CFIndex)
                              -> bool {
        unsafe {
            CTFontGetGlyphsForCharacters(self.obj, characters, glyphs, count)
        }
    }

    fn get_advances_for_glyphs(&self,
                               orientation: CTFontOrientation,
                               glyphs: *CGGlyph,
                               advances: *CGSize,
                               count: CFIndex)
                            -> float {
        unsafe {
            CTFontGetAdvancesForGlyphs(self.obj, orientation, glyphs, advances, count) as float
        }
    }

    fn get_font_table(&self, tag: u32) -> Option<CFData> {
        unsafe {
            let result = CTFontCopyTable(self.obj,
                                         tag as CTFontTableTag,
                                         kCTFontTableOptionsExcludeSynthetic);
            return match result.is_null() {
                true => None,
                false => Some(CFData::wrap_owned(result)),
            }
        }
    }
}

// Helper methods
priv fn get_string_by_name_key(font: &CTFont, name_key: CFStringRef) -> Option<~str> {
    unsafe {
        let result = CTFontCopyName(*font.borrow_ref(), name_key);
        if result.is_null() { return None; }

        return Some(CFString::wrap_owned(result).to_str());
    }
}

pub fn debug_font_names(font: &CTFont) {
    fn get_key(font: &CTFont, key: CFStringRef) -> ~str {
        get_string_by_name_key(font, key).unwrap()
    }

    io::println(fmt!("kCTFontFamilyNameKey: %s", get_key(font, kCTFontFamilyNameKey)));
    io::println(fmt!("kCTFontSubFamilyNameKey: %s", get_key(font, kCTFontSubFamilyNameKey)));
    io::println(fmt!("kCTFontStyleNameKey: %s", get_key(font, kCTFontStyleNameKey)));
    io::println(fmt!("kCTFontUniqueNameKey: %s", get_key(font, kCTFontUniqueNameKey)));
    io::println(fmt!("kCTFontFullNameKey: %s", get_key(font, kCTFontFullNameKey)));
    io::println(fmt!("kCTFontPostScriptNameKey: %s", get_key(font, kCTFontPostScriptNameKey)));
}

pub fn debug_font_traits(font: &CTFont) {
    let sym = font.symbolic_traits();
    io::println(fmt!("kCTFontItalicTrait: %b", sym.is_italic()));
    io::println(fmt!("kCTFontBoldTrait: %b", sym.is_bold()));
    io::println(fmt!("kCTFontExpandedTrait: %b", sym.is_expanded()));
    io::println(fmt!("kCTFontCondensedTrait: %b", sym.is_condensed()));
    io::println(fmt!("kCTFontMonoSpaceTrait: %b", sym.is_monospace()));

    let traits = font.all_traits();
    io::println(fmt!("kCTFontWeightTrait: %f", traits.normalized_weight()));
//    io::println(fmt!("kCTFontWidthTrait: %f", traits.normalized_width()));
//    io::println(fmt!("kCTFontSlantTrait: %f", traits.normalized_slant()));
}

#[nolink]
#[link_args = "-framework ApplicationServices"]
extern {
    /*
     * CTFont.h
     */

    /* Name Specifier Constants */
    static kCTFontCopyrightNameKey: CFStringRef;
    static kCTFontFamilyNameKey: CFStringRef;
    static kCTFontSubFamilyNameKey: CFStringRef;
    static kCTFontStyleNameKey: CFStringRef;
    static kCTFontUniqueNameKey: CFStringRef;
    static kCTFontFullNameKey: CFStringRef;
    static kCTFontVersionNameKey: CFStringRef;
    static kCTFontPostScriptNameKey: CFStringRef;
    static kCTFontTrademarkNameKey: CFStringRef;
    static kCTFontManufacturerNameKey: CFStringRef;
    static kCTFontDesignerNameKey: CFStringRef;
    static kCTFontDescriptionNameKey: CFStringRef;
    static kCTFontVendorURLNameKey: CFStringRef;
    static kCTFontDesignerURLNameKey: CFStringRef;
    static kCTFontLicenseNameKey: CFStringRef;
    static kCTFontLicenseURLNameKey: CFStringRef;
    static kCTFontSampleTextNameKey: CFStringRef;
    static kCTFontPostScriptCIDNameKey: CFStringRef;

    static kCTFontVariationAxisIdentifierKey: CFStringRef;
    static kCTFontVariationAxisMinimumValueKey: CFStringRef;
    static kCTFontVariationAxisMaximumValueKey: CFStringRef;
    static kCTFontVariationAxisDefaultValueKey: CFStringRef;
    static kCTFontVariationAxisNameKey: CFStringRef;

    static kCTFontFeatureTypeIdentifierKey: CFStringRef;
    static kCTFontFeatureTypeNameKey: CFStringRef;
    static kCTFontFeatureTypeExclusiveKey: CFStringRef;
    static kCTFontFeatureTypeSelectorsKey: CFStringRef;
    static kCTFontFeatureSelectorIdentifierKey: CFStringRef;
    static kCTFontFeatureSelectorNameKey: CFStringRef;
    static kCTFontFeatureSelectorDefaultKey: CFStringRef;
    static kCTFontFeatureSelectorSettingKey: CFStringRef;

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
    fn CTFontCopyFontDescriptor(font: CTFontRef) -> CTFontDescriptorRef;
    fn CTFontCopyAttribute(font: CTFontRef) -> CFTypeRef;
    fn CTFontGetSize(font: CTFontRef) -> CGFloat;
    //fn CTFontGetMatrix
    fn CTFontGetSymbolicTraits(font: CTFontRef) -> CTFontSymbolicTraits;
    fn CTFontCopyTraits(font: CTFontRef) -> CFDictionaryRef;

    /* Getting Font Names */
    fn CTFontCopyPostScriptName(font: CTFontRef) -> CFStringRef;
    fn CTFontCopyFamilyName(font: CTFontRef) -> CFStringRef;
    fn CTFontCopyFullName(font: CTFontRef) -> CFStringRef;
    fn CTFontCopyDisplayName(font: CTFontRef) -> CFStringRef;
    fn CTFontCopyName(font: CTFontRef, nameKey: CFStringRef) -> CFStringRef;
    fn CTFontCopyLocalizedName(font: CTFontRef, nameKey: CFStringRef, 
                               language: *CFStringRef) -> CFStringRef;

    /* Working With Encoding */
    //fn CTFontCopyCharacterSet
    //fn CTFontGetStringEncoding
    //fn CTFontCopySupportedLanguages

    /* Getting Font Metrics */
    fn CTFontGetAscent(font: CTFontRef) -> CGFloat;
    fn CTFontGetDescent(font: CTFontRef) -> CGFloat;
    fn CTFontGetLeading(font: CTFontRef) -> CGFloat;
    fn CTFontGetUnitsPerEm(font: CTFontRef) -> libc::c_uint;
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
    fn CTFontCopyAvailableTables(font: CTFontRef, options: CTFontTableOptions) -> CFArrayRef;
    fn CTFontCopyTable(font: CTFontRef, table: CTFontTableTag, options: CTFontTableOptions) -> CFDataRef;

    fn CTFontGetTypeID() -> CFTypeID;
    
}
