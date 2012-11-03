use cf = core_foundation;
use cf::array::CFArrayRef;
use cf::base::{
    AbstractCFType,
    AbstractCFTypeRef,
    CFAllocatorRef,
    CFIndex,
    CFRange,
    CFRelease,
    CFTypeRef,
    kCFAllocatorDefault,
};
use cf::dictionary::{CFDictionary, CFDictionaryRef};
use cf::number::CFNumber;
use cf::string::{CFString, CFStringRef};

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
    static fn wrap(obj: CTFontCollectionRef) -> CTFontCollection {
        CTFontCollection { obj: obj }
    }

    static fn unwrap(wrapper: CTFontCollection) -> CTFontCollectionRef {
        wrapper.obj
    }

    pure fn as_type_ref(&self) -> CFTypeRef {
        unsafe {
            cast::transmute(self.obj)
        }
    }
}

pub impl CTFontCollection {
    static fn new() -> CTFontCollection {
        let options = CFDictionary::new([
            (CFString::new_wrapped(kCTFontCollectionRemoveDuplicatesOption), CFNumber::new(1_i8))
        ]);
        let collection = CTFontCollectionCreateFromAvailableFonts(*options.borrow_ref());

        CTFontCollection { obj: move collection }
    }
}

extern {
    /*
     * CTFontCollection.h
     */

    const kCTFontCollectionRemoveDuplicatesOption: CFStringRef;

    fn CTFontCollectionCreateCopyWithFontDescriptors(descriptors: CFArrayRef, options: CFDictionaryRef) ->CTFontCollectionRef;
    fn CTFontCollectionCreateFromAvailableFonts(options: CFDictionaryRef) -> CTFontCollectionRef;
    fn CTFontCollectionCreateMatchingFontDescriptors(collection: CTFontCollectionRef) -> CFArrayRef;
    fn CTFontCollectionCreateWithFontDescriptors(original: CTFontCollectionRef, 
                                                 descriptors: CFArrayRef,
                                                 options: CFDictionaryRef) -> CTFontCollectionRef;
    //fn CTFontCollectionCreateMatchingFontDescriptorsSortedWithCallback;
    //fn CTFontCollectionGetTypeID;
}