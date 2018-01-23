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
use libc::c_void;
use std::mem;
use std::marker::PhantomData;
use std;
use std::ops::Deref;

use base::{CFIndexConvertible, TCFType, CFRange};

/// A heterogeneous immutable array.
pub struct CFArray<T = *const c_void>(CFArrayRef, PhantomData<T>);

/// A trait describing how to convert from the stored *const c_void to the desired T
pub unsafe trait FromVoid {
    unsafe fn from_void(x: *const c_void) -> Self;
}

unsafe impl FromVoid for u32 {
    unsafe fn from_void(x: *const c_void) -> u32 {
        x as usize as u32
    }
}

unsafe impl<T> FromVoid for *const T {
    unsafe fn from_void(x: *const c_void) -> Self {
        x as *const T
    }
}

unsafe impl<T: TCFType> FromVoid for T where T::Ref: FromVoid {
    unsafe fn from_void(x: *const c_void) -> Self {
        TCFType::wrap_under_get_rule(T::Ref::from_void(x))
    }
}

pub struct TCFTypeBorrow<'a, T: 'a>(T, PhantomData<&'a T>);

impl<'a, T> Deref for TCFTypeBorrow<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
/// A trait describing how to convert from the stored *const c_void to the desired T
pub unsafe trait FromVoidBorrow {
    unsafe fn from_void_borrow<'a>(x: *const c_void) -> TCFTypeBorrow<'a, Self> where Self: std::marker::Sized;
}

unsafe impl FromVoidBorrow for u32 {
    unsafe fn from_void_borrow<'a>(x: *const c_void) -> TCFTypeBorrow<'a, Self> {
        TCFTypeBorrow(x as usize as u32, PhantomData)
    }
}

unsafe impl<T> FromVoidBorrow for *const T {
    unsafe fn from_void_borrow<'a>(x: *const c_void) -> TCFTypeBorrow<'a, Self> {
        TCFTypeBorrow(x as *const T, PhantomData)
    }
}

unsafe impl<T: TCFType> FromVoidBorrow for T where T::Ref: FromVoidBorrow {
    unsafe fn from_void_borrow<'a>(x: *const c_void) -> TCFTypeBorrow<'a, Self> {
        TCFTypeBorrow(TCFType::wrap_under_create_rule(T::Ref::from_void_borrow(x).0), PhantomData)
    }
}

impl<T> Drop for CFArray<T> {
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_CFTypeRef()) }
    }
}

pub struct CFArrayIterator<'a, T: 'a> {
    array: &'a CFArray<T>,
    index: CFIndex,
}

impl<'a, T: FromVoidBorrow> Iterator for CFArrayIterator<'a, T> {
    type Item = TCFTypeBorrow<'a, T>;

    fn next(&mut self) -> Option<TCFTypeBorrow<'a, T>> {
        if self.index >= self.array.len() {
            None
        } else {
            let value = self.array.get_borrow(self.index);
            self.index += 1;
            Some(value)
        }
    }
}

impl<'a, T: FromVoidBorrow> ExactSizeIterator for CFArrayIterator<'a, T> {
    fn len(&self) -> usize {
        (self.array.len() - self.index) as usize
    }
}

impl_TCFTypeGeneric!(CFArray, CFArrayRef, CFArrayGetTypeID);
impl_CFTypeDescriptionGeneric!(CFArray);

impl<T> CFArray<T> {
    /// Creates a new `CFArray` with the given elements, which must be `CFType` objects.
    pub fn from_CFTypes(elems: &[T]) -> CFArray<T> where T: TCFType {
        unsafe {
            let elems: Vec<CFTypeRef> = elems.iter().map(|elem| elem.as_CFTypeRef()).collect();
            let array_ref = CFArrayCreate(kCFAllocatorDefault,
                                          mem::transmute(elems.as_ptr()),
                                          elems.len().to_CFIndex(),
                                          &kCFTypeArrayCallBacks);
            TCFType::wrap_under_create_rule(array_ref)
        }
    }

    pub fn as_untyped(&self) -> CFArray {
        unsafe { CFArray::wrap_under_get_rule(self.0) }
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
    pub fn get(&self, index: CFIndex) -> T where T: FromVoid {
        assert!(index < self.len());
        unsafe { T::from_void(CFArrayGetValueAtIndex(self.0, index)) }
    }

    #[inline]
    pub fn get_borrow<'a>(&'a self, index: CFIndex) -> TCFTypeBorrow<'a, T> where T: FromVoidBorrow {
        assert!(index < self.len());
        unsafe { T::from_void_borrow(CFArrayGetValueAtIndex(self.0, index)) }
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

impl<'a, T: FromVoidBorrow> IntoIterator for &'a CFArray<T> {
    type Item = TCFTypeBorrow<'a, T>;
    type IntoIter = CFArrayIterator<'a, T>;

    fn into_iter(self) -> CFArrayIterator<'a, T> {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use base::CFType;

    #[test]
    fn as_untyped_correct_retain_count() {
        let array = CFArray::<CFType>::from_CFTypes(&[]);
        assert_eq!(array.retain_count(), 1);

        let untyped_array = array.as_untyped();
        assert_eq!(array.retain_count(), 2);
        assert_eq!(untyped_array.retain_count(), 2);

        mem::drop(array);
        assert_eq!(untyped_array.retain_count(), 1);
    }

    #[test]
    fn borrow() {
        use number::CFNumber;

        let n0 = CFNumber::from(0);
        let n1 = CFNumber::from(1);

        let arr: CFArray<CFNumber> = CFArray::from_CFTypes(&[
            n0,
            n1
        ]);
        let p = arr.get_borrow(0);
        assert_eq!(p.to_i64().unwrap(), 0)
    }

    #[test]
    fn should_box_and_unbox() {
        use number::CFNumber;

        let n0 = CFNumber::from(0);
        let n1 = CFNumber::from(1);
        let n2 = CFNumber::from(2);
        let n3 = CFNumber::from(3);
        let n4 = CFNumber::from(4);
        let n5 = CFNumber::from(5);

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

        let mut sum = 0;

        let mut iter = arr.iter();
        assert_eq!(iter.len(), 6);
        assert!(iter.next().is_some());
        assert_eq!(iter.len(), 5);

        for elem in iter {
            let number: CFNumber = elem.downcast::<CFNumber>().unwrap();
            sum += number.to_i64().unwrap()
        }

        assert!(sum == 15);

        for elem in arr.iter() {
            let number: CFNumber = elem.downcast::<CFNumber>().unwrap();
            sum += number.to_i64().unwrap()
        }

        assert!(sum == 30);
    }
}
