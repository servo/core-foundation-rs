// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base::{id, class};
use libc;
use std::ffi::CString;

#[cfg(target_pointer_width = "32")]
pub type NSInteger = libc::c_int;
#[cfg(target_pointer_width = "32")]
pub type NSUInteger = libc::c_uint;

#[cfg(target_pointer_width = "64")]
pub type NSInteger = libc::c_long;
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = libc::c_ulong;

#[repr(C)]
pub struct NSPoint {
    pub x: f64,
    pub y: f64,
}

impl NSPoint {
    #[inline]
    pub fn new(x: f64, y: f64) -> NSPoint {
        NSPoint {
            x: x,
            y: y,
        }
    }
}

#[repr(C)]
pub struct NSSize {
    pub width: f64,
    pub height: f64,
}

impl NSSize {
    #[inline]
    pub fn new(width: f64, height: f64) -> NSSize {
        NSSize {
            width: width,
            height: height,
        }
    }
}

#[repr(C)]
pub struct NSRect {
    pub origin: NSPoint,
    pub size: NSSize,
}

impl NSRect {
    #[inline]
    pub fn new(origin: NSPoint, size: NSSize) -> NSRect {
        NSRect {
            origin: origin,
            size: size
        }
    }
}

// Same as CGRectEdge
#[repr(u32)]
pub enum NSRectEdge {
    NSRectMinXEdge,
    NSRectMinYEdge,
    NSRectMaxXEdge,
    NSRectMaxYEdge,
}

#[link(name = "Foundation", kind = "framework")]
extern {
    pub static NSDefaultRunLoopMode: id;
}

pub trait NSAutoreleasePool {
    unsafe fn new(_: Self) -> id {
        msg_send![class("NSAutoreleasePool"), new]
    }

    unsafe fn autorelease(self) -> Self;
    unsafe fn drain(self);
}

impl NSAutoreleasePool for id {
    unsafe fn autorelease(self) -> id {
        msg_send![self, autorelease]
    }

    unsafe fn drain(self) {
        msg_send![self, drain]
    }
}

pub trait NSProcessInfo {
    unsafe fn processInfo(_: Self) -> id {
        msg_send![class("NSProcessInfo"), processInfo]
    }

    unsafe fn processName(self) -> id;
}

impl NSProcessInfo for id {
    unsafe fn processName(self) -> id {
        msg_send![self, processName]
    }
}

pub type NSTimeInterval = libc::c_double;

pub trait NSString {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class("NSString"), alloc]
    }

    unsafe fn initWithUTF8String_(self, c_string: *const i8) -> id;
    unsafe fn stringByAppendingString_(self, other: id) -> id;
    unsafe fn init_str(self, string: &str) -> Self;
    unsafe fn UTF8String(self) -> *const libc::c_char;
}

impl NSString for id {
    unsafe fn initWithUTF8String_(self, c_string: *const i8) -> id {
        msg_send![self, initWithUTF8String:c_string as id]
    }

    unsafe fn stringByAppendingString_(self, other: id) -> id {
        msg_send![self, stringByAppendingString:other]
    }

    unsafe fn init_str(self, string: &str) -> id {
        let cstring = CString::new(string).unwrap();
        self.initWithUTF8String_(cstring.as_ptr())
    }

    unsafe fn UTF8String(self) -> *const libc::c_char {
        msg_send![self, UTF8String]
    }
}

pub trait NSDate {
    unsafe fn distantPast(_: Self) -> id {
        msg_send![class("NSDate"), distantPast]
    }

    unsafe fn distantFuture(_: Self) -> id {
        msg_send![class("NSDate"), distantFuture]
    }
}

impl NSDate for id {

}
