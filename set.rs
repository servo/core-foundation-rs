// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! An immutable bag of elements.

use base::{CFAllocatorRef, CFIndex, CFIndexConvertible, CFRelease, CFType, CFTypeID, CFTypeRef};
use base::{TCFType, kCFAllocatorDefault};

use libc::c_void;
use std::mem;

pub type CFSetRetainCallBack = *u8;
pub type CFSetReleaseCallBack = *u8;
pub type CFSetCopyDescriptionCallBack = *u8;
pub type CFSetEqualCallBack = *u8;
pub type CFSetHashCallBack = *u8;

pub struct CFSetCallBacks {
    version: CFIndex,
    retain: CFSetRetainCallBack,
    release: CFSetReleaseCallBack,
    copyDescription: CFSetCopyDescriptionCallBack,
    equal: CFSetEqualCallBack,
    hash: CFSetHashCallBack,
}

struct __CFSet;

pub type CFSetRef = *__CFSet;

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
    fn as_concrete_TypeRef(&self) -> CFSetRef {
        self.obj
    }

    unsafe fn wrap_under_create_rule(obj: CFSetRef) -> CFSet {
        CFSet {
            obj: obj,
        }
    }

    #[inline]
    fn type_id(_: Option<CFSet>) -> CFTypeID {
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
    fn CFSetCreate(allocator: CFAllocatorRef, values: **c_void, numValues: CFIndex, 
                   callBacks: *CFSetCallBacks) -> CFSetRef;

    /* Applying a Function to Set Members */
    //fn CFSetApplyFunction

    fn CFSetGetTypeID() -> CFTypeID;
}

