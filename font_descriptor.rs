// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core_foundation;
use core_foundation::array::CFArrayRef;
use core_foundation::base::AbstractCFTypeRef;
use core_foundation::base::{CFTypeID, CFTypeRef, CFWrapper};
use core_foundation::dictionary::{CFDictionaryRef, UntypedCFDictionary};
use core_foundation::number::{CFNumber, CFNumberRef};
use core_foundation::set::CFSetRef;
use core_foundation::string::{CFString, CFStringRef};
use core_foundation::url::{CFURLRef};
use core_graphics::base::CGFloat;

/*
* CTFontTraits.h
*/
// actually, these are extern enums
pub type CTFontFormat = u32;
pub static kCTFontFormatUnrecognized: CTFontFormat = 0;
pub static kCTFontFormatOpenTypePostScript: CTFontFormat = 1;
pub static kCTFontFormatOpenTypeTrueType: CTFontFormat = 2;
pub static kCTFontFormatTrueType: CTFontFormat = 3;
pub static kCTFontFormatPostScript: CTFontFormat = 4;
pub static kCTFontFormatBitmap: CTFontFormat = 5;

pub static kCTFontClassMaskShift: u32 = 28;

pub type CTFontSymbolicTraits = u32;
pub static kCTFontItalicTrait: CTFontSymbolicTraits = (1 << 0);
pub static kCTFontBoldTrait: CTFontSymbolicTraits = (1 << 1);
pub static kCTFontExpandedTrait: CTFontSymbolicTraits = (1 << 5);
pub static kCTFontCondensedTrait: CTFontSymbolicTraits = (1 << 6);
pub static kCTFontMonoSpaceTrait: CTFontSymbolicTraits = (1 << 10);
pub static kCTFontVerticalTrait: CTFontSymbolicTraits = (1 << 11);
pub static kCTFontUIOptimizedTrait: CTFontSymbolicTraits = (1 << 12);
pub static kCTFontClassMaskTrait: CTFontSymbolicTraits = (15 << kCTFontClassMaskShift);

pub trait SymbolicTraitAccessors {
    fn is_italic(&self) -> bool;
    fn is_bold(&self) -> bool;
    fn is_expanded(&self) -> bool;
    fn is_condensed(&self) -> bool;
    fn is_monospace(&self) -> bool;
}

impl SymbolicTraitAccessors for CTFontSymbolicTraits {
    fn is_italic(&self) -> bool { (*self & kCTFontItalicTrait) != 0 }
    fn is_bold(&self) -> bool { (*self & kCTFontBoldTrait) != 0 }
    fn is_expanded(&self) -> bool { (*self & kCTFontExpandedTrait) != 0 }
    fn is_condensed(&self) -> bool { (*self & kCTFontCondensedTrait) != 0 }
    fn is_monospace(&self) -> bool { (*self & kCTFontMonoSpaceTrait) != 0 }
}

pub type CTFontStylisticClass = u32;
pub static kCTFontUnknownClass: CTFontStylisticClass = (0 << kCTFontClassMaskShift);
pub static kCTFontOldStyleSerifsClass: CTFontStylisticClass = (1 << kCTFontClassMaskShift);
pub static kCTFontTransitionalSerifsClass: CTFontStylisticClass = (2 << kCTFontClassMaskShift);
pub static kCTFontModernSerifsClass: CTFontStylisticClass = (3 << kCTFontClassMaskShift);
pub static kCTFontClarendonSerifsClass: CTFontStylisticClass = (4 << kCTFontClassMaskShift);
pub static kCTFontSlabSerifsClass: CTFontStylisticClass = (5 << kCTFontClassMaskShift);
pub static kCTFontFreeformSerifsClass: CTFontStylisticClass = (7 << kCTFontClassMaskShift);
pub static kCTFontSansSerifClass: CTFontStylisticClass = (8 << kCTFontClassMaskShift);
pub static kCTFontOrnamentalsClass: CTFontStylisticClass = (9 << kCTFontClassMaskShift);
pub static kCTFontScriptsClass: CTFontStylisticClass = (10 << kCTFontClassMaskShift);
pub static kCTFontSymbolicClass: CTFontStylisticClass = (12 << kCTFontClassMaskShift);

pub trait StylisticClassAccessors {
    fn is_serif(&self) -> bool;
    fn is_sans_serif(&self) -> bool;
    fn is_script(&self) -> bool;
    fn is_fantasy(&self) -> bool;
    fn is_symbols(&self) -> bool;
}

impl StylisticClassAccessors for CTFontStylisticClass {
    fn is_serif(&self) -> bool {
        let any_serif_class = kCTFontOldStyleSerifsClass 
            | kCTFontTransitionalSerifsClass
            | kCTFontModernSerifsClass
            | kCTFontClarendonSerifsClass
            | kCTFontSlabSerifsClass
            | kCTFontFreeformSerifsClass;

        return (*self & any_serif_class) != 0;
    }

    fn is_sans_serif(&self) -> bool {
        return (*self & kCTFontSansSerifClass) != 0;
    }

    fn is_script(&self) -> bool {
        return (*self & kCTFontScriptsClass) != 0;
    }

    fn is_fantasy(&self) -> bool {
        return (*self & kCTFontOrnamentalsClass) != 0;
    }

    fn is_symbols(&self) -> bool {
        return (*self & kCTFontSymbolicClass) != 0;
    }
}

pub type CTFontAttributes = UntypedCFDictionary;
pub type CTFontTraits = UntypedCFDictionary;

pub trait TraitAccessors {
    fn symbolic_traits(&self) -> CTFontSymbolicTraits;
    fn normalized_weight(&self) -> float;
    fn normalized_width(&self) -> float;
    fn normalized_slant(&self) -> float;
}

priv trait TraitAccessorPrivate {
    fn extract_number_for_key(&self, key: CFStringRef) -> CFNumber;
}

impl TraitAccessorPrivate for CTFontTraits {
    priv fn extract_number_for_key(&self, key: CFStringRef) -> CFNumber {
        let value = self.get(&key);
        CFNumber::wrap_shared(core_foundation::base::downcast::<CFNumberRef>(value))
    }

}

impl TraitAccessors for CTFontTraits {
    fn symbolic_traits(&self) -> CTFontSymbolicTraits {
        unsafe {
            let number = self.extract_number_for_key(kCTFontSymbolicTrait);
            cast::transmute(number.to_i32())
        }
    }

    fn normalized_weight(&self) -> float {
        unsafe {
            let number = self.extract_number_for_key(kCTFontWeightTrait);
            cast::transmute(number.to_float())
        }
    }

    fn normalized_width(&self) -> float {
        unsafe {
            let number = self.extract_number_for_key(kCTFontWidthTrait);
            cast::transmute(number.to_float())
        }
    }

    fn normalized_slant(&self) -> float {
        unsafe {
            let number = self.extract_number_for_key(kCTFontSlantTrait);
            cast::transmute(number.to_float())
        }
    }
}

/*
* CTFontDescriptor.h
*/
pub type CTFontOrientation = u32;
pub static kCTFontDefaultOrientation: CTFontOrientation = 0;
pub static kCTFontHorizontalOrientation: CTFontOrientation = 1;
pub static kCTFontVerticalOrientation: CTFontOrientation = 2;

pub type CTFontPriority = u32;
pub static kCTFontPrioritySystem: CTFontPriority = 10000;
pub static kCTFontPriorityNetwork: CTFontPriority = 20000;
pub static kCTFontPriorityComputer: CTFontPriority = 30000;
pub static kCTFontPriorityUser: CTFontPriority = 40000;
pub static kCTFontPriorityDynamic: CTFontPriority = 50000;
pub static kCTFontPriorityProcess: CTFontPriority = 60000;

struct __CTFontDescriptor { private: () }
pub type CTFontDescriptorRef = *__CTFontDescriptor;

impl AbstractCFTypeRef for CTFontDescriptorRef {
    fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }

    fn type_id() -> CFTypeID {
        unsafe {
            CTFontDescriptorGetTypeID()
        }
    }
}

pub type CTFontDescriptor = CFWrapper<CTFontDescriptorRef, (), ()>;

pub trait CTFontDescriptorMethods {
    fn family_name(&self) -> ~str;
    fn font_name(&self) -> ~str;
    fn style_name(&self) -> ~str;
    fn display_name(&self) -> ~str;
    fn font_path(&self) -> ~str;
}

priv trait CTFontDescriptorMethodsPrivate {
    fn get_string_attribute(&self, attribute: CFStringRef) -> Option<~str>;
}

impl CTFontDescriptorMethodsPrivate for CTFontDescriptor {
    priv fn get_string_attribute(&self, attribute: CFStringRef) -> Option<~str> {
        unsafe {
            let value = CTFontDescriptorCopyAttribute(self.obj, attribute);
            if value.is_null() {
                return None;
            }

            Some(CFString::wrap_owned(core_foundation::base::downcast::<CFStringRef>(
                    value)).to_str())
        }
    }

}

impl CTFontDescriptorMethods for CTFontDescriptor {
    fn family_name(&self) -> ~str {
        let value = self.get_string_attribute(kCTFontDisplayNameAttribute);
        value.expect(~"A font must have a non-null font family name.")
    }

    fn font_name(&self) -> ~str {
        let value = self.get_string_attribute(kCTFontNameAttribute);
        value.expect(~"A font must have a non-null name.")
    }

    fn style_name(&self) -> ~str {
        let value = self.get_string_attribute(kCTFontStyleNameAttribute);
        value.expect(~"A font must have a non-null style name.")
    }

    fn display_name(&self) -> ~str {
        let value = self.get_string_attribute(kCTFontDisplayNameAttribute);
        value.expect(~"A font must have a non-null display name.")
    }

    fn font_path(&self) -> ~str {
        unsafe {
            let value = CTFontDescriptorCopyAttribute(self.obj, kCTFontURLAttribute);
            assert!(value.is_not_null());

            CFWrapper::wrap_owned(core_foundation::base::downcast::<CFURLRef>(value)).to_str()
        }
    }
}

pub fn new_from_attributes(attributes: &CFWrapper<CFDictionaryRef, CFStringRef, CFTypeRef>)
                        -> CTFontDescriptor {
    unsafe {
        let result: CTFontDescriptorRef =
            CTFontDescriptorCreateWithAttributes(*attributes.borrow_ref());
        CFWrapper::wrap_owned(result)
    }
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
    static kCTFontSymbolicTrait: CFStringRef;
    static kCTFontWeightTrait: CFStringRef;
    static kCTFontWidthTrait: CFStringRef;
    static kCTFontSlantTrait: CFStringRef;

    /*
     * CTFontDescriptor.h
     */

    // font attribute constants. Note that the name-related attributes
    // here are somewhat flaky. Servo creates CTFont instances and
    // then uses CTFontCopyName to get more fine-grained names.
    static kCTFontURLAttribute:                  CFStringRef; // value: CFURLRef
    static kCTFontNameAttribute:                 CFStringRef; // value: CFStringRef
    static kCTFontDisplayNameAttribute:          CFStringRef; // value: CFStringRef
    static kCTFontFamilyNameAttribute:           CFStringRef; // value: CFStringRef
    static kCTFontStyleNameAttribute:            CFStringRef; // value: CFStringRef
    static kCTFontTraitsAttribute:               CFStringRef;
    static kCTFontVariationAttribute:            CFStringRef;
    static kCTFontSizeAttribute:                 CFStringRef;
    static kCTFontMatrixAttribute:               CFStringRef;
    static kCTFontCascadeListAttribute:          CFStringRef;
    static kCTFontCharacterSetAttribute:         CFStringRef;
    static kCTFontLanguagesAttribute:            CFStringRef;
    static kCTFontBaselineAdjustAttribute:       CFStringRef;
    static kCTFontMacintoshEncodingsAttribute:   CFStringRef;
    static kCTFontFeaturesAttribute:             CFStringRef;
    static kCTFontFeatureSettingsAttribute:      CFStringRef;
    static kCTFontFixedAdvanceAttribute:         CFStringRef;
    static kCTFontOrientationAttribute:          CFStringRef;
    static kCTFontFormatAttribute:               CFStringRef;
    static kCTFontRegistrationScopeAttribute:    CFStringRef;
    static kCTFontPriorityAttribute:             CFStringRef;
    static kCTFontEnabledAttribute:              CFStringRef;

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
    fn CTFontDescriptorCreateWithAttributes(attributes: CFDictionaryRef) -> CTFontDescriptorRef;
    fn CTFontDescriptorCreateWithNameAndSize(name: CFStringRef, size: CGFloat) -> CTFontDescriptorRef;
    fn CTFontDescriptorGetTypeID() -> CFTypeID;
}

pub extern {
    fn CTFontDescriptorCreateMatchingFontDescriptors(descriptor: CTFontDescriptorRef,
                                                     mandatoryAttributes: CFSetRef) -> CFArrayRef;
}
