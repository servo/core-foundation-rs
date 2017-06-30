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

pub trait NSAutoreleasePool: Sized {
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

pub trait NSProcessInfo: Sized {
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

pub trait NSValue: Sized {
    unsafe fn valueWithPoint(_: Self, point: NSPoint) -> id {
        msg_send![class("NSValue"), valueWithPoint:point]
    }

    unsafe fn valueWithSize(_: Self, size: NSSize) -> id {
        msg_send![class("NSValue"), valueWithSize:size]
    }
}

impl NSValue for id {
}

pub trait NSArray: Sized {
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

pub trait NSDictionary: Sized {
    unsafe fn dictionary(_: Self) -> id {
        msg_send![class("NSDictionary"), dictionary]
    }

    unsafe fn dictionaryWithContentsOfFile_(_: Self, path: id) -> id {
        msg_send![class("NSDictionary"), dictionaryWithContentsOfFile:path]
    }

    unsafe fn dictionaryWithContentsOfURL_(_: Self, aURL: id) -> id {
        msg_send![class("NSDictionary"), dictionaryWithContentsOfURL:aURL]
    }

    unsafe fn dictionaryWithDictionary_(_: Self, otherDictionary: id) -> id {
        msg_send![class("NSDictionary"), dictionaryWithDictionary:otherDictionary]
    }

    unsafe fn dictionaryWithObject_forKey_(_: Self, anObject: id, aKey: id) -> id {
        msg_send![class("NSDictionary"), dictionaryWithObject:anObject forKey:aKey]
    }

    unsafe fn dictionaryWithObjects_forKeys_(_: Self, objects: id, keys: id) -> id {
        msg_send![class("NSDictionary"), dictionaryWithObjects:objects forKeys:keys]
    }

    unsafe fn dictionaryWithObjects_forKeys_count_(_: Self, objects: *const id, keys: *const id, count: NSUInteger) -> id {
        msg_send![class("NSDictionary"), dictionaryWithObjects:objects forKeys:keys count:count]
    }

    unsafe fn dictionaryWithObjectsAndKeys_(_: Self, firstObject: id) -> id {
        msg_send![class("NSDictionary"), dictionaryWithObjectsAndKeys:firstObject]
    }

    unsafe fn init(self) -> id;
    unsafe fn initWithContentsOfFile_(self, path: id) -> id;
    unsafe fn initWithContentsOfURL_(self, aURL: id) -> id;
    unsafe fn initWithDictionary_(self, otherDicitonary: id) -> id;
    unsafe fn initWithDictionary_copyItems_(self, otherDicitonary: id, flag: BOOL) -> id;
    unsafe fn initWithObjects_forKeys_(self, objects: id, keys: id) -> id;
    unsafe fn initWithObjects_forKeys_count_(self, objects: id, keys: id, count: NSUInteger) -> id;
    unsafe fn initWithObjectsAndKeys_(self, firstObject: id) -> id;

    unsafe fn sharedKeySetForKeys_(_: Self, keys: id) -> id {
        msg_send![class("NSDictionary"), sharedKeySetForKeys:keys]
    }

    unsafe fn count(self) -> NSUInteger;

    unsafe fn isEqualToDictionary_(self, otherDictionary: id) -> BOOL;

    unsafe fn allKeys(self) -> id;
    unsafe fn allKeysForObject_(self, anObject: id) -> id;
    unsafe fn allValues(self) -> id;
    unsafe fn objectForKey_(self, aKey: id) -> id;
    unsafe fn objectForKeyedSubscript_(self, key: id) -> id;
    unsafe fn objectsForKeys_notFoundMarker_(self, keys: id, anObject: id) -> id;
    unsafe fn valueForKey_(self, key: id) -> id;

    unsafe fn keyEnumerator(self) -> id;
    unsafe fn objectEnumerator(self) -> id;
    unsafe fn enumerateKeysAndObjectsUsingBlock_(self, block: *mut Block<(id, id, *mut BOOL), ()>);
    unsafe fn enumerateKeysAndObjectsWithOptions_usingBlock_(self, opts: NSEnumerationOptions,
                                                             block: *mut Block<(id, id, *mut BOOL), ()>);

    unsafe fn keysSortedByValueUsingSelector_(self, comparator: SEL) -> id;
    unsafe fn keysSortedByValueUsingComparator_(self, cmptr: NSComparator) -> id;
    unsafe fn keysSortedByValueWithOptions_usingComparator_(self, opts: NSEnumerationOptions, cmptr: NSComparator) -> id;

    unsafe fn keysOfEntriesPassingTest_(self, predicate: *mut Block<(id, id, *mut BOOL), BOOL>) -> id;
    unsafe fn keysOfEntriesWithOptions_PassingTest_(self, opts: NSEnumerationOptions,
                                                    predicate: *mut Block<(id, id, *mut BOOL), BOOL>) -> id;

    unsafe fn writeToFile_atomically_(self, path: id, flag: BOOL) -> BOOL;
    unsafe fn writeToURL_atomically_(self, aURL: id, flag: BOOL) -> BOOL;

    unsafe fn fileCreationDate(self) -> id;
    unsafe fn fileExtensionHidden(self) -> BOOL;
    unsafe fn fileGroupOwnerAccountID(self) -> id;
    unsafe fn fileGroupOwnerAccountName(self) -> id;
    unsafe fn fileIsAppendOnly(self) -> BOOL;
    unsafe fn fileIsImmutable(self) -> BOOL;
    unsafe fn fileModificationDate(self) -> id;
    unsafe fn fileOwnerAccountID(self) -> id;
    unsafe fn fileOwnerAccountName(self) -> id;
    unsafe fn filePosixPermissions(self) -> NSUInteger;
    unsafe fn fileSize(self) -> libc::c_ulonglong;
    unsafe fn fileSystemFileNumber(self) -> NSUInteger;
    unsafe fn fileSystemNumber(self) -> NSInteger;
    unsafe fn fileType(self) -> id;

    unsafe fn description(self) -> id;
    unsafe fn descriptionInStringsFileFormat(self) -> id;
    unsafe fn descriptionWithLocale_(self, locale: id) -> id;
    unsafe fn descriptionWithLocale_indent_(self, locale: id, indent: NSUInteger) -> id;
}

impl NSDictionary for id {
    unsafe fn init(self) -> id {
        msg_send![self, init]
    }

    unsafe fn initWithContentsOfFile_(self, path: id) -> id {
        msg_send![self, initWithContentsOfFile:path]
    }

    unsafe fn initWithContentsOfURL_(self, aURL: id) -> id {
        msg_send![self, initWithContentsOfURL:aURL]
    }

    unsafe fn initWithDictionary_(self, otherDictionary: id) -> id {
        msg_send![self, initWithDictionary:otherDictionary]
    }

    unsafe fn initWithDictionary_copyItems_(self, otherDictionary: id, flag: BOOL) -> id {
        msg_send![self, initWithDictionary:otherDictionary copyItems:flag]
    }

    unsafe fn initWithObjects_forKeys_(self, objects: id, keys: id) -> id {
        msg_send![self, initWithObjects:objects forKeys:keys]
    }

    unsafe fn initWithObjects_forKeys_count_(self, objects: id, keys: id, count: NSUInteger) -> id {
        msg_send![self, initWithObjects:objects forKeys:keys count:count]
    }

    unsafe fn initWithObjectsAndKeys_(self, firstObject: id) -> id {
        msg_send![self, initWithObjectsAndKeys:firstObject]
    }

    unsafe fn count(self) -> NSUInteger {
        msg_send![self, count]
    }

    unsafe fn isEqualToDictionary_(self, otherDictionary: id) -> BOOL {
        msg_send![self, isEqualToDictionary:otherDictionary]
    }

    unsafe fn allKeys(self) -> id {
        msg_send![self, allKeys]
    }

    unsafe fn allKeysForObject_(self, anObject: id) -> id {
        msg_send![self, allKeysForObject:anObject]
    }

    unsafe fn allValues(self) -> id {
        msg_send![self, allValues]
    }

    unsafe fn objectForKey_(self, aKey: id) -> id {
        msg_send![self, objectForKey:aKey]
    }

    unsafe fn objectForKeyedSubscript_(self, key: id) -> id {
        msg_send![self, objectForKeyedSubscript:key]
    }

    unsafe fn objectsForKeys_notFoundMarker_(self, keys: id, anObject: id) -> id {
        msg_send![self, objectsForKeys:keys notFoundMarker:anObject]
    }

    unsafe fn valueForKey_(self, key: id) -> id {
        msg_send![self, valueForKey:key]
    }

    unsafe fn keyEnumerator(self) -> id {
        msg_send![self, keyEnumerator]
    }

    unsafe fn objectEnumerator(self) -> id {
        msg_send![self, objectEnumerator]
    }

    unsafe fn enumerateKeysAndObjectsUsingBlock_(self, block: *mut Block<(id, id, *mut BOOL), ()>) {
        msg_send![self, enumerateKeysAndObjectsUsingBlock:block]
    }

    unsafe fn enumerateKeysAndObjectsWithOptions_usingBlock_(self, opts: NSEnumerationOptions,
                                                     block: *mut Block<(id, id, *mut BOOL), ()>) {
        msg_send![self, enumerateKeysAndObjectsWithOptions:opts usingBlock:block]
    }

    unsafe fn keysSortedByValueUsingSelector_(self, comparator: SEL) -> id {
        msg_send![self, keysSortedByValueUsingSelector:comparator]
    }

    unsafe fn keysSortedByValueUsingComparator_(self, cmptr: NSComparator) -> id {
        msg_send![self, keysSortedByValueUsingComparator:cmptr]
    }

    unsafe fn keysSortedByValueWithOptions_usingComparator_(self, opts: NSEnumerationOptions, cmptr: NSComparator) -> id {
        let rv: id = msg_send![self, keysSortedByValueWithOptions:opts usingComparator:cmptr];
        rv
    }

    unsafe fn keysOfEntriesPassingTest_(self, predicate: *mut Block<(id, id, *mut BOOL), BOOL>) -> id {
        msg_send![self, keysOfEntriesPassingTest:predicate]
    }

    unsafe fn keysOfEntriesWithOptions_PassingTest_(self, opts: NSEnumerationOptions,
                                                    predicate: *mut Block<(id, id, *mut BOOL), BOOL>) -> id {
        msg_send![self, keysOfEntriesWithOptions:opts PassingTest:predicate]
    }

    unsafe fn writeToFile_atomically_(self, path: id, flag: BOOL) -> BOOL {
        msg_send![self, writeToFile:path atomically:flag]
    }

    unsafe fn writeToURL_atomically_(self, aURL: id, flag: BOOL) -> BOOL {
        msg_send![self, writeToURL:aURL atomically:flag]
    }

    unsafe fn fileCreationDate(self) -> id {
        msg_send![self, fileCreationDate]
    }

    unsafe fn fileExtensionHidden(self) -> BOOL {
        msg_send![self, fileExtensionHidden]
    }

    unsafe fn fileGroupOwnerAccountID(self) -> id {
        msg_send![self, fileGroupOwnerAccountID]
    }

    unsafe fn fileGroupOwnerAccountName(self) -> id {
        msg_send![self, fileGroupOwnerAccountName]
    }

    unsafe fn fileIsAppendOnly(self) -> BOOL {
        msg_send![self, fileIsAppendOnly]
    }

    unsafe fn fileIsImmutable(self) -> BOOL {
        msg_send![self, fileIsImmutable]
    }

    unsafe fn fileModificationDate(self) -> id {
        msg_send![self, fileModificationDate]
    }

    unsafe fn fileOwnerAccountID(self) -> id {
        msg_send![self, fileOwnerAccountID]
    }

    unsafe fn fileOwnerAccountName(self) -> id {
        msg_send![self, fileOwnerAccountName]
    }

    unsafe fn filePosixPermissions(self) -> NSUInteger {
        msg_send![self, filePosixPermissions]
    }

    unsafe fn fileSize(self) -> libc::c_ulonglong {
        msg_send![self, fileSize]
    }

    unsafe fn fileSystemFileNumber(self) -> NSUInteger {
        msg_send![self, fileSystemFileNumber]
    }

    unsafe fn fileSystemNumber(self) -> NSInteger {
        msg_send![self, fileSystemNumber]
    }

    unsafe fn fileType(self) -> id {
        msg_send![self, fileType]
    }

    unsafe fn description(self) -> id {
        msg_send![self, description]
    }

    unsafe fn descriptionInStringsFileFormat(self) -> id {
        msg_send![self, descriptionInStringsFileFormat]
    }

    unsafe fn descriptionWithLocale_(self, locale: id) -> id {
        msg_send![self, descriptionWithLocale:locale]
    }

    unsafe fn descriptionWithLocale_indent_(self, locale: id, indent: NSUInteger) -> id {
        msg_send![self, descriptionWithLocale:locale indent:indent]
    }
}

bitflags! {
    pub struct NSEnumerationOptions: libc::c_ulonglong {
        const NSEnumerationConcurrent = 1 << 0;
        const NSEnumerationReverse = 1 << 1;
    }
}

pub type NSComparator = *mut Block<(id, id), NSComparisonResult>;

#[repr(isize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NSComparisonResult {
    NSOrderedAscending = -1,
    NSOrderedSame = 0,
    NSOrderedDescending = 1
}

pub trait NSString: Sized {
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

pub trait NSDate: Sized {
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

pub trait NSFastEnumeration: Sized {
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

pub trait NSRunLoop: Sized {
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

pub trait NSData: Sized {
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
    pub struct NSDataReadingOptions: libc::c_ulonglong {
       const NSDataReadingMappedIfSafe = 1 << 0;
       const NSDataReadingUncached = 1 << 1;
       const NSDataReadingMappedAlways = 1 << 3;
    }
}

bitflags! {
    pub struct NSDataBase64EncodingOptions: libc::c_ulonglong {
        const NSDataBase64Encoding64CharacterLineLength = 1 << 0;
        const NSDataBase64Encoding76CharacterLineLength = 1 << 1;
        const NSDataBase64EncodingEndLineWithCarriageReturn = 1 << 4;
        const NSDataBase64EncodingEndLineWithLineFeed = 1 << 5;
    }
}

bitflags! {
    pub struct NSDataBase64DecodingOptions: libc::c_ulonglong {
       const NSDataBase64DecodingIgnoreUnknownCharacters = 1 << 0;
    }
}

bitflags! {
    pub struct NSDataWritingOptions: libc::c_ulonglong {
        const NSDataWritingAtomic = 1 << 0;
        const NSDataWritingWithoutOverwriting = 1 << 1;
    }
}

bitflags! {
    pub struct NSDataSearchOptions: libc::c_ulonglong {
        const NSDataSearchBackwards = 1 << 0;
        const NSDataSearchAnchored = 1 << 1;
    }
}
