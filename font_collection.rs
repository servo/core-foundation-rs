// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use font_descriptor;
use font_descriptor::CTFontDescriptor;
use font_descriptor::{CTFontDescriptorCreateMatchingFontDescriptors, CTFontDescriptorRef};
use font_manager::CTFontManagerCopyAvailableFontFamilyNames;

use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::{AbstractCFTypeRef, CFTypeID};
use core_foundation::base::{CFTypeRef, CFWrapper};
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::number::CFNumber;
use core_foundation::set::CFSet;
use core_foundation::string::{CFString, CFStringRef};

struct __CTFontCollection { private: () }
pub type CTFontCollectionRef = *__CTFontCollection;

impl AbstractCFTypeRef for CTFontCollectionRef {
    fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }

    #[fixed_stack_segment]
    fn type_id() -> CFTypeID {
        unsafe {
            CTFontCollectionGetTypeID()
        }
    }
}

pub type CTFontCollection = CFWrapper<CTFontCollectionRef, (), ()>;

pub trait CTFontCollectionMethods {
    fn get_descriptors(&self) -> CFArray<CTFontDescriptorRef>;
}

impl CTFontCollectionMethods for CTFontCollection {
    #[fixed_stack_segment]
    fn get_descriptors(&self) -> CFArray<CTFontDescriptorRef> {
        // surprise! this function follows the Get rule, despite being named *Create*.
        // So we have to addRef it to avoid CTFontCollection from double freeing it later.
        unsafe {
            CFArray::wrap_shared(CTFontCollectionCreateMatchingFontDescriptors(self.obj))
        }
    }
}

#[fixed_stack_segment]
pub fn new_from_descriptors(descs: &CFArray<CTFontDescriptorRef>) -> CTFontCollection {
    let key = CFString::wrap_shared(kCTFontCollectionRemoveDuplicatesOption);
    let value = CFNumber::new(1_i8);
    let options = CFDictionary::new([
        (*key.contents.borrow_ref(), *value.contents.borrow_type_ref())
    ]);
    unsafe {
        let result = CTFontCollectionCreateWithFontDescriptors(*descs.contents.borrow_ref(),
                                                               *options.contents.borrow_ref());
        CFWrapper::wrap_owned(result)
    }
}

#[fixed_stack_segment]
pub fn create_for_all_families() -> CTFontCollection {
    let key = CFString::wrap_shared(kCTFontCollectionRemoveDuplicatesOption);
    let value = CFNumber::new(1_i8);
    let options = CFDictionary::new([
        (*key.contents.borrow_ref(), *value.contents.borrow_type_ref())
    ]);
    unsafe {
        let result = CTFontCollectionCreateFromAvailableFonts(*options.contents.borrow_ref());
        CFWrapper::wrap_owned(result)
    }
}

#[fixed_stack_segment]
pub fn create_for_family(family: &str) -> CTFontCollection {
    use font_descriptor::kCTFontFamilyNameAttribute;
   
    let family_attr = CFString::wrap_shared(kCTFontFamilyNameAttribute);
    let family_name = CFString::new(family);

    let specified_attrs: CFDictionary<CFStringRef,CFTypeRef> =
        CFDictionary::new([
            (*family_attr.contents.borrow_ref(), *family_name.contents.borrow_type_ref())
        ]);

    let wildcard_desc: CTFontDescriptor =
        font_descriptor::new_from_attributes(&specified_attrs.contents);
    let mandatory_attrs = CFSet::new([ *family_attr.contents.borrow_ref() ]);
    let matched_descs = unsafe {
        CTFontDescriptorCreateMatchingFontDescriptors(
            *wildcard_desc.borrow_ref(),
            *mandatory_attrs.contents.borrow_ref())
    };

    let matched_descs: CFArray<CTFontDescriptorRef> = CFArray::wrap_owned(matched_descs);

    // I suppose one doesn't even need the CTFontCollection object at this point.
    // But we stick descriptors into and out of it just to provide a nice wrapper API.
    new_from_descriptors(&matched_descs)
}

#[fixed_stack_segment]
pub fn get_family_names() -> CFArray<CFStringRef> {
    unsafe {
        CFArray::wrap_owned(CTFontManagerCopyAvailableFontFamilyNames())
    }
}

extern {
    /*
     * CTFontCollection.h
     */

    static kCTFontCollectionRemoveDuplicatesOption: CFStringRef;

    fn CTFontCollectionCreateCopyWithFontDescriptors(original: CTFontCollectionRef,
                                                     descriptors: CFArrayRef,
                                                     options: CFDictionaryRef) -> CTFontCollectionRef;
    fn CTFontCollectionCreateFromAvailableFonts(options: CFDictionaryRef) -> CTFontCollectionRef;
    // this stupid function doesn't actually do any wildcard expansion; 
    // it just chooses the best match. Use
    // CTFontDescriptorCreateMatchingDescriptors instead.
    fn CTFontCollectionCreateMatchingFontDescriptors(collection: CTFontCollectionRef) -> CFArrayRef;
    fn CTFontCollectionCreateWithFontDescriptors(descriptors: CFArrayRef,
                                                 options: CFDictionaryRef) -> CTFontCollectionRef;
    //fn CTFontCollectionCreateMatchingFontDescriptorsSortedWithCallback;
    fn CTFontCollectionGetTypeID() -> CFTypeID;
}
