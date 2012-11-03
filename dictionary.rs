use base::{AbstractCFType, AbstractCFTypeRef, CFAllocatorRef, CFIndex, CFRelease, CFTypeRef, kCFAllocatorDefault};
use cast::reinterpret_cast;
use dvec::DVec;
use libc::c_void;
use ptr::to_unsafe_ptr;
use vec::raw::to_ptr;

pub type CFDictionaryRetainCallBack = *u8;
pub type CFDictionaryReleaseCallBack = *u8;
pub type CFDictionaryCopyDescriptionCallBack = *u8;
pub type CFDictionaryEqualCallBack = *u8;
pub type CFDictionaryHashCallBack = *u8;

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

impl CFDictionaryRef : AbstractCFTypeRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }
}

struct CFDictionary<KeyRefType   : AbstractCFTypeRef,
                    ValueRefType : AbstractCFTypeRef,
                    KeyType      : AbstractCFType<KeyRefType>,
                    ValueType    : AbstractCFType<ValueRefType>> {
    obj: CFDictionaryRef,

    drop {
        unsafe {
            CFRelease(reinterpret_cast(&self.obj));
        }
    }
}

pub impl<KeyRefType   : AbstractCFTypeRef,
         ValueRefType : AbstractCFTypeRef,
         KeyType      : AbstractCFType<KeyRefType>,
         ValueType    : AbstractCFType<ValueRefType>>
    CFDictionary<KeyRefType, ValueRefType, KeyType, ValueType> : AbstractCFType<CFDictionaryRef> {

    pure fn get_ref() -> CFDictionaryRef { self.obj }

    static fn wrap(obj: CFDictionaryRef) -> CFDictionary<KeyRefType, ValueRefType, KeyType, ValueType> {
        CFDictionary { obj: obj }
    }

    static fn unwrap(wrapper: CFDictionary<KeyRefType, ValueRefType, KeyType, ValueType>) -> CFDictionaryRef {
        wrapper.obj
    }
}

pub impl<KeyRefType   : AbstractCFTypeRef,
         ValueRefType : AbstractCFTypeRef,
         KeyType      : AbstractCFType<KeyRefType>,
         ValueType    : AbstractCFType<ValueRefType>>
    CFDictionary<KeyRefType, ValueRefType, KeyType, ValueType> {

    static fn new(pairs: &[(KeyType,ValueType)]) -> CFDictionary<KeyRefType, ValueRefType, KeyType, ValueType> {
        let (keys, values) = (DVec(), DVec());
        for pairs.each |pair| {
            // FIXME: "let" would be much nicer here, but that doesn't work yet.
            match *pair {
                (ref key, ref value) => {
                    // TODO: should be able to say key.get_type_ref(), but resolve isn't having any of it.
                    keys.push(key.get_ref().as_type_ref());
                    values.push(value.get_ref().as_type_ref());
                }
            }
        }

        assert keys.len() == values.len();
        let keys = dvec::unwrap(move keys);
        let values = dvec::unwrap(move values);

        let dictionary_ref : CFDictionaryRef;
        unsafe {
            dictionary_ref = CFDictionaryCreate(kCFAllocatorDefault,
                                                reinterpret_cast(&to_ptr(keys)),
                                                reinterpret_cast(&to_ptr(values)),
                                                keys.len() as CFIndex,
                                                to_unsafe_ptr(&kCFTypeDictionaryKeyCallBacks),
                                                to_unsafe_ptr(&kCFTypeDictionaryValueCallBacks));
        }

        return base::wrap(dictionary_ref);
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

    fn CFDictionaryCreate(allocator: CFAllocatorRef, keys: **c_void, values: **c_void,
                          numValues: CFIndex, keyCallBacks: *CFDictionaryKeyCallBacks,
                          valueCallBacks: *CFDictionaryValueCallBacks)
                       -> CFDictionaryRef;
}

