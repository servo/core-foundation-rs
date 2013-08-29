// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base::{
    AbstractCFTypeRef,
    Boolean,
    CFAllocatorRef,
    CFIndex,
    CFTypeID,
    CFTypeRef,
    CFWrapper,
    kCFAllocatorDefault
};
use string::CFStringRef;

use std::cast;
use std::libc::c_void;
use std::ptr;
use std::vec;

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
    fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }

    #[fixed_stack_segment]
    fn type_id() -> CFTypeID {
        unsafe {
            CFDictionaryGetTypeID()
        }
    }
}

// FIXME: Should be a newtype struct, but that fails due to a Rust compiler
// bug.
pub struct CFDictionary<KeyRefType, ValueRefType> {
    contents: CFWrapper<CFDictionaryRef, KeyRefType, ValueRefType>
}

pub type UntypedCFDictionary = CFDictionary<CFStringRef, CFTypeRef>;

impl<KeyRefType: Clone + AbstractCFTypeRef, ValueRefType: Clone + AbstractCFTypeRef>
         CFDictionary<KeyRefType, ValueRefType> {
    pub fn wrap_owned(dictionary: CFDictionaryRef) -> CFDictionary<KeyRefType, ValueRefType> {
        CFDictionary {
            contents: CFWrapper::wrap_owned(dictionary)
        }
    }

    #[fixed_stack_segment]
    pub fn new(pairs: &[(KeyRefType,ValueRefType)]) -> CFDictionary<KeyRefType, ValueRefType> {
        let mut keys : ~[CFTypeRef] = ~[];
        let mut values : ~[CFTypeRef] = ~[];
        for pair in pairs.iter() {
            // FIXME: "let" would be much nicer here, but that doesn't work yet.
            match *pair {
                (ref key, ref value) => {
                    keys.push(key.as_type_ref());
                    values.push(value.as_type_ref());
                }
            }
        }

        assert!(keys.len() == values.len());

        let dictionary_ref: CFDictionaryRef;
        unsafe {
            dictionary_ref = CFDictionaryCreate(kCFAllocatorDefault,
                                                cast::transmute::<*CFTypeRef, **c_void>(
                                                    vec::raw::to_ptr(keys)),
                                                cast::transmute::<*CFTypeRef, **c_void>(
                                                    vec::raw::to_ptr(values)),
                                                keys.len() as CFIndex,
                                                &kCFTypeDictionaryKeyCallBacks,
                                                &kCFTypeDictionaryValueCallBacks);
        }

        CFDictionary {
            contents: CFWrapper::wrap_owned(dictionary_ref)
        }
    }

    #[fixed_stack_segment]
    pub fn len(&self) -> uint {
        unsafe {
            CFDictionaryGetCount(self.contents.obj) as uint
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[fixed_stack_segment]
    pub fn contains_key(&self, key: &KeyRefType) -> bool {
        unsafe {
            CFDictionaryContainsKey(self.contents.obj, 
                                    cast::transmute::<CFTypeRef, *c_void>(key.as_type_ref()))
                                        as bool
        }
    }

    #[fixed_stack_segment]
    pub fn find(&self, key: &KeyRefType) -> Option<ValueRefType> {
        unsafe {
            let value : *c_void = ptr::null();
            let did_find_value = CFDictionaryGetValueIfPresent(
                self.contents.obj,
                cast::transmute::<CFTypeRef, *c_void>(key.as_type_ref()),
                cast::transmute::<&*c_void, **c_void>(&value)) as bool;

            // FIXME: this will not handle non-CF dictionary entries
            // or ptr::null() values correctly.
            if did_find_value {
                Some(cast::transmute::<*c_void, ValueRefType>(value))
            } else {
                None
            }
        }
    }

    pub fn get(&self, key: &KeyRefType) -> ValueRefType {
        let value = self.find(key);
        if value.is_none() {
            fail!(fmt!("No entry found for key: %?", key));
        }
        return value.unwrap();
    }

    // FIXME: this should be an iterator
    pub fn each(&self, blk: &fn(&KeyRefType, &ValueRefType) -> bool) -> bool {
        unsafe {
            let len = self.len();
            let null_keys = cast::transmute::<*c_void,KeyRefType>(ptr::null());
            let keys: ~[KeyRefType] = vec::from_elem(len, null_keys);
            let null_vals = cast::transmute::<*c_void,ValueRefType>(ptr::null());
            let values: ~[ValueRefType] = vec::from_elem(len, null_vals);

            for i in range(0, len) {
                if !blk(&keys[i], &values[i]) { return false; }
            }

            true
        }
    }
}


#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    /*
     * CFDictionary.h
     */

    static kCFTypeDictionaryKeyCallBacks: CFDictionaryKeyCallBacks;
    static kCFTypeDictionaryValueCallBacks: CFDictionaryValueCallBacks;

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

