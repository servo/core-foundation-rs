// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! An immutable bag of elements.

use core_foundation_sys::base::{CFAllocatorRef, CFIndex, CFRelease};
use core_foundation_sys::base::{CFTypeID, CFTypeRef, kCFAllocatorDefault};

use base::{CFIndexConvertible, TCFType};

use libc::c_void;
use std::mem;

pub type CFSetRetainCallBack = *const u8;
pub type CFSetReleaseCallBack = *const u8;
pub type CFSetCopyDescriptionCallBack = *const u8;
pub type CFSetEqualCallBack = *const u8;
pub type CFSetHashCallBack = *const u8;

#[allow(dead_code)]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CFSetCallBacks {
    version: CFIndex,
    retain: CFSetRetainCallBack,
    release: CFSetReleaseCallBack,
    copyDescription: CFSetCopyDescriptionCallBack,
    equal: CFSetEqualCallBack,
    hash: CFSetHashCallBack,
}

#[repr(C)]
struct __CFSet;

pub type CFSetRef = *const __CFSet;

/// An immutable bag of elements.
pub struct CFSet(CFSetRef);

impl Drop for CFSet {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl_TCFType!(CFSet, CFSetRef, CFSetGetTypeID);

impl CFSet {
    /// Creates a new set from a list of `CFType` instances.
    pub fn from_slice<R, T>(elems: &[T]) -> CFSet where T: TCFType<R> {
        unsafe {
            let elems: Vec<CFTypeRef> = elems.iter().map(|elem| elem.as_CFTypeRef()).collect();
            let set_ref = CFSetCreate(kCFAllocatorDefault,
                                      mem::transmute(elems.as_ptr()),
                                      elems.len().to_CFIndex(),
                                      &kCFTypeSetCallBacks);
            TCFType::wrap_under_create_rule(set_ref)
        }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern {
    /*
     * CFSet.h
     */

    static kCFTypeSetCallBacks: CFSetCallBacks;

    /* Creating Sets */
    fn CFSetCreate(allocator: CFAllocatorRef, values: *const *const c_void, numValues: CFIndex,
                   callBacks: *const CFSetCallBacks) -> CFSetRef;

    /* Applying a Function to Set Members */
    //fn CFSetApplyFunction

    fn CFSetGetTypeID() -> CFTypeID;
}

