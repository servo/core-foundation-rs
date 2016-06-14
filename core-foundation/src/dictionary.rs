// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Dictionaries of key-value pairs.

pub use core_foundation_sys::dictionary::*;
use core_foundation_sys::base::CFRelease;
use core_foundation_sys::base::{CFTypeRef, kCFAllocatorDefault};
use core_foundation_sys::string::CFStringRef;

use libc::c_void;
use std::mem;
use std::ptr;

use base::{CFType, CFIndexConvertible, TCFType};

/// An immutable dictionary of key-value pairs.
pub struct CFDictionary(pub CFDictionaryRef);

impl Drop for CFDictionary {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl_TCFType!(CFDictionary, CFDictionaryRef, CFDictionaryGetTypeID);

impl CFDictionary {
    pub fn from_CFType_pairs<R1, R2, K, V>(pairs: &[(K, V)]) -> CFDictionary
            where K: TCFType<R1>, V: TCFType<R2> {
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
            CFDictionaryGetCount(self.0) as usize
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn contains_key(&self, key: *const c_void) -> bool {
        unsafe {
            CFDictionaryContainsKey(self.0, key) != 0
        }
    }

    #[inline]
    pub fn find(&self, key: CFStringRef) -> Option<*const c_void> {
        unsafe {
            let mut value: *const c_void = ptr::null();
            if CFDictionaryGetValueIfPresent(self.0, key, &mut value) != 0 {
                Some(value)
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn get(&self, key: CFStringRef) -> *const c_void {
        let value = self.find(key);
        if value.is_none() {
            panic!("No entry found for key {:p}", key);
        }
        value.unwrap()
    }
    pub fn kvs(&self) -> (*const c_void, *const c_void) {
        unsafe {
            let mut values: *const c_void = ptr::null();
            let mut keys:   *const c_void = ptr::null();
            CFDictionaryGetKeysAndValues(self.0, &mut keys, &mut values);
            (keys, values)
        }
    }
    /// A convenience function to retrieve `CFType` instances.
    #[inline]
    pub unsafe fn get_CFType(&self, key: CFStringRef) -> CFType {
        let value: CFTypeRef = mem::transmute(self.get(key));
        TCFType::wrap_under_get_rule(value)
    }

    #[inline]
    pub unsafe fn set_value(&self, key: *const c_void, value: *const c_void) {
        CFDictionarySetValue(self.0, key, value)
    }
}
