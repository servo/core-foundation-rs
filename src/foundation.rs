// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base::{id, class, BOOL, nil};
use libc;
use objc;

#[cfg(target_pointer_width = "32")]
pub type NSInteger = libc::c_int;
#[cfg(target_pointer_width = "32")]
pub type NSUInteger = libc::c_uint;

#[cfg(target_pointer_width = "64")]
pub type NSInteger = libc::c_long;
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = libc::c_ulong;

const UTF8_ENCODING: usize = 4;

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

unsafe impl objc::Encode for NSPoint {
    fn encode() -> objc::Encoding {
        let encoding = format!("{{CGPoint={}{}}}",
                               f64::encode().as_str(),
                               f64::encode().as_str());
        unsafe { objc::Encoding::from_str(&encoding) }
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

unsafe impl objc::Encode for NSSize {
    fn encode() -> objc::Encoding {
        let encoding = format!("{{CGSize={}{}}}",
                               f64::encode().as_str(),
                               f64::encode().as_str());
        unsafe { objc::Encoding::from_str(&encoding) }
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

#[repr(C)]
pub struct NSRange {
    pub location: NSUInteger,
    pub length: NSUInteger,
}

impl NSRange {
    #[inline]
    pub fn new(location: NSUInteger, length: NSUInteger) -> NSRange {
        NSRange {
            location: location,
            length: length
        }
    }
}

unsafe impl objc::Encode for NSRect {
    fn encode() -> objc::Encoding {
        let encoding = format!("{{CGRect={}{}}}",
                               NSPoint::encode().as_str(),
                               NSSize::encode().as_str());
        unsafe { objc::Encoding::from_str(&encoding) }
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

pub trait NSValue {
    unsafe fn valueWithPoint(_: Self, point: NSPoint) -> id {
        msg_send![class("NSValue"), valueWithPoint:point]
    }

    unsafe fn valueWithSize(_: Self, size: NSSize) -> id {
        msg_send![class("NSValue"), valueWithSize:size]
    }
}

impl NSValue for id {
}

pub trait NSArray {
    unsafe fn array(_: Self) -> id {
        msg_send![class("NSArray"), array]
    }

    unsafe fn arrayWithObjects(_: Self, objects: &[id]) -> id {
        msg_send![class("NSArray"), arrayWithObjects:objects.as_ptr()
                                    count:objects.len()]
    }

    unsafe fn arrayWithObject(_: Self, object: id) -> id {
        msg_send![class("NSArray"), arrayWithObject:object]
    }

    unsafe fn arrayByAddingObjectFromArray(self, object: id) -> id;
    unsafe fn arrayByAddingObjectsFromArray(self, objects: id) -> id;
}

impl NSArray for id {
    unsafe fn arrayByAddingObjectFromArray(self, object: id) -> id {
        msg_send![self, arrayByAddingObjectFromArray:object]
    }

    unsafe fn arrayByAddingObjectsFromArray(self, objects: id) -> id {
        msg_send![self, arrayByAddingObjectsFromArray:objects]
    }
}

pub trait NSString {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class("NSString"), alloc]
    }

    unsafe fn stringByAppendingString_(self, other: id) -> id;
    unsafe fn init_str(self, string: &str) -> Self;
    unsafe fn UTF8String(self) -> *const libc::c_char;
    unsafe fn len(self) -> usize;
    unsafe fn isEqualToString(self, &str) -> bool;
}

impl NSString for id {
    unsafe fn isEqualToString(self, other: &str) -> bool {
        let other = NSString::alloc(nil).init_str(other);
        let rv: BOOL = msg_send![self, isEqualToString:other];
        rv != 0
    }

    unsafe fn stringByAppendingString_(self, other: id) -> id {
        msg_send![self, stringByAppendingString:other]
    }

    unsafe fn init_str(self, string: &str) -> id {
        return msg_send![self,
                         initWithBytes:string.as_ptr()
                             length:string.len()
                             encoding:UTF8_ENCODING as id];
    }

    unsafe fn len(self) -> usize {
        msg_send![self, lengthOfBytesUsingEncoding:UTF8_ENCODING]
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
