// Copyright 2013-2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base::{CFObject, CFType};
use sync::CFRef;

pub type CFLocaleRef = CFRef<CFLocale>;

#[repr(C)]
pub struct CFLocale { obj: CFObject }

unsafe impl Send for CFLocale {}
unsafe impl Sync for CFLocale {}

unsafe impl CFType for CFLocale {
    #[inline]
    fn as_object(&self) -> &CFObject {
        &self.obj
    }
}
