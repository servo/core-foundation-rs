// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[allow(non_uppercase_statics)];

use base::{AbstractCFTypeRef, Boolean, CFAllocatorRef, CFTypeID, CFTypeRef, CFWrapper};
use base::{kCFAllocatorDefault};

use std::cast;
use std::libc;
use std::libc::c_void;

pub type CFNumberType = u32;

// members of enum CFNumberType
static kCFNumberSInt8Type:     CFNumberType = 1;
static kCFNumberSInt16Type:    CFNumberType = 2;
static kCFNumberSInt32Type:    CFNumberType = 3;
static kCFNumberSInt64Type:    CFNumberType = 4;
static kCFNumberFloat32Type:   CFNumberType = 5;
static kCFNumberFloat64Type:   CFNumberType = 6;
static kCFNumberCharType:      CFNumberType = 7;
static kCFNumberShortType:     CFNumberType = 8;
static kCFNumberIntType:       CFNumberType = 9;
static kCFNumberLongType:      CFNumberType = 10;
static kCFNumberLongLongType:  CFNumberType = 11;
static kCFNumberFloatType:     CFNumberType = 12;
static kCFNumberDoubleType:    CFNumberType = 13;
static kCFNumberCFIndexType:   CFNumberType = 14;
static kCFNumberNSIntegerType: CFNumberType = 15;
static kCFNumberCGFloatType:   CFNumberType = 16;
static kCFNumberMaxType:       CFNumberType = 16;

struct __CFNumber { private: () }
pub type CFNumberRef = *__CFNumber;

impl AbstractCFTypeRef for CFNumberRef {
    fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }

    #[fixed_stack_segment]
    fn type_id(_dummy: Option<CFNumberRef>) -> CFTypeID {
        unsafe {
            CFNumberGetTypeID()
        }
    }
}

pub struct CFNumber {
    contents: CFWrapper<CFNumberRef, (), ()>
}

impl CFNumber {
    pub fn wrap_owned(number: CFNumberRef) -> CFNumber {
        CFNumber {
            contents: CFWrapper::wrap_owned(number)
        }
    }

    pub fn wrap_shared(number: CFNumberRef) -> CFNumber {
        CFNumber {
            contents: CFWrapper::wrap_shared(number)
        }
    }

    #[fixed_stack_segment]
    pub fn new<T:Clone + ConvertibleToCFNumber>(n: T) -> CFNumber {
        unsafe {
            let objref = CFNumberCreate(kCFAllocatorDefault,
                                        n.cf_number_type(),
                                        cast::transmute::<&T, *c_void>(&n));
            CFNumber {
                contents: CFWrapper::wrap_owned(objref)
            }
        }
    }

    #[fixed_stack_segment]
    pub fn to_i8(&self) -> i8 {
        let ty = kCFNumberSInt8Type;
        assert!(self.has_number_type(ty));
        unsafe {
            let mut val: i8 = 0i8;
            if !CFNumberGetValue(self.contents.obj, ty, cast::transmute::<&mut i8, *mut c_void>(&mut val)) {
                fail!(~"Error in unwrapping CFNumber to i8");
            }
            return val;
        }
    }

    #[fixed_stack_segment]
    pub fn to_i16(&self) -> i16 {
        let ty = kCFNumberSInt16Type;
        assert!(self.has_number_type(ty));
        unsafe {
            let mut val: i16 = 0i16;
            if !CFNumberGetValue(self.contents.obj, ty, cast::transmute::<&mut i16, *mut c_void>(&mut val)) {
                fail!(~"Error in unwrapping CFNumber to i16");
            }
            return val;
        }
    }

    #[fixed_stack_segment]
    pub fn to_i32(&self) -> i32 {
        let ty = kCFNumberSInt32Type;
        assert!(self.has_number_type(ty));
        unsafe {
            let mut val: i32 = 0i32;
            if !CFNumberGetValue(self.contents.obj, ty, cast::transmute::<&mut i32, *mut c_void>(&mut val)) {
                fail!(~"Error in unwrapping CFNumber to i32");
            }
            return val;
        }
    }

    #[fixed_stack_segment]
    pub fn to_float(&self) -> float {
        unsafe {
            assert!(self.has_float_type());
            let ty = CFNumberGetType(self.contents.obj);
            if ty == kCFNumberFloat32Type || ty == kCFNumberFloatType {
                let mut val: libc::c_float = 0.0f as libc::c_float;
                if !CFNumberGetValue(self.contents.obj,
                                     ty,
                                     cast::transmute::<&mut libc::c_float, *mut c_void>(&mut val)) {
                    fail!(~"Error in unwrapping CFNumber to libc::c_float");
                }
                return val as float;
            }
            else if ty == kCFNumberFloat64Type || ty == kCFNumberDoubleType {
                let mut val: libc::c_double = 0.0f as libc::c_double;
                if !CFNumberGetValue(self.contents.obj,
                                     ty,
                                     cast::transmute::<&mut libc::c_double, *mut c_void>(&mut val)) {
                        fail!(~"Error in unwrapping CFNumber to libc::c_double");
                    }
                return val as float;
            }

            fail!(fmt!("Unable to wrap CFNumber into float: with type tag=%?", ty))
        }
    }

    #[fixed_stack_segment]
    fn has_float_type(&self) -> bool {
        unsafe {
            CFNumberIsFloatType(self.contents.obj) != 0
        }
    }

    #[fixed_stack_segment]
    fn has_number_type(&self, ty: CFNumberType) -> bool {
        unsafe {
            CFNumberGetType(self.contents.obj) == ty
        }
    }
}

pub trait ConvertibleToCFNumber {
    // FIXME: Should be static, but that breaks.
    fn cf_number_type(&self) -> CFNumberType;
}

impl ConvertibleToCFNumber for i8 {
    fn cf_number_type(&self) -> CFNumberType {
        kCFNumberSInt8Type as CFNumberType
    }
}

impl ConvertibleToCFNumber for i16 {
    fn cf_number_type(&self) -> CFNumberType {
        kCFNumberSInt16Type as CFNumberType
    }
}

impl ConvertibleToCFNumber for i32 {
    fn cf_number_type(&self) -> CFNumberType {
        kCFNumberSInt32Type as CFNumberType
    }
}

impl ConvertibleToCFNumber for i64 {
    fn cf_number_type(&self) -> CFNumberType {
        kCFNumberSInt64Type as CFNumberType
    }
}

impl ConvertibleToCFNumber for float {
    fn cf_number_type(&self) -> CFNumberType {
        kCFNumberFloatType as CFNumberType
    }
}

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    /*
     * CFNumber.h
     */

    static kCFNumberNaN: CFNumberRef;
    static kCFNumberNegativeInfinity: CFNumberRef;
    static kCFNumberPositiveInfinity: CFNumberRef;

    fn CFNumberCreate(allocator: CFAllocatorRef, theType: CFNumberType, valuePtr: *c_void)
                   -> CFNumberRef;
    //fn CFNumberGetByteSize
    fn CFNumberGetType(number: CFNumberRef) -> CFNumberType;
    fn CFNumberGetValue(number: CFNumberRef, theType: CFNumberType, valuePtr: *mut c_void) -> bool;
    fn CFNumberIsFloatType(number: CFNumberRef) -> Boolean;
    //fn CFNumberCompare
    fn CFNumberGetTypeID() -> CFTypeID;
}

#[test]
#[should_fail]
fn should_fail_on_bad_downcast() {
    use base;
    use boolean::CFBooleanRef;

    let CFNumber { contents: one } = CFNumber::new(1_i32);
    let one = CFWrapper::to_CFType(one);
    base::downcast::<CFBooleanRef>(*one.borrow_ref());
}
