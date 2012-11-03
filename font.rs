extern mod core_foundation;
extern mod core_graphics;

use libc::c_uint;

use font_descriptor::{CTFontDescriptorRef, CTFontOrientation};

use cf = core_foundation;
use cf::string::{CFStringRef, UniChar};
use cf::base::{
    AbstractCFType,
    AbstractCFTypeRef,
    CFIndex,
    CFOptionFlags,
    CFRelease,
    CFTypeRef,
};

use cg = core_graphics;
use cg::base::{
    CGAffineTransform,
    CGFloat,
};
use cg::font::{
    CGGlyph,
    CGFontRef,
};
use cg::geometry::{
    CGRect,
    CGSize,
};

pub type CTFontUIFontType = u32;
// kCTFontNoFontType: CTFontUIFontType = -1;
pub const kCTFontUserFontType: CTFontUIFontType = 0;
pub const kCTFontUserFixedPitchFontType: CTFontUIFontType = 1;
pub const kCTFontSystemFontType: CTFontUIFontType = 2;
pub const kCTFontEmphasizedSystemFontType: CTFontUIFontType = 3;
pub const kCTFontSmallSystemFontType: CTFontUIFontType = 4;
pub const kCTFontSmallEmphasizedSystemFontType: CTFontUIFontType = 5;
pub const kCTFontMiniSystemFontType: CTFontUIFontType = 6;
pub const kCTFontMiniEmphasizedSystemFontType: CTFontUIFontType = 7;
pub const kCTFontViewsFontType: CTFontUIFontType = 8;
pub const kCTFontApplicationFontType: CTFontUIFontType = 9;
pub const kCTFontLabelFontType: CTFontUIFontType = 10;
pub const kCTFontMenuTitleFontType: CTFontUIFontType = 11;
pub const kCTFontMenuItemFontType: CTFontUIFontType = 12;
pub const kCTFontMenuItemMarkFontType: CTFontUIFontType = 13;
pub const kCTFontMenuItemCmdKeyFontType: CTFontUIFontType = 14;
pub const kCTFontWindowTitleFontType: CTFontUIFontType = 15;
pub const kCTFontPushButtonFontType: CTFontUIFontType = 16;
pub const kCTFontUtilityWindowTitleFontType: CTFontUIFontType = 17;
pub const kCTFontAlertHeaderFontType: CTFontUIFontType = 18;
pub const kCTFontSystemDetailFontType: CTFontUIFontType = 19;
pub const kCTFontEmphasizedSystemDetailFontType: CTFontUIFontType = 20;
pub const kCTFontToolbarFontType: CTFontUIFontType = 21;
pub const kCTFontSmallToolbarFontType: CTFontUIFontType = 22;
pub const kCTFontMessageFontType: CTFontUIFontType = 23;
pub const kCTFontPaletteFontType: CTFontUIFontType = 24;
pub const kCTFontToolTipFontType: CTFontUIFontType = 25;
pub const kCTFontControlContentFontType: CTFontUIFontType = 26;

pub type CTFontTableTag = u32;
// TODO: create bindings for enum with 'chars' values

pub type CTFontTableOptions = u32;
pub const kCTFontTableOptionsNoOptions: CTFontTableOptions = 0;
pub const kCTFontTableOptionsExcludeSynthetic: CTFontTableOptions = (1 << 0);

pub type CTFontOptions = CFOptionFlags;
pub const kCTFontOptionsDefault: CTFontOptions = 0;
pub const kCTFontOptionsPreventAutoActivation: CTFontOptions = (1 << 0);
pub const kCTFontOptionsPreferSystemFont: CTFontOptions = (1 << 2);

struct __CTFont { private: () }
pub type CTFontRef = *__CTFont;

impl CTFontRef : AbstractCFTypeRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }
}

struct CTFont {
    obj: CTFontRef,

    drop {
        unsafe {
            CFRelease(cast::transmute(self.obj))
        }
    }
}

pub impl CTFont : AbstractCFType<CTFontRef> {
    pure fn get_ref() -> CTFontRef { self.obj }

    static fn wrap(obj: CTFontRef) -> CTFont {
        CTFont { obj: obj }
    }

    static fn unwrap(wrapper: CTFont) -> CTFontRef {
        wrapper.obj
    }
}


#[nolink]
#[link_args = "-framework ApplicationServices"]
extern {
    /*
     * CTFont.h
     */

    /* Name Specifier Constants */
    const kCTFontCopyrightNameKey: CFStringRef;
    const kCTFontFamilyNameKey: CFStringRef;
    const kCTFontSubFamilyNameKey: CFStringRef;
    const kCTFontStyleNameKey: CFStringRef;
    const kCTFontUniqueNameKey: CFStringRef;
    const kCTFontFullNameKey: CFStringRef;
    const kCTFontVersionNameKey: CFStringRef;
    const kCTFontPostScriptNameKey: CFStringRef;
    const kCTFontTrademarkNameKey: CFStringRef;
    const kCTFontManufacturerNameKey: CFStringRef;
    const kCTFontDesignerNameKey: CFStringRef;
    const kCTFontDescriptionNameKey: CFStringRef;
    const kCTFontVendorURLNameKey: CFStringRef;
    const kCTFontDesignerURLNameKey: CFStringRef;
    const kCTFontLicenseNameKey: CFStringRef;
    const kCTFontLicenseURLNameKey: CFStringRef;
    const kCTFontSampleTextNameKey: CFStringRef;
    const kCTFontPostScriptCIDNameKey: CFStringRef;

    const kCTFontVariationAxisIdentifierKey: CFStringRef;
    const kCTFontVariationAxisMinimumValueKey: CFStringRef;
    const kCTFontVariationAxisMaximumValueKey: CFStringRef;
    const kCTFontVariationAxisDefaultValueKey: CFStringRef;
    const kCTFontVariationAxisNameKey: CFStringRef;

    const kCTFontFeatureTypeIdentifierKey: CFStringRef;
    const kCTFontFeatureTypeNameKey: CFStringRef;
    const kCTFontFeatureTypeExclusiveKey: CFStringRef;
    const kCTFontFeatureTypeSelectorsKey: CFStringRef;
    const kCTFontFeatureSelectorIdentifierKey: CFStringRef;
    const kCTFontFeatureSelectorNameKey: CFStringRef;
    const kCTFontFeatureSelectorDefaultKey: CFStringRef;
    const kCTFontFeatureSelectorSettingKey: CFStringRef;

    // N.B. Unlike most Cocoa bindings, this extern block is organized according
    // to the documentation's Functions By Task listing, because there so many functions.

    /* Creating Fonts */
    //fn CTFontCreateWithName
    //fn CTFontCreateWithNameAndOptions
    //fn CTFontCreateWithFontDescriptor
    //fn CTFontCreateWithFontDescriptorAndOptions
    //fn CTFontCreateUIFontForLanguage
    //fn CTFontCreateCopyWithAttributes
    //fn CTFontCreateCopyWithSymbolicTraits
    //fn CTFontCreateCopyWithFamily
    //fn CTFontCreateForString

    /* Getting Font Data */
    //fn CTFontCopyFontDescriptor
    //fn CTFontCopyAttribute
    fn CTFontGetSize(font: CTFontRef) -> CGFloat;
    //fn CTFontGetMatricx
    //fn CTFontGetSymbolicTraits
    //fn CTFontCopyTraits

    /* Getting Font Names */
    //fn CTFontCopyPostScriptName
    //fn CTFontCopyFamilyName
    //fn CTFontCopyFullName
    //fn CTFontCopyDisplayName
    //fn CTFontCopyName
    //fn CTFontCopyLocalizedName

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
    //fn CTFontCopyGraphicsFont
    fn CTFontCreateWithGraphicsFont(graphicsFont: CGFontRef, size: CGFloat, 
                                    matrix: *CGAffineTransform, 
                                    attributes: CTFontDescriptorRef) -> CTFontRef;
    //fn CTFontGetPlatformFont
    //fn CTFontCreateWithPlatformFont
    //fn CTFontCreateWithQuickdrawInstance

    /* Getting Font Table Data */
    //fn CTFontCopyAvailableTables
    //fn CTFontCopyTable

    //fn CTFontGetTypeID
    
}
