// Copyright 2013-2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Binds the `CFArray` type.

use allocator::CFAllocator;
use base::{CFComparatorFunction, CFComparisonResult, CFDowncast, CFIndex};
use base::{CFObject, CFRange, CFType, CFTypeID, FromCFIndex, IntoCFIndex};
use std::fmt;
use std::mem;
use std::ops::Range;
use std::os::raw::c_void;
use std::ptr;
use string::CFString;
use sync::{CFRef, CFShared};
use version::CFVersion0;

pub type CFArrayRef = CFRef<CFArray>;

/// Encapsulate array values.
///
/// Most of the methods on this type are unsafe because it doesn't abstract
/// over the underlying `*const c_void` values passed around in the Core
/// Foundation framework.
#[repr(C)]
pub struct CFArray { obj: CFObject }

unsafe impl Send for CFArray {}
unsafe impl Sync for CFArray {}

unsafe impl CFType for CFArray {
    #[inline]
    fn as_object(&self) -> &CFObject {
        &self.obj
    }
}

unsafe impl CFDowncast for CFArray {
    #[inline]
    fn type_id() -> CFTypeID {
        unsafe { CFArrayGetTypeID() }
    }
}

impl CFArray {
    #[inline]
    pub fn from_objects<T: CFType>(input: &[&CFShared<T>]) -> CFArrayRef {
        unsafe {
            CFArray::new(
                &*(input as *const _ as *const _), Some(&kCFTypeArrayCallBacks))
        }
    }

    #[inline]
    pub unsafe fn new(
            values: &[*const c_void], callbacks: Option<&CFArrayCallBacks>)
            -> CFArrayRef {
        CFRef::from_retained(
            CFArrayCreate(
                None, values.as_ptr(), values.len().into_index(), callbacks))
    }

    #[inline]
    pub fn duplicate(&self) -> CFArrayRef {
        unsafe { CFRef::from_retained(CFArrayCreateCopy(None, self)) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        unsafe { usize::from_index(CFArrayGetCount(self)) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub unsafe fn count_value(&self, value: *const c_void) -> usize {
        self.count_value_in_range(value, 0..self.len())
    }

    #[inline]
    pub unsafe fn count_value_in_range(
            &self, value: *const c_void, range: Range<usize>)
            -> usize {
        let len = self.len();
        assert!(range.end <= len);
        usize::from_index(CFArrayGetCountOfValue(self, range.into(), value))
    }

    pub unsafe fn contains_value(&self, value: *const c_void) -> bool {
        self.contains_value_in_range(value, 0..self.len())
    }

    #[inline]
    pub unsafe fn contains_value_in_range(
            &self, value: *const c_void, range: Range<usize>)
            -> bool {
        CFArrayContainsValue(self, range.into(), value)
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<*const c_void> {
        if index < self.len() {
            Some(unsafe { CFArrayGetValueAtIndex(self, index as i64) })
        } else {
            None
        }
    }

    #[inline]
    pub fn values(&self) -> Vec<*const c_void> {
        self.values_in_range(0..self.len())
    }

    #[inline]
    pub fn values_in_range(&self, range: Range<usize>) -> Vec<*const c_void> {
        let len = self.len();
        assert!(range.end <= len);
        let mut output = vec![ptr::null(); self.len()];
        unsafe { CFArrayGetValues(self, range.into(), output.as_mut_ptr()) }
        output
    }

    #[inline]
    pub unsafe fn inspect<T>(
            &self,
            applier: unsafe extern fn(value: *const c_void, context: &mut T),
            context: &mut T) {
        self.inspect_in_range(applier, context, 0..self.len())
    }

    #[inline]
    pub unsafe fn inspect_in_range<T>(
            &self,
            applier: unsafe extern fn(value: *const c_void, context: &mut T),
            context: &mut T,
            range: Range<usize>) {
        let len = self.len();
        assert!(range.end <= len);
        CFArrayApplyFunction(
            self,
            range.into(),
            mem::transmute(applier),
            &mut *(context as *mut _ as *mut _));
    }

    #[inline]
    pub fn iter(&self) -> Iter {
        self.into_iter()
    }

    #[inline]
    pub unsafe fn position(&self, value: *const c_void) -> Option<usize> {
        self.position_in_range(value, 0..self.len())
    }

    #[inline]
    pub unsafe fn position_in_range(
            &self, value: *const c_void, range: Range<usize>)
            -> Option<usize> {
        let len = self.len();
        assert!(range.end <= len);
        let index = CFArrayGetFirstIndexOfValue(self, range.into(), value);
        if index < 0 {
            None
        } else {
            Some(usize::from_index(index))
        }
    }

    #[inline]
    pub unsafe fn rposition(&self, value: *const c_void) -> Option<usize> {
        self.position_in_range(value, 0..self.len())
    }

    #[inline]
    pub unsafe fn rposition_in_range(
            &self, value: *const c_void, range: Range<usize>)
            -> Option<usize> {
        let len = self.len();
        assert!(range.end <= len);
        let index = CFArrayGetLastIndexOfValue(self, range.into(), value);
        if index < 0 {
            None
        } else {
            Some(usize::from_index(index))
        }
    }

    #[inline]
    pub unsafe fn binary_search<T>(
            &self,
            value: *const c_void,
            comparator:
                unsafe extern fn(
                    value1: *const c_void,
                    value2: *const c_void,
                    context: &mut T)
                    -> CFComparisonResult,
            context: &mut T)
            -> usize {
        self.binary_search_in_range(value, comparator, context, 0..self.len())
    }

    #[inline]
    pub unsafe fn binary_search_in_range<T>(
            &self,
            value: *const c_void,
            comparator:
                unsafe extern fn(
                    value1: *const c_void,
                    value2: *const c_void,
                    context: &mut T)
                    -> CFComparisonResult,
            context: &mut T,
            range: Range<usize>)
            -> usize {
        let len = self.len();
        assert!(range.end <= len);
        let index = CFArrayBSearchValues(
            self,
            range.into(),
            value,
            mem::transmute(comparator),
            &mut *(context as *mut _ as *mut _));
        assert!(index >= 0);
        usize::from_index(index)
    }

    #[inline]
    pub fn iter_in_range(&self, range: Range<usize>) -> Iter {
        let len = self.len();
        assert!(range.start <= len);
        assert!(range.end <= len);
        Iter {
            array: self,
            range: range,
        }
    }
}

impl fmt::Debug for CFArray {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        formatter
            .debug_tuple("CFArray")
            .field(&(self as *const _))
            .field(&self.iter())
            .finish()
    }
}

impl<'a, T: CFType> From<&'a [&'a CFShared<T>]> for CFArrayRef {
    #[inline]
    fn from(input: &'a [&'a CFShared<T>]) -> Self {
        CFArray::from_objects(input)
    }
}

impl<'a> From<&'a CFArray> for Vec<*const c_void> {
    fn from(input: &'a CFArray) -> Self {
        input.values()
    }
}

macro_rules! into_iter_impl {
    ($name:ident for $type_:ty) => {
        into_iter_impl!($name[] for $type_);
    };
    ($name:ident<$($lt:tt),*> for $type_:ty) => {
        into_iter_impl!($name[$($lt),*] for $type_);
    };
    ($name:ident[$($lt:tt),*] for $type_:ty) => {
        impl<$($lt),*> IntoIterator for $type_ {
            type Item = *const c_void;
            type IntoIter = $name<$($lt),*>;

            #[inline]
            fn into_iter(self) -> $name<$($lt),*> {
                let len = self.len();
                $name {
                    array: self,
                    range: 0..len,
                }
            }
        }

        #[derive(Clone)]
        pub struct $name<$($lt),*> {
            array: $type_,
            range: Range<usize>,
        }

        impl<$($lt),*> Iterator for $name<$($lt),*> {
            type Item = *const c_void;

            #[inline]
            fn next(&mut self) -> Option<*const c_void> {
                self.range.next().map(|index| {
                    self.array.get(index).unwrap()
                })
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.range.size_hint()
            }
        }

        impl<$($lt),*> DoubleEndedIterator for $name<$($lt),*> {
            #[inline]
            fn next_back(&mut self) -> Option<*const c_void> {
                self.range.next_back().map(|index| {
                    self.array.get(index).unwrap()
                })
            }
        }

        impl<$($lt),*> ExactSizeIterator for $name<$($lt),*> {
            #[inline]
            fn len(&self) -> usize {
                self.range.len()
            }
        }

        impl<$($lt),*> fmt::Debug for $name<$($lt),*> {
            fn fmt(
                    &self, formatter: &mut fmt::Formatter)
                    -> Result<(), fmt::Error> {
                formatter.debug_list().entries(self.clone()).finish()
            }
        }
    };
}

into_iter_impl!(Iter<'a> for &'a CFArray);
into_iter_impl!(IntoIter for CFArrayRef);

#[derive(Default)]
#[repr(C)]
pub struct CFArrayCallBacks {
    pub version: CFVersion0,
    pub retain: Option<CFArrayRetainCallBack>,
    pub release: Option<CFArrayReleaseCallBack>,
    pub copyDescription: Option<CFArrayCopyDescriptionCallBack>,
    pub equal: Option<CFArrayEqualCallBack>,
}

pub type CFArrayRetainCallBack =
    unsafe extern fn(allocator: &CFAllocator, value: *const c_void)
                     -> *const c_void;

pub type CFArrayReleaseCallBack =
    unsafe extern fn(allocator: &CFAllocator, value: *const c_void);

pub type CFArrayCopyDescriptionCallBack =
    unsafe extern fn(
        value: *const c_void)
        -> Option<&'static CFShared<CFString>>;

pub type CFArrayEqualCallBack =
    unsafe extern fn(value1: *const c_void, value2: *const c_void) -> bool;

pub type CFArrayApplierFunction =
    unsafe extern fn(value: *const c_void, context: *mut c_void);

#[test]
fn test_should_box_and_unbox() {
    use base::CFObject;
    use number::CFNumber;
    use sync::CFShared;

    let arr = CFArray::from_objects(&[
        &CFNumber::from_i64(1),
        &CFNumber::from_i64(2),
        &CFNumber::from_i64(3),
        &CFNumber::from_i64(4),
        &CFNumber::from_i64(5),
    ]);

    unsafe {
        let mut sum = 0;

        for elem in arr.iter() {
            let object = &*(elem as *const CFShared<CFObject>);
            let number = object.downcast::<CFNumber>().unwrap();
            sum += number.to_i64().unwrap();
        }

        assert!(sum == 15);

        for elem in arr.iter() {
            let object = &*(elem as *const CFShared<CFObject>);
            let number = object.downcast::<CFNumber>().unwrap();
            sum += number.to_i64().unwrap();
        }

        assert!(sum == 30);
    }
}

extern {
    pub static kCFTypeArrayCallBacks: CFArrayCallBacks;

    pub fn CFArrayGetTypeID() -> CFTypeID;

    pub fn CFArrayCreate(
            allocator: Option<&'static CFAllocator>,
            values: *const *const c_void,
            numValues: CFIndex,
            callBacks: Option<&CFArrayCallBacks>)
            -> *const CFShared<CFArray>;

    pub fn CFArrayCreateCopy(
            allocator: Option<&'static CFAllocator>, theArray: &CFArray)
            -> *const CFShared<CFArray>;

    pub fn CFArrayGetCount(theArray: &CFArray) -> CFIndex;

    pub fn CFArrayGetCountOfValue(
            theArray: &CFArray, range: CFRange, value: *const c_void)
            -> CFIndex;

    pub fn CFArrayContainsValue(
            theArray: &CFArray, range: CFRange, value: *const c_void) -> bool;

    pub fn CFArrayGetValueAtIndex(
            theArray: &CFArray, idx: CFIndex)
            -> *const c_void;

    pub fn CFArrayGetValues(
            theArray: &CFArray, range: CFRange, values: *mut *const c_void);

    pub fn CFArrayApplyFunction(
            theArray: &CFArray,
            range: CFRange,
            applier: CFArrayApplierFunction,
            context: *mut c_void);

    pub fn CFArrayGetFirstIndexOfValue(
            theArray: &CFArray, range: CFRange, value: *const c_void)
            -> CFIndex;

    pub fn CFArrayGetLastIndexOfValue(
            theArray: &CFArray, range: CFRange, value: *const c_void)
            -> CFIndex;

    pub fn CFArrayBSearchValues(
            theArray: &CFArray,
            range: CFRange,
            value: *const c_void,
            comparator: CFComparatorFunction,
            context: *mut c_void)
            -> CFIndex;
}
