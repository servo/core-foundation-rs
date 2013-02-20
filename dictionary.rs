use base::{
    AbstractCFTypeRef,
    Boolean,
    CFAllocatorRef,
    CFIndex,
    CFType,
    CFTypeID,
    CFTypeRef,
    CFWrapper,
    kCFAllocatorDefault
};
use string::{CFString, CFStringRef};

use dvec::DVec;
use libc::c_void;

pub type CFDictionaryApplierFunction = *u8;
pub type CFDictionaryCopyDescriptionCallBack = *u8;
pub type CFDictionaryEqualCallBack = *u8;
pub type CFDictionaryHashCallBack = *u8;
pub type CFDictionaryReleaseCallBack = *u8;
pub type CFDictionaryRetainCallBack = *u8;

pub struct CFDictionaryKeyCallBacks {
    version: CFIndex,
    retain: CFDictionaryRetainCallBack,
    release: CFDictionaryReleaseCallBack,
    copyDescription: CFDictionaryCopyDescriptionCallBack,
    equal: CFDictionaryEqualCallBack,
    hash: CFDictionaryHashCallBack
}

pub struct CFDictionaryValueCallBacks {
    version: CFIndex,
    retain: CFDictionaryRetainCallBack,
    release: CFDictionaryReleaseCallBack,
    copyDescription: CFDictionaryCopyDescriptionCallBack,
    equal: CFDictionaryEqualCallBack
}

struct __CFDictionary { private: () }
pub type CFDictionaryRef = *__CFDictionary;

impl AbstractCFTypeRef for CFDictionaryRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }

    static pure fn type_id() -> CFTypeID {
        unsafe {
            CFDictionaryGetTypeID()
        }
    }
}

pub type CFDictionary<KeyRefType, ValueRefType> = CFWrapper<CFDictionaryRef, KeyRefType, ValueRefType>;
pub type UntypedCFDictionary = CFDictionary<CFStringRef, CFTypeRef>;

pub impl<KeyRefType: Copy AbstractCFTypeRef, ValueRefType: Copy AbstractCFTypeRef>
    CFDictionary<KeyRefType, ValueRefType> {

    static fn new(pairs: &[(KeyRefType,ValueRefType)]) -> CFDictionary<KeyRefType, ValueRefType> {
        let (keys, values) = (DVec(), DVec());
        for pairs.each |pair| {
            // FIXME: "let" would be much nicer here, but that doesn't work yet.
            match *pair {
                (key, value) => {
                    keys.push(key.as_type_ref());
                    values.push(value.as_type_ref());
                }
            }
        }

        assert keys.len() == values.len();
        let keys = dvec::unwrap(keys);
        let values = dvec::unwrap(values);

        let dictionary_ref : CFDictionaryRef;
        unsafe {
            dictionary_ref = CFDictionaryCreate(kCFAllocatorDefault,
                                                cast::transmute(vec::raw::to_ptr(keys)),
                                                cast::transmute(vec::raw::to_ptr(values)),
                                                keys.len() as CFIndex,
                                                ptr::to_unsafe_ptr(&kCFTypeDictionaryKeyCallBacks),
                                                ptr::to_unsafe_ptr(&kCFTypeDictionaryValueCallBacks));
        }

        CFWrapper::wrap_owned(dictionary_ref)
    }
}

pub impl<KeyRefType   : AbstractCFTypeRef Copy,
         ValueRefType : AbstractCFTypeRef Copy>
    CFDictionary<KeyRefType, ValueRefType> {
    pure fn len() -> uint {
        unsafe {
            return CFDictionaryGetCount(self.obj) as uint;
        }
    }

    pure fn is_empty() -> bool { self.len() == 0 }

    pure fn contains_key(key: &KeyRefType) -> bool {
        unsafe {
            return CFDictionaryContainsKey(self.obj, cast::transmute(key.as_type_ref())) as bool;
        }
    }

    pure fn find(key: &KeyRefType) -> Option<ValueRefType> {
        unsafe {
            let value : *c_void = ptr::null();
            let did_find_value = CFDictionaryGetValueIfPresent(self.obj,
                                                               cast::transmute(key.as_type_ref()),
                                                               cast::transmute(&value)) as bool;

            // FIXME: this will not handle non-CF dictionary entries
            // or ptr::null() values correctly.
            return if did_find_value {
                Some(cast::transmute::<*c_void, ValueRefType>(value))
            } else {
                None
            }
        }
    }

    pure fn get(key: &KeyRefType) -> ValueRefType {
        let value = self.find(key);
        if value.is_none() {
            fail!(fmt!("No entry found for key: %?", key));
        }
        return option::unwrap(value);
    }

    fn each(blk: fn&(&KeyRefType, &ValueRefType) -> bool) {
        unsafe {
            let len = self.len();
            let null_keys = cast::transmute::<*c_void,KeyRefType>(ptr::null());
            let keys: ~[KeyRefType] = vec::from_elem(len, null_keys);
            let null_vals = cast::transmute::<*c_void,ValueRefType>(ptr::null());
            let values: ~[ValueRefType] = vec::from_elem(len, null_vals);

            do uint::range(0,len) |i| { blk(&keys[i], &values[i]) }
        }
    }
}


#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    /*
     * CFDictionary.h
     */

    const kCFTypeDictionaryKeyCallBacks: CFDictionaryKeyCallBacks;
    const kCFTypeDictionaryValueCallBacks: CFDictionaryValueCallBacks;

    fn CFDictionaryApplyFunction(theDict: CFDictionaryRef, applier: CFDictionaryApplierFunction,
                                 context: *c_void);
    fn CFDictionaryContainsKey(theDict: CFDictionaryRef, key: *c_void) -> Boolean;
    fn CFDictionaryContainsValue(theDict: CFDictionaryRef, value: *c_void) -> Boolean;
    fn CFDictionaryCreate(allocator: CFAllocatorRef, keys: **c_void, values: **c_void,
                          numValues: CFIndex, keyCallBacks: *CFDictionaryKeyCallBacks,
                          valueCallBacks: *CFDictionaryValueCallBacks)
                       -> CFDictionaryRef;
    fn CFDictionaryCreateCopy(allocator: CFAllocatorRef,
                              theDict: CFDictionaryRef)
                           -> CFDictionaryRef;
    fn CFDictionaryGetCount(theDict: CFDictionaryRef) -> CFIndex;
    fn CFDictionaryGetCountOfKey(theDict: CFDictionaryRef, key: *c_void) -> CFIndex;
    fn CFDictionaryGetCountOfValue(theDict: CFDictionaryRef, value: *c_void) -> CFIndex;
    fn CFDictionaryGetKeysAndValues(theDict: CFDictionaryRef, keys: **c_void, values: **c_void);
    fn CFDictionaryGetTypeID() -> CFTypeID;
    fn CFDictionaryGetValue(theDict: CFDictionaryRef, key: *c_void) -> *c_void;
    fn CFDictionaryGetValueIfPresent(theDict: CFDictionaryRef,
                                     key: *c_void,
                                     value: **c_void)
                                  -> Boolean;
}

