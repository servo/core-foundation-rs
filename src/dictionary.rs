// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Dictionaries of key-value pairs.

use base::{Boolean, CFAllocatorRef, CFIndex, CFIndexConvertible, CFRelease, CFRetain};
use base::{CFType, CFTypeID, CFTypeRef, TCFType, kCFAllocatorDefault};

use libc::c_void;
use std::mem;
use std::ptr;

pub type CFDictionaryApplierFunction = *const u8;
pub type CFDictionaryCopyDescriptionCallBack = *const u8;
pub type CFDictionaryEqualCallBack = *const u8;
pub type CFDictionaryHashCallBack = *const u8;
pub type CFDictionaryReleaseCallBack = *const u8;
pub type CFDictionaryRetainCallBack = *const u8;

#[allow(dead_code)]
#[repr(C)]
#[derive(Copy)]
pub struct CFDictionaryKeyCallBacks {
    version: CFIndex,
    retain: CFDictionaryRetainCallBack,
    release: CFDictionaryReleaseCallBack,
    copyDescription: CFDictionaryCopyDescriptionCallBack,
    equal: CFDictionaryEqualCallBack,
    hash: CFDictionaryHashCallBack
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Copy)]
pub struct CFDictionaryValueCallBacks {
    version: CFIndex,
    retain: CFDictionaryRetainCallBack,
    release: CFDictionaryReleaseCallBack,
    copyDescription: CFDictionaryCopyDescriptionCallBack,
    equal: CFDictionaryEqualCallBack
}

#[repr(C)]
struct __CFDictionary;

pub type CFDictionaryRef = *const __CFDictionary;

/// An immutable dictionary of key-value pairs.
///
/// FIXME(pcwalton): Should be a newtype struct, but that panics due to a Rust compiler bug.
pub struct CFDictionary {
    obj: CFDictionaryRef,
}

impl Drop for CFDictionary {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl TCFType<CFDictionaryRef> for CFDictionary {
    #[inline]
    fn as_concrete_TypeRef(&self) -> CFDictionaryRef {
        self.obj
    }

    #[inline]
    unsafe fn wrap_under_get_rule(reference: CFDictionaryRef) -> CFDictionary {
        let reference: CFDictionaryRef = mem::transmute(CFRetain(mem::transmute(reference)));
        TCFType::wrap_under_create_rule(reference)
    }

    #[inline]
    fn as_CFTypeRef(&self) -> CFTypeRef {
        unsafe {
            mem::transmute(self.as_concrete_TypeRef())
        }
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
        let (keys, values): (Vec<CFTypeRef>,Vec<CFTypeRef>) =
            pairs.iter()
            .map(|&(ref key, ref value)| (key.as_CFTypeRef(), value.as_CFTypeRef()))
            .unzip();

        unsafe {
            let dictionary_ref = CFDictionaryCreate(kCFAllocatorDefault,
                                                    mem::transmute(keys.as_ptr()),
                                                    mem::transmute(values.as_ptr()),
                                                    keys.len().to_CFIndex(),
                                                    &kCFTypeDictionaryKeyCallBacks,
                                                    &kCFTypeDictionaryValueCallBacks);
            TCFType::wrap_under_create_rule(dictionary_ref)
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        unsafe {
            CFDictionaryGetCount(self.obj) as usize
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn contains_key(&self, key: *const c_void) -> bool {
        unsafe {
            CFDictionaryContainsKey(self.obj, key) != 0
        }
    }

    #[inline]
    pub fn find(&self, key: *const c_void) -> Option<*const c_void> {
        unsafe {
            let mut value: *const c_void = ptr::null();
            if CFDictionaryGetValueIfPresent(self.obj, key, &mut value) != 0 {
                Some(value)
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn get(&self, key: *const c_void) -> *const c_void {
        let value = self.find(key);
        if value.is_none() {
            panic!("No entry found for key {:p}", key);
        }
        value.unwrap()
    }

    /// A convenience function to retrieve `CFType` instances.
    #[inline]
    pub unsafe fn get_CFType(&self, key: *const c_void) -> CFType {
        let value: CFTypeRef = mem::transmute(self.get(key));
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

    fn CFDictionaryContainsKey(theDict: CFDictionaryRef, key: *const c_void) -> Boolean;
    fn CFDictionaryCreate(allocator: CFAllocatorRef, keys: *const *const c_void, values: *const *const c_void,
                          numValues: CFIndex, keyCallBacks: *const CFDictionaryKeyCallBacks,
                          valueCallBacks: *const CFDictionaryValueCallBacks)
                       -> CFDictionaryRef;
    fn CFDictionaryGetCount(theDict: CFDictionaryRef) -> CFIndex;
    fn CFDictionaryGetTypeID() -> CFTypeID;
    fn CFDictionaryGetValueIfPresent(theDict: CFDictionaryRef, key: *const c_void, value: *mut *const c_void)
                                     -> Boolean;
}
