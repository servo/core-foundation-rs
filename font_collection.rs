use font_descriptor;
use font_descriptor::{CTFontAttributes, CTFontDescriptor};
use font_descriptor::{CTFontDescriptorCreateMatchingFontDescriptors, CTFontDescriptorRef};
use font_manager::CTFontManagerCopyAvailableFontFamilyNames;

use core::libc::c_void;
use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::{AbstractCFTypeRef, CFAllocatorRef, CFIndex, CFRange, CFTypeID};
use core_foundation::base::{CFTypeRef, CFWrapper, kCFAllocatorDefault};
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef, UntypedCFDictionary};
use core_foundation::number::CFNumber;
use core_foundation::set::CFSet;
use core_foundation::string::{CFString, CFStringRef};

struct __CTFontCollection { private: () }
pub type CTFontCollectionRef = *__CTFontCollection;

impl AbstractCFTypeRef for CTFontCollectionRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }

    static pure fn type_id() -> CFTypeID {
        unsafe {
            CTFontCollectionGetTypeID()
        }
    }
}

pub type CTFontCollection = CFWrapper<CTFontCollectionRef, (), ()>;

pub trait CTFontCollectionMethods {
    pure fn get_descriptors() -> CFArray<CTFontDescriptorRef>;
}

pub impl CTFontCollectionMethods for CTFontCollection {
    pure fn get_descriptors() -> CFArray<CTFontDescriptorRef> {
        use core_foundation::base::CFRetain;

        // surprise! this function follows the Get rule, despite being named *Create*.
        // So we have to addRef it to avoid CTFontCollection from double freeing it later.
        unsafe {
            CFWrapper::wrap_shared(CTFontCollectionCreateMatchingFontDescriptors(self.obj))
        }
    }
}

pub fn new_from_descriptors(descs: &CFArray<CTFontDescriptorRef>) -> CTFontCollection {
    let key = CFString::wrap_extern(kCTFontCollectionRemoveDuplicatesOption);
    let value = CFNumber::new(1_i8);
    let options = CFDictionary::new([ (*key.borrow_ref(), *value.borrow_type_ref()) ]);
    unsafe {
        let result = CTFontCollectionCreateWithFontDescriptors(*descs.borrow_ref(),
                                                               *options.borrow_ref());
        CFWrapper::wrap_owned(result)
    }
}

pub fn create_for_all_families() -> CTFontCollection {
    let key = CFString::wrap_extern(kCTFontCollectionRemoveDuplicatesOption);
    let value = CFNumber::new(1_i8);
    let options = CFDictionary::new([ (*key.borrow_ref(), *value.borrow_type_ref()) ]);
    unsafe {
        let result = CTFontCollectionCreateFromAvailableFonts(*options.borrow_ref());
        CFWrapper::wrap_owned(result)
    }
}

pub fn create_for_family(family: &str) -> CTFontCollection {
    use font_descriptor::kCTFontFamilyNameAttribute;
   
    let family_attr = CFString::wrap_extern(kCTFontFamilyNameAttribute);
    let family_name = CFString::new(family);

    let specified_attrs: CFWrapper<CFDictionaryRef,CFStringRef,CFTypeRef> =
        CFDictionary::new([
            (*family_attr.borrow_ref(), *family_name.borrow_type_ref())
        ]);

    let wildcard_desc: CTFontDescriptor =
        font_descriptor::new_from_attributes(&specified_attrs);
    let mandatory_attrs = CFSet::new([ *family_attr.borrow_ref() ]);
    let matched_descs = unsafe {
        CTFontDescriptorCreateMatchingFontDescriptors(
            *wildcard_desc.borrow_ref(),
            *mandatory_attrs.borrow_ref())
    };

    let matched_descs: CFArray<CTFontDescriptorRef> = CFWrapper::wrap_owned(matched_descs);

    // I suppose one doesn't even need the CTFontCollection object at this point.
    // But we stick descriptors into and out of it just to provide a nice wrapper API.
    new_from_descriptors(&matched_descs)
}

pub pure fn get_family_names() -> CFArray<CFStringRef> {
    unsafe {
        CFWrapper::wrap_owned(CTFontManagerCopyAvailableFontFamilyNames())
    }
}

extern {
    /*
     * CTFontCollection.h
     */

    const kCTFontCollectionRemoveDuplicatesOption: CFStringRef;

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
