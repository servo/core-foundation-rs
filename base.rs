// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use appkit::NSRect;

use std::libc;
use std::libc::c_long;

pub type id = libc::intptr_t;
pub type Class = libc::intptr_t;
pub type IMP = extern "C" fn(id, SEL) -> id;
pub type SEL = libc::intptr_t;
pub type Ivar = libc::intptr_t;

pub static NIL : id = 0 as id;

extern {
    fn class_addMethod(cls: Class, name: SEL, imp: IMP, types: *libc::c_char) -> bool;
    fn class_addIvar(cls : Class,
                     name : *libc::c_char,
                     size : libc::size_t,
                     alignment: u8,
		             types: *libc::c_char) -> bool;
    fn object_setInstanceVariable(obj : id,
                                  name : *libc::c_char,
                                  value : *libc::c_void);
    fn object_getInstanceVariable(obj : id,
                                  name : *libc::c_char,
                                  outValue : **libc::c_void);
    fn objc_allocateClassPair(superclass : Class,
                              name : *libc::c_char,
                              extraBytes : libc::size_t) -> Class;
    fn objc_getClass(name : *libc::c_char) -> id;
    fn objc_msgSend(theReceiver : id, theSelector : SEL) -> id;
    fn objc_registerClassPair(cls : Class);
    fn sel_registerName(name : *libc::c_char) -> SEL;
}

#[cfg(test)]
mod test {
    use std::{io, libc};
    use super::*;

    #[test]
    #[fixed_stack_segment]
    pub fn test_nsapp() {
        let klass = do "NSApplication".to_c_str().with_ref |s| {
            unsafe {
                objc_getClass(s)
            }
        };

        let sel = do "sharedApplication".to_c_str().with_ref |s| {
            unsafe {
                sel_registerName(s)
            }
        };

        unsafe {
            let nsapp = objc_msgSend(klass, sel);
            io::println(fmt!("nsapp: %d", (nsapp as int)));
        }
    }

    #[test]
    #[fixed_stack_segment]
    pub fn test_custom_obj() {
        extern fn MyObject_doSomething(this : id, _sel : SEL) -> id {
            io::println("doSomething");
            return this;
        }

        let NSObject = do "NSObject".to_c_str().with_ref |s| {
            unsafe {
                objc_getClass(s)
            }
        };
        let MyObject = do "MyObject".to_c_str().with_ref |s| {
            unsafe {
                objc_allocateClassPair(NSObject, s, 0 as libc::size_t)
            }
        };
        let doSomething = do "doSomething".to_c_str().with_ref |s| {
            unsafe {
                sel_registerName(s)
            }
        };
        let _ = do "@@:".to_c_str().with_ref |types| {
            unsafe {
                class_addMethod(MyObject,
                                doSomething,
                                MyObject_doSomething,
                                types)
            }
        };

        unsafe {
            objc_registerClassPair(MyObject);
        }

        let alloc = do "alloc".to_c_str().with_ref |s| {
            unsafe { sel_registerName(s) }
        };
        let init = do "init".to_c_str().with_ref |s| {
            unsafe { sel_registerName(s) }
        };

        unsafe {
            let mut obj = objc_msgSend(MyObject, alloc);
            obj = objc_msgSend(obj, init);
            objc_msgSend(obj, doSomething);
        }
    }
}

/// Invokes the given selector, which must have the signature:
///
///     double f();
#[fixed_stack_segment]
pub fn msg_send_double(theReceiver: id, theSelector: SEL) -> f64 {
    unsafe {
        invoke_msg_double(theReceiver, theSelector)
    }
}

/// Invokes the given selector, which must have the signature:
///
///     id f();
#[fixed_stack_segment]
pub fn msg_send_id(theReceiver: id, theSelector: SEL) -> id {
    unsafe {
        invoke_msg_id(theReceiver, theSelector)
    }
}

/// Invokes the given selector, which must have the signature:
///
///     id f(NSRect a);
#[fixed_stack_segment]
pub fn msg_send_id_NSRect(theReceiver: id, theSelector: SEL, a: NSRect) -> id {
    unsafe {
        invoke_msg_id_NSRect(theReceiver, theSelector, &a)
    }
}

/// Invokes the given selector, which must have the signature:
///
///     id f(id a, id b, id c, id e, id f);
#[fixed_stack_segment]
pub fn msg_send_id_id_id_id_id_id(theReceiver: id,
                                  theSelector: SEL,
                                  a: id,
                                  b: id,
                                  c: id,
                                  d: id,
                                  e: id)
                                  -> id {
    unsafe {
        invoke_msg_id_id_id_id_id_id(theReceiver, theSelector, a, b, c, d, e)
    }
}

/// Invokes the given selector, which must have the signature:
///
///     long f();
#[fixed_stack_segment]
pub fn msg_send_long(theReceiver: id, theSelector: SEL) -> c_long {
    unsafe {
        invoke_msg_long(theReceiver, theSelector)
    }
}

/// Invokes the given selector, which must have the signature:
///
///     void f();
#[fixed_stack_segment]
pub fn msg_send_void(theReceiver: id, theSelector: SEL) {
    unsafe {
        invoke_msg_void(theReceiver, theSelector)
    }
}

/// Invokes the given selector, which must have the signature:
///
///     void f(BOOL a);
#[fixed_stack_segment]
pub fn msg_send_void_bool(theReceiver: id, theSelector: SEL, a: bool) {
    unsafe {
        invoke_msg_void_bool(theReceiver, theSelector, a)
    }
}

/// Invokes the given selector, which must have the signature:
///
///     void f(id a);
#[fixed_stack_segment]
pub fn msg_send_void_id(theReceiver: id, theSelector: SEL, a: id) {
    unsafe {
        invoke_msg_void_id(theReceiver, theSelector, a)
    }
}

#[link_args = "-L. -lmsgsend"]
#[nolink]
extern {
    fn invoke_msg_double(theReceiver: id, theSelector: SEL) -> f64;
    fn invoke_msg_id(theReceiver: id, theSelector: SEL) -> id;
    fn invoke_msg_id_id_id_id_id_id(theReceiver: id,
                                    theSelector: SEL,
                                    a: id,
                                    b: id,
                                    c: id,
                                    d: id,
                                    e: id)
                                    -> id;
    fn invoke_msg_id_NSRect(theReceiver: id, theSelector: SEL, a: &NSRect) -> id;
    fn invoke_msg_long(theReceiver: id, theSelector: SEL) -> c_long;
    fn invoke_msg_void(theReceiver: id, theSelector: SEL);
    fn invoke_msg_void_bool(theReceiver: id, theSelector: SEL, a: bool);
    fn invoke_msg_void_id(theReceiver: id, theSelector: SEL, a: id);
}

