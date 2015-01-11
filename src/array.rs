// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Heterogeneous immutable arrays.

use base::{CFAllocatorRef, CFIndex, CFIndexConvertible, CFRelease, CFRetain};
use base::{CFType, CFTypeID, CFTypeRef, TCFType};
use base::{kCFAllocatorDefault};
use libc::c_void;
use std::mem;
use std::num::ToPrimitive;

/// FIXME(pcwalton): This is wrong.
pub type CFArrayRetainCallBack = *const u8;

/// FIXME(pcwalton): This is wrong.
pub type CFArrayReleaseCallBack = *const u8;

/// FIXME(pcwalton): This is wrong.
pub type CFArrayCopyDescriptionCallBack = *const u8;

/// FIXME(pcwalton): This is wrong.
pub type CFArrayEqualCallBack = *const u8;

#[allow(dead_code)]
#[repr(C)]
#[deriving(Copy)]
pub struct CFArrayCallBacks {
    version: CFIndex,
    retain: CFArrayRetainCallBack,
    release: CFArrayReleaseCallBack,
    copyDescription: CFArrayCopyDescriptionCallBack,
    equal: CFArrayEqualCallBack,
}

#[repr(C)]
struct __CFArray;

pub type CFArrayRef = *const __CFArray;

/// A heterogeneous immutable array.
///
/// FIXME(pcwalton): Should be a newtype struct, but that fails due to a Rust compiler bug.
pub struct CFArray {
    obj: CFArrayRef,
}

impl Drop for CFArray {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

pub struct CFArrayIterator<'a> {
    array: &'a CFArray,
    index: CFIndex,
}

impl<'a> Iterator for CFArrayIterator<'a> {
    type Item = *const c_void;

    fn next(&mut self) -> Option<*const c_void> {
        if self.index >= self.array.len() {
            None
        } else {
            let value = self.array.get(self.index);
            self.index += 1;
            Some(value)
        }
    }
}

impl TCFType<CFArrayRef> for CFArray {
    #[inline]
    fn as_concrete_TypeRef(&self) -> CFArrayRef {
        self.obj
    }

    #[inline]
    unsafe fn wrap_under_get_rule(reference: CFArrayRef) -> CFArray {
        let reference: CFArrayRef = mem::transmute(CFRetain(mem::transmute(reference)));
        TCFType::wrap_under_create_rule(reference)
    }

    #[inline]
    fn as_CFTypeRef(&self) -> CFTypeRef {
        unsafe {
            mem::transmute(self.as_concrete_TypeRef())
        }
    }

    #[inline]
    unsafe fn wrap_under_create_rule(obj: CFArrayRef) -> CFArray {
        CFArray {
            obj: obj,
        }
    }

    #[inline]
    fn type_id(_: Option<CFArray>) -> CFTypeID {
        unsafe {
            CFArrayGetTypeID()
        }
    }
}

impl CFArray {
    /// Creates a new `CFArray` with the given elements, which must be `CFType` objects.
    pub fn from_CFTypes(elems: &[CFType]) -> CFArray {
        unsafe {
            let elems: Vec<CFTypeRef> = elems.iter().map(|elem| elem.as_CFTypeRef()).collect();
            let array_ref = CFArrayCreate(kCFAllocatorDefault,
                                          mem::transmute(elems.as_ptr()),
                                          elems.len().to_CFIndex(),
                                          &kCFTypeArrayCallBacks);
            TCFType::wrap_under_create_rule(array_ref)
        }
    }

    /// Iterates over the elements of this `CFArray`.
    ///
    /// Careful; the loop body must wrap the reference properly. Generally, when array elements are
    /// Core Foundation objects (not always true), they need to be wrapped with
    /// `TCFType::wrap_under_get_rule()`. The safer `iter_CFTypes` method will do this for you.
    #[inline]
    pub fn iter<'a>(&'a self) -> CFArrayIterator<'a> {
        CFArrayIterator {
            array: self,
            index: 0
        }
    }

    #[inline]
    pub fn len(&self) -> CFIndex {
        unsafe {
            CFArrayGetCount(self.obj)
        }
    }

    #[inline]
    pub fn get(&self, index: CFIndex) -> *const c_void {
        assert!(index < self.len());
        unsafe {
            CFArrayGetValueAtIndex(self.obj, index)
        }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern {
    /*
     * CFArray.h
     */
    static kCFTypeArrayCallBacks: CFArrayCallBacks;

    fn CFArrayCreate(allocator: CFAllocatorRef, values: *const *const c_void,
                     numValues: CFIndex, callBacks: *const CFArrayCallBacks) -> CFArrayRef;
    // CFArrayCreateCopy
    // CFArrayBSearchValues
    // CFArrayContainsValue
    fn CFArrayGetCount(theArray: CFArrayRef) -> CFIndex;
    // CFArrayGetCountOfValue
    // CFArrayGetFirstIndexOfValue
    // CFArrayGetLastIndexOfValue
    // CFArrayGetValues
    fn CFArrayGetValueAtIndex(theArray: CFArrayRef, idx: CFIndex) -> *const c_void;
    // CFArrayApplyFunction
    fn CFArrayGetTypeID() -> CFTypeID;
}

#[test]
fn should_box_and_unbox() {
    use number::{CFNumber, number};

    let arr = CFArray::from_CFTypes(&[
        number(1).as_CFType(),
        number(2).as_CFType(),
        number(3).as_CFType(),
        number(4).as_CFType(),
        number(5).as_CFType(),
    ]);

    unsafe {
        let mut sum = 0i32;

        for elem in arr.iter() {
            let number: CFNumber = TCFType::wrap_under_get_rule(mem::transmute(elem));
            sum += number.to_i32().unwrap()
        }

        assert!(sum == 15);

        for elem in arr.iter() {
            let number: CFNumber = TCFType::wrap_under_get_rule(mem::transmute(elem));
            sum += number.to_i32().unwrap()
        }

        assert!(sum == 30);
    }
}
