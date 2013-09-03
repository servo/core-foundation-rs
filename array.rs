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
    CFAllocatorRef,
    CFIndex,
    CFTypeID,
    CFTypeRef,
    CFWrapper,
    kCFAllocatorDefault,
};
use std::cast;
use std::libc::c_void;
use std::ptr;
use std::vec;

pub type CFArrayRetainCallBack = *u8;
pub type CFArrayReleaseCallBack = *u8;
pub type CFArrayCopyDescriptionCallBack = *u8;
pub type CFArrayEqualCallBack = *u8;

pub struct CFArrayCallBacks {
    version: CFIndex,
    retain: CFArrayRetainCallBack,
    release: CFArrayReleaseCallBack,
    copyDescription: CFArrayCopyDescriptionCallBack,
    equal: CFArrayEqualCallBack,
}

struct __CFArray { private: () }
pub type CFArrayRef = *__CFArray;

impl AbstractCFTypeRef for CFArrayRef {
    fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }
    #[fixed_stack_segment]
    fn type_id(_dummy: Option<CFArrayRef>) -> CFTypeID { unsafe { CFArrayGetTypeID() } }
}

// FIXME: Should be a newtype struct, but that fails due to a Rust compiler bug.
pub struct CFArray<ElemRefType> {
    contents: CFWrapper<CFArrayRef, ElemRefType, ()>
}

pub struct CFArrayIterator<'self, ElemRefType> {
    priv array: &'self CFArray<ElemRefType>,
    priv index: uint,
}

impl<'self, ElemRefType: AbstractCFTypeRef> Iterator<ElemRefType> for CFArrayIterator<'self, ElemRefType> {
    fn next(&mut self) -> Option<ElemRefType> {
        if self.index >= self.array.len() {
            None
        } else {
            let v = self.array[self.index];
            self.index += 1;
            Some(v)
        }
    }
}

impl<ElemRefType:AbstractCFTypeRef> CFArray<ElemRefType> {
    pub fn wrap_shared(array: CFArrayRef) -> CFArray<ElemRefType> {
        CFArray {
            contents: CFWrapper::wrap_shared(array)
        }
    }

    pub fn wrap_owned(array: CFArrayRef) -> CFArray<ElemRefType> {
        CFArray {
            contents: CFWrapper::wrap_owned(array)
        }
    }

    #[fixed_stack_segment]
    pub fn new(elems: &[ElemRefType]) -> CFArray<ElemRefType> {
        let array_ref: CFArrayRef;
        let elems_refs = do elems.map |e: &ElemRefType| { e.as_type_ref() };

        unsafe {
            array_ref = CFArrayCreate(kCFAllocatorDefault,
                                      cast::transmute::<*CFTypeRef, **c_void>(vec::raw::to_ptr(elems_refs)),
                                      elems.len() as CFIndex,
                                      ptr::to_unsafe_ptr(&kCFTypeArrayCallBacks));
        }

        CFArray {
            contents: CFWrapper::wrap_owned(array_ref)
        }
    }

    // Careful; the loop body must wrap the reference properly.
    // Generally, when array elements are Core Foundation objects (not
    // always true), they need to be wrapped with CFWrapper::wrap_shared.
    pub fn iter<'t>(&'t self) -> CFArrayIterator<'t, ElemRefType> {
        CFArrayIterator {
            array: self,
            index: 0
        }
    }

    #[fixed_stack_segment]
    pub fn len(&self) -> uint {
        unsafe {
            return CFArrayGetCount(*self.contents.borrow_ref()) as uint;
        }
    }
}

impl<ElemRefType:AbstractCFTypeRef> Index<uint,ElemRefType> for CFArray<ElemRefType> {
    // Careful; the caller must wrap any returned reference properly.
    // Generally, when array elements are Core Foundation objects (not
    // always true), they need to be wrapped with CFWrapper::wrap_shared.
    #[fixed_stack_segment]
    fn index(&self, idx: &uint) -> ElemRefType {
        assert!(*idx < self.len());
        unsafe { 
            let elem = CFArrayGetValueAtIndex(*self.contents.borrow_ref(), *idx as CFIndex);
            // Don't return a wrapped thing, since we don't know whether
            // it needs base::wrap_shared() or base::wrap_owned()
            cast::transmute::<*c_void,ElemRefType>(elem)
        }
    }
}

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    /*
     * CFArray.h
     */
    static kCFTypeArrayCallBacks: CFArrayCallBacks;

    fn CFArrayCreate(allocator: CFAllocatorRef, values: **c_void,
                     numValues: CFIndex, callBacks: *CFArrayCallBacks) -> CFArrayRef;
    // CFArrayCreateCopy
    // CFArrayBSearchValues
    // CFArrayContainsValue
    fn CFArrayGetCount(theArray: CFArrayRef) -> CFIndex;
    // CFArrayGetCountOfValue
    // CFArrayGetFirstIndexOfValue
    // CFArrayGetLastIndexOfValue
    // CFArrayGetValues
    fn CFArrayGetValueAtIndex(theArray: CFArrayRef, idx: CFIndex) -> *c_void;
    // CFArrayApplyFunction
    fn CFArrayGetTypeID() -> CFTypeID;
}

#[test]
fn should_box_and_unbox() {
    use number::CFNumber;

    let one = CFNumber::new(1 as i32);
    let two = CFNumber::new(2 as i32);
    let thr = CFNumber::new(3 as i32);
    let fou = CFNumber::new(4 as i32);
    let fiv = CFNumber::new(5 as i32);

    let arr = CFArray::new([
        *one.contents.borrow_ref(),
        *two.contents.borrow_ref(),
        *thr.contents.borrow_ref(),
        *fou.contents.borrow_ref(),
        *fiv.contents.borrow_ref()
    ]);

    let mut sum = 0i32;

    for elem in arr.iter() {
        sum += CFNumber::wrap_shared(elem).to_i32();
    }

    assert!(sum == 15);

    for elem in arr.iter() {
        sum += CFNumber::wrap_shared(elem).to_i32();
    }

    assert!(sum == 30);
}
