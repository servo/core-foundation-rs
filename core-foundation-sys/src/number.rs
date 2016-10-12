// Copyright 2013-2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Binds the `CFNumber` type.

use allocator::CFAllocator;
use base::{CFComparisonResult, CFDowncast, CFIndex, CFObject, CFType};
use base::{CFTypeID, FromCFIndex};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::os::raw::c_void;
use sync::{CFRef, CFShared};

pub type CFNumberRef = CFRef<CFNumber>;

/// Encapsulates C scalar numeric types.
#[repr(C)]
pub struct CFNumber { obj: CFObject }

unsafe impl Send for CFNumber {}
unsafe impl Sync for CFNumber {}

unsafe impl CFType for CFNumber {
    #[inline]
    fn as_object(&self) -> &CFObject {
        &self.obj
    }
}

unsafe impl CFDowncast for CFNumber {
    #[inline]
    fn type_id() -> CFTypeID {
        unsafe { CFNumberGetTypeID() }
    }
}

impl CFNumber {
    /// Returns the number of bytes used by this number to store its value.
    #[inline]
    pub fn number_type(&self) -> CFNumberType {
        unsafe { CFNumberGetType(self) }
    }

    /// Returns the type used by this number to store its value.
    #[inline]
    pub fn byte_size(&self) -> usize {
        unsafe { usize::from_index(CFNumberGetByteSize(self)) }
    }

    /// Returns whether this number represents a floating-point value.
    #[inline]
    pub fn is_float(&self) -> bool {
        unsafe { CFNumberIsFloatType(self) }
    }
}

pub trait FromCFNumber: Sized {
    #[inline]
    fn from_number(number: &CFNumber) -> Result<Self, ()>;
}

macro_rules! number_conversion {
    ($type_:ident, $number_type:ident, $from:ident, $to:ident, $test:ident) => {
        impl CFNumber {
            /// Creates a new `CFNumberRef` from a primitive type.
            ///
            /// Also available as `CFNumberRef::from`.
            #[inline]
            pub fn $from(input: $type_) -> CFNumberRef {
                unsafe {
                    CFRef::from_retained(
                        CFNumberCreate(
                            None,
                            CFNumberType::$number_type,
                            &input as *const _ as *const _))
                }
            }

            /// Converts this number to a primitive type or returns an error.
            ///
            /// Also available as `FromCFNumber::from_number`.
            #[inline]
            pub fn $to(&self) -> Result<$type_, ()> {
                let mut value = $type_::default();
                let success = unsafe {
                    CFNumberGetValue(
                        self,
                        CFNumberType::$number_type,
                        &mut value as *mut _ as *mut _)
                };
                if success {
                    Ok(value)
                } else {
                    Err(())
                }
            }
        }

        impl FromCFNumber for $type_ {
            #[inline]
            fn from_number(input: &CFNumber) -> Result<Self, ()> {
                input.$to()
            }
        }

        impl From<$type_> for CFNumberRef {
            #[inline]
            fn from(input: $type_) -> Self {
                CFNumber::$from(input)
            }
        }
    }
}

number_conversion!(i8, SInt8, from_i8, to_i8, test_i8);
number_conversion!(i16, SInt16, from_i16, to_i16, test_i16);
number_conversion!(i32, SInt32, from_i32, to_i32, test_i32);
number_conversion!(i64, SInt64, from_i64, to_i64, test_i64);
number_conversion!(f32, Float32, from_f32, to_f32, test_f32);
number_conversion!(f64, Float64, from_f64, to_f64, test_f64);

impl Eq for CFNumber {}

impl PartialEq for CFNumber {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Ord for CFNumber {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        unsafe {
            let context = CFNumberCompareContext::default();
            CFNumberCompare(self, other, context).into()
        }
    }
}

impl PartialOrd for CFNumber {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[repr(u64)]
pub enum CFNumberType {
    SInt8 = 1,
    SInt16 = 2,
    SInt32 = 3,
    SInt64 = 4,
    Float32 = 5,
    Float64 = 6,
    Char = 7,
    Short = 8,
    Int = 9,
    Long = 10,
    LongLong = 11,
    Float = 12,
    Double = 13,
    CFIndex = 14,
    NSInteger = 15,
    CGFloat = 16,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct CFNumberCompareContext { cx: usize }

impl CFNumberCompareContext {
    #[inline]
    pub fn new() -> Self {
        CFNumberCompareContext { cx: 0 }
    }
}

impl Default for CFNumberCompareContext {
    #[inline]
    fn default() -> Self {
        CFNumberCompareContext::new()
    }
}

extern {
    pub fn CFNumberGetTypeID() -> CFTypeID;

    pub fn CFNumberCreate(
            allocator: Option<&'static CFAllocator>,
            theType: CFNumberType,
            valuePtr: *const c_void)
            -> *const CFShared<CFNumber>;

    pub fn CFNumberGetType(number: &CFNumber) -> CFNumberType;
    pub fn CFNumberGetByteSize(number: &CFNumber) -> CFIndex;
    pub fn CFNumberIsFloatType(number: &CFNumber) -> bool;

    pub fn CFNumberGetValue(
            number: &CFNumber, theType: CFNumberType, valuePtr: *mut c_void)
            -> bool;

    pub fn CFNumberCompare(
            number: &CFNumber,
            otherNumber: &CFNumber,
            context: CFNumberCompareContext)
            -> CFComparisonResult;
}
