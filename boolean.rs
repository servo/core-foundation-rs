// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base::{
    AbstractCFTypeRef,
    CFTypeRef,
    CFTypeID,
    CFWrapper,
};

pub type Boolean = u32;

struct __CFBoolean { private: () }
pub type CFBooleanRef = *__CFBoolean;

impl AbstractCFTypeRef for CFBooleanRef {
    fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }

    #[fixed_stack_segment]
    fn type_id() -> CFTypeID {
        unsafe { CFBooleanGetTypeID() }
    }
}

// FIXME: Should be a newtype struct, but that fails due to a Rust compiler
// bug.
pub struct CFBoolean {
    contents: CFWrapper<CFBooleanRef, (), ()>
}

impl CFBoolean {
    pub fn true_value() -> CFBoolean {
        CFBoolean {
            contents: CFWrapper::wrap_shared(kCFBooleanTrue)
        }
    }

    pub fn false_value() -> CFBoolean {
        CFBoolean {
            contents: CFWrapper::wrap_shared(kCFBooleanFalse)
        }
    }
}

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    static kCFBooleanTrue: CFBooleanRef;
    static kCFBooleanFalse: CFBooleanRef;

    fn CFBooleanGetValue(boolean: CFBooleanRef) -> Boolean;
    fn CFBooleanGetTypeID() -> CFTypeID;
}

