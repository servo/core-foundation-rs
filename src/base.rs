// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc::c_char;
use libc;

use std::ffi::CString;
use std::mem;

pub type Class = libc::intptr_t;
pub type IMP = extern "C" fn(id, SEL) -> id;
pub type Ivar = libc::intptr_t;
pub type SEL = libc::intptr_t;
#[allow(non_camel_case_types)]
pub type id = libc::intptr_t;

#[cfg(target_pointer_width = "32")]
pub type NSInteger = libc::c_int;
#[cfg(target_pointer_width = "32")]
pub type NSUInteger = libc::c_uint;

#[cfg(target_pointer_width = "64")]
pub type NSInteger = libc::c_long;
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = libc::c_ulong;

#[allow(non_upper_case_globals)]
pub const nil: id = 0;
#[allow(non_upper_case_globals)]
pub const Nil: Class = 0 as Class;

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
    pub fn objc_msgSend_stret(theReceiver: id, theSelector: SEL, ...);
    pub fn objc_registerClassPair(cls: Class);
    pub fn sel_registerName(name: *const libc::c_char) -> SEL;
}

/// Returns an Objective-C message send function that returns a type `T`.
pub unsafe fn msg_send<T>() -> extern fn(theReceiver: id, theSelector: SEL, ...) -> T {
    mem::transmute(objc_msgSend)
}

pub unsafe fn msg_send_stret<T>() -> extern fn(theReceiver: id, theSelector: SEL, ...) -> T {
    mem::transmute(objc_msgSend_stret)
}

/// A convenience method to convert the name of a class to the class object itself.
#[inline]
pub fn class(name: &str) -> id {
    let name_c_str = CString::from_slice(name.as_bytes());
    unsafe {
        objc_getClass(name_c_str.as_ptr())
    }
}

/// A convenience method to convert the name of a selector to the selector object.
#[inline]
pub fn selector(name: &str) -> SEL {
    let name_c_str = CString::from_slice(name.as_bytes());
    unsafe {
        sel_registerName(name_c_str.as_ptr())
    }
}

#[cfg(test)]
mod test {
    use libc;
    use std::ffi::CString;
    use super::*;

    #[test]
    pub fn test_nsapp() {
        unsafe {
            let _nsApp: id = msg_send()(class("NSApplication"), selector("sharedApplication"));
        }
    }

    #[test]
    pub fn test_custom_obj() {
        extern fn MyObject_doSomething(this: id, _: SEL) -> id {
            println!("doSomething");
            return this;
        }

        let ns_object = class("NSObject");
        let name_c_str = CString::from_slice("MyObject".as_bytes());
        let my_object = unsafe {
            objc_allocateClassPair(ns_object, name_c_str.as_ptr(), 0 as libc::size_t)
        };

        let doSomething = selector("doSomething");
        let types_c_str = CString::from_slice("@@:".as_bytes());
        unsafe {
            let _ = class_addMethod(my_object, doSomething, MyObject_doSomething,
                                    types_c_str.as_ptr());

            objc_registerClassPair(my_object);

            let mut obj: id = msg_send()(my_object, selector("alloc"));
            obj = msg_send()(obj, selector("init"));
            let _: () = msg_send()(obj, selector("doSomething"));
        }
    }
}
