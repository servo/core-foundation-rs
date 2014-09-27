// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use appkit::NSRect;

use libc::{c_double, c_long};
use libc;

pub type Class = libc::intptr_t;
pub type IMP = extern "C" fn(id, SEL) -> id;
pub type Ivar = libc::intptr_t;
pub type SEL = libc::intptr_t;
#[allow(non_camel_case_types)]
pub type id = libc::intptr_t;

pub static nil: id = 0 as id;

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
    pub fn objc_msgSend(theReceiver: id, theSelector: SEL) -> id;
    pub fn objc_registerClassPair(cls: Class);
    pub fn sel_registerName(name: *const libc::c_char) -> SEL;
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
///     let string = "NSString".send("alloc").send("initWithCString:", "Hello world!");
pub trait ObjCMethodCall {
    unsafe fn send<S:ObjCSelector,A:ObjCMethodArgs>(self, selector: S, args: A) -> id;
    unsafe fn send_double<S:ObjCSelector,A:ObjCMethodDoubleArgs>(self, selector: S, args: A)
                          -> c_double;
    unsafe fn send_long<S:ObjCSelector,A:ObjCMethodLongArgs>(self, selector: S, args: A) -> c_long;
    unsafe fn send_void<S:ObjCSelector,A:ObjCMethodVoidArgs>(self, selector: S, args: A);
}

impl ObjCMethodCall for id {
    unsafe fn send<S:ObjCSelector,A:ObjCMethodArgs>(self, selector: S, args: A) -> id {
        args.send_args(self, selector.as_selector())
    }
    unsafe fn send_double<S:ObjCSelector,A:ObjCMethodDoubleArgs>(self, selector: S, args: A)
                          -> c_double {
        args.send_double_args(self, selector.as_selector())
    }
    unsafe fn send_long<S:ObjCSelector,A:ObjCMethodLongArgs>(self, selector: S, args: A)
                        -> c_long {
        args.send_long_args(self, selector.as_selector())
    }
    unsafe fn send_void<S:ObjCSelector,A:ObjCMethodVoidArgs>(self, selector: S, args: A) {
        args.send_void_args(self, selector.as_selector())
    }
}

/// A convenience implementation that allows methods on class names to be called directly, as in:
/// 
///     "NSString".send("alloc")
impl<'a> ObjCMethodCall for &'a str {
    unsafe fn send<S:ObjCSelector,A:ObjCMethodArgs>(self, selector: S, args: A) -> id {
        args.send_args(class(self), selector.as_selector())
    }
    unsafe fn send_double<S:ObjCSelector,A:ObjCMethodDoubleArgs>(self, selector: S, args: A)
                          -> c_double {
        args.send_double_args(class(self), selector.as_selector())
    }
    unsafe fn send_long<S:ObjCSelector,A:ObjCMethodLongArgs>(self, selector: S, args: A)
                        -> c_long {
        args.send_long_args(class(self), selector.as_selector())
    }
    unsafe fn send_void<S:ObjCSelector,A:ObjCMethodVoidArgs>(self, selector: S, args: A) {
        args.send_void_args(class(self), selector.as_selector())
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

impl ObjCMethodArgs for () {
    #[inline]
    unsafe fn send_args(self, receiver: id, selector: SEL) -> id {
        invoke_msg_id(receiver, selector)
    }
}

impl ObjCMethodArgs for (id, id, id, id, id) {
    #[inline]
    unsafe fn send_args(self, receiver: id, selector: SEL) -> id {
        let (first, second, third, fourth, fifth) = self;
        invoke_msg_id_id_id_id_id_id(receiver, selector, first, second, third, fourth, fifth)
    }
}

impl ObjCMethodArgs for NSRect {
    #[inline]
    unsafe fn send_args(self, receiver: id, selector: SEL) -> id {
        invoke_msg_id_NSRect(receiver, selector, &self)
    }
}

impl ObjCMethodDoubleArgs for () {
    #[inline]
    unsafe fn send_double_args(self, receiver: id, selector: SEL) -> f64 {
        invoke_msg_double(receiver, selector)
    }
}

impl ObjCMethodLongArgs for () {
    #[inline]
    unsafe fn send_long_args(self, receiver: id, selector: SEL) -> c_long {
        invoke_msg_long(receiver, selector)
    }
}

impl ObjCMethodVoidArgs for () {
    #[inline]
    unsafe fn send_void_args(self, receiver: id, selector: SEL) {
        invoke_msg_void(receiver, selector)
    }
}

impl ObjCMethodVoidArgs for bool {
    #[inline]
    unsafe fn send_void_args(self, receiver: id, selector: SEL) {
        invoke_msg_void_bool(receiver, selector, self)
    }
}

impl ObjCMethodVoidArgs for id {
    #[inline]
    unsafe fn send_void_args(self, receiver: id, selector: SEL) {
        invoke_msg_void_id(receiver, selector, self)
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

#[link(name = "msgsend", kind = "static")]
extern {
    pub fn invoke_msg_double(theReceiver: id, theSelector: SEL) -> f64;
    pub fn invoke_msg_id(theReceiver: id, theSelector: SEL) -> id;
    pub fn invoke_msg_id_id_id_id_id_id(theReceiver: id,
                                        theSelector: SEL,
                                        a: id,
                                        b: id,
                                        c: id,
                                        d: id,
                                        e: id)
                                        -> id;
    pub fn invoke_msg_id_NSRect(theReceiver: id, theSelector: SEL, a: &NSRect) -> id;
    pub fn invoke_msg_long(theReceiver: id, theSelector: SEL) -> c_long;
    pub fn invoke_msg_void(theReceiver: id, theSelector: SEL);
    pub fn invoke_msg_void_bool(theReceiver: id, theSelector: SEL, a: bool);
    pub fn invoke_msg_void_id(theReceiver: id, theSelector: SEL, a: id);
}

