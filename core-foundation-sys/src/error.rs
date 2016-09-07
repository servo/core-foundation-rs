// Copyright 2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use allocator::CFAllocator;
use base::{CFDowncast, CFIndex, CFObject, CFType, CFTypeID};
use dictionary::{CFDictionary, CFDictionaryRef};
use std::os::raw::c_void;
use string::{CFString, CFStringRef};
use sync::{CFRef, CFShared};

pub type CFErrorRef = CFRef<CFError>;

#[repr(C)]
pub struct CFError { obj: CFObject }

unsafe impl Send for CFError {}
unsafe impl Sync for CFError {}

unsafe impl CFType for CFError {
    #[inline]
    fn as_object(&self) -> &CFObject {
        &self.obj
    }
}

unsafe impl CFDowncast for CFError {
    #[inline]
    fn type_id() -> CFTypeID {
        unsafe { CFErrorGetTypeID() }
    }
}

impl CFError {
    #[inline]
    pub fn new(
            domain: &CFString, code: CFIndex, user_info: Option<&CFDictionary>)
            -> CFErrorRef {
        unsafe {
            CFRef::from_retained(
                CFErrorCreate(None, domain, code, user_info))
        }
    }

    #[inline]
    pub fn domain(&self) -> &CFShared<CFString> {
        unsafe { CFErrorGetDomain(self).unwrap() }
    }

    #[inline]
    pub fn code(&self) -> CFIndex {
        unsafe { CFErrorGetCode(self) }
    }

    #[inline]
    pub fn user_info(&self) -> CFDictionaryRef {
        unsafe { CFRef::from_retained(CFErrorCopyUserInfo(self)) }
    }

    #[inline]
    pub fn description(&self) -> CFStringRef {
        unsafe { CFRef::from_retained(CFErrorCopyDescription(self)) }
    }

    #[inline]
    pub fn failure_reason(&self) -> Option<CFStringRef> {
        unsafe { CFRef::try_from_retained(CFErrorCopyFailureReason(self)).ok() }
    }

    #[inline]
    pub fn recovery_suggestion(&self) -> Option<CFStringRef> {
        unsafe {
            CFRef::try_from_retained(CFErrorCopyRecoverySuggestion(self)).ok()
        }
    }
}

extern {
    pub fn CFErrorGetTypeID() -> CFTypeID;

    pub fn CFErrorCreate(
            allocator: Option<&'static CFAllocator>,
            domain: &CFString,
            code: CFIndex,
            userInfo: Option<&CFDictionary>)
            -> *const CFShared<CFError>;

    pub fn CFErrorCreateWithUserInfoKeysAndValues(
            allocator: Option<&'static CFAllocator>,
            domain: &CFShared<CFString>,
            code: CFIndex,
            userInfoKeys: *const *const c_void,
            userInfoValues: *const *const c_void,
            numUserInfoValues: CFIndex)
            -> *const CFShared<CFError>;

    pub fn CFErrorGetDomain(err: &CFError) -> Option<&CFShared<CFString>>;
    pub fn CFErrorGetCode(err: &CFError) -> CFIndex;
    pub fn CFErrorCopyUserInfo(err: &CFError) -> *const CFShared<CFDictionary>;
    pub fn CFErrorCopyDescription(err: &CFError) -> *const CFShared<CFString>;
    pub fn CFErrorCopyFailureReason(err: &CFError) -> *const CFShared<CFString>;

    pub fn CFErrorCopyRecoverySuggestion(
            err: &CFError)
            -> *const CFShared<CFString>;
}
