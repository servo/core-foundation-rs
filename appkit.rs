// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base::objc;
use base;

#[nolink]
#[link_args="-framework AppKit"]
pub extern mod appkit {
    fn NSBeep();
}

pub fn NSApp() -> base::id {
    unsafe {
        let klass = str::as_c_str(~"NSApplication", |s| objc::objc_getClass(s));
        let sel = str::as_c_str(~"sharedApplication", |s| objc::sel_registerName(s));
        objc::objc_msgSend(klass, sel)
    }
}

