// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cast;
use std::libc;
use std::libc::c_long;

// a raw Core Foundation reference. It may or may not have been
// CFRetain'ed, depending on whether it was obtained via ownership or
// borrow semantics. 
pub trait AbstractCFTypeRef {
    fn as_type_ref(&self) -> CFTypeRef;
    fn type_id() -> CFTypeID;
}

pub type Boolean = u8;

pub type CFIndex = c_long;
pub type CFOptionFlags = u32;
pub struct CFRange {
    location: CFIndex,
    length: CFIndex
}

pub fn CFRangeMake(off: CFIndex, len: CFIndex) -> CFRange {
    CFRange { location: off, length: len }
}

struct __CFAllocator { private: () }
pub type CFAllocatorRef = *__CFAllocator;

struct __CFNull { private: () }
pub type CFNullRef = *__CFNull;

pub type CFHashCode = libc::c_ulong;
pub type CFTypeID = libc::c_ulong;

struct __CFType { private: () }
pub type CFTypeRef = *__CFType;


impl AbstractCFTypeRef for CFTypeRef {
    fn as_type_ref(&self) -> CFTypeRef { *self }
    // this can't be used, because CFType is the supertype and has no type id.
    fn type_id() -> CFTypeID { fail!(); }
}

pub fn downcast<T:AbstractCFTypeRef>(r: CFTypeRef) -> T {
    unsafe {
        assert!(CFGetTypeID(r) == AbstractCFTypeRef::type_id::<T>());
        cast::transmute::<CFTypeRef, T>(r)
    }
}

pub struct RawCFWrapper {
    obj: CFTypeRef
}

pub struct CFWrapper<T, PlaceholderType1, PlaceholderType2> {
    obj: T
}

#[unsafe_destructor]
impl<T,E1,E2> Drop for CFWrapper<T,E1,E2> {
    fn finalize(&self) {
        unsafe {
            // sadly, cannot use obj.as_type_ref() here, because drop
            // cannot make virtual method calls using trait
            // types. Instead, just transmute the bugger.
            let this: &RawCFWrapper = cast::transmute::<&CFWrapper<T,E1,E2>, &RawCFWrapper>(self);
            assert!(CFGetRetainCount(this.obj) > 0 as CFIndex);
            CFRelease(this.obj)
        }
    }
}

pub type CFType = CFWrapper<CFTypeRef, (), ()>;

impl<'self, T:Copy + AbstractCFTypeRef, E1, E2> CFWrapper<T,E1,E2> {
    pub fn borrow_ref(&'self self) -> &'self T {
        &self.obj
    }

    pub fn borrow_type_ref(&self) -> &'self CFTypeRef {
        unsafe {
            cast::transmute::<&T, &CFTypeRef>(&self.obj)
        }
    }

    // Use this when following Core Foundation's "Create" rule; i.e., the wrapper assumes ownership.
    // The object has already been retained, so we need not increment the retain count ourself.
    pub fn wrap_owned(some_ref: T) -> CFWrapper<T,E1,E2> {
        // N.B. we can't make any assertions about retain count here,
        // because returned things are only guaranteed to be already
        // retained. Strings, for example, could be interned.
        CFWrapper { obj: some_ref }
    }

    // Use this when following Core Foundation's "Get" rule. The wrapper does not have ownership.
    // Twe need to increment object's the retain count so it isn't freed out from under our noses.
    pub fn wrap_shared(some_ref: T) -> CFWrapper<T,E1,E2> {
        unsafe { CFRetain(some_ref.as_type_ref()); }
        CFWrapper { obj: some_ref }
    }

    // Unwraps the wrapper, returning the underlying AbstractCFType.
    pub fn unwrap(wrapper: CFWrapper<T,E1,E2>) -> T {
        copy wrapper.obj
    }

    pub fn to_CFType(wrapper: CFWrapper<T,E1,E2>) -> CFType {
        unsafe {
            cast::transmute::<CFWrapper<T, E1, E2>, CFType>(wrapper)
        }
    }

    pub fn from_CFType(wrapper: CFType) -> CFWrapper<T,E1,E2> {
        unsafe {
            assert!(wrapper.type_id() == AbstractCFTypeRef::type_id::<T>());
            cast::transmute::<CFType,CFWrapper<T,E1,E2>>(wrapper)
        }
    }

    pub fn clone(wrapper: &CFWrapper<T,E1,E2>) -> CFWrapper<T,E1,E2> {
        CFWrapper::wrap_shared(copy *wrapper.borrow_ref())
    }

    pub fn retain_count(&self) -> CFIndex {
        unsafe {
            CFGetRetainCount(*self.borrow_type_ref())
        }
    }

    pub fn type_id(&self) -> CFTypeID {
        unsafe {
            CFGetTypeID(*self.borrow_type_ref())
        }
    }

    pub fn show(&self) {
        unsafe {
            CFShow(*self.borrow_type_ref());
        }
    }
}

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    /*
     * CFBase.h
     */

    /* CFAllocator Reference */
    // N.B. Many CFAllocator functions and constants are omitted here.
    pub static kCFAllocatorDefault: CFAllocatorRef;
    pub static kCFAllocatorSystemDefault: CFAllocatorRef;
    pub static kCFAllocatorMalloc: CFAllocatorRef;
    pub static kCFAllocatorMallocZone: CFAllocatorRef;
    pub static kCFAllocatorNull: CFAllocatorRef;
    pub static kCFAllocatorUseContext: CFAllocatorRef;

    /* CFNull Reference */

    pub static kCFNull: CFNullRef;

    /* CFType Reference */

    //fn CFCopyDescription
    //fn CFCopyTypeIDDescription
    //fn CFEqual
    //fn CFGetAllocator
    pub fn CFGetRetainCount(cf: CFTypeRef) -> CFIndex;
    pub fn CFGetTypeID(cf: CFTypeRef) -> CFTypeID;
    pub fn CFHash(cf: CFTypeRef) -> CFHashCode;
    //fn CFMakeCollectable
    pub fn CFRelease(cf: CFTypeRef);
    pub fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    pub fn CFShow(obj: CFTypeRef);

    /* Base Utilities Reference */
    // N.B. Some things missing here.
}
