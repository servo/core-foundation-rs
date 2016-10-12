// Copyright 2013-2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Binds the `CFBoolean` type.

use base::{CFDowncast, CFObject, CFType, CFTypeID};
use std::cmp::{Eq, PartialEq};
use std::fmt;
use sync::{CFRef, CFShared};

pub type CFBooleanRef = CFRef<CFBoolean>;

/// Encapsulates boolean values.
#[repr(C)]
pub struct CFBoolean { obj: CFObject }

unsafe impl Send for CFBoolean {}
unsafe impl Sync for CFBoolean {}

unsafe impl CFType for CFBoolean {
    #[inline]
    fn as_object(&self) -> &CFObject {
        &self.obj
    }
}

unsafe impl CFDowncast for CFBoolean {
    #[inline]
    fn type_id() -> CFTypeID {
        unsafe { CFBooleanGetTypeID() }
    }
}

impl CFBoolean {
    /// Returns a static reference to one of the two `CFBoolean` values.
    ///
    /// Also available as `<&'static CFShared<CFBoolean> as From<bool>>::from`.
    #[inline]
    pub fn new(input: bool) -> &'static CFShared<CFBoolean> {
        if input {
            kCFBooleanTrue.unwrap()
        } else {
            kCFBooleanFalse.unwrap()
        }
    }

    /// Returns the raw boolean value of this instance of `CFBoolean`.
    ///
    /// Also available as `bool::from`.
    #[inline]
    pub fn to_bool(&self) -> bool {
        unsafe { CFBooleanGetValue(self) }
    }
}

impl<'a> From<&'a CFBoolean> for bool {
    #[inline]
    fn from(input: &'a CFBoolean) -> bool {
        input.to_bool()
    }
}

impl From<bool> for &'static CFShared<CFBoolean> {
    #[inline]
    fn from(input: bool) -> Self {
        CFBoolean::new(input)
    }
}

impl fmt::Debug for CFBoolean {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        formatter
            .debug_tuple("CFBoolean")
            .field(&self.to_bool())
            .finish()
    }
}

impl Default for &'static CFShared<CFBoolean> {
    #[inline]
    fn default() -> Self {
        kCFBooleanFalse.unwrap()
    }
}

impl Eq for CFBoolean {}

impl PartialEq for CFBoolean {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self as *const _ == other as *const _
    }
}

#[test]
fn test_conversions() {
    let cf_true = CFBoolean::new(true);
    assert_eq!(cf_true.to_bool(), true);

    let cf_false = CFBoolean::new(true);
    assert_eq!(cf_false.to_bool(), true);
}

#[test]
fn test_equality() {
    assert_eq!(CFBoolean::new(true), CFBoolean::new(true));
    assert_eq!(CFBoolean::new(false), CFBoolean::new(false));

    assert!(CFBoolean::new(true) != CFBoolean::new(false));
    assert!(CFBoolean::new(false) != CFBoolean::new(true));
}

extern {
    pub static kCFBooleanTrue: Option<&'static CFShared<CFBoolean>>;
    pub static kCFBooleanFalse: Option<&'static CFShared<CFBoolean>>;

    pub fn CFBooleanGetTypeID() -> CFTypeID;
    pub fn CFBooleanGetValue(boolean: &CFBoolean) -> bool;
}
