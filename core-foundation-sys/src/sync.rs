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
use std::mem;
use std::ops::{Deref, DerefMut};

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

    fn into_mut(self) -> Result<CFMut<T>, Self> {
        if self.as_object().retain_count() == 1 {
            let result = CFMut { ptr: self.ptr as *const _ as *mut _ };
            mem::forget(self);
            Ok(result)
        } else {
            Err(self)
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

#[repr(C)]
pub struct CFMut<T: CFType> { ptr: *mut T }

impl<T: CFType> CFMut<T> {
    #[inline]
    pub unsafe fn from_retained(retained: *const CFShared<T>) -> Self {
        assert!(!retained.is_null());
        assert!((&*retained).as_object().retain_count() == 1);
        CFMut { ptr: retained as *const _ as *mut _ }
    }

    fn into_ref(self) -> CFRef<T> {
        let result = CFRef { ptr: self.ptr as *const _ as *const _ };
        mem::forget(self);
        result
    }
}

impl<T: CFType> Deref for CFMut<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<T: CFType> DerefMut for CFMut<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}

impl<T: CFType> Drop for CFMut<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_object()) }
    }
}

impl<T: CFType + fmt::Debug> fmt::Debug for CFMut<T> {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        (&**self).fmt(formatter)
    }
}

impl<T: CFType> Hash for CFMut<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_object().hash(state)
    }
}

impl<T: CFType + Eq> Eq for CFMut<T> {}

impl<T: CFType + PartialEq> PartialEq for CFMut<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (&**self).eq(&**other)
    }
}

impl<T: CFType + Ord> Ord for CFMut<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        (&**self).cmp(&**other)
    }
}

impl<T: CFType + PartialOrd> PartialOrd for CFMut<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (&**self).partial_cmp(&**other)
    }
}

unsafe impl<T: CFType> Send for CFMut<T> {}
unsafe impl<T: CFType> Sync for CFMut<T> {}
