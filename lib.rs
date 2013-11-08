// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern mod std;
extern mod core_foundation = "rust-core-foundation";
extern mod geom = "rust-geom";
extern mod opengles = "rust-opengles";

// Rust bindings to the IOSurface framework on Mac OS X.

use core_foundation::base::{AbstractCFTypeRef, CFTypeID, CFTypeRef, CFWrapper};
use core_foundation::dictionary::{CFDictionaryRef, UntypedCFDictionary};
use core_foundation::string::CFStringRef;
use geom::size::Size2D;
use opengles::cgl::{kCGLNoError, CGLGetCurrentContext, CGLTexImageIOSurface2D};
use opengles::gl2::{BGRA, GLenum, GLsizei, RGBA, TEXTURE_RECTANGLE_ARB, UNSIGNED_INT_8_8_8_8_REV};
use std::cast;
use std::libc::{c_int, c_void, size_t};
use std::vec::bytes;

static kIOSurfaceLockReadOnly: u32 = 0x1;
static kIOSurfaceLockAvoidSync: u32 = 0x2;

type IOReturn = c_int;

struct __IOSurface { private: () }

pub type IOSurfaceRef = *__IOSurface;

impl AbstractCFTypeRef for IOSurfaceRef {
    fn as_type_ref(&self) -> CFTypeRef {
        *self as CFTypeRef
    }

    #[fixed_stack_segment]
    fn type_id(_dummy: Option<IOSurfaceRef>) -> CFTypeID {
        unsafe {
            IOSurfaceGetTypeID()
        }
    }
}

pub struct IOSurface {
    contents: CFWrapper<IOSurfaceRef, (), ()>,
}

pub type IOSurfaceID = u32;

impl Clone for IOSurface {
    fn clone(&self) -> IOSurface {
        IOSurface {
            contents: CFWrapper::clone(&self.contents),
        }
    }
}

#[fixed_stack_segment]
pub fn new(properties: &UntypedCFDictionary) -> IOSurface {
    unsafe {
        let result = IOSurfaceCreate(*properties.contents.borrow_ref());
        let result = IOSurface {
            contents: CFWrapper::wrap_owned(result),
        };
        result
    }
}

#[fixed_stack_segment]
pub fn lookup(csid: IOSurfaceID) -> IOSurface {
    unsafe {
        let result = IOSurfaceLookup(csid);
        IOSurface {
            contents: CFWrapper::wrap_owned(result),
        }
    }
}

impl IOSurface {
    #[fixed_stack_segment]
    pub fn get_id(&self) -> IOSurfaceID {
        unsafe {
            IOSurfaceGetID(*self.contents.borrow_ref())
        }
    }

    /// Binds to the current GL texture.
    #[fixed_stack_segment]
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
                                                  cast::transmute(*self.contents.borrow_ref()),
                                                  0);

            assert_eq!(gl_error, kCGLNoError);
        }
    }

    #[fixed_stack_segment]
    pub fn upload(&self, data: &[u8]) {
        unsafe {
            let surface = *self.contents.borrow_ref();
            let mut seed = 0;

            IOSurfaceLock(surface, 0, &mut seed);

            let height = IOSurfaceGetHeight(surface);
            let stride = IOSurfaceGetBytesPerRow(surface);
            let size = (height * stride) as uint;
            let dest: &mut [u8] = cast::transmute((IOSurfaceGetBaseAddress(surface), size));
            bytes::copy_memory(dest, data, data.len());

            // FIXME(pcwalton): RAII
            IOSurfaceUnlock(surface, 0, &mut seed);
        }
    }
}

#[link_args="-framework IOSurface"]
#[nolink]
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

