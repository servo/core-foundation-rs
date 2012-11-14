use cf = core_foundation;
use cf::array::{CFArray, CFArrayRef};
use cf::base::{
    AbstractCFType,
    AbstractCFTypeRef,
    CFAllocatorRef,
    CFIndex,
    CFRange,
    CFRelease,
    CFTypeID,
    CFTypeOps,
    CFTypeRef,
    kCFAllocatorDefault,
};
use cf::dictionary::{CFDictionary, CFDictionaryRef};
use cf::number::CFNumber;
use cf::set::CFSet;
use cf::string::{CFString, CFStringRef};

use font_descriptor::{
    CTFontDescriptor,
    CTFontDescriptorCreateMatchingFontDescriptors,
    CTFontDescriptorRef
};
use font_manager::CTFontManagerCopyAvailableFontFamilyNames;

use libc::c_void;

struct __CTFontCollection { private: () }
pub type CTFontCollectionRef = *__CTFontCollection;

impl CTFontCollectionRef : AbstractCFTypeRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }
}

struct CTFontCollection {
    obj: CTFontCollectionRef,

    drop {
        unsafe {
            CFRelease(cast::transmute(self.obj))
        }
    }
}

pub impl CTFontCollection : AbstractCFType<CTFontCollectionRef> {
    pure fn get_ref() -> CTFontCollectionRef { self.obj }

    static fn wrap(obj: CTFontCollectionRef) -> CTFontCollection {
        CTFontCollection { obj: obj }
    }

    static fn unwrap(wrapper: CTFontCollection) -> CTFontCollectionRef {
        wrapper.obj
    }
}

pub impl CTFontCollection {
    static fn new_from_descriptors(descs: &CFArray<CTFontDescriptorRef, CTFontDescriptor>) -> CTFontCollection {
        let options = CFDictionary::new([
            (CFString::wrap_extern(kCTFontCollectionRemoveDuplicatesOption), CFNumber::new(1_i8))
        ]);

        let collection = CTFontCollectionCreateWithFontDescriptors(descs.get_ref(), options.get_ref());
        CTFontCollection { obj : move collection }
    }

    static fn create_for_all_families() -> CTFontCollection {
        let options = CFDictionary::new([
            (CFString::wrap_extern(kCTFontCollectionRemoveDuplicatesOption), CFNumber::new(1_i8))
        ]);
        let collection = CTFontCollectionCreateFromAvailableFonts(options.get_ref());
        CTFontCollection { obj: move collection }
    }

    static fn create_for_family(family: &str) -> CTFontCollection unsafe {
        use font_descriptor::kCTFontFamilyNameAttribute;
 
        // FIXME: ugly because of Rust #3902
        let family_as_cftype = cf::base::as_CFType::<CFStringRef, CFString>(move CFString::new(family));
        let specified_attrs = CFDictionary::new([
            (CFString::wrap_extern(kCTFontFamilyNameAttribute), move family_as_cftype),
        ]);

        let wildcard_desc = CTFontDescriptor::new_from_attributes(&specified_attrs);
        let mandatory_attrs = CFSet::new([
            CFString::wrap_extern(kCTFontFamilyNameAttribute)
        ]);

        let matched_descs = CTFontDescriptorCreateMatchingFontDescriptors(wildcard_desc.get_ref(),
                                                                          mandatory_attrs.get_ref());

        let matched_descs : CFArray<CTFontDescriptorRef, CTFontDescriptor> = cf::base::wrap(matched_descs);

        // I suppose one doesn't even need the CTFontCollection object at this point.
        // But we stick descriptors into and out of it just to provide a nice wrapper API.
        CTFontCollection::new_from_descriptors(&matched_descs)
    }

    static pure fn get_family_names() -> CFArray<CFStringRef, CFString> unsafe {
        cf::base::wrap(CTFontManagerCopyAvailableFontFamilyNames())
    }

    pure fn get_descriptors() -> CFArray<CTFontDescriptorRef, CTFontDescriptor> unsafe {
        use cf::base::CFRetain;

        // surprise! this function follows the Get rule, despite being named *Create*.
        // So we have to addRef it to avoid CTFontCollection from double freeing it later.
        let wrapper : CFArray<CTFontDescriptorRef, CTFontDescriptor> = cf::base::wrap(CTFontCollectionCreateMatchingFontDescriptors(self.obj));
        CFRetain(wrapper.get_ref().as_type_ref());
        return move wrapper;
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