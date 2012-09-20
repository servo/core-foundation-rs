use base::{AbstractCFType, CFAllocatorRef, CFIndex, CFRelease, CFTypeRef, kCFAllocatorDefault};
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

struct CFDictionary<K:AbstractCFType,V:AbstractCFType> {
    obj: CFDictionaryRef,

    drop {
        unsafe {
            CFRelease(reinterpret_cast(&self.obj));
        }
    }
}

mod CFDictionary {
    fn wrap<K:AbstractCFType,V:AbstractCFType>(obj: CFDictionaryRef) -> CFDictionary<K,V> {
        CFDictionary { obj: obj }
    }

    fn new_dictionary<K:AbstractCFType,V:AbstractCFType>(pairs: &[(K,V)]) -> CFDictionary<K,V> {
        let (keys, values) = (DVec(), DVec());
        for pairs.each |pair| {
            // FIXME: "let" would be much nicer here, but that doesn't work yet.
            match *pair {
                (ref key, ref value) => {
                    keys.push(key.as_type_ref());
                    values.push(value.as_type_ref());
                }
            }
        }

        assert keys.len() == values.len();
        let keys = dvec::unwrap(keys);
        let values = dvec::unwrap(values);

        let dictionary_ref;
        unsafe {
            dictionary_ref = CFDictionaryCreate(kCFAllocatorDefault,
                                                reinterpret_cast(&to_ptr(keys)),
                                                reinterpret_cast(&to_ptr(values)),
                                                keys.len() as CFIndex,
                                                to_unsafe_ptr(&kCFTypeDictionaryKeyCallBacks),
                                                to_unsafe_ptr(&kCFTypeDictionaryValueCallBacks));
        }

        return wrap(dictionary_ref);
    }
}

impl<K:AbstractCFType,V:AbstractCFType> CFDictionary<K,V> : AbstractCFType {
    pure fn as_type_ref(&self) -> CFTypeRef {
        unsafe {
            reinterpret_cast(&self.obj)
        }
    }
}

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    const kCFTypeDictionaryKeyCallBacks: CFDictionaryKeyCallBacks;
    const kCFTypeDictionaryValueCallBacks: CFDictionaryValueCallBacks;

    fn CFDictionaryCreate(allocator: CFAllocatorRef, keys: **c_void, values: **c_void,
                          numValues: CFIndex, keyCallBacks: *CFDictionaryKeyCallBacks,
                          valueCallBacks: *CFDictionaryValueCallBacks)
                       -> CFDictionaryRef;
}

