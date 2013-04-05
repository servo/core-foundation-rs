// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base::{
    AbstractCFTypeRef,
    Boolean,
    CFAllocatorRef,
    CFIndex,
    CFTypeRef,
    CFTypeID,
    CFWrapper,
    kCFAllocatorDefault,
};
use core::libc::c_void;

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

struct __CFSet { private: () }
pub type CFSetRef = *__CFSet;

impl AbstractCFTypeRef for CFSetRef {
    fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }

    fn type_id() -> CFTypeID {
        unsafe {
            CFSetGetTypeID()
        }
    }
}

pub type CFSet<ElemRefType> = CFWrapper<CFSetRef, ElemRefType, ()>;

pub impl<ElemRefType : AbstractCFTypeRef>
    CFSet<ElemRefType> {
    fn new(elems: &[ElemRefType]) -> CFSet<ElemRefType> {
        let result : CFSetRef;
        let elems_refs = do vec::map(elems) |e: &ElemRefType| { e.as_type_ref() };

        unsafe {
            result = CFSetCreate(kCFAllocatorDefault,
                                  cast::transmute::<*CFTypeRef,**c_void>(vec::raw::to_ptr(elems_refs)),
                                  elems.len() as CFIndex,
                                  ptr::to_unsafe_ptr(&kCFTypeSetCallBacks));
        }
        CFWrapper::wrap_owned(result)
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

