// Copyright 2013-2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base::{CFDowncast, CFIndex, CFObject, CFOptionFlags, CFType, CFTypeID};
use std::fmt;
use std::os::raw::c_void;
use string::CFString;
use sync::{CFShared, CFRef};
use version::CFVersion0;

pub type CFAllocatorRef = CFRef<CFAllocator>;

#[repr(C)]
pub struct CFAllocator(CFObject);

unsafe impl Send for CFAllocator {}
unsafe impl Sync for CFAllocator {}

unsafe impl CFType for CFAllocator {
    #[inline]
    fn as_object(&self) -> &CFObject {
        &self.0
    }
}

unsafe impl CFDowncast for CFAllocator {
    #[inline]
    fn type_id() -> CFTypeID {
        unsafe { CFAllocatorGetTypeID() }
    }
}

impl CFAllocator {
    #[inline]
    pub fn null_allocator() -> &'static CFShared<CFAllocator> {
        kCFAllocatorNull.unwrap()
    }
}

impl fmt::Debug for CFAllocator {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        formatter
            .debug_tuple("CFAllocator")
            .field(&format_args!("{:p}", self))
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct CFAllocatorContext {
    pub version: CFVersion0,
    pub info: *mut c_void,
    pub retain: Option<CFAllocatorRetainCallBack>,
    pub release: Option<CFAllocatorRetainCallBack>,
    pub copyDescription: Option<CFAllocatorCopyDescriptionCallBack>,
    pub allocate: CFAllocatorAllocateCallBack,
    pub reallocate: Option<CFAllocatorReallocateCallBack>,
    pub deallocate: Option<CFAllocatorDeallocateCallBack>,
    pub preferredSize: CFAllocatorPreferredSizeCallBack,
}

pub type CFAllocatorRetainCallBack =
    unsafe extern fn(info: *const c_void) -> *const c_void;

pub type CFAllocatorReleaseCallBack =
    unsafe extern fn(info: *const c_void);

pub type CFAllocatorCopyDescriptionCallBack =
    unsafe extern fn(
        info: *const c_void)
        -> Option<&'static CFShared<CFString>>;

pub type CFAllocatorAllocateCallBack =
    unsafe extern fn(
        allocSize: CFIndex, hint: CFSizeHint, info: *mut c_void)
        -> *mut c_void;

pub type CFAllocatorReallocateCallBack =
    unsafe extern fn(
        ptr: *mut c_void,
        newsize: CFIndex,
        hint: CFSizeHint,
        info: *mut c_void)
        -> *mut c_void;

pub type CFAllocatorDeallocateCallBack =
    unsafe extern fn(ptr: *mut c_void, info: *mut c_void);

pub type CFAllocatorPreferredSizeCallBack =
    unsafe extern fn(
        size: CFIndex, hint: CFSizeHint, info: *mut c_void) -> CFIndex;

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct CFSizeHint { flags: CFOptionFlags }

impl CFSizeHint {
    #[inline]
    pub fn new() -> Self {
        CFSizeHint { flags: 0 }
    }
}

extern {
    pub static kCFAllocatorNull: Option<&'static CFShared<CFAllocator>>;

    pub fn CFAllocatorGetTypeID() -> CFTypeID;
    pub fn CFAllocatorSetDefault(allocator: &'static CFAllocator);
    pub fn CFAllocatorGetDefault() -> Option<&'static CFShared<CFAllocator>>;

    pub fn CFAllocatorCreate(
            allocator: Option<&'static CFAllocator>,
            context: &'static mut CFAllocatorContext)
            -> *const CFShared<CFAllocator>;

    pub fn CFAllocatorAllocate(
            allocator: Option<&'static CFAllocator>,
            size: CFIndex,
            hint: CFSizeHint)
            -> *mut c_void;

    pub fn CFAllocatorReallocate(
            allocator: Option<&'static CFAllocator>,
            ptr: *mut c_void,
            newsize: CFIndex,
            hint: CFSizeHint)
            -> *mut c_void;

    pub fn CFAllocatorDeallocate(
            allocator: Option<&'static CFAllocator>, ptr: *mut c_void);

    pub fn CFAllocatorGetPreferredSizeForSize(
            allocator: Option<&'static CFAllocator>,
            size: CFIndex,
            hint: CFSizeHint)
            -> CFIndex;

    pub fn CFAllocatorGetContext(
            allocator: Option<&'static CFAllocator>,
            context: &mut CFAllocatorContext);
}
