use base::{CFAllocatorRef, CFIndex, CFRelease, CFType, CFTypeRef, kCFAllocatorDefault};
use dvec::DVec;
use libc::c_void;
use ptr::to_unsafe_ptr;
use unsafe::reinterpret_cast;
use vec::unsafe::to_ptr_slice;

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

struct CFDictionary<K:CFType,V:CFType> {
    obj: CFDictionaryRef,

    drop {
        unsafe {
            CFRelease(reinterpret_cast(self.obj));
        }
    }
}

mod CFDictionary {
    fn wrap<K:CFType,V:CFType>(obj: CFDictionaryRef) -> CFDictionary<K,V> {
        CFDictionary { obj: obj }
    }

    fn new_dictionary<K:CFType,V:CFType>(pairs: &[(K,V)]) -> CFDictionary<K,V> {
        let (keys, values) = (DVec(), DVec());
        for pairs.each |pair| {
            // FIXME: "let" would be much nicer here, but that doesn't work yet.
            match pair {
                (ref key, ref value) => {
                    keys.push(key.get());
                    values.push(value.get());
                }
            }
        }

        assert keys.len() == values.len();
        let keys = vec::from_mut(dvec::unwrap(keys));
        let values = vec::from_mut(dvec::unwrap(values));

        let dictionary_ref;
        unsafe {
            dictionary_ref = CFDictionaryCreate(kCFAllocatorDefault,
                                                reinterpret_cast(to_ptr_slice(keys)),
                                                reinterpret_cast(to_ptr_slice(values)),
                                                keys.len() as CFIndex,
                                                to_unsafe_ptr(&kCFTypeDictionaryKeyCallBacks),
                                                to_unsafe_ptr(&kCFTypeDictionaryValueCallBacks));
        }

        return wrap(dictionary_ref);
    }
}

impl<K:CFType,V:CFType> CFDictionary<K,V> : CFType {
    pure fn get(&self) -> CFTypeRef {
        unsafe {
            reinterpret_cast(self.obj)
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

