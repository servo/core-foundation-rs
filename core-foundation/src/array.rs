// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Heterogeneous immutable arrays.

pub use core_foundation_sys::array::*;
pub use core_foundation_sys::base::{CFIndex, CFRelease};
use core_foundation_sys::base::{CFTypeRef, kCFAllocatorDefault};
use base::CFType;
use libc::c_void;
use std::mem;
use std::marker::PhantomData;

use base::{CFIndexConvertible, TCFType, CFRange};

/// A heterogeneous immutable array.
pub struct CFArray<T = *const c_void>(CFArrayRef, PhantomData<T>);

/// A trait describing how to convert from the stored *const c_void to the desired T
pub trait FromVoid<T> {
    fn from_void(x: *const c_void) -> T;
}

impl FromVoid<u32> for u32 {
    fn from_void(x: *const c_void) -> u32 {
        x as usize as u32
    }
}

impl FromVoid<*const c_void> for *const c_void {
    fn from_void(x: *const c_void) -> *const c_void {
        x
    }
}

impl FromVoid<CFType> for CFType {
    fn from_void(x: *const c_void) -> CFType {
        unsafe { TCFType::wrap_under_get_rule(mem::transmute(x)) }
    }
}

impl<T> Drop for CFArray<T> {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

pub struct CFArrayIterator<'a, T: 'a> {
    array: &'a CFArray<T>,
    index: CFIndex,
}

impl<'a, T: FromVoid<T>> Iterator for CFArrayIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.index >= self.array.len() {
            None
        } else {
            let value = self.array.get(self.index);
            self.index += 1;
            Some(value)
        }
    }
}

impl<'a, T: FromVoid<T>> ExactSizeIterator for CFArrayIterator<'a, T> {
    fn len(&self) -> usize {
        (self.array.len() - self.index) as usize
    }
}

impl_TCFTypeGeneric!(CFArray, CFArrayRef, CFArrayGetTypeID);
impl_CFTypeDescriptionGeneric!(CFArray);

impl<T> CFArray<T> {
    /// Creates a new `CFArray` with the given elements, which must be `CFType` objects.
    pub fn from_CFTypes<R>(elems: &[T]) -> CFArray<T> where T: TCFType<R> {
        unsafe {
            let elems: Vec<CFTypeRef> = elems.iter().map(|elem| elem.as_CFTypeRef()).collect();
            let array_ref = CFArrayCreate(kCFAllocatorDefault,
                                          mem::transmute(elems.as_ptr()),
                                          elems.len().to_CFIndex(),
                                          &kCFTypeArrayCallBacks);
            TCFType::wrap_under_create_rule(array_ref)
        }
    }

    pub fn to_untyped(self) -> CFArray {
        CFArray(self.0, PhantomData)
    }

    /// Iterates over the elements of this `CFArray`.
    ///
    /// Careful; the loop body must wrap the reference properly. Generally, when array elements are
    /// Core Foundation objects (not always true), they need to be wrapped with
    /// `TCFType::wrap_under_get_rule()`.
    #[inline]
    pub fn iter<'a>(&'a self) -> CFArrayIterator<'a, T> {
        CFArrayIterator {
            array: self,
            index: 0
        }
    }

    #[inline]
    pub fn len(&self) -> CFIndex {
        unsafe {
            CFArrayGetCount(self.0)
        }
    }

    #[inline]
    pub fn get(&self, index: CFIndex) -> T where T: FromVoid<T> {
        assert!(index < self.len());
        T::from_void(unsafe { CFArrayGetValueAtIndex(self.0, index) })
    }

    pub fn get_values(&self, range: CFRange) -> Vec<*const c_void> {
        let mut vec = Vec::with_capacity(range.length as usize);
        unsafe {
            CFArrayGetValues(self.0, range, vec.as_mut_ptr());
            vec.set_len(range.length as usize);
            vec
        }
    }

    pub fn get_all_values(&self) -> Vec<*const c_void> {
        self.get_values(CFRange {
            location: 0,
            length: self.len()
        })
    }
}

impl<'a, T: FromVoid<T>> IntoIterator for &'a CFArray<T> {
    type Item = T;
    type IntoIter = CFArrayIterator<'a, T>;

    fn into_iter(self) -> CFArrayIterator<'a, T> {
        self.iter()
    }
}

#[test]
fn should_box_and_unbox() {
    use number::{CFNumber, number};

    let n0 = number(0);
    let n1 = number(1);
    let n2 = number(2);
    let n3 = number(3);
    let n4 = number(4);
    let n5 = number(5);

    let arr = CFArray::from_CFTypes(&[
        n0.as_CFType(),
        n1.as_CFType(),
        n2.as_CFType(),
        n3.as_CFType(),
        n4.as_CFType(),
        n5.as_CFType(),
    ]);

    assert!(arr.get_all_values() == &[n0.as_CFTypeRef(),
                                      n1.as_CFTypeRef(),
                                      n2.as_CFTypeRef(),
                                      n3.as_CFTypeRef(),
                                      n4.as_CFTypeRef(),
                                      n5.as_CFTypeRef()]);

    unsafe {
        let mut sum = 0;

        let mut iter = arr.iter();
        assert_eq!(iter.len(), 6);
        assert!(iter.next().is_some());
        assert_eq!(iter.len(), 5);

        for elem in iter {
            let number: CFNumber = TCFType::wrap_under_get_rule(mem::transmute(elem));
            sum += number.to_i64().unwrap()
        }

        assert!(sum == 15);

        for elem in arr.iter() {
            let number: CFNumber = TCFType::wrap_under_get_rule(mem::transmute(elem));
            sum += number.to_i64().unwrap()
        }

        assert!(sum == 30);
    }
}
