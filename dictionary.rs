use base::{
    AbstractCFType,
    AbstractCFTypeRef,
    Boolean,
    CFAllocatorRef,
    CFIndex,
    CFRelease,
    CFTypeID,
    CFTypeRef,
    kCFAllocatorDefault
};
use cast::reinterpret_cast;
use dvec::DVec;
use libc::c_void;
use ptr::to_unsafe_ptr;
use vec::raw::to_ptr;

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

trait DictionaryMethods {
    // TODO
}

pub impl<KeyRefType   : AbstractCFTypeRef Copy,
         ValueRefType : AbstractCFTypeRef Copy,
         KeyType      : AbstractCFType<KeyRefType>,
         ValueType    : AbstractCFType<ValueRefType>>
    CFDictionary<KeyRefType, ValueRefType, KeyType, ValueType> {
    pure fn len() -> uint unsafe {
        return CFDictionaryGetCount(self.obj) as uint;
    }

    pure fn is_empty() -> bool { self.len() == 0 }

    pure fn contains_key(key: &KeyType) -> bool unsafe {
        return CFDictionaryContainsKey(self.obj, cast::transmute(key.get_ref())) as bool;
    }

    pure fn find(key: &KeyType) -> Option<ValueType> unsafe {
        let value : *c_void = ptr::null();
        let did_find_value = CFDictionaryGetValueIfPresent(self.obj,
                                                           cast::transmute(key.get_ref()),
                                                           cast::transmute(&value)) as bool;

        // FIXME: this will not handle non-CF dictionary entries
        // or ptr::null() values correctly.
        return if did_find_value {
            Some(base::wrap(cast::transmute::<*c_void, ValueRefType>(value)))
        } else {
            None
        }
    }

    pure fn get(key: &KeyType) -> ValueType {
        let value = self.find(key);
        if value.is_none() {
            fail fmt!("No entry found for key: %?", key);
        }
        option::unwrap(move value)
    }

             fn each(blk: fn&(&KeyType, &ValueType) -> bool) unsafe {
                 let len = self.len();
                 let keys: ~[KeyRefType] = vec::from_elem(len, cast::transmute::<*c_void, KeyRefType>(ptr::null()));
                 let values: ~[ValueRefType] = vec::from_elem(len, cast::transmute::<*c_void, ValueRefType>(ptr::null()));

                 do uint::range(0,len) |i| {
                     let key = base::wrap(keys[i]);
                     let value = base::wrap(values[i]);
                     blk(&key, &value)
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
    fn CFDictionaryCreateCopy(allocator: CFAllocatorRef, theDict: CFDictionaryRef) -> CFDictionaryRef;
    fn CFDictionaryGetCount(theDict: CFDictionaryRef) -> CFIndex;
    fn CFDictionaryGetCountOfKey(theDict: CFDictionaryRef, key: *c_void) -> CFIndex;
    fn CFDictionaryGetCountOfValue(theDict: CFDictionaryRef, value: *c_void) -> CFIndex;
    fn CFDictionaryGetKeysAndValues(theDict: CFDictionaryRef, keys: **c_void, values: **c_void);
    fn CFDictionaryGetTypeID() -> CFTypeID;
    fn CFDictionaryGetValue(theDict: CFDictionaryRef, key: *c_void) -> *c_void;
    fn CFDictionaryGetValueIfPresent(theDict: CFDictionaryRef, key: *c_void, value: **c_void) -> Boolean;
}

