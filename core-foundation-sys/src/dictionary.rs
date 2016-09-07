// Copyright 2013-2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use allocator::CFAllocator;
use base::{CFDowncast, CFObject, CFHashCode, CFIndex, CFType};
use base::{CFTypeID, FromCFIndex, IntoCFIndex};
use std::fmt;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use string::CFString;
use sync::{CFRef, CFShared};
use version::CFVersion0;

pub type CFDictionaryRef = CFRef<CFDictionary>;

#[repr(C)]
pub struct CFDictionary { obj: CFObject }

unsafe impl Send for CFDictionary {}
unsafe impl Sync for CFDictionary {}

unsafe impl CFType for CFDictionary {
    #[inline]
    fn as_object(&self) -> &CFObject {
        &self.obj
    }
}

unsafe impl CFDowncast for CFDictionary {
    #[inline]
    fn type_id() -> CFTypeID {
        unsafe { CFDictionaryGetTypeID() }
    }
}

impl CFDictionary {
    #[inline]
    pub fn from_objects<K, V>(
            keys: &[&CFShared<K>],
            values: &[&CFShared<V>])
            -> CFDictionaryRef
        where K: CFType, V: CFType
    {
        unsafe {
            CFDictionary::new(
                &*(keys as *const _ as *const _),
                &*(values as *const _ as *const _),
                Some(&kCFTypeDictionaryKeyCallBacks),
                Some(&kCFTypeDictionaryValueCallBacks))
        }
    }

    #[inline]
    pub fn from_duplicated_strings_and_objects<V>(
            keys: &[&CFShared<CFString>],
            values: &[&CFShared<V>])
            -> CFDictionaryRef
        where V: CFType
    {
        unsafe {
            CFDictionary::new(
                &*(keys as *const _ as *const _),
                &*(values as *const _ as *const _),
                Some(&kCFCopyStringDictionaryKeyCallBacks),
                Some(&kCFTypeDictionaryValueCallBacks))
        }
    }

    #[inline]
    pub unsafe fn new(
            keys: &[*const c_void],
            values: &[*const c_void],
            key_callbacks: Option<&CFDictionaryKeyCallBacks>,
            value_callbacks: Option<&CFDictionaryValueCallBacks>)
            -> CFDictionaryRef {
        assert!(keys.len() == values.len());
        CFRef::from_retained(
            CFDictionaryCreate(
                None,
                keys.as_ptr(),
                values.as_ptr(),
                keys.len().into_index(),
                key_callbacks,
                value_callbacks))
    }

    #[inline]
    pub fn duplicate(&self) -> CFDictionaryRef {
        unsafe {
            CFRef::from_retained(CFDictionaryCreateCopy(None, self))
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        unsafe { usize::from_index(CFDictionaryGetCount(self)) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub unsafe fn count_key(&self, key: *const c_void) -> usize {
        usize::from_index(CFDictionaryGetCountOfKey(self, key))
    }

    #[inline]
    pub unsafe fn count_value(&self, value: *const c_void) -> usize {
        usize::from_index(CFDictionaryGetCountOfValue(self, value))
    }

    #[inline]
    pub unsafe fn contains_key(&self, key: *const c_void) -> bool {
        CFDictionaryContainsKey(self, key)
    }

    #[inline]
    pub unsafe fn contains_value(&self, value: *const c_void) -> bool {
        CFDictionaryContainsValue(self, value)
    }

    #[inline]
    pub unsafe fn get(&self, key: *const c_void) -> Option<*const c_void> {
        let mut value = ptr::null();
        if CFDictionaryGetValueIfPresent(self, key, &mut value) {
            Some(value)
        } else {
            None
        }
    }

    #[inline]
    pub unsafe fn inspect<T>(
            &self,
            applier:
                unsafe extern fn(
                    key: *const c_void, value: *const c_void, context: &mut T),
            context: &mut T) {
        CFDictionaryApplyFunction(
            self, mem::transmute(applier), &mut *(context as *mut _ as *mut _));
    }
}

impl fmt::Debug for CFDictionary {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        formatter
            .debug_tuple("CFDictionary")
            .field(&format_args!("{:p}", self))
            .field(&self.len())
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct CFDictionaryKeyCallBacks {
    version: CFVersion0,
    retain: Option<CFDictionaryRetainCallBack>,
    release: Option<CFDictionaryReleaseCallBack>,
    copyDescription: Option<CFDictionaryCopyDescriptionCallBack>,
    equal: Option<CFDictionaryEqualCallBack>,
    hash: Option<CFDictionaryHashCallBack>,
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct CFDictionaryValueCallBacks {
    pub version: CFVersion0,
    pub retain: Option<CFDictionaryRetainCallBack>,
    pub release: Option<CFDictionaryReleaseCallBack>,
    pub copyDescription: Option<CFDictionaryCopyDescriptionCallBack>,
    pub equal: Option<CFDictionaryEqualCallBack>,
}

pub type CFDictionaryRetainCallBack =
    unsafe extern fn(
        allocator: Option<&'static CFAllocator>, value: *const c_void)
        -> *const c_void;

pub type CFDictionaryReleaseCallBack =
    unsafe extern fn(
        allocator: Option<&'static CFAllocator>, value: *const c_void);

pub type CFDictionaryCopyDescriptionCallBack =
    unsafe extern fn(
        value: *const c_void)
        -> Option<&'static CFShared<CFString>>;

pub type CFDictionaryEqualCallBack =
    unsafe extern fn(value1: *const c_void, value2: *const c_void) -> bool;

pub type CFDictionaryHashCallBack =
    unsafe extern fn(value: *const c_void) -> CFHashCode;

pub type CFDictionaryApplierFunction =
    unsafe extern fn(
            key: *const c_void, value: *const c_void, context: *mut c_void);

extern {
    pub static kCFTypeDictionaryKeyCallBacks: CFDictionaryKeyCallBacks;
    pub static kCFCopyStringDictionaryKeyCallBacks: CFDictionaryKeyCallBacks;
    pub static kCFTypeDictionaryValueCallBacks: CFDictionaryValueCallBacks;

    pub fn CFDictionaryGetTypeID() -> CFTypeID;

    pub fn CFDictionaryCreate(
            allocator: Option<&'static CFAllocator>,
            keys: *const *const c_void,
            values: *const *const c_void,
            numValues: CFIndex,
            keyCallBacks: Option<&CFDictionaryKeyCallBacks>,
            valueCallBacks: Option<&CFDictionaryValueCallBacks>)
            -> *const CFShared<CFDictionary>;

    pub fn CFDictionaryCreateCopy(
            allocator: Option<&'static CFAllocator>, theDict: &CFDictionary)
            -> *const CFShared<CFDictionary>;

    pub fn CFDictionaryGetCount(theDict: &CFDictionary) -> CFIndex;

    pub fn CFDictionaryGetCountOfKey(
            theDict: &CFDictionary, key: *const c_void)
            -> CFIndex;

    pub fn CFDictionaryGetCountOfValue(
            theDict: &CFDictionary, value: *const c_void)
            -> CFIndex;

    pub fn CFDictionaryContainsKey(
            theDict: &CFDictionary, key: *const c_void)
            -> bool;

    pub fn CFDictionaryContainsValue(
            theDict: &CFDictionary, value: *const c_void)
            -> bool;

    pub fn CFDictionaryGetValue(
            theDict: &CFDictionary, key: *const c_void)
            -> *const c_void;

    pub fn CFDictionaryGetValueIfPresent(
            theDict: &CFDictionary,
            key: *const c_void,
            value: &mut *const c_void)
            -> bool;

    pub fn CFDictionaryGetKeysAndValues(
            theDict: &CFDictionary,
            keys: *mut *const c_void,
            values: *mut *const c_void);

    pub fn CFDictionaryApplyFunction(
            theDict: &CFDictionary,
            applier: CFDictionaryApplierFunction,
            context: *mut c_void);
}
