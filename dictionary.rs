// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Dictionaries of key-value pairs.

use base::{Boolean, CFAllocatorRef, CFIndex, CFIndexConvertible, CFRelease, CFType, CFTypeID};
use base::{CFTypeRef, TCFType, kCFAllocatorDefault};

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

struct __CFDictionary;

pub type CFDictionaryRef = *__CFDictionary;

/// An immutable dictionary of key-value pairs.
///
/// FIXME(pcwalton): Should be a newtype struct, but that fails due to a Rust compiler bug.
pub struct CFDictionary {
    priv obj: CFDictionaryRef,
}

impl Drop for CFDictionary {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl TCFType<CFDictionaryRef> for CFDictionary {
    fn as_concrete_TypeRef(&self) -> CFDictionaryRef {
        self.obj
    }

    unsafe fn wrap_under_create_rule(obj: CFDictionaryRef) -> CFDictionary {
        CFDictionary {
            obj: obj,
        }
    }

    #[inline]
    fn type_id(_: Option<CFDictionary>) -> CFTypeID {
        unsafe {
            CFDictionaryGetTypeID()
        }
    }
}

impl CFDictionary {
    pub fn from_CFType_pairs(pairs: &[(CFType, CFType)]) -> CFDictionary {
        let (keys, values) =
            vec::unzip(pairs.iter()
                            .map(|&(ref key, ref value)| (key.as_CFTypeRef(),
                                                          value.as_CFTypeRef())));
        unsafe {
            let dictionary_ref = CFDictionaryCreate(kCFAllocatorDefault,
                                                    cast::transmute(keys.as_ptr()),
                                                    cast::transmute(values.as_ptr()),
                                                    keys.len().to_CFIndex(),
                                                    &kCFTypeDictionaryKeyCallBacks,
                                                    &kCFTypeDictionaryValueCallBacks);
            TCFType::wrap_under_create_rule(dictionary_ref)
        }
    }

    #[inline]
    pub fn len(&self) -> uint {
        unsafe {
            CFDictionaryGetCount(self.obj) as uint
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn contains_key(&self, key: *c_void) -> bool {
        unsafe {
            CFDictionaryContainsKey(self.obj, key) != 0
        }
    }

    #[inline]
    pub fn find(&self, key: *c_void) -> Option<*c_void> {
        unsafe {
            let mut value: *c_void = ptr::null();
            if CFDictionaryGetValueIfPresent(self.obj, key, &mut value) != 0 {
                Some(value)
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn get(&self, key: *c_void) -> *c_void {
        let value = self.find(key);
        if value.is_none() {
            fail!("No entry found for key: {:?}", key);
        }
        value.unwrap()
    }

    /// A convenience function to retrieve `CFType` instances.
    #[inline]
    pub unsafe fn get_CFType(&self, key: *c_void) -> CFType {
        let value: CFTypeRef = cast::transmute(self.get(key));
        TCFType::wrap_under_get_rule(value)
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern {
    /*
     * CFDictionary.h
     */

    static kCFTypeDictionaryKeyCallBacks: CFDictionaryKeyCallBacks;
    static kCFTypeDictionaryValueCallBacks: CFDictionaryValueCallBacks;

    fn CFDictionaryContainsKey(theDict: CFDictionaryRef, key: *c_void) -> Boolean;
    fn CFDictionaryCreate(allocator: CFAllocatorRef, keys: **c_void, values: **c_void,
                          numValues: CFIndex, keyCallBacks: *CFDictionaryKeyCallBacks,
                          valueCallBacks: *CFDictionaryValueCallBacks)
                       -> CFDictionaryRef;
    fn CFDictionaryGetCount(theDict: CFDictionaryRef) -> CFIndex;
    fn CFDictionaryGetTypeID() -> CFTypeID;
    fn CFDictionaryGetValueIfPresent(theDict: CFDictionaryRef, key: *c_void, value: *mut *c_void)
                                     -> Boolean;
}

