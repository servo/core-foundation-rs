// Copyright 2013-2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use allocator::CFAllocator;
use base::{CFDowncast, CFIndex, CFObject, CFOptionFlags, CFRange, CFType};
use base::{CFTypeID, FromCFIndex, IntoCFIndex};
use std::borrow::Borrow;
use std::fmt;
use std::ops::{Deref, Range};
use std::slice;
use sync::{CFRef, CFShared};

pub type CFDataRef = CFRef<CFData>;

#[repr(C)]
pub struct CFData { obj: CFObject }

unsafe impl Send for CFData {}
unsafe impl Sync for CFData {}

unsafe impl CFType for CFData {
    #[inline]
    fn as_object(&self) -> &CFObject {
        &self.obj
    }
}

unsafe impl CFDowncast for CFData {
    #[inline]
    fn type_id() -> CFTypeID {
        unsafe { CFDataGetTypeID() }
    }
}

impl CFData {
    #[inline]
    pub fn from_slice(input: &[u8]) -> CFDataRef {
        unsafe {
            CFRef::from_retained(
                CFDataCreate(
                    None, input.as_ptr(), input.len().into_index()))
        }
    }

    #[inline]
    pub fn from_static_slice(input: &'static [u8]) -> CFDataRef {
        unsafe {
            CFRef::from_retained(
                CFDataCreateWithBytesNoCopy(
                    None,
                    input.as_ptr(),
                    input.len().into_index(),
                    Some(CFAllocator::null_allocator())))
        }
    }

    #[inline]
    pub fn duplicate(&self) -> CFDataRef {
        unsafe { CFRef::from_retained(CFDataCreateCopy(None, self)) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        unsafe { usize::from_index(CFDataGetLength(self)) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn to_slice(&self) -> &[u8] {
        unsafe {
            let ptr = CFDataGetBytePtr(self);
            assert!(!ptr.is_null());
            slice::from_raw_parts(ptr, self.len())
        }
    }

    #[inline]
    pub fn find(
            &self, needle: &CFData, options: CFDataSearchFlags)
            -> Option<Range<usize>> {
        self.find_in_range(needle, options, 0..self.len())
    }

    #[inline]
    pub fn find_in_range(
            &self,
            needle: &CFData,
            options: CFDataSearchFlags,
            range: Range<usize>)
            -> Option<Range<usize>> {
        let len = self.len();
        assert!(range.end <= len);
        let result = unsafe { CFDataFind(self, needle, range.into(), options) };
        if result.location < 0 {
            None
        } else {
            Some(result.into())
        }
    }
}

impl fmt::Debug for CFData {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        formatter
            .debug_tuple("CFData")
            .field(&format_args!("{:p}", self))
            .field(&*self)
            .finish()
    }
}

impl Deref for CFData {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.to_slice()
    }
}

impl Borrow<[u8]> for CFData {
    #[inline]
    fn borrow(&self) -> &[u8] {
        self.to_slice()
    }
}

impl<'a> From<&'a [u8]> for CFDataRef {
    #[inline]
    fn from(input: &'a [u8]) -> Self {
        CFData::from_slice(input)
    }
}

bitflags! {
    #[repr(C)]
    pub flags CFDataSearchFlags: CFOptionFlags {
        const SEARCH_BACKWARDS = 1,
        const SEARCH_ANCHORED = 2,
    }
}

extern {
    pub fn CFDataGetTypeID() -> CFTypeID;

    pub fn CFDataCreate(
            allocator: Option<&'static CFAllocator>,
            bytes: *const u8,
            length: CFIndex)
            -> *const CFShared<CFData>;

    pub fn CFDataCreateWithBytesNoCopy(
            allocator: Option<&'static CFAllocator>,
            bytes: *const u8,
            length: CFIndex,
            bytesDeallocator: Option<&'static CFAllocator>)
            -> *const CFShared<CFData>;

    pub fn CFDataCreateCopy(
            allocator: Option<&'static CFAllocator>,
            theData: &CFData)
            -> *const CFShared<CFData>;

    pub fn CFDataGetLength(theData: &CFData) -> CFIndex;
    pub fn CFDataGetBytePtr(theData: &CFData) -> *const u8;
    pub fn CFDataGetBytes(theData: &CFData, range: CFRange, buffer: *mut u8);

    pub fn CFDataFind(
            theData: &CFData,
            dataToFind: &CFData,
            searchRange: CFRange,
            compareOptions: CFDataSearchFlags)
            -> CFRange;
}
