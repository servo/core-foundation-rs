// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use appkit::{CGFloat, NSPoint, NSRect, NSSize, NSEventType};
use appkit::{NSWindowOrderingMode, NSAlignmentOptions};

use libc::{c_double, c_long, c_ulong, c_char};
use libc;

use std::mem;

pub type Class = libc::intptr_t;
pub type IMP = extern "C" fn(id, SEL) -> id;
pub type Ivar = libc::intptr_t;
pub type SEL = libc::intptr_t;
#[allow(non_camel_case_types)]
pub type id = libc::intptr_t;

#[cfg(target_word_size = "32")]
pub type NSInteger = libc::c_int;
#[cfg(target_word_size = "32")]
pub type NSUInteger = libc::c_uint;

#[cfg(target_word_size = "64")]
pub type NSInteger = libc::c_long;
#[cfg(target_word_size = "64")]
pub type NSUInteger = libc::c_ulong;

#[allow(non_upper_case_globals)]
pub const nil: id = 0;

extern {
    pub fn class_addMethod(cls: Class, name: SEL, imp: IMP, types: *const libc::c_char) -> bool;
    pub fn class_addIvar(cls: Class,
                         name: *const libc::c_char,
                         size: libc::size_t,
                         alignment: u8,
                         types: *const libc::c_char)
                         -> bool;
    pub fn object_setInstanceVariable(obj: id, name: *const libc::c_char, value: *mut libc::c_void);
    pub fn object_getInstanceVariable(obj: id, name: *const libc::c_char, outValue: *mut *mut libc::c_void);
    pub fn objc_allocateClassPair(superclass: Class, name: *const libc::c_char, extraBytes: libc::size_t)
                                  -> Class;
    pub fn objc_getClass(name: *const libc::c_char) -> id;
    pub fn objc_msgSend(theReceiver: id, theSelector: SEL, ...) -> id;
    pub fn objc_registerClassPair(cls: Class);
    pub fn sel_registerName(name: *const libc::c_char) -> SEL;
}

/// Returns an Objective-C message send function that returns a type `T`.
pub unsafe fn msg_send<T>() -> extern fn(theReceiver: id, theSelector: SEL, ...) -> T {
    mem::transmute(objc_msgSend)
}

/// A convenience method to convert the name of a class to the class object itself.
#[inline]
pub fn class(name: &str) -> id {
    unsafe {
        objc_getClass(name.to_c_str().as_ptr())
    }
}

/// A convenience method to convert the name of a selector to the selector object.
#[inline]
pub fn selector(name: &str) -> SEL {
    unsafe {
        sel_registerName(name.to_c_str().as_ptr())
    }
}

/// A trait that allows syntax like:
///
/// ~~~rust
/// # use cocoa::base::{ObjCMethodCall, id};
/// # unsafe {
/// let string = "NSString".send("alloc", ())
///                        .send("initWithUTF8String:", "Hello world!".as_ptr() as id);
/// # }
/// ~~~
pub trait ObjCMethodCall {
    unsafe fn send<S:ObjCSelector,A:ObjCMethodArgs>(self, selector: S, args: A) -> id;
    unsafe fn send_double<S:ObjCSelector,A:ObjCMethodDoubleArgs>(self, selector: S, args: A)
                          -> c_double;
    unsafe fn send_long<S:ObjCSelector,A:ObjCMethodLongArgs>(self, selector: S, args: A) -> c_long;
    unsafe fn send_void<S:ObjCSelector,A:ObjCMethodVoidArgs>(self, selector: S, args: A);
    unsafe fn send_bool<S:ObjCSelector,A:ObjCMethodBoolArgs>(self, selector: S, args: A) -> bool;
    unsafe fn send_float<S:ObjCSelector,A:ObjCMethodFloatArgs>(self, selector: S, args: A) -> CGFloat;
    unsafe fn send_integer<S:ObjCSelector,A:ObjCMethodIntegerArgs>(self, selector: S, args: A) -> NSInteger;
    unsafe fn send_point<S:ObjCSelector,A:ObjCMethodPointArgs>(self, selector: S, args: A) -> NSPoint;
    unsafe fn send_rect<S:ObjCSelector,A:ObjCMethodRectArgs>(self, selector: S, args: A) -> NSRect;
    unsafe fn send_size<S:ObjCSelector,A:ObjCMethodSizeArgs>(self, selector: S, args: A) -> NSSize;
    unsafe fn send_event<S:ObjCSelector,A:ObjCMethodEventArgs>(self, selector: S, args: A) -> NSEventType;
    unsafe fn send_string<S:ObjCSelector,A:ObjCMethodStringArgs>(self, selector: S, args: A) -> *const libc::c_char;
    unsafe fn send_ushort<S:ObjCSelector,A:ObjCMethodUShortArgs>(self, selector: S, args: A) -> libc::c_ushort;
    unsafe fn send_NSUInteger<S:ObjCSelector,A:ObjCMethodNSUIntegerArgs>(self, selector: S, args: A) -> NSUInteger;
}

impl ObjCMethodCall for id {
    #[inline]
    unsafe fn send<S:ObjCSelector,A:ObjCMethodArgs>(self, selector: S, args: A) -> id {
        args.send_args(self, selector.as_selector())
    }
    #[inline]
    unsafe fn send_double<S:ObjCSelector,A:ObjCMethodDoubleArgs>(self, selector: S, args: A)
                          -> c_double {
        args.send_double_args(self, selector.as_selector())
    }
    #[inline]
    unsafe fn send_long<S:ObjCSelector,A:ObjCMethodLongArgs>(self, selector: S, args: A)
                        -> c_long {
        args.send_long_args(self, selector.as_selector())
    }
    #[inline]
    unsafe fn send_void<S:ObjCSelector,A:ObjCMethodVoidArgs>(self, selector: S, args: A) {
        args.send_void_args(self, selector.as_selector())
    }
    #[inline]
    unsafe fn send_bool<S:ObjCSelector,A:ObjCMethodBoolArgs>(self, selector: S, args: A)
                        -> bool {
        args.send_bool_args(self, selector.as_selector())
    }
    #[inline]
    unsafe fn send_float<S:ObjCSelector,A:ObjCMethodFloatArgs>(self, selector: S, args: A)
                        -> CGFloat {
        args.send_float_args(self, selector.as_selector())
    }
    #[inline]
    unsafe fn send_integer<S:ObjCSelector,A:ObjCMethodIntegerArgs>(self, selector: S, args: A)
                        -> NSInteger {
        args.send_integer_args(self, selector.as_selector())
    }
    #[inline]
    unsafe fn send_point<S:ObjCSelector,A:ObjCMethodPointArgs>(self, selector: S, args: A)
                        -> NSPoint {
        args.send_point_args(self, selector.as_selector())
    }
    #[inline]
    unsafe fn send_rect<S:ObjCSelector,A:ObjCMethodRectArgs>(self, selector: S, args: A)
                        -> NSRect {
        args.send_rect_args(self, selector.as_selector())
    }
    #[inline]
    unsafe fn send_size<S:ObjCSelector,A:ObjCMethodSizeArgs>(self, selector: S, args: A)
                        -> NSSize {
        args.send_size_args(self, selector.as_selector())
    }
    #[inline]
    unsafe fn send_event<S:ObjCSelector,A:ObjCMethodEventArgs>(self, selector: S, args: A)
                        -> NSEventType {
        args.send_event_args(self, selector.as_selector())
    }
    #[inline]
    unsafe fn send_string<S:ObjCSelector,A:ObjCMethodStringArgs>(self, selector: S, args: A)
                        -> *const libc::c_char {
        args.send_string_args(self, selector.as_selector())
    }
    #[inline]
    unsafe fn send_ushort<S:ObjCSelector,A:ObjCMethodUShortArgs>(self, selector: S, args: A)
                        -> libc::c_ushort {
        args.send_ushort_args(self, selector.as_selector())
    }
    #[inline]
    unsafe fn send_NSUInteger<S:ObjCSelector,A:ObjCMethodNSUIntegerArgs>(self, selector: S, args: A)
                        -> NSUInteger {
        args.send_NSUInteger_args(self, selector.as_selector())
    }
}

/// A convenience implementation that allows methods on class names to be called directly, as in:
///
/// ~~~rust
/// # use cocoa::base::ObjCMethodCall;
/// # unsafe {
/// "NSString".send("alloc", ())
/// # };
/// ~~~
impl<'a> ObjCMethodCall for &'a str {
    #[inline]
    unsafe fn send<S:ObjCSelector,A:ObjCMethodArgs>(self, selector: S, args: A) -> id {
        args.send_args(class(self), selector.as_selector())
    }
    #[inline]
    unsafe fn send_double<S:ObjCSelector,A:ObjCMethodDoubleArgs>(self, selector: S, args: A)
                          -> c_double {
        args.send_double_args(class(self), selector.as_selector())
    }
    #[inline]
    unsafe fn send_long<S:ObjCSelector,A:ObjCMethodLongArgs>(self, selector: S, args: A)
                        -> c_long {
        args.send_long_args(class(self), selector.as_selector())
    }
    #[inline]
    unsafe fn send_void<S:ObjCSelector,A:ObjCMethodVoidArgs>(self, selector: S, args: A) {
        args.send_void_args(class(self), selector.as_selector())
    }
    #[inline]
    unsafe fn send_bool<S:ObjCSelector,A:ObjCMethodBoolArgs>(self, selector: S, args: A)
                        -> bool {
        args.send_bool_args(class(self), selector.as_selector())
    }
    #[inline]
    unsafe fn send_float<S:ObjCSelector,A:ObjCMethodFloatArgs>(self, selector: S, args: A)
                        -> CGFloat {
        args.send_float_args(class(self), selector.as_selector())
    }
    #[inline]
    unsafe fn send_integer<S:ObjCSelector,A:ObjCMethodIntegerArgs>(self, selector: S, args: A)
                        -> NSInteger {
        args.send_integer_args(class(self), selector.as_selector())
    }
    #[inline]
    unsafe fn send_point<S:ObjCSelector,A:ObjCMethodPointArgs>(self, selector: S, args: A)
                        -> NSPoint {
        args.send_point_args(class(self), selector.as_selector())
    }
    #[inline]
    unsafe fn send_rect<S:ObjCSelector,A:ObjCMethodRectArgs>(self, selector: S, args: A)
                        -> NSRect {
        args.send_rect_args(class(self), selector.as_selector())
    }
    #[inline]
    unsafe fn send_size<S:ObjCSelector,A:ObjCMethodSizeArgs>(self, selector: S, args: A)
                        -> NSSize {
        args.send_size_args(class(self), selector.as_selector())
    }
    #[inline]
    unsafe fn send_event<S:ObjCSelector,A:ObjCMethodEventArgs>(self, selector: S, args: A)
                        -> NSEventType {
        args.send_event_args(class(self), selector.as_selector())
    }
    #[inline]
    unsafe fn send_string<S:ObjCSelector,A:ObjCMethodStringArgs>(self, selector: S, args: A)
                        -> *const libc::c_char {
        args.send_string_args(class(self), selector.as_selector())
    }
    #[inline]
    unsafe fn send_ushort<S:ObjCSelector,A:ObjCMethodUShortArgs>(self, selector: S, args: A)
                        -> libc::c_ushort {
        args.send_ushort_args(class(self), selector.as_selector())
    }
    #[inline]
    unsafe fn send_NSUInteger<S:ObjCSelector,A:ObjCMethodNSUIntegerArgs>(self, selector: S, args: A)
                        -> NSUInteger {
        args.send_NSUInteger_args(class(self), selector.as_selector())
    }
}

/// A trait that allows C strings to be used as selectors without having to convert them first.
pub trait ObjCSelector {
    fn as_selector(self) -> SEL;
}

impl<'a> ObjCSelector for &'a str {
    #[inline]
    fn as_selector(self) -> SEL {
        // TODO(pcwalton): Cache somehow.
        unsafe {
            sel_registerName(self.to_c_str().as_ptr())
        }
    }
}

impl ObjCSelector for SEL {
    #[inline]
    fn as_selector(self) -> SEL {
        self
    }
}

/// Traits that simulate variadic parameters for convenience when sending messages.
pub trait ObjCMethodArgs {
    unsafe fn send_args(self, receiver: id, selector: SEL) -> id;
}
pub trait ObjCMethodDoubleArgs {
    unsafe fn send_double_args(self, receiver: id, selector: SEL) -> c_double;
}
pub trait ObjCMethodLongArgs {
    unsafe fn send_long_args(self, receiver: id, selector: SEL) -> c_long;
}
pub trait ObjCMethodVoidArgs {
    unsafe fn send_void_args(self, receiver: id, selector: SEL);
}
pub trait ObjCMethodBoolArgs {
    unsafe fn send_bool_args(self, receiver: id, selector: SEL) -> bool;
}
pub trait ObjCMethodFloatArgs {
    unsafe fn send_float_args(self, receiver: id, selector: SEL) -> CGFloat;
}
pub trait ObjCMethodIntegerArgs {
    unsafe fn send_integer_args(self, receiver: id, selector: SEL) -> NSInteger;
}
pub trait ObjCMethodPointArgs {
    unsafe fn send_point_args(self, receiver: id, selector: SEL) -> NSPoint;
}
pub trait ObjCMethodRectArgs {
    unsafe fn send_rect_args(self, receiver: id, selector: SEL) -> NSRect;
}
pub trait ObjCMethodSizeArgs {
    unsafe fn send_size_args(self, receiver: id, selector: SEL) -> NSSize;
}
pub trait ObjCMethodEventArgs {
    unsafe fn send_event_args(self, receiver: id, selector: SEL) -> NSEventType;
}

pub trait ObjCMethodStringArgs {
    unsafe fn send_string_args(self, receiver: id, selector: SEL) -> *const libc::c_char;
}

pub trait ObjCMethodUShortArgs {
    unsafe fn send_ushort_args(self, received: id, selector: SEL) -> libc::c_ushort;
}

pub trait ObjCMethodNSUIntegerArgs {
    unsafe fn send_NSUInteger_args(self, received: id, selector: SEL) -> NSUInteger;
}

impl ObjCMethodArgs for () {
    #[inline]
    unsafe fn send_args(self, receiver: id, selector: SEL) -> id {
        msg_send()(receiver, selector)
    }
}

impl ObjCMethodArgs for id {
    #[inline]
    unsafe fn send_args(self, receiver: id, selector: SEL) -> id {
        msg_send()(receiver, selector, self)
    }
}

impl ObjCMethodArgs for NSRect {
    #[inline]
    unsafe fn send_args(self, receiver: id, selector: SEL) -> id {
        msg_send()(receiver, selector, &self)
    }
}

impl ObjCMethodArgs for (id, SEL, id) {
    #[inline]
    unsafe fn send_args(self, receiver: id, selector: SEL) -> id {
        let (first, second, third) = self;
        msg_send()(receiver, selector, first, second, third)
    }
}

impl ObjCMethodArgs for (NSRect, c_ulong, c_ulong, bool) {
    #[inline]
    unsafe fn send_args(self, receiver: id, selector: SEL) -> id {
        let (first, second, third, fourth) = self;
        msg_send()(receiver, selector, first, second, third, fourth as libc::c_int)
    }
}

impl ObjCMethodArgs for (id, id, id, id, id) {
    #[inline]
    unsafe fn send_args(self, receiver: id, selector: SEL) -> id {
        let (first, second, third, fourth, fifth) = self;
        msg_send()(receiver, selector, first, second, third, fourth, fifth)
    }
}

impl ObjCMethodArgs for (NSRect, id) {
    #[inline]
    unsafe fn send_args(self, receiver: id, selector: SEL) -> id {
        let (first, second) = self;
        msg_send()(receiver, selector, first, second)
    }
}

impl<'a> ObjCMethodArgs for &'a [uint] {
    #[inline]
    unsafe fn send_args(self, receiver: id, selector: SEL) -> id {
        msg_send()(receiver, selector, self)
    }
}

impl ObjCMethodArgs for (id, id) {
    #[inline]
    unsafe fn send_args(self, receiver: id, selector: SEL) -> id {
        let (first, second) = self;
        msg_send()(receiver, selector, first, second)
    }
}

impl ObjCMethodArgs for (NSUInteger, id, id, bool) {
    #[inline]
    unsafe fn send_args(self, receiver: id, selector: SEL) -> id {
        let (first, second, third, fourth) = self;
        msg_send()(receiver, selector, first, second, third, fourth as libc::c_int)
    }
}

impl ObjCMethodDoubleArgs for () {
    #[inline]
    unsafe fn send_double_args(self, receiver: id, selector: SEL) -> f64 {
        msg_send()(receiver, selector)
    }
}

impl ObjCMethodLongArgs for () {
    #[inline]
    unsafe fn send_long_args(self, receiver: id, selector: SEL) -> c_long {
        msg_send()(receiver, selector)
    }
}

impl ObjCMethodVoidArgs for () {
    #[inline]
    unsafe fn send_void_args(self, receiver: id, selector: SEL) {
        msg_send()(receiver, selector)
    }
}

impl ObjCMethodVoidArgs for bool {
    #[inline]
    unsafe fn send_void_args(self, receiver: id, selector: SEL) {
        msg_send()(receiver, selector, self as libc::c_int)
    }
}

impl ObjCMethodVoidArgs for id {
    #[inline]
    unsafe fn send_void_args(self, receiver: id, selector: SEL) {
        msg_send()(receiver, selector, self)
    }
}

// Conflicts with id... should id be a newtype? - bjz
// impl ObjCMethodVoidArgs for NSInteger {
//     #[inline]
//     unsafe fn send_void_args(self, receiver: id, selector: SEL) {
//         msg_send()(receiver, selector, self);
//     }
// }

impl ObjCMethodVoidArgs for NSPoint {
    #[inline]
    unsafe fn send_void_args(self, receiver: id, selector: SEL) {
        msg_send()(receiver, selector, self)
    }
}

impl ObjCMethodVoidArgs for NSSize {
    #[inline]
    unsafe fn send_void_args(self, receiver: id, selector: SEL) {
        msg_send()(receiver, selector, self)
    }
}

impl ObjCMethodVoidArgs for (NSRect, bool) {
    #[inline]
    unsafe fn send_void_args(self, receiver: id, selector: SEL) {
        let (first, second) = self;
        msg_send()(receiver, selector, first, second as libc::c_int)
    }
}

impl ObjCMethodVoidArgs for (NSWindowOrderingMode, NSInteger) {
    #[inline]
    unsafe fn send_void_args(self, receiver: id, selector: SEL) {
        let (first, second) = self;
        msg_send()(receiver, selector, first, second)
    }
}

impl ObjCMethodBoolArgs for () {
    #[inline]
    unsafe fn send_bool_args(self, receiver: id, selector: SEL) -> bool {
        msg_send()(receiver, selector)
    }
}

impl ObjCMethodBoolArgs for c_long {
    #[inline]
    unsafe fn send_bool_args(self, receiver: id, selector: SEL) -> bool {
        msg_send()(receiver, selector, self)
    }
}

impl ObjCMethodFloatArgs for () {
    #[inline]
    unsafe fn send_float_args(self, receiver: id, selector: SEL) -> CGFloat {
        msg_send()(receiver, selector)
    }
}

impl ObjCMethodIntegerArgs for () {
    #[inline]
    unsafe fn send_integer_args(self, receiver: id, selector: SEL) -> NSInteger {
        msg_send()(receiver, selector)
    }
}

impl ObjCMethodPointArgs for NSPoint {
    #[inline]
    unsafe fn send_point_args(self, receiver: id, selector: SEL) -> NSPoint {
        msg_send()(receiver, selector, self)
    }
}

impl ObjCMethodRectArgs for () {
    #[inline]
    unsafe fn send_rect_args(self, receiver: id, selector: SEL) -> NSRect {
        msg_send()(receiver, selector)
    }
}

impl ObjCMethodRectArgs for (NSRect, NSAlignmentOptions) {
    #[inline]
    unsafe fn send_rect_args(self, receiver: id, selector: SEL) -> NSRect {
        let (first, second) = self;
        msg_send()(receiver, selector, first, second)
    }
}

impl ObjCMethodRectArgs for NSRect {
    #[inline]
    unsafe fn send_rect_args(self, receiver: id, selector: SEL) -> NSRect {
        msg_send()(receiver, selector, self)
    }
}

impl ObjCMethodSizeArgs for () {
    #[inline]
    unsafe fn send_size_args(self, receiver: id, selector: SEL) -> NSSize {
        msg_send()(receiver, selector)
    }
}

impl ObjCMethodPointArgs for () {
    #[inline]
    unsafe fn send_point_args(self, receiver: id, selector: SEL) -> NSPoint {
        msg_send()(receiver, selector)
    }
}

impl ObjCMethodEventArgs for () {
    #[inline]
    unsafe fn send_event_args(self, receiver: id, selector: SEL) -> NSEventType {
        msg_send()(receiver, selector)
    }
}

impl ObjCMethodPointArgs for (NSPoint, id) {
    #[inline]
    unsafe fn send_point_args(self, receiver: id, selector: SEL) -> NSPoint {
        let (first, second) = self;
        msg_send()(receiver, selector, first, second)
    }
}

impl ObjCMethodStringArgs for () {
    #[inline]
    unsafe fn send_string_args(self, receiver: id, selector: SEL) -> *const libc::c_char {
        msg_send()(receiver, selector)
    }
}

impl ObjCMethodUShortArgs for () {
    #[inline]
    unsafe fn send_ushort_args(self, receiver: id, selector: SEL) -> libc::c_ushort {
        msg_send()(receiver, selector)
    }
}

impl ObjCMethodNSUIntegerArgs for () {
    #[inline]
    unsafe fn send_NSUInteger_args(self, receiver: id, selector: SEL) -> NSUInteger {
        msg_send()(receiver, selector)
    }
}

/// A trait that simulates variadic parameters for method calls.

#[cfg(test)]
mod test {
    use libc;
    use super::*;

    #[test]
    pub fn test_nsapp() {
        unsafe {
            let _nsApp = "NSApplication".send("sharedApplication", ());
        }
    }

    #[test]
    pub fn test_custom_obj() {
        extern fn MyObject_doSomething(this: id, _: SEL) -> id {
            println!("doSomething");
            return this;
        }

        let ns_object = class("NSObject");
        let my_object = unsafe {
            objc_allocateClassPair(ns_object, "MyObject".to_c_str().as_ptr(), 0 as libc::size_t)
        };
        unsafe {
            let doSomething = selector("doSomething");
            let _ = class_addMethod(my_object, doSomething, MyObject_doSomething, "@@:".to_c_str().as_ptr());

            objc_registerClassPair(my_object);

            let mut obj: id = my_object.send("alloc", ());
            obj = obj.send("init", ());
            obj.send_void("doSomething", ());
        }
    }
}
