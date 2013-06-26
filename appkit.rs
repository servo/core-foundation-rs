// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base;
use std::str;

pub type CGFloat = f32;

pub struct NSPoint {
    x: float,
    y: float,
}

impl NSPoint {
    pub fn new(x: float, y: float) -> NSPoint {
        NSPoint {
            x: x,
            y: y,
        }
    }
}

pub struct NSSize {
    width: float,
    height: float,
}

impl NSSize {
    pub fn new(width: float, height: float) -> NSSize {
        NSSize {
            width: width,
            height: height,
        }
    }
}

pub struct NSRect {
    origin: NSPoint,
    size: NSSize,
}

#[nolink]
#[link_args="-framework AppKit"]
extern {
    fn NSBeep();
}

pub fn NSApp() -> base::id {
    unsafe {
        let klass = str::as_c_str("NSApplication", |s| base::objc_getClass(s));
        let sel = str::as_c_str("sharedApplication", |s| base::sel_registerName(s));
        base::objc_msgSend(klass, sel)
    }
}

