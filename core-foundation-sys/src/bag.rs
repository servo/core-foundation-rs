// Copyright 2013-2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Binds the `CFBag` type.

use allocator::CFAllocator;
use base::{CFDowncast, CFIndex, CFHashCode, CFObject, CFType, CFTypeID};
use std::os::raw::c_void;
use string::CFString;
use sync::{CFRef, CFShared};

pub type CFBagRef = CFRef<CFBag>;

#[repr(C)]
pub struct CFBag { obj: CFObject }

unsafe impl Send for CFBag {}
unsafe impl Sync for CFBag {}

unsafe impl CFType for CFBag {
    #[inline]
    fn as_object(&self) -> &CFObject {
        &self.obj
    }
}

unsafe impl CFDowncast for CFBag {
    #[inline]
    fn type_id() -> CFTypeID {
        unsafe { CFBagGetTypeID() }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct CFBagCallBacks {
    version: CFIndex,
    retain: Option<CFBagRetainCallBack>,
    release: Option<CFBagReleaseCallBack>,
    copyDescription: Option<CFBagCopyDescriptionCallBack>,
    equal: Option<CFBagEqualCallBack>,
    hash: Option<CFBagHashCallBack>,
}

pub type CFBagRetainCallBack =
    unsafe extern fn(
        allocator: Option<&'static CFAllocator>, value: *const c_void)
        -> *const c_void;

pub type CFBagReleaseCallBack =
    unsafe extern fn(
        allocator: Option<&'static CFAllocator>, value: *const c_void);

pub type CFBagCopyDescriptionCallBack =
    unsafe extern fn(
        value: *const c_void)
        -> Option<&'static CFShared<CFString>>;

pub type CFBagEqualCallBack =
    unsafe extern fn(value1: *const c_void, value2: *const c_void) -> bool;

pub type CFBagHashCallBack =
    unsafe extern fn(value: *const c_void) -> CFHashCode;

pub type CFBagApplierFunction =
    unsafe extern fn(value: *const c_void, context: *mut c_void);

extern {
    pub static kCFTypeBagCallBacks: CFBagCallBacks;
    pub static kCFCopyStringBagCallBacks: CFBagCallBacks;

    pub fn CFBagGetTypeID() -> CFTypeID;

    pub fn CFBagCreate(
            allocator: Option<&'static CFAllocator>,
            values: *const *const c_void,
            numValues: CFIndex,
            callBacks: Option<&CFBagCallBacks>)
            -> *const CFShared<CFBag>;

    pub fn CFBagCreateCopy(
            allocator: Option<&'static CFAllocator>, theBag: &CFBag)
            -> *const CFShared<CFBag>;

    pub fn CFBagGetCount(theBag: &CFBag) -> CFIndex;

    pub fn CFBagGetCountOfValue(
            theBag: &CFBag, value: *const c_void)
            -> CFIndex;

    pub fn CFBagContainsValue(theBag: &CFBag, value: *const c_void) -> bool;
    pub fn CFBagGetValue(theBag: &CFBag, value: *const c_void) -> *const c_void;

    pub fn CFBagGetValueIfPresent(
            theBag: &CFBag,
            candidate: *const c_void,
            value: &mut *const c_void)
            -> bool;

    pub fn CFBagGetValues(theBag: &CFBag, values: *mut *const c_void);

    pub fn CFBagApplyFunction(
            theBag: &CFBag,
            applier: CFBagApplierFunction,
            context: *mut c_void);
}
