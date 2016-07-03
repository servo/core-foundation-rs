// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_upper_case_globals)]

use std::mem;
use std::ptr;
use base::{id, class, BOOL, SEL, nil};
use block::Block;
use core_graphics::base::CGFloat;
use core_graphics::geometry::CGRect;
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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

    #[inline]
    pub fn as_CGRect(&self) -> &CGRect {
        unsafe {
            mem::transmute::<&NSRect, &CGRect>(self)
        }
    }

    #[inline]
    pub fn inset(&self, x: CGFloat, y: CGFloat) -> NSRect {
        unsafe {
            NSInsetRect(*self, x, y)
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

#[repr(C)]
struct NSFastEnumerationState {
    pub state: libc::c_ulong,
    pub items_ptr: *mut id,
    pub mutations_ptr: *mut libc::c_ulong,
    pub extra: [libc::c_ulong; 5]
}

const NS_FAST_ENUM_BUF_SIZE: usize = 16;

pub struct NSFastIterator {
    state: NSFastEnumerationState,
    buffer: [id; NS_FAST_ENUM_BUF_SIZE],
    mut_val: Option<libc::c_ulong>,
    len: usize,
    idx: usize,
    object: id
}

impl Iterator for NSFastIterator {
    type Item = id;

    fn next(&mut self) -> Option<id> {
        if self.idx >= self.len {
            self.len = unsafe {
                msg_send![self.object, countByEnumeratingWithState:&mut self.state objects:self.buffer.as_mut_ptr() count:NS_FAST_ENUM_BUF_SIZE]
            };
            self.idx = 0;
        }

        let new_mut = unsafe {
            *self.state.mutations_ptr
        };

        if let Some(old_mut) = self.mut_val {
            assert!(old_mut == new_mut, "The collection was mutated while being enumerated");
        }

        if self.idx < self.len {
            let object = unsafe {
                *self.state.items_ptr.offset(self.idx as isize)
            };        
            self.mut_val = Some(new_mut);
            self.idx += 1;
            Some(object)
        } else {
            None
        }
    }
}

pub trait NSFastEnumeration {
    unsafe fn iter(self) -> NSFastIterator;
}

impl NSFastEnumeration for id {
    unsafe fn iter(self) -> NSFastIterator {
        NSFastIterator {
            state: NSFastEnumerationState {
                state: 0,
                items_ptr: ptr::null_mut(),
                mutations_ptr: ptr::null_mut(),
                extra: [0; 5]
            },
            buffer: [nil; NS_FAST_ENUM_BUF_SIZE],
            mut_val: None,
            len: 0,
            idx: 0,
            object: self
        }
    }
}

#[link(name = "Foundation", kind = "framework")]
extern {
    fn NSInsetRect(rect: NSRect, x: CGFloat, y: CGFloat) -> NSRect;
}

pub trait NSRunLoop {
    unsafe fn currentRunLoop() -> Self;

    unsafe fn performSelector_target_argument_order_modes_(self,
                                                           aSelector: SEL,
                                                           target: id,
                                                           anArgument: id,
                                                           order: NSUInteger,
                                                           modes: id);
}

impl NSRunLoop for id {
    unsafe fn currentRunLoop() -> id {
        msg_send![class("NSRunLoop"), currentRunLoop]
    }

    unsafe fn performSelector_target_argument_order_modes_(self,
                                                           aSelector: SEL,
                                                           target: id,
                                                           anArgument: id,
                                                           order: NSUInteger,
                                                           modes: id) {
        msg_send![self, performSelector:aSelector
                                 target:target
                               argument:anArgument
                                  order:order
                                  modes:modes]
    }
}

pub trait NSData {
    unsafe fn data(_: Self) -> id {
        msg_send![class("NSData"), data]
    }

    unsafe fn dataWithBytes_length_(_: Self, bytes: *const libc::c_void, length: NSUInteger) -> id {
        msg_send![class("NSData"), dataWithBytes:bytes length:length]
    }

    unsafe fn dataWithBytesNoCopy_length_(_: Self, bytes: *const libc::c_void, length: NSUInteger) -> id {
        msg_send![class("NSData"), dataWithBytesNoCopy:bytes length:length]
    }

    unsafe fn dataWithBytesNoCopy_length_freeWhenDone_(_: Self, bytes: *const libc::c_void,
                                                      length: NSUInteger, freeWhenDone: BOOL) -> id {
        msg_send![class("NSData"), dataWithBytesNoCopy:bytes length:length freeWhenDone:freeWhenDone]
    }

    unsafe fn dataWithContentsOfFile_(_: Self, path: id) -> id {
        msg_send![class("NSData"), dataWithContentsOfFile:path]
    }

    unsafe fn dataWithContentsOfFile_options_error_(_: Self, path: id, mask: NSDataReadingOptions,
                                                    errorPtr: *mut id) -> id {
        msg_send![class("NSData"), dataWithContentsOfFile:path options:mask error:errorPtr]
    }

    unsafe fn dataWithContentsOfURL_(_: Self, aURL: id) -> id {
        msg_send![class("NSData"), dataWithContentsOfURL:aURL]
    }

    unsafe fn dataWithContentsOfURL_options_error_(_: Self, aURL: id, mask: NSDataReadingOptions,
                                                   errorPtr: *mut id) -> id {
        msg_send![class("NSData"), dataWithContentsOfURL:aURL options:mask error:errorPtr]
    }

    unsafe fn dataWithData_(_: Self, aData: id) -> id {
        msg_send![class("NSData"), dataWithData:aData]
    }

    unsafe fn initWithBase64EncodedData_options_(self, base64Data: id, options: NSDataBase64DecodingOptions)
                                                 -> id;
    unsafe fn initWithBase64EncodedString_options_(self, base64String: id, options: NSDataBase64DecodingOptions)
                                                   -> id;
    unsafe fn initWithBytes_length_(self, bytes: *const libc::c_void, length: NSUInteger) -> id;
    unsafe fn initWithBytesNoCopy_length_(self, bytes: *const libc::c_void, length: NSUInteger) -> id;
    unsafe fn initWithBytesNoCopy_length_deallocator_(self, bytes: *const libc::c_void, length: NSUInteger,
                                                      deallocator: *mut Block<(*const libc::c_void, NSUInteger), ()>)
                                                      -> id;
    unsafe fn initWithBytesNoCopy_length_freeWhenDone_(self, bytes: *const libc::c_void,
                                                       length: NSUInteger, freeWhenDone: BOOL) -> id;
    unsafe fn initWithContentsOfFile_(self, path: id) -> id;
    unsafe fn initWithContentsOfFile_options_error(self, path: id, mask: NSDataReadingOptions, errorPtr: *mut id)
                                                   -> id;
    unsafe fn initWithContentsOfURL_(self, aURL: id) -> id;
    unsafe fn initWithContentsOfURL_options_error_(self, aURL: id, mask: NSDataReadingOptions, errorPtr: *mut id)
                                                   -> id;
    unsafe fn initWithData_(self, data: id) -> id;

    unsafe fn bytes(self) -> *const libc::c_void;
    unsafe fn description(self) -> id;
    unsafe fn enumerateByteRangesUsingBlock_(self, block: *mut Block<(*const libc::c_void, NSRange, *mut BOOL), ()>);
    unsafe fn getBytes_length_(self, buffer: *mut libc::c_void, length: NSUInteger);
    unsafe fn getBytes_range_(self, buffer: *mut libc::c_void, range: NSRange);
    unsafe fn subdataWithRange_(self, range: NSRange) -> id;
    unsafe fn rangeOfData_options_range_(self, dataToFind: id, options: NSDataSearchOptions, searchRange: NSRange)
                                         -> NSRange;

    unsafe fn base64EncodedDataWithOptions_(self, options: NSDataBase64EncodingOptions) -> id;
    unsafe fn base64EncodedStringWithOptions_(self, options: NSDataBase64EncodingOptions) -> id;

    unsafe fn isEqualToData_(self, otherData: id) -> id;
    unsafe fn length(self) -> NSUInteger;

    unsafe fn writeToFile_atomically_(self, path: id, atomically: BOOL) -> BOOL;
    unsafe fn writeToFile_options_error_(self, path: id, mask: NSDataWritingOptions, errorPtr: *mut id) -> BOOL;
    unsafe fn writeToURL_atomically_(self, aURL: id, atomically: BOOL) -> BOOL;
    unsafe fn writeToURL_options_error_(self, aURL: id, mask: NSDataWritingOptions, errorPtr: *mut id) -> BOOL;
}

impl NSData for id {
    unsafe fn initWithBase64EncodedData_options_(self, base64Data: id, options: NSDataBase64DecodingOptions)
                                                 -> id {
        msg_send![self, initWithBase64EncodedData:base64Data options:options]
    }

    unsafe fn initWithBase64EncodedString_options_(self, base64String: id, options: NSDataBase64DecodingOptions)
                                                   -> id {
        msg_send![self, initWithBase64EncodedString:base64String options:options]
    }

    unsafe fn initWithBytes_length_(self, bytes: *const libc::c_void, length: NSUInteger) -> id {
        msg_send![self,initWithBytes:bytes length:length]
    }

    unsafe fn initWithBytesNoCopy_length_(self, bytes: *const libc::c_void, length: NSUInteger) -> id {
        msg_send![self, initWithBytesNoCopy:bytes length:length]
    }

    unsafe fn initWithBytesNoCopy_length_deallocator_(self, bytes: *const libc::c_void, length: NSUInteger,
                                                      deallocator: *mut Block<(*const libc::c_void, NSUInteger), ()>)
                                                      -> id {
        msg_send![self, initWithBytesNoCopy:bytes length:length deallocator:deallocator]
    }

    unsafe fn initWithBytesNoCopy_length_freeWhenDone_(self, bytes: *const libc::c_void,
                                                       length: NSUInteger, freeWhenDone: BOOL) -> id {
        msg_send![self, initWithBytesNoCopy:bytes length:length freeWhenDone:freeWhenDone]
    }

    unsafe fn initWithContentsOfFile_(self, path: id) -> id {
        msg_send![self, initWithContentsOfFile:path]
    }

    unsafe fn initWithContentsOfFile_options_error(self, path: id, mask: NSDataReadingOptions, errorPtr: *mut id)
                                                   -> id {
        msg_send![self, initWithContentsOfFile:path options:mask error:errorPtr]
    }

    unsafe fn initWithContentsOfURL_(self, aURL: id) -> id {
        msg_send![self, initWithContentsOfURL:aURL]
    }

    unsafe fn initWithContentsOfURL_options_error_(self, aURL: id, mask: NSDataReadingOptions, errorPtr: *mut id)
                                                   -> id {
        msg_send![self, initWithContentsOfURL:aURL options:mask error:errorPtr]
    }

    unsafe fn initWithData_(self, data: id) -> id {
        msg_send![self, initWithData:data]
    }

    unsafe fn bytes(self) -> *const libc::c_void {
        msg_send![self, bytes]
    }

    unsafe fn description(self) -> id {
        msg_send![self, description]
    }

    unsafe fn enumerateByteRangesUsingBlock_(self, block: *mut Block<(*const libc::c_void, NSRange, *mut BOOL), ()>) {
        msg_send![self, enumerateByteRangesUsingBlock:block]
    }

    unsafe fn getBytes_length_(self, buffer: *mut libc::c_void, length: NSUInteger) {
        msg_send![self, getBytes:buffer length:length]
    }

    unsafe fn getBytes_range_(self, buffer: *mut libc::c_void, range: NSRange) {
        msg_send![self, getBytes:buffer range:range]
    }

    unsafe fn subdataWithRange_(self, range: NSRange) -> id {
        msg_send![self, subdataWithRange:range]
    }

    unsafe fn rangeOfData_options_range_(self, dataToFind: id, options: NSDataSearchOptions, searchRange: NSRange)
                                         -> NSRange {
        msg_send![self, rangeOfData:dataToFind options:options range:searchRange]
    }

    unsafe fn base64EncodedDataWithOptions_(self, options: NSDataBase64EncodingOptions) -> id {
        msg_send![self, base64EncodedDataWithOptions:options]
    }

    unsafe fn base64EncodedStringWithOptions_(self, options: NSDataBase64EncodingOptions) -> id {
        msg_send![self, base64EncodedStringWithOptions:options]
    }

    unsafe fn isEqualToData_(self, otherData: id) -> id {
        msg_send![self, isEqualToData:otherData]
    }

    unsafe fn length(self) -> NSUInteger {
        msg_send![self, length]
    }

    unsafe fn writeToFile_atomically_(self, path: id, atomically: BOOL) -> BOOL {
        msg_send![self, writeToFile:path atomically:atomically]
    }

    unsafe fn writeToFile_options_error_(self, path: id, mask: NSDataWritingOptions, errorPtr: *mut id) -> BOOL {
        msg_send![self, writeToFile:path options:mask error:errorPtr]
    }

    unsafe fn writeToURL_atomically_(self, aURL: id, atomically: BOOL) -> BOOL {
        msg_send![self, writeToURL:aURL atomically:atomically]
    }

    unsafe fn writeToURL_options_error_(self, aURL: id, mask: NSDataWritingOptions, errorPtr: *mut id) -> BOOL {
        msg_send![self, writeToURL:aURL options:mask error:errorPtr]
    }
}

bitflags! {
    pub flags NSDataReadingOptions: libc::c_ulonglong {
       const NSDataReadingMappedIfSafe = 1 << 0,
       const NSDataReadingUncached = 1 << 1,
       const NSDataReadingMappedAlways = 1 << 3
    }
}

bitflags! {
    pub flags NSDataBase64EncodingOptions: libc::c_ulonglong {
        const NSDataBase64Encoding64CharacterLineLength = 1 << 0,
        const NSDataBase64Encoding76CharacterLineLength = 1 << 1,
        const NSDataBase64EncodingEndLineWithCarriageReturn = 1 << 4,
        const NSDataBase64EncodingEndLineWithLineFeed = 1 << 5
    }
}

bitflags! {
    pub flags NSDataBase64DecodingOptions: libc::c_ulonglong {
       const NSDataBase64DecodingIgnoreUnknownCharacters = 1 << 0
    }
}

bitflags! {
    pub flags NSDataWritingOptions: libc::c_ulonglong {
        const NSDataWritingAtomic = 1 << 0,
        const NSDataWritingWithoutOverwriting = 1 << 1
    }
}

bitflags! {
    pub flags NSDataSearchOptions: libc::c_ulonglong {
        const NSDataSearchBackwards = 1 << 0,
        const NSDataSearchAnchored = 1 << 1
    }
}
