// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! An immutable bag of elements.

use base::{Boolean, CFAllocatorRef, CFIndex, CFIndexConvertible, CFRelease, CFType, CFTypeID};
use base::{TCFType, kCFAllocatorDefault};

use std::cast;
use std::libc::c_void;
use std::vec;

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
    priv obj: CFSetRef,
}

impl Drop for CFSet {
    #[fixed_stack_segment]
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

    #[fixed_stack_segment]
    #[inline]
    fn type_id(_: Option<CFSet>) -> CFTypeID {
        unsafe {
            CFSetGetTypeID()
        }
    }
}

impl CFSet {
    /// Creates a new set from a list of `CFType` instances.
    #[fixed_stack_segment]
    pub fn from_slice(elems: &[CFType]) -> CFSet {
        unsafe {
            let elems = elems.map(|elem| elem.as_CFTypeRef());
            let set_ref = CFSetCreate(kCFAllocatorDefault,
                                      cast::transmute(vec::raw::to_ptr(elems)),
                                      elems.len().to_CFIndex(),
                                      &kCFTypeSetCallBacks);
            TCFType::wrap_under_create_rule(set_ref)
        }
    }
}

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    /*
     * CFSet.h
     */

    static kCFTypeSetCallBacks: CFSetCallBacks;
    static kCFTypeCopyStringSetCallBacks: CFSetCallBacks;

    /* Creating Sets */
    fn CFSetCreate(allocator: CFAllocatorRef, values: **c_void, numValues: CFIndex, 
                   callBacks: *CFSetCallBacks) -> CFSetRef;
    fn CFSetCreateCopy(allocator: CFAllocatorRef, theSet: CFSetRef) -> CFSetRef;

    /* Examining a Set */
    fn CFSetContainsValue(theSet: CFSetRef, value: *c_void) -> Boolean;
    fn CFSetGetCount(theSet: CFSetRef) -> CFIndex;
    fn CFSetGetCountOfValue(theSet: CFSetRef, value: *c_void) -> CFIndex;
    fn CFSetGetValue(theSet: CFSetRef, value: *c_void) -> *c_void;
    fn CFSetGetValueIfPresent(theSet: CFSetRef, candidate: *c_void, value: **c_void) -> Boolean;
    fn CFSetGetValues(theSet: CFSetRef, values: **c_void);

    /* Applying a Function to Set Members */
    //fn CFSetApplyFunction

    fn CFSetGetTypeID() -> CFTypeID;
}

