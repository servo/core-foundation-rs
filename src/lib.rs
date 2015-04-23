// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "io_surface"]
#![crate_type = "rlib"]

#![feature(core)]

extern crate libc;
extern crate core_foundation;
extern crate geom;
extern crate cgl;
extern crate gleam;

// Rust bindings to the IOSurface framework on Mac OS X.

#[cfg(target_os="macos")]
use core_foundation::base::{CFRelease, CFRetain, CFTypeID, CFTypeRef, TCFType};
#[cfg(target_os="macos")]
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
#[cfg(target_os="macos")]
use core_foundation::string::CFStringRef;
#[cfg(target_os="macos")]
use geom::size::Size2D;
#[cfg(target_os="macos")]
use cgl::{kCGLNoError, CGLGetCurrentContext, CGLTexImageIOSurface2D};
#[cfg(target_os="macos")]
use gleam::gl::{BGRA, GLenum, RGBA, TEXTURE_RECTANGLE_ARB, UNSIGNED_INT_8_8_8_8_REV};
#[cfg(target_os="macos")]
use libc::{c_int, c_void, size_t};
#[cfg(target_os="macos")]
use std::mem;
#[cfg(target_os="macos")]
use std::slice::bytes::copy_memory;
use std::slice;


//static kIOSurfaceLockReadOnly: u32 = 0x1;
//static kIOSurfaceLockAvoidSync: u32 = 0x2;

#[cfg(target_os="macos")]
type IOReturn = c_int;

#[cfg(target_os="macos")]
#[repr(C)]
struct __IOSurface;

#[cfg(target_os="macos")]
pub type IOSurfaceRef = *const __IOSurface;

#[cfg(target_os="macos")]
#[repr(C)]
pub struct IOSurface {
    pub obj: IOSurfaceRef,
}

#[cfg(target_os="macos")]
impl Drop for IOSurface {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

#[cfg(target_os="macos")]
pub type IOSurfaceID = u32;

#[cfg(target_os="macos")]
impl Clone for IOSurface {
    #[inline]
    fn clone(&self) -> IOSurface {
        unsafe {
            TCFType::wrap_under_get_rule(self.obj)
        }
    }
}

#[cfg(target_os="macos")]
impl TCFType<IOSurfaceRef> for IOSurface {
    #[inline]
    fn as_concrete_TypeRef(&self) -> IOSurfaceRef {
        self.obj
    }

    #[inline]
    unsafe fn wrap_under_get_rule(reference: IOSurfaceRef) -> IOSurface {
        let reference: IOSurfaceRef = mem::transmute(CFRetain(mem::transmute(reference)));
        TCFType::wrap_under_create_rule(reference)
    }

    #[inline]
    fn as_CFTypeRef(&self) -> CFTypeRef {
        unsafe {
            mem::transmute(self.as_concrete_TypeRef())
        }
    }

    #[inline]
    unsafe fn wrap_under_create_rule(obj: IOSurfaceRef) -> IOSurface {
        IOSurface {
            obj: obj,
        }
    }

    #[inline]
    fn type_id() -> CFTypeID {
        unsafe {
            IOSurfaceGetTypeID()
        }
    }
}

#[cfg(target_os="macos")]
pub fn new(properties: &CFDictionary) -> IOSurface {
    unsafe {
        TCFType::wrap_under_create_rule(IOSurfaceCreate(properties.as_concrete_TypeRef()))
    }
}

/// Looks up an `IOSurface` by its global ID.
///
/// FIXME(pcwalton): This should return an `Option`.
#[cfg(target_os="macos")]
pub fn lookup(csid: IOSurfaceID) -> IOSurface {
    unsafe {
        TCFType::wrap_under_create_rule(IOSurfaceLookup(csid))
    }
}

#[cfg(target_os="macos")]
impl IOSurface {
    pub fn get_id(&self) -> IOSurfaceID {
        unsafe {
            IOSurfaceGetID(self.as_concrete_TypeRef())
        }
    }

    /// Binds to the current GL texture.
    pub fn bind_to_gl_texture(&self, size: Size2D<i32>) {
        unsafe {
            let context = CGLGetCurrentContext();
            let gl_error = CGLTexImageIOSurface2D(context,
                                                  TEXTURE_RECTANGLE_ARB,
                                                  RGBA as GLenum,
                                                  size.width,
                                                  size.height,
                                                  BGRA as GLenum,
                                                  UNSIGNED_INT_8_8_8_8_REV,
                                                  mem::transmute(self.as_concrete_TypeRef()),
                                                  0);

            assert_eq!(gl_error, kCGLNoError);
        }
    }

    pub fn upload(&self, data: &[u8]) {
        unsafe {
            let surface = self.as_concrete_TypeRef();
            let mut seed = 0;

            IOSurfaceLock(surface, 0, &mut seed);

            let height = IOSurfaceGetHeight(surface);
            let stride = IOSurfaceGetBytesPerRow(surface);
            let size = (height * stride) as usize;
            let address = IOSurfaceGetBaseAddress(surface) as *mut u8;
            let dest: &mut [u8] = slice::from_raw_parts_mut(address, size);
            copy_memory(data, dest);

            // FIXME(pcwalton): RAII
            IOSurfaceUnlock(surface, 0, &mut seed);
        }
    }
}

#[cfg(target_os="macos")]
#[link(name = "IOSurface", kind = "framework")]
extern {
    pub static kIOSurfaceAllocSize: CFStringRef;
    pub static kIOSurfaceWidth: CFStringRef;
    pub static kIOSurfaceHeight: CFStringRef;
    pub static kIOSurfaceBytesPerRow: CFStringRef;
    pub static kIOSurfaceBytesPerElement: CFStringRef;
    pub static kIOSurfaceElementWidth: CFStringRef;
    pub static kIOSurfaceElementHeight: CFStringRef;
    pub static kIOSurfaceOffset: CFStringRef;

    pub static kIOSurfacePlaneInfo: CFStringRef;
    pub static kIOSurfacePlaneWidth: CFStringRef;
    pub static kIOSurfacePlaneHeight: CFStringRef;
    pub static kIOSurfacePlaneBytesPerRow: CFStringRef;
    pub static kIOSurfacePlaneOffset: CFStringRef;
    pub static kIOSurfacePlaneSize: CFStringRef;

    pub static kIOSurfacePlaneBase: CFStringRef;
    pub static kIOSurfacePlaneBytesPerElement: CFStringRef;
    pub static kIOSurfacePlaneElementWidth: CFStringRef;
    pub static kIOSurfacePlaneElementHeight: CFStringRef;

    pub static kIOSurfaceCacheMode: CFStringRef;
    pub static kIOSurfaceIsGlobal: CFStringRef;
    pub static kIOSurfacePixelFormat: CFStringRef;

    fn IOSurfaceCreate(properties: CFDictionaryRef) -> IOSurfaceRef;
    fn IOSurfaceLookup(csid: IOSurfaceID) -> IOSurfaceRef;
    fn IOSurfaceGetID(buffer: IOSurfaceRef) -> IOSurfaceID;

    fn IOSurfaceGetTypeID() -> CFTypeID;

    fn IOSurfaceLock(buffer: IOSurfaceRef, options: u32, seed: *mut u32) -> IOReturn;
    fn IOSurfaceUnlock(buffer: IOSurfaceRef, options: u32, seed: *mut u32) -> IOReturn;

    fn IOSurfaceGetHeight(buffer: IOSurfaceRef) -> size_t;
    fn IOSurfaceGetBytesPerRow(buffer: IOSurfaceRef) -> size_t;
    fn IOSurfaceGetBaseAddress(buffer: IOSurfaceRef) -> *mut c_void;
}
