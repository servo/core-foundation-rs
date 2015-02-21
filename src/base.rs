// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc;
use std::ffi;
use std::mem;

pub type Category = libc::intptr_t;
pub type Class = libc::intptr_t;
#[allow(non_camel_case_types)]
pub type id = libc::intptr_t;
pub type IMP = extern "C" fn(id, SEL) -> id;
pub type Ivar = libc::intptr_t;
pub type Method = libc::intptr_t;
#[allow(non_camel_case_types)]
pub type objc_AssociationPolicy = libc::intptr_t;
#[allow(non_camel_case_types)]
pub type objc_property_t = libc::intptr_t;
pub type Protocol = libc::intptr_t;
pub type SEL = libc::intptr_t;

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

pub type BOOL = libc::c_schar;
pub const NO: BOOL = 0;
pub const YES: BOOL = 1;

#[allow(non_camel_case_types, missing_copy_implementations)]
#[repr(C)]
pub struct objc_method_description {
    name: SEL,
    types: *mut libc::c_char,
}

#[allow(non_camel_case_types, missing_copy_implementations)]
#[repr(C)]
pub struct objc_property_attribute_t {
    name: *const libc::c_char,
    value: *const libc::c_char,
}

#[allow(non_camel_case_types, missing_copy_implementations)]
#[repr(C)]
pub struct objc_super {
    receiver: id,
    cls: Class,
    super_class: Class,
}

#[link(name = "objc")]
extern {
    pub fn class_addProperty(cls: Class,
                             name: *const libc::c_char,
                             attributes: *const objc_property_attribute_t,
                             attributeCount: libc::uint32_t) -> BOOL;
    pub fn class_addProtocol(cls: Class, protocol: *mut Protocol) -> BOOL;
    pub fn class_addMethod(cls: Class, name: SEL, imp: IMP, types: *const libc::c_char) -> BOOL;
    pub fn class_addIvar(cls: Class,
                         name: *const libc::c_char,
                         size: libc::size_t,
                         alignment: libc::uint8_t,
                         types: *const libc::c_char) -> BOOL;
    pub fn class_conformsToProtocol(cls: Class, protocol: *mut Protocol) -> BOOL;
    pub fn class_copyIvarList(cls: Class, outCount: *mut libc::uint32_t) -> *mut Ivar;
    pub fn class_copyMethodList(cls: Class, outCount: *mut libc::uint32_t) -> *mut Method;
    pub fn class_copyPropertyList(cls: Class,
                                  outCount: *mut libc::uint32_t) -> *mut objc_property_t;
    pub fn class_copyProtocolList(cls: Class, outCount: *mut libc::uint32_t) -> *mut *mut Protocol;
    pub fn class_createInstance(cls: Class, extraBytes: libc::size_t) -> id;
    pub fn class_getClassMethod(cls: Class, name: SEL) -> Method;
    pub fn class_getFutureClass(name: *const libc::c_char) -> Class;
    pub fn class_getIvarLayout(cls: Class) -> *const libc::uint8_t;
    pub fn class_getImageName(cls: Class) -> *const libc::c_char;
    pub fn class_getInstanceMethod(cls: Class, name: SEL) -> Method;
    pub fn class_getInstanceSize(cls: Class) -> libc::size_t;
    pub fn class_getInstanceVariable(cls: Class, name: *const libc::c_char) -> Ivar;
    pub fn class_getMethodImplementation(cls: Class, name: SEL) -> IMP;
    pub fn class_getMethodImplementation_stret(cls: Class, name: SEL) -> IMP;
    pub fn class_getName(cls: Class) -> *const libc::c_char;
    pub fn class_getProperty(cls: Class, name: *const libc::c_char) -> objc_property_t;
    pub fn class_getSuperclass(cls: Class) -> Class;
    pub fn class_getVersion(cls: Class) -> libc::int32_t;
    pub fn class_getWeakIvarLayout(cls: Class) -> *const libc::uint8_t;
    pub fn class_isMetaClass(cls: Class) -> BOOL;
    pub fn class_replaceMethod(cls: Class, name: SEL, imp: IMP, types: *const libc::c_char) -> IMP;
    pub fn class_replaceProperty(cls: Class,
                                 name: *const libc::c_char,
                                 attributes: *const objc_property_attribute_t,
                                 attributeCount: libc::uint32_t);
    pub fn class_respondsToSelector(cls: Class, sel: SEL) -> BOOL;
    pub fn class_setFutureClass(cls: Class, name: *const libc::c_char);
    pub fn class_setIvarLayout(cls: Class, layout: *const libc::uint8_t);
    pub fn class_setSuperclass(cls: Class, newSuper: Class);
    pub fn class_setWeakIvarLayout(cls: Class, layout: *const libc::uint8_t);
    pub fn class_setVersion(cls: Class, name: libc::int32_t);
    pub fn imp_getBlock(imp: IMP) -> id;
    pub fn imp_implementationWithBlock(block: id) -> IMP;
    pub fn imp_removeBlock(imp: IMP) -> BOOL;
    pub fn ivar_getName(ivar: Ivar) -> *const libc::c_char;
    pub fn ivar_getOffset(ivar: Ivar) -> libc::ptrdiff_t;
    pub fn ivar_getTypeEncoding(ivar: Ivar) -> *const libc::c_char;
    pub fn method_copyArgumentType(m: Method, index: libc::uint32_t) -> *mut libc::c_char;
    pub fn method_copyReturnType(m: Method) -> *mut libc::c_char;
    pub fn method_getArgumentType(m: Method,
                                  index: libc::uint32_t,
                                  dst: *mut libc::c_char,
                                  dst_len: libc::size_t);
    pub fn method_getDescription(m: Method) -> *mut objc_method_description;
    pub fn method_exchangeImplementations(m1: Method, m2: Method);
    pub fn method_getImplementation(m: Method) -> IMP;
    pub fn method_getName(m: Method) -> SEL;
    pub fn method_getNumberOfArguments(m: Method) -> libc::uint32_t;
    pub fn method_getReturnType(m: Method, dst: *mut libc::c_char, dst_len: libc::size_t);
    pub fn method_getTypeEncoding(m: Method) -> *const libc::c_char;
    pub fn method_invoke(receiver: id, m: Method, ...) -> id;
    pub fn method_invoke_stret(receiver: id, m: Method, ...);
    pub fn method_setImplementation(m: Method, imp: IMP) -> IMP;
    pub fn objc_allocateClassPair(superclass: Class,
                                  name: *const libc::c_char,
                                  extraBytes: libc::size_t) -> Class;
    pub fn objc_allocateProtocol(name: *const libc::c_char) -> *mut Protocol;
    pub fn objc_constructInstance(cls: Class, bytes: *mut libc::c_void) -> id;
    pub fn objc_copyClassList(outCount: *mut libc::uint32_t) -> *mut Class;
    pub fn objc_copyClassNamesForImage(image: *const libc::c_char,
                                       outCount: *mut libc::uint32_t) -> *const *const libc::c_char;
    pub fn objc_copyImageNames(outCount: *mut libc::uint32_t) -> *const *const libc::c_char;
    pub fn objc_copyProtocolList(outCount: *mut libc::uint32_t) -> *mut *mut Protocol;
    pub fn objc_destructInstance(obj: id) -> *mut libc::c_void;
    pub fn objc_disposeClassPair(cls: Class);
    pub fn objc_duplicateClass(original: Class,
                               name: *const libc::c_char,
                               extraBytes: libc::size_t) -> Class;
    pub fn objc_enumerationMutation(obj: id);
    pub fn objc_getAssociatedObject(object: id, key: *const libc::c_void) -> id;
    pub fn objc_getClass(name: *const libc::c_char) -> Class;
    pub fn objc_getClassList(buffer: *mut Class, bufferCount: libc::int32_t) -> libc::int32_t;
    pub fn objc_getMetaClass(name: *const libc::c_char) -> Class;
    pub fn objc_getProtocol(name: *const libc::c_char) -> *mut Protocol;
    pub fn objc_getRequiredClass(name: *const libc::c_char) -> Class;
    pub fn objc_loadWeak(location: *mut id) -> id;
    pub fn objc_storeWeak(location: *mut id, obj: id) -> id;
    pub fn objc_msgSend(target: id, selector: SEL, ...) -> id;
    pub fn objc_msgSend_fpret(target: id, selector: SEL, ...) -> libc::c_double;
    pub fn objc_msgSend_stret(target: id, selector: SEL, ...);
    pub fn objc_msgSendSuper(sup: *mut objc_super, op: SEL, ...) -> id;
    pub fn objc_msgSendSuper_stret(sup: *mut objc_super, op: SEL, ...);
    pub fn objc_registerClassPair(cls: Class);
    pub fn objc_registerProtocol(proto: *mut Protocol);
    pub fn objc_removeAssociatedObjects(object: id);
    pub fn objc_setAssociatedObject(object: id,
                                    key: *const libc::c_void,
                                    value: id,
                                    policy: objc_AssociationPolicy);
    pub fn objc_setEnumerationMutationHandler(handler: extern fn (id));
    pub fn object_copy(obj: id, size: libc::size_t) -> id;
    pub fn object_dispose(obj: id) -> id;
    pub fn object_getClass(obj: id) -> Class;
    pub fn object_getClassName(obj: id) -> *const libc::c_char;
    pub fn object_getIndexedIvars(obj: id) -> *mut libc::c_void;
    pub fn object_getInstanceVariable(obj: id,
                                      name: *const libc::c_char,
                                      outValue: *mut *mut libc::c_void);
    pub fn object_getIvar(obj: id, ivar: Ivar) -> id;
    pub fn object_setClass(obj: id, cls: Class) -> Class;
    pub fn object_setInstanceVariable(obj: id,
                                      name: *const libc::c_char,
                                      value: *mut libc::c_void);
    pub fn object_setIvar(obj: id, ivar: Ivar, value: id);
    pub fn protocol_addMethodDescription(proto: *mut Protocol,
                                         name: SEL,
                                         types: *const libc::c_char,
                                         isRequiredMethod: BOOL,
                                         isInstanceMethod: BOOL);
    pub fn protocol_addProperty(proto: *mut Protocol,
                                name: *const libc::c_char,
                                attributes: *const objc_property_attribute_t,
                                attributeCount: libc::uint32_t,
                                isRequiredMethod: BOOL,
                                isInstanceMethod: BOOL);
    pub fn protocol_addProtocol(proto: *mut Protocol, addition: *mut Protocol);
    pub fn protocol_conformsToProtocol(proto: *mut Protocol, other: *mut Protocol) -> BOOL;
    pub fn protocol_copyMethodDescriptionList(proto: *mut Protocol,
                                              isRequiredMethod: BOOL,
                                              isInstanceMethod: BOOL,
                                              outCount: *mut libc::uint32_t
                                             ) -> *mut objc_method_description;
    pub fn protocol_copyPropertyList(proto: *mut Protocol,
                                     outCount: *mut libc::uint32_t) -> *mut objc_property_t;
    pub fn protocol_copyProtocolList(proto: *mut Protocol,
                                     outCount: *mut libc::uint32_t) -> *mut *mut Protocol;
    pub fn protocol_getMethodDescription(proto: *mut Protocol,
                                         name: SEL,
                                         isRequiredMethod: BOOL,
                                         isInstanceMethod: BOOL) -> objc_method_description;
    pub fn protocol_getProperty(proto: *mut Protocol,
                                name: *const libc::c_char,
                                isRequiredMethod: BOOL,
                                isInstanceMethod: BOOL) -> objc_property_t;
    pub fn protocol_getName(proto: *mut Protocol) -> *const libc::c_char;
    pub fn protocol_isEqual(proto: *mut Protocol, other: *mut Protocol) -> BOOL;
    pub fn property_copyAttributeList(property: objc_property_t,
                                      outCount: *mut libc::uint32_t
                                     ) -> *mut objc_property_attribute_t;
    pub fn property_copyAttributeValue(property: objc_property_t,
                                       attributeName: *const libc::c_char) -> *mut libc::c_char;
    pub fn property_getAttributes(property: objc_property_t) -> *const libc::c_char;
    pub fn property_getName(property: objc_property_t) -> *const libc::c_char;
    pub fn sel_getName(sel: SEL) -> *const libc::c_char;
    pub fn sel_getUid(string: *const libc::c_char) -> SEL;
    pub fn sel_isEqual(lhs: SEL, rhs: SEL) -> BOOL;
    pub fn sel_registerName(name: *const libc::c_char) -> SEL;
}

/// Returns an Objective-C message send function that returns a type `T`.
///
/// # Example
///
/// ```
/// # use cocoa::base::{class, selector, msg_send, nil};
/// assert!(unsafe { msg_send()(class("NSObject"), selector("class")) } != nil);
/// ```
pub unsafe fn msg_send<T>() -> extern fn(target: id, selector: SEL, ...) -> T {
    mem::transmute(objc_msgSend)
}

/// Returns an Objective-C message send function that returns a floating-point type.
pub unsafe fn msg_send_fpret<T>() -> extern fn(target: id, selector: SEL, ...) -> T {
    mem::transmute(objc_msgSend_fpret)
}

/// Returns an Objective-C message send function that returns a type `T`, where `T` is a struct
/// with the attribute `#[repr(C)]`.
pub unsafe fn msg_send_stret<T>() -> extern fn(target: id, selector: SEL, ...) -> T {
    mem::transmute(objc_msgSend_stret)
}

/// A convenience method to convert the name of a class to the class object itself.
#[inline]
pub fn class(name: &str) -> Class {
    let name_c_str = ffi::CString::from_slice(name.as_bytes());
    unsafe {
        objc_getClass(name_c_str.as_ptr())
    }
}

/// A convenience method to convert the name of a selector to the selector object.
#[inline]
pub fn selector(name: &str) -> SEL {
    let name_c_str = ffi::CString::from_slice(name.as_bytes());
    unsafe {
        sel_registerName(name_c_str.as_ptr())
    }
}

#[cfg(test)]
mod test {
    use libc;
    use std::ffi;
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
        let name_c_str = ffi::CString::from_slice("MyObject".as_bytes());
        let my_object = unsafe {
            objc_allocateClassPair(ns_object, name_c_str.as_ptr(), 0 as libc::size_t)
        };

        let doSomething = selector("doSomething");
        let types_c_str = ffi::CString::from_slice("@@:".as_bytes());
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
