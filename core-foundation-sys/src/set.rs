// Copyright 2013-2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use allocator::CFAllocator;
use base::{CFDowncast, CFIndex, CFHashCode, CFObject, CFType, CFTypeID};
use base::{FromCFIndex, IntoCFIndex};
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use string::CFString;
use sync::{CFRef, CFShared};
use version::CFVersion0;

pub type CFSetRef = CFRef<CFSet>;

#[repr(C)]
pub struct CFSet { obj: CFObject } 

unsafe impl Send for CFSet {}
unsafe impl Sync for CFSet {}

unsafe impl CFType for CFSet {
    #[inline]
    fn as_object(&self) -> &CFObject {
        &self.obj
    }
}

unsafe impl CFDowncast for CFSet {
    #[inline]
    fn type_id() -> CFTypeID {
        unsafe { CFSetGetTypeID() }
    }
}

impl CFSet {
    #[inline]
    pub fn from_objects<T: CFType>(input: &[&CFShared<T>]) -> CFSetRef {
        unsafe {
            CFSet::new(
                &*(input as *const _ as *const _), Some(&kCFTypeSetCallBacks))
        }
    }

    #[inline]
    pub fn from_duplicated_strings(input: &[&CFShared<CFString>]) -> CFSetRef {
        unsafe {
            CFSet::new(
                &*(input as *const _ as *const _),
                Some(&kCFCopyStringSetCallBacks))
        }
    }

    #[inline]
    pub unsafe fn new(
            values: &[*const c_void], callbacks: Option<&CFSetCallBacks>)
            -> CFSetRef {
        CFRef::from_retained(
            CFSetCreate(
                None, values.as_ptr(), values.len().into_index(), callbacks))
    }

    #[inline]
    pub fn duplicate(&self) -> CFSetRef {
        unsafe { CFRef::from_retained(CFSetCreateCopy(None, self)) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        unsafe { usize::from_index(CFSetGetCount(self)) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub unsafe fn count_value(&self, value: *const c_void) -> usize {
        usize::from_index(CFSetGetCountOfValue(self, value))
    }

    #[inline]
    pub unsafe fn contains_value(&self, value: *const c_void) -> bool {
        CFSetContainsValue(self, value)
    }

    #[inline]
    pub unsafe fn get(&self, value: *const c_void) -> Option<*const c_void> {
        let mut result = ptr::null();
        if CFSetGetValueIfPresent(self, value, &mut result) {
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    pub fn values(&self) -> Vec<*const c_void> {
        let mut result = vec![ptr::null(); self.len()];
        unsafe { CFSetGetValues(self, result.as_mut_ptr()) }
        result
    }

    #[inline]
    pub unsafe fn inspect<T>(
            &self,
            applier: unsafe extern fn(value: *const c_void, context: &mut T),
            context: &mut T) {
        CFSetApplyFunction(
            self,
            mem::transmute(applier),
            &mut *(context as *mut _ as *mut _));
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct CFSetCallBacks {
    version: CFVersion0,
    retain: Option<CFSetRetainCallBack>,
    release: Option<CFSetReleaseCallBack>,
    copyDescription: Option<CFSetCopyDescriptionCallBack>,
    equal: Option<CFSetEqualCallBack>,
    hash: Option<CFSetHashCallBack>,
}

pub type CFSetRetainCallBack =
    unsafe extern fn(
        allocator: Option<&'static CFAllocator>, value: *const c_void)
        -> *const c_void;

pub type CFSetReleaseCallBack =
    unsafe extern fn(
        allocator: Option<&'static CFAllocator>, value: *const c_void);

pub type CFSetCopyDescriptionCallBack =
    unsafe extern fn(
        value: *const c_void)
        -> Option<&'static CFShared<CFString>>;

pub type CFSetEqualCallBack =
    unsafe extern fn(value1: *const c_void, value2: *const c_void) -> bool;

pub type CFSetHashCallBack =
    unsafe extern fn(value: *const c_void) -> CFHashCode;

pub type CFSetApplierFunction =
    unsafe extern fn(value: *const c_void, context: *mut c_void);

extern {
    pub static kCFTypeSetCallBacks: CFSetCallBacks;
    pub static kCFCopyStringSetCallBacks: CFSetCallBacks;

    pub fn CFSetGetTypeID() -> CFTypeID;

    pub fn CFSetCreate<'values>(
            allocator: Option<&'static CFAllocator>,
            values: *const *const c_void,
            numValues: CFIndex,
            callBacks: Option<&CFSetCallBacks>)
            -> *const CFShared<CFSet>;

    pub fn CFSetCreateCopy<'values>(
            allocator: Option<&'static CFAllocator>,
            theSet: &CFSet)
            -> *const CFShared<CFSet>;

    pub fn CFSetGetCount(theSet: &CFSet) -> CFIndex;

    pub fn CFSetGetCountOfValue(
            theSet: &CFSet, value: *const c_void)
            -> CFIndex;

    pub fn CFSetContainsValue(theSet: &CFSet, value: *const c_void) -> bool;
    pub fn CFSetGetValue(theSet: &CFSet, value: *const c_void) -> *const c_void;

    pub fn CFSetGetValueIfPresent(
            theSet: &CFSet, candidate: *const c_void, value: &mut *const c_void)
            -> bool;

    pub fn CFSetGetValues(theSet: &CFSet, values: *mut *const c_void);

    pub fn CFSetApplyFunction(
            theSet: &CFSet,
            applier: CFSetApplierFunction,
            context: *mut c_void);
}
