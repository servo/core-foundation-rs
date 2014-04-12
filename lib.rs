// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_id = "github.com/mozilla-servo/rust-io-surface#io_surface:0.1"]
#![crate_type = "lib"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

extern crate libc;
extern crate std;
extern crate core_foundation;
extern crate geom;
extern crate opengles;

// Rust bindings to the IOSurface framework on Mac OS X.

use core_foundation::base::{CFRelease, CFTypeID, TCFType};
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::string::CFStringRef;
use geom::size::Size2D;
use opengles::cgl::{kCGLNoError, CGLGetCurrentContext, CGLTexImageIOSurface2D};
use opengles::gl2::{BGRA, GLenum, GLsizei, RGBA, TEXTURE_RECTANGLE_ARB, UNSIGNED_INT_8_8_8_8_REV};
use libc::{c_int, c_void, size_t};
use std::cast;

//static kIOSurfaceLockReadOnly: u32 = 0x1;
//static kIOSurfaceLockAvoidSync: u32 = 0x2;

type IOReturn = c_int;

struct __IOSurface;

pub type IOSurfaceRef = *__IOSurface;

pub struct IOSurface {
    obj: IOSurfaceRef,
}

impl Drop for IOSurface {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

pub type IOSurfaceID = u32;

impl Clone for IOSurface {
    #[inline]
    fn clone(&self) -> IOSurface {
        unsafe {
            TCFType::wrap_under_get_rule(self.obj)
        }
    }
}

impl TCFType<IOSurfaceRef> for IOSurface {
    fn as_concrete_TypeRef(&self) -> IOSurfaceRef {
        self.obj
    }

    unsafe fn wrap_under_create_rule(obj: IOSurfaceRef) -> IOSurface {
        IOSurface {
            obj: obj,
        }
    }

    #[inline]
    fn type_id(_: Option<IOSurface>) -> CFTypeID {
        unsafe {
            IOSurfaceGetTypeID()
        }
    }
}

pub fn new(properties: &CFDictionary) -> IOSurface {
    unsafe {
        TCFType::wrap_under_create_rule(IOSurfaceCreate(properties.as_concrete_TypeRef()))
    }
}

/// Looks up an `IOSurface` by its global ID.
///
/// FIXME(pcwalton): This should return an `Option`.
pub fn lookup(csid: IOSurfaceID) -> IOSurface {
    unsafe {
        TCFType::wrap_under_create_rule(IOSurfaceLookup(csid))
    }
}

impl IOSurface {
    pub fn get_id(&self) -> IOSurfaceID {
        unsafe {
            IOSurfaceGetID(self.as_concrete_TypeRef())
        }
    }

    /// Binds to the current GL texture.
    pub fn bind_to_gl_texture(&self, size: Size2D<int>) {
        unsafe {
            let context = CGLGetCurrentContext();
            let gl_error = CGLTexImageIOSurface2D(context,
                                                  TEXTURE_RECTANGLE_ARB,
                                                  RGBA as GLenum,
                                                  size.width as GLsizei,
                                                  size.height as GLsizei,
                                                  BGRA as GLenum,
                                                  UNSIGNED_INT_8_8_8_8_REV,
                                                  cast::transmute(self.as_concrete_TypeRef()),
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
            let size = (height * stride) as uint;
            let dest: &mut [u8] = cast::transmute((IOSurfaceGetBaseAddress(surface), size));
            dest.copy_memory(data);

            // FIXME(pcwalton): RAII
            IOSurfaceUnlock(surface, 0, &mut seed);
        }
    }
}

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

