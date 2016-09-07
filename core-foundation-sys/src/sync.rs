// Copyright 2013-2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Encodes the Core Foundation refcounting machinery with Rust types.

use base::{CFRelease, CFRetain, CFType};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

#[repr(C)]
pub struct CFRef<T: CFType> { ptr: *const CFShared<T> }

impl<T: CFType> CFRef<T> {
    #[inline]
    pub unsafe fn from_retained(retained: *const CFShared<T>) -> Self {
        assert!(!retained.is_null());
        CFRef { ptr: retained }
    }

    #[inline]
    pub unsafe fn try_from_retained(
            retained: *const CFShared<T>)
            -> Result<Self, ()> {
        if !retained.is_null() {
            Ok(CFRef { ptr: retained })
        } else {
            Err(())
        }
    }
}

impl<T: CFType> Clone for CFRef<T> {
    #[inline]
    fn clone(&self) -> Self {
        self.retain()
    }
}

impl<T: CFType> Deref for CFRef<T> {
    type Target = CFShared<T>;

    #[inline]
    fn deref(&self) -> &CFShared<T> {
        unsafe { &*self.ptr }
    }
}

impl<T: CFType> Drop for CFRef<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_object()) }
    }
}

impl<T: CFType + fmt::Debug> fmt::Debug for CFRef<T> {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        (&**self).fmt(formatter)
    }
}

impl<T: CFType> Hash for CFRef<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_object().hash(state)
    }
}

impl<T: CFType + Eq> Eq for CFRef<T> {}

impl<T: CFType + PartialEq> PartialEq for CFRef<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (&**self).eq(&**other)
    }
}

impl<T: CFType + Ord> Ord for CFRef<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        (&**self).cmp(&**other)
    }
}

impl<T: CFType + PartialOrd> PartialOrd for CFRef<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (&**self).partial_cmp(&**other)
    }
}

unsafe impl<T: CFType> Send for CFRef<T> {}
unsafe impl<T: CFType> Sync for CFRef<T> {}

#[repr(C)]
pub struct CFShared<T: CFType> { contents: T }

impl<T: CFType> CFShared<T> {
    #[inline]
    pub fn retain(&self) -> CFRef<T> {
        unsafe {
            let ptr = CFRetain(&*(self as *const _ as *const _));
            assert!(!ptr.is_null());
            CFRef { ptr: ptr as *const _ as *const _ }
        }
    }
}

impl<T: CFType> Deref for CFShared<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.contents
    }
}

impl<T: CFType + fmt::Debug> fmt::Debug for CFShared<T> {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        (&**self).fmt(formatter)
    }
}

impl<T: CFType + Eq> Eq for CFShared<T> {}

impl<T: CFType + PartialEq> PartialEq for CFShared<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (&**self).eq(&**other)
    }
}

impl<T: CFType + Ord> Ord for CFShared<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        (&**self).cmp(&**other)
    }
}

impl<T: CFType + PartialOrd> PartialOrd for CFShared<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (&**self).partial_cmp(&**other)
    }
}

unsafe impl<T: CFType + Send> Send for CFShared<T> {}
unsafe impl<T: CFType + Sync> Sync for CFShared<T> {}
