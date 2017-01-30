use core_foundation::base::{CFRelease, CFRetain, CFTypeID, CFTypeRef, TCFType};
use color_space::{CGColorSpace, CGColorSpaceRef};
use libc::{size_t};
use std::mem;

#[repr(C)]
pub enum CGImageAlphaInfo {
    CGImageAlphaNone, /* For example, RGB. */
    CGImageAlphaPremultipliedLast, /* For example, premultiplied RGBA */ 
    CGImageAlphaPremultipliedFirst, /* For example, premultiplied ARGB */
    CGImageAlphaLast, /* For example, non-premultiplied RGBA */
    CGImageAlphaFirst, /* For example, non-premultiplied ARGB */
    CGImageAlphaNoneSkipLast, /* For example, RBGX. */
    CGImageAlphaNoneSkipFirst, /* For example, XRBG. */
    CGImageAlphaOnly /* No color data, alpha data only */
}

#[repr(C)]
pub enum CGImageByteOrderInfo {
    CGImageByteOrderMask = 0x7000,
    CGImageByteOrder16Little = (1 << 12),
    CGImageByteOrder32Little = (2 << 12),
    CGImageByteOrder16Big = (3 << 12),
    CGImageByteOrder32Big = (4 << 12)
}

#[repr(C)]
pub struct __CGImage;

pub type CGImageRef = *const __CGImage;

pub struct CGImage {
    obj: CGImageRef,
}

impl Drop for CGImage {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl Clone for CGImage {
    fn clone(&self) -> CGImage {
        unsafe {
            TCFType::wrap_under_get_rule(self.as_concrete_TypeRef())
        }
    }
}

impl TCFType<CGImageRef> for CGImage {
    #[inline]
    fn as_concrete_TypeRef(&self) -> CGImageRef {
        self.obj
    }

    #[inline]
    unsafe fn wrap_under_get_rule(reference: CGImageRef) -> CGImage {
        let reference: CGImageRef = mem::transmute(CFRetain(mem::transmute(reference)));
        TCFType::wrap_under_create_rule(reference)
    }

    #[inline]
    fn as_CFTypeRef(&self) -> CFTypeRef {
        unsafe {
            mem::transmute(self.as_concrete_TypeRef())
        }
    }

    #[inline]
    unsafe fn wrap_under_create_rule(obj: CGImageRef) -> CGImage {
        CGImage {
            obj: obj,
        }
    }

    #[inline]
    fn type_id() -> CFTypeID {
        unsafe {
            CGImageGetTypeID()
        }
    }
}

impl CGImage {
    pub fn width(&self) -> size_t {
        unsafe {
            CGImageGetWidth(self.as_concrete_TypeRef())
        }
    }

    pub fn height(&self) -> size_t {
        unsafe {
            CGImageGetHeight(self.as_concrete_TypeRef())
        }
    }

    pub fn bits_per_component(&self) -> size_t {
        unsafe {
            CGImageGetBitsPerComponent(self.as_concrete_TypeRef())
        }
    }

    pub fn bits_per_pixel(&self) -> size_t {
        unsafe {
            CGImageGetBitsPerPixel(self.as_concrete_TypeRef())
        }
    }

    pub fn bytes_per_row(&self) -> size_t {
        unsafe {
            CGImageGetBytesPerRow(self.as_concrete_TypeRef())
        }
    }

    pub fn color_space(&self) -> CGColorSpace {
        unsafe {
            TCFType::wrap_under_get_rule(CGImageGetColorSpace(self.as_concrete_TypeRef()))
        }
    }
}

#[link(name = "ApplicationServices", kind = "framework")]
extern {
    fn CGImageGetTypeID() -> CFTypeID;
    fn CGImageGetWidth(image: CGImageRef) -> size_t;
    fn CGImageGetHeight(image: CGImageRef) -> size_t;
    fn CGImageGetBitsPerComponent(image: CGImageRef) -> size_t;
    fn CGImageGetBitsPerPixel(image: CGImageRef) -> size_t;
    fn CGImageGetBytesPerRow(image: CGImageRef) -> size_t;
    fn CGImageGetColorSpace(image: CGImageRef) -> CGColorSpaceRef;
}
