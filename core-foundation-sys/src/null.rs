// Copyright 2013-2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base::{CFDowncast, CFObject, CFType, CFTypeID};
use sync::{CFRef, CFShared};

pub type CFNullRef = CFRef<CFNull>;

#[repr(C)]
pub struct CFNull { obj: CFObject }

unsafe impl Send for CFNull {}
unsafe impl Sync for CFNull {}

unsafe impl CFType for CFNull {
    #[inline]
    fn as_object(&self) -> &CFObject {
        &self.obj
    }
}

unsafe impl CFDowncast for CFNull {
    #[inline]
    fn type_id() -> CFTypeID {
        unsafe { CFNullGetTypeID() }
    }
}

impl CFNull {
    #[inline]
    pub fn null() -> &'static CFShared<Self> {
        kCFNull.unwrap()
    }
}

extern {
    pub fn CFNullGetTypeID() -> CFTypeID;

    pub static kCFNull: Option<&'static CFShared<CFNull>>;
}
