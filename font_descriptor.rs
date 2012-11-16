use cf = core_foundation;
use cf::array::CFArrayRef;
use cf::base::{
    AbstractCFTypeRef,
    CFAllocatorRef,
    CFIndex,
    CFGetTypeID,
    CFRange,
    CFType,
    CFTypeID,
    CFTypeRef,
    CFWrapper,
    kCFAllocatorDefault,
};
use cf::dictionary::{
    CFDictionary,
    CFDictionaryRef,
    UntypedCFDictionary,
};
use cf::number::{CFNumber, CFNumberRef};
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

pub trait SymbolicTraitAccessors {
    pure fn is_italic() -> bool;
    pure fn is_bold() -> bool;
    pure fn is_expanded() -> bool;
    pure fn is_condensed() -> bool;
    pure fn is_monospace() -> bool;
}

pub impl CTFontSymbolicTraits : SymbolicTraitAccessors {
    pure fn is_italic() -> bool { (self & kCTFontItalicTrait) != 0 }
    pure fn is_bold() -> bool { (self & kCTFontBoldTrait) != 0 }
    pure fn is_expanded() -> bool { (self & kCTFontExpandedTrait) != 0 }
    pure fn is_condensed() -> bool { (self & kCTFontCondensedTrait) != 0 }
    pure fn is_monospace() -> bool { (self & kCTFontMonoSpaceTrait) != 0 }
}

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

pub trait StylisticClassAccessors {
    pure fn is_serif() -> bool;
    pure fn is_sans_serif() -> bool;
    pure fn is_script() -> bool;
    pure fn is_fantasy() -> bool;
    pure fn is_symbols() -> bool;
}

pub impl CTFontStylisticClass : StylisticClassAccessors {
    pure fn is_serif() -> bool {
        let any_serif_class = kCTFontOldStyleSerifsClass 
            | kCTFontTransitionalSerifsClass
            | kCTFontModernSerifsClass
            | kCTFontClarendonSerifsClass
            | kCTFontSlabSerifsClass
            | kCTFontFreeformSerifsClass;

        return (self & any_serif_class) != 0;
    }

    pure fn is_sans_serif() -> bool {
        return (self & kCTFontSansSerifClass) != 0;
    }

    pure fn is_script() -> bool {
        return (self & kCTFontScriptsClass) != 0;
    }

    pure fn is_fantasy() -> bool {
        return (self & kCTFontOrnamentalsClass) != 0;
    }

    pure fn is_symbols() -> bool {
        return (self & kCTFontSymbolicClass) != 0;
    }
}

pub type CTFontAttributes = UntypedCFDictionary;
pub type CTFontTraits = UntypedCFDictionary;

pub trait TraitAccessors {
    fn symbolic_traits() -> CTFontSymbolicTraits;
    fn normalized_weight() -> float;
    fn normalized_width() -> float;
    fn normalized_slant() -> float;
}

impl CTFontTraits : TraitAccessors {
    priv fn extract_number_for_key(key: CFStringRef) -> CFNumber {
        let value = self.get(&key);
        CFWrapper::wrap_shared(cf::base::downcast::<CFNumberRef>(value))
    }

    fn symbolic_traits() -> CTFontSymbolicTraits unsafe {
        let number = self.extract_number_for_key(kCTFontSymbolicTrait);
        cast::transmute(number.to_i32())
    }

    fn normalized_weight() -> float unsafe {
        let number = self.extract_number_for_key(kCTFontWeightTrait);
        cast::transmute(number.to_float())
    }

    fn normalized_width() -> float unsafe {
        let number = self.extract_number_for_key(kCTFontWidthTrait);
        cast::transmute(number.to_float())
    }

    fn normalized_slant() -> float unsafe {
        let number = self.extract_number_for_key(kCTFontSlantTrait);
        cast::transmute(number.to_float())
    }
}

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
    static pure fn type_id() -> CFTypeID unsafe { CTFontDescriptorGetTypeID() }
}

pub type CTFontDescriptor = CFWrapper<CTFontDescriptorRef, (), ()>;

pub trait CTFontDescriptorMethods {
    fn family_name() -> ~str;
    fn font_name() -> ~str;
    fn style_name() -> ~str;
    fn display_name() -> ~str;
    fn font_path() -> ~str;
}

pub impl CTFontDescriptor : CTFontDescriptorMethods {
    priv fn get_string_attribute(attribute: CFStringRef) -> Option<~str> {
        let value = CTFontDescriptorCopyAttribute(self.obj, attribute);
        if value.is_null() { return None; }

        Some(CFWrapper::wrap_owned(cf::base::downcast::<CFStringRef>(value)).to_str())
    }

    fn family_name() -> ~str {
        let value = self.get_string_attribute(kCTFontDisplayNameAttribute);
        option::expect(move value, ~"A font must have a non-null font family name.")
    }

    fn font_name() -> ~str {
        let value = self.get_string_attribute(kCTFontNameAttribute);
        option::expect(move value, ~"A font must have a non-null name.")
    }

    fn style_name() -> ~str {
        let value = self.get_string_attribute(kCTFontStyleNameAttribute);
        option::expect(move value, ~"A font must have a non-null style name.")
    }

    fn display_name() -> ~str {
        let value = self.get_string_attribute(kCTFontDisplayNameAttribute);
        option::expect(move value, ~"A font must have a non-null display name.")
    }

    fn font_path() -> ~str {
        let value = CTFontDescriptorCopyAttribute(self.obj, kCTFontURLAttribute);
        assert value.is_not_null();

        CFWrapper::wrap_owned(cf::base::downcast::<CFURLRef>(value)).to_str()
    }
}

pub fn new_from_attributes(attributes: &CFWrapper<CFDictionaryRef, CFStringRef, CFTypeRef>) -> CTFontDescriptor {
    let result : CTFontDescriptorRef = CTFontDescriptorCreateWithAttributes(*attributes.borrow_ref());
    CFWrapper::wrap_owned(result)
}

pub fn debug_descriptor(desc: &CTFontDescriptor) {
    io::println(fmt!("family: %s", desc.family_name()));
    io::println(fmt!("name: %s", desc.font_name()));
    io::println(fmt!("style: %s", desc.style_name()));
    io::println(fmt!("display: %s", desc.display_name()));
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

    // font attribute constants. Note that the name-related attributes
    // here are somewhat flaky. Servo creates CTFont instances and
    // then uses CTFontCopyName to get more fine-grained names.
    const kCTFontURLAttribute:                  CFStringRef; // value: CFURLRef
    const kCTFontNameAttribute:                 CFStringRef; // value: CFStringRef
    const kCTFontDisplayNameAttribute:          CFStringRef; // value: CFStringRef
    const kCTFontFamilyNameAttribute:           CFStringRef; // value: CFStringRef
    const kCTFontStyleNameAttribute:            CFStringRef; // value: CFStringRef
    const kCTFontTraitsAttribute:               CFStringRef;
    const kCTFontVariationAttribute:            CFStringRef;
    const kCTFontSizeAttribute:                 CFStringRef;
    const kCTFontMatrixAttribute:               CFStringRef;
    const kCTFontCascadeListAttribute:          CFStringRef;
    const kCTFontCharacterSetAttribute:         CFStringRef;
    const kCTFontLanguagesAttribute:            CFStringRef;
    const kCTFontBaselineAdjustAttribute:       CFStringRef;
    const kCTFontMacintoshEncodingsAttribute:   CFStringRef;
    const kCTFontFeaturesAttribute:             CFStringRef;
    const kCTFontFeatureSettingsAttribute:      CFStringRef;
    const kCTFontFixedAdvanceAttribute:         CFStringRef;
    const kCTFontOrientationAttribute:          CFStringRef;
    const kCTFontFormatAttribute:               CFStringRef;
    const kCTFontRegistrationScopeAttribute:    CFStringRef;
    const kCTFontPriorityAttribute:             CFStringRef;
    const kCTFontEnabledAttribute:              CFStringRef;

    fn CTFontDescriptorCopyAttribute(descriptor: CTFontDescriptorRef,
                                     attribute: CFStringRef) -> CFTypeRef;
    fn CTFontDescriptorCopyAttributes(descriptor: CTFontDescriptorRef) -> CFDictionaryRef;
    fn CTFontDescriptorCopyLocalizedAttribute(descriptor: CTFontDescriptorRef,
                                              attribute: CFStringRef,
                                              language: *CFStringRef) -> CFTypeRef;
    fn CTFontDescriptorCreateCopyWithAttributes(original: CTFontDescriptorRef, 
                                                attributes: CFDictionaryRef) -> CTFontDescriptorRef;
    fn CTFontDescriptorCreateCopyWithFeature(original: CTFontDescriptorRef,
                                             featureTypeIdentifier: CFNumberRef,
                                             featureSelectorIdentifier: CFNumberRef) -> CTFontDescriptorRef;
    fn CTFontDescriptorCreateCopyWithVariation(original: CTFontDescriptorRef, 
                                               variationIdentifier: CFNumberRef,
                                               variationValue: CGFloat) -> CTFontDescriptorRef;
    fn CTFontDescriptorCreateMatchingFontDescriptor(descriptor: CTFontDescriptorRef,
                                                    mandatoryAttributes: CFSetRef) -> CTFontDescriptorRef;
    fn CTFontDescriptorCreateMatchingFontDescriptors(descriptor: CTFontDescriptorRef,
                                                     mandatoryAttributes: CFSetRef) -> CFArrayRef;
    fn CTFontDescriptorCreateWithAttributes(attributes: CFDictionaryRef) -> CTFontDescriptorRef;
    fn CTFontDescriptorCreateWithNameAndSize(name: CFStringRef, size: CGFloat) -> CTFontDescriptorRef;
    fn CTFontDescriptorGetTypeID() -> CFTypeID;
}