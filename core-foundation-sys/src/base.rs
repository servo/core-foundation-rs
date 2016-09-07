// Copyright 2013-2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use allocator::CFAllocator;
use std::cmp::{Ordering, PartialEq};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::Range;
use std::os::raw::{c_ulong, c_void};
use string::{CFString, CFStringRef};
use sync::{CFRef, CFShared};

pub type CFObjectRef = CFRef<CFObject>;

#[repr(C)]
pub struct CFObject { opaque: [c_void; 0] }

unsafe impl Send for CFObject {}
unsafe impl Sync for CFObject {}

unsafe impl CFType for CFObject {
    #[inline]
    fn as_object(&self) -> &CFObject {
        self
    }
}

impl CFObject {
    /// Returns the type id of this object.
    #[inline]
    pub fn type_id(&self) -> CFTypeID {
        unsafe { CFGetTypeID(self) }
    }

    /// Returns the reference count of this object.
    #[inline]
    pub fn retain_count(&self) -> usize {
        unsafe { CFGetRetainCount(self) as usize }
    }

    /// Returns the CF hash of this object.
    #[inline]
    pub fn hash_code(&self) -> CFHashCode {
        unsafe { CFHash(self) }
    }

    /// Describes the type of this object.
    #[inline]
    pub fn describe(&self) -> CFStringRef {
        unsafe { CFRef::from_retained(CFCopyDescription(self)) }
    }

    /// Returns whether this object is an instance of a more specific CF type.
    #[inline]
    pub fn is<T: CFDowncast>(&self) -> bool {
        self.type_id() == T::type_id()
    }

    /// Downcasts this object to a more specific CF type.
    ///
    /// Also available as `<CFRef<T>>::downcast` and `<&CFShared<T>>::downcast`.
    #[inline]
    pub fn downcast<T: CFDowncast>(&self) -> Result<&T, ()> {
        if self.is::<T>() {
            Ok(unsafe { &*(self as *const _ as *const _) })
        } else {
            Err(())
        }
    }
}

impl CFShared<CFObject> {
    /// Downcasts this object to a more specific CF type.
    #[inline]
    pub fn downcast<T: CFDowncast>(&self) -> Result<&CFShared<T>, ()> {
        (&**self).downcast::<T>().map(|result| {
            unsafe { &*(result as *const _ as *const _) }
        })
    }
}

impl CFObjectRef {
    /// Downcasts this object to a more specific CF type.
    ///
    /// If the operation was not possible, `Err(self)` is returned to be
    /// able to get the original object back again.
    #[inline]
    pub fn downcast<T: CFDowncast>(self) -> Result<CFRef<T>, Self> {
        if self.is::<T>() {
            let result = unsafe {
                CFRef::from_retained(&*self as *const _ as *const _)
            };
            mem::forget(self);
            Ok(result)
        } else {
            Err(self)
        }
    }
}

impl fmt::Debug for CFObject {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        formatter
            .debug_tuple("CFObject")
            .field(&format_args!("{:p}", self))
            .finish()
    }
}

impl Hash for CFObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash_code().hash(state)
    }
}

impl PartialEq for CFObject {
    fn eq(&self, other: &Self) -> bool {
        unsafe { CFEqual(self, other) }
    }
}

/// Represents a Core Foundation type.
pub unsafe trait CFType {
    #[inline]
    fn as_object(&self) -> &CFObject;
}

impl<T: 'static + CFType> CFShared<T> {
    #[inline]
    pub fn as_shared_object(&self) -> &CFShared<CFObject> {
        unsafe { &*(self.as_object() as *const _ as *const _) }
    }
}

pub unsafe trait CFDowncast: CFType {
    #[inline]
    fn type_id() -> CFTypeID;
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct CFTypeID { id: c_ulong }

impl CFTypeID {
    pub fn describe(self) -> CFStringRef {
        unsafe { CFRef::from_retained(CFCopyTypeIDDescription(self)) }
    }
}

pub type CFOptionFlags = c_ulong;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct CFHashCode { id: c_ulong }

pub type CFIndex = i64;

pub const kCFNotFound: CFIndex = -1;

pub trait IntoCFIndex {
    #[inline]
    fn into_index(self) -> CFIndex;
}

impl IntoCFIndex for usize {
    #[inline]
    fn into_index(self) -> CFIndex {
        assert!(self as u64 <= CFIndex::max_value() as u64);
        self as CFIndex
    }
}

pub trait FromCFIndex {
    #[inline]
    fn from_index(input: CFIndex) -> Self;
}

impl FromCFIndex for usize {
    #[inline]
    fn from_index(input: CFIndex) -> Self {
        assert!(input >= 0);
        assert!(input as u64 <= usize::max_value() as u64);
        input as usize
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(i64)]
pub enum CFComparisonResult {
    Less = -1,
    Equal = 0,
    Greater = 1,
}

impl From<CFComparisonResult> for Ordering {
    #[inline]
    fn from(result: CFComparisonResult) -> Self {
        match result {
            CFComparisonResult::Less => Ordering::Less,
            CFComparisonResult::Equal => Ordering::Equal,
            CFComparisonResult::Greater => Ordering::Greater,
        }
    }
}

pub type CFComparatorFunction =
    unsafe extern fn(
        val1: *const c_void, val2: *const c_void, context: *mut c_void)
        -> CFComparisonResult;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct CFRange {
    pub location: CFIndex,
    pub length: CFIndex,
}

impl From<Range<usize>> for CFRange {
    #[inline]
    fn from(input: Range<usize>) -> Self {
        assert!(input.start < input.end);
        let location = input.start.into_index();
        CFRange {
            location: location,
            length: input.end.into_index() - location,
        }
    }
}

impl From<CFRange> for Range<usize> {
    #[inline]
    fn from(input: CFRange) -> Self {
        let start = usize::from_index(input.location);
        Range {
            start: start,
            end: start.checked_add(usize::from_index(input.length)).unwrap(),
        }
    }
}

extern {
    pub fn CFGetTypeID(cf: &CFObject) -> CFTypeID;

    pub fn CFCopyTypeIDDescription(
                type_id: CFTypeID)
                -> *const CFShared<CFString>;

    pub fn CFRetain(cf: &CFShared<CFObject>) -> *const CFShared<CFObject>;
    pub fn CFRelease(cf: *const CFObject);
    pub fn CFGetRetainCount(cf: &CFObject) -> CFIndex;
    pub fn CFEqual(cf1: &CFObject, cf2: &CFObject) -> bool;
    pub fn CFHash(cf: &CFObject) -> CFHashCode;
    pub fn CFCopyDescription(cf: &CFObject) -> *const CFShared<CFString>;
    pub fn CFGetAllocator(cf: &CFObject) -> *const CFAllocator;
}
