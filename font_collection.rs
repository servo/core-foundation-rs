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
    CFTypeRef,
    kCFAllocatorDefault,
};
use cf::dictionary::{CFDictionary, CFDictionaryRef};
use cf::number::CFNumber;
use cf::string::{CFString, CFStringRef};

use font_descriptor::{CTFontDescriptor, CTFontDescriptorRef};

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
    static fn new() -> CTFontCollection {
        let options = CFDictionary::new([
            (CFString::wrap_extern(kCTFontCollectionRemoveDuplicatesOption), CFNumber::new(1_i8))
        ]);
        let collection = CTFontCollectionCreateFromAvailableFonts(options.get_ref());
        CTFontCollection { obj: move collection }
    }

    fn get_descriptors() -> CFArray<CTFontDescriptorRef, CTFontDescriptor> {
        cf::base::wrap(CTFontCollectionCreateMatchingFontDescriptors(self.obj))
    }
}

extern {
    /*
     * CTFontCollection.h
     */

    const kCTFontCollectionRemoveDuplicatesOption: CFStringRef;

    fn CTFontCollectionCreateCopyWithFontDescriptors(descriptors: CFArrayRef, options: CFDictionaryRef) -> CTFontCollectionRef;
    fn CTFontCollectionCreateFromAvailableFonts(options: CFDictionaryRef) -> CTFontCollectionRef;
    fn CTFontCollectionCreateMatchingFontDescriptors(collection: CTFontCollectionRef) -> CFArrayRef;
    fn CTFontCollectionCreateWithFontDescriptors(original: CTFontCollectionRef, 
                                                 descriptors: CFArrayRef,
                                                 options: CFDictionaryRef) -> CTFontCollectionRef;
    //fn CTFontCollectionCreateMatchingFontDescriptorsSortedWithCallback;
    fn CTFontCollectionGetTypeID() -> CFTypeID;
}