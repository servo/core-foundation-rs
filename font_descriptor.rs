use cf = core_foundation;
use cf::array::CFArrayRef;
use cf::base::{
    AbstractCFType,
    AbstractCFTypeRef,
    CFAllocatorRef,
    CFIndex,
    CFGetTypeID,
    CFRange,
    CFRelease,
    CFTypeID,
    CFTypeOps,
    CFTypeRef,
    kCFAllocatorDefault,
};
use cf::dictionary::CFDictionaryRef;
use cf::number::CFNumberRef;
use cf::set::CFSetRef;
use cf::string::{
    CFString,
    CFStringRef,
    CFStringGetTypeID,
};
use cf::url::{
    CFURL,
    CFURLRef,
    CFURLGetTypeID,
};

use cg = core_graphics;
use cg::base::CGFloat;

use libc::c_void;

/*
* CTFontTraits.h
*/
// actually, these are extern enums
pub type CTFontFormat = u32;
pub const kCTFontFormatUnrecognized: CTFontFormat = 0;
pub const kCTFontFormatOpenTypePostScript: CTFontFormat = 1;
pub const kCTFontFormatOpenTypeTrueType: CTFontFormat = 2;
pub const kCTFontFormatTrueType: CTFontFormat = 3;
pub const kCTFontFormatPostScript: CTFontFormat = 4;
pub const kCTFontFormatBitmap: CTFontFormat = 5;

pub const kCTFontClassMaskShift: u32 = 28;

pub type CTFontSymbolicTraits = u32;
pub const kCTFontItalicTrait: CTFontSymbolicTraits = (1 << 0);
pub const kCTFontBoldTrait: CTFontSymbolicTraits = (1 << 1);
pub const kCTFontExpandedTrait: CTFontSymbolicTraits = (1 << 5);
pub const kCTFontCondensedTrait: CTFontSymbolicTraits = (1 << 6);
pub const kCTFontMonoSpaceTrait: CTFontSymbolicTraits = (1 << 10);
pub const kCTFontVerticalTrait: CTFontSymbolicTraits = (1 << 11);
pub const kCTFontUIOptimizedTrait: CTFontSymbolicTraits = (1 << 12);
pub const kCTFontClassMaskTrait: CTFontSymbolicTraits = (15 << kCTFontClassMaskShift);

pub type CTFontStylisticClass = u32;
pub const kCTFontUnknownClass: CTFontStylisticClass = (0 << kCTFontClassMaskShift);
pub const kCTFontOldStyleSerifsClass: CTFontStylisticClass = (1 << kCTFontClassMaskShift);
pub const kCTFontTransitionalSerifsClass: CTFontStylisticClass = (2 << kCTFontClassMaskShift);
pub const kCTFontModernSerifsClass: CTFontStylisticClass = (3 << kCTFontClassMaskShift);
pub const kCTFontClarendonSerifsClass: CTFontStylisticClass = (4 << kCTFontClassMaskShift);
pub const kCTFontSlabSerifsClass: CTFontStylisticClass = (5 << kCTFontClassMaskShift);
pub const kCTFontFreeformSerifsClass: CTFontStylisticClass = (7 << kCTFontClassMaskShift);
pub const kCTFontSansSerifClass: CTFontStylisticClass = (8 << kCTFontClassMaskShift);
pub const kCTFontOrnamentalsClass: CTFontStylisticClass = (9 << kCTFontClassMaskShift);
pub const kCTFontScriptsClass: CTFontStylisticClass = (10 << kCTFontClassMaskShift);
pub const kCTFontSymbolicClass: CTFontStylisticClass = (12 << kCTFontClassMaskShift);

/*
* CTFontDescriptor.h
*/
pub type CTFontOrientation = u32;
pub const kCTFontDefaultOrientation: CTFontOrientation = 0;
pub const kCTFontHorizontalOrientation: CTFontOrientation = 1;
pub const kCTFontVerticalOrientation: CTFontOrientation = 2;

pub type CTFontPriority = u32;
pub const kCTFontPrioritySystem: CTFontPriority = 10000;
pub const kCTFontPriorityNetwork: CTFontPriority = 20000;
pub const kCTFontPriorityComputer: CTFontPriority = 30000;
pub const kCTFontPriorityUser: CTFontPriority = 40000;
pub const kCTFontPriorityDynamic: CTFontPriority = 50000;
pub const kCTFontPriorityProcess: CTFontPriority = 60000;

struct __CTFontDescriptor { private: () }
pub type CTFontDescriptorRef = *__CTFontDescriptor;

impl CTFontDescriptorRef : AbstractCFTypeRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }
}

struct CTFontDescriptor {
    obj: CTFontDescriptorRef,

    drop {
        unsafe {
            CFRelease(cast::transmute(self.obj))
        }
    }
}

pub impl CTFontDescriptor : AbstractCFType<CTFontDescriptorRef> {
    pure fn get_ref() -> CTFontDescriptorRef { self.obj }

    static fn wrap(obj: CTFontDescriptorRef) -> CTFontDescriptor {
        CTFontDescriptor { obj: obj }
    }

    static fn unwrap(wrapper: CTFontDescriptor) -> CTFontDescriptorRef {
        wrapper.obj
    }
}

pub impl CTFontDescriptor {
    fn family_name() -> ~str {
        let value = CTFontDescriptorCopyAttribute(self.obj, kCTFontDisplayNameAttribute);
        // family name should never be null.
        assert value.is_not_null();
        assert CFGetTypeID(value) == CFStringGetTypeID();

        let name : CFString = cf::base::wrap(value as CFStringRef);
        return name.to_str();
    }

    fn font_path() -> ~str {
        let value = CTFontDescriptorCopyAttribute(self.obj, kCTFontURLAttribute);
        assert value.is_not_null();
        assert CFGetTypeID(value) == CFURLGetTypeID();

        let cfurl : CFURL = cf::base::wrap(value as CFURLRef);
        return cfurl.to_str();
    }
}

pub fn debug_descriptor(desc: &CTFontDescriptor) {
    io::println(fmt!("family: %s", desc.family_name()));
    io::println(fmt!("path: %s", desc.font_path()));
    desc.show();
}

extern {
    /*
     * CTFontTraits.h
     */

    // font trait constants
    const kCTFontSymbolicTrait: CFStringRef;
    const kCTFontWeightTrait: CFStringRef;
    const kCTFontWidthTrait: CFStringRef;
    const kCTFontSlantTrait: CFStringRef;

    /*
     * CTFontDescriptor.h
     */

    // font attribute constants
    const kCTFontURLAttribute: CFStringRef;
    const kCTFontNameAttribute: CFStringRef;
    const kCTFontDisplayNameAttribute: CFStringRef;
    const kCTFontFamilyNameAttribute: CFStringRef;
    const kCTFontStyleNameAttribute: CFStringRef;
    const kCTFontTraitsAttribute: CFStringRef;
    const kCTFontVariationAttribute: CFStringRef;
    const kCTFontSizeAttribute: CFStringRef;
    const kCTFontMatrixAttribute: CFStringRef;
    const kCTFontCascadeListAttribute: CFStringRef;
    const kCTFontCharacterSetAttribute: CFStringRef;
    const kCTFontLanguagesAttribute: CFStringRef;
    const kCTFontBaselineAdjustAttribute: CFStringRef;
    const kCTFontMacintoshEncodingsAttribute: CFStringRef;
    const kCTFontFeaturesAttribute: CFStringRef;
    const kCTFontFeatureSettingsAttribute: CFStringRef;
    const kCTFontFixedAdvanceAttribute: CFStringRef;
    const kCTFontOrientationAttribute: CFStringRef;
    const kCTFontFormatAttribute: CFStringRef;
    const kCTFontRegistrationScopeAttribute: CFStringRef;
    const kCTFontPriorityAttribute: CFStringRef;
    const kCTFontEnabledAttribute: CFStringRef;

    fn CTFontDescriptorCopyAttribute(descriptor: CTFontDescriptorRef, attribute: CFStringRef) -> CFTypeRef;
    fn CTFontDescriptorCopyAttributes(descriptor: CTFontDescriptorRef) -> CFDictionaryRef;
    fn CTFontDescriptorCopyLocalizedAttribute(descriptor: CTFontDescriptorRef, attribute: CFStringRef, language: *CFStringRef) -> CFTypeRef;
    fn CTFontDescriptorCreateCopyWithAttributes(original: CTFontDescriptorRef, attributes: CFDictionaryRef) -> CTFontDescriptorRef;
    fn CTFontDescriptorCreateCopyWithFeature(original: CTFontDescriptorRef, featureTypeIdentifier: CFNumberRef, featureSelectorIdentifier: CFNumberRef) -> CTFontDescriptorRef;
    fn CTFontDescriptorCreateCopyWithVariation(original: CTFontDescriptorRef, variationIdentifier: CFNumberRef, variationValue: CGFloat) -> CTFontDescriptorRef;
    fn CTFontDescriptorCreateMatchingFontDescriptor(descriptor: CTFontDescriptorRef, mandatoryAttributes: CFSetRef) -> CTFontDescriptorRef;
    fn CTFontDescriptorCreateMatchingFontDescriptors(descriptor: CTFontDescriptorRef, mandatoryAttributes: CFSetRef) -> CTFontDescriptorRef;
    fn CTFontDescriptorCreateWithAttributes(attributes: CFDictionaryRef) -> CTFontDescriptorRef;
    fn CTFontDescriptorCreateWithNameAndSize(name: CFStringRef, size: CGFloat) -> CTFontDescriptorRef;
    fn CTFontDescriptorGetTypeID() -> CFTypeID;
}