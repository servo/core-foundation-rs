// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! An immutable bag of elements.

use base::{CFAllocatorRef, CFIndex, CFIndexConvertible, CFRelease, CFRetain};
use base::{CFType, CFTypeID, CFTypeRef, TCFType, kCFAllocatorDefault};

use libc::c_void;
use std::mem;

pub type CFSetRetainCallBack = *const u8;
pub type CFSetReleaseCallBack = *const u8;
pub type CFSetCopyDescriptionCallBack = *const u8;
pub type CFSetEqualCallBack = *const u8;
pub type CFSetHashCallBack = *const u8;

#[allow(dead_code)]
#[repr(C)]
#[derive(Copy)]
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
///
/// FIXME(pcwalton): Should be a newtype struct, but that fails due to a Rust compiler bug.
pub struct CFSet {
    obj: CFSetRef,
}

impl Drop for CFSet {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl TCFType<CFSetRef> for CFSet {
    #[inline]
    fn as_concrete_TypeRef(&self) -> CFSetRef {
        self.obj
    }

    #[inline]
    unsafe fn wrap_under_get_rule(reference: CFSetRef) -> CFSet {
        let reference: CFSetRef = mem::transmute(CFRetain(mem::transmute(reference)));
        TCFType::wrap_under_create_rule(reference)
    }

    #[inline]
    fn as_CFTypeRef(&self) -> CFTypeRef {
        unsafe {
            mem::transmute(self.as_concrete_TypeRef())
        }
    }

    unsafe fn wrap_under_create_rule(obj: CFSetRef) -> CFSet {
        CFSet {
            obj: obj,
        }
    }

    #[inline]
    fn type_id() -> CFTypeID {
        unsafe {
            CFSetGetTypeID()
        }
    }
}

impl CFSet {
    /// Creates a new set from a list of `CFType` instances.
    pub fn from_slice(elems: &[CFType]) -> CFSet {
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

