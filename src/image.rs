use core_foundation::base::{CFRetain, CFTypeID, TCFType};
use core_foundation::data::CFData;
use color_space::{CGColorSpace, CGColorSpaceRef};
use data_provider::{CGDataProvider, CGDataProviderRef};
use libc::size_t;
use foreign_types::ForeignType;

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

foreign_type! {
    type CType = ::sys::CGImage;
    fn drop = CGImageRelease;
    fn clone = |p| CFRetain(p as *const _) as *mut _;
    pub struct CGImage;
    pub struct CGImageRef;
}

impl CGImage {
    pub fn type_id() -> CFTypeID {
        unsafe {
            CGImageGetTypeID()
        }
    }
}

impl CGImageRef {
    pub fn width(&self) -> size_t {
        unsafe {
            CGImageGetWidth(self.as_ptr())
        }
    }

    pub fn height(&self) -> size_t {
        unsafe {
            CGImageGetHeight(self.as_ptr())
        }
    }

    pub fn bits_per_component(&self) -> size_t {
        unsafe {
            CGImageGetBitsPerComponent(self.as_ptr())
        }
    }

    pub fn bits_per_pixel(&self) -> size_t {
        unsafe {
            CGImageGetBitsPerPixel(self.as_ptr())
        }
    }

    pub fn bytes_per_row(&self) -> size_t {
        unsafe {
            CGImageGetBytesPerRow(self.as_ptr())
        }
    }

    pub fn color_space(&self) -> CGColorSpace {
        unsafe {
            TCFType::wrap_under_get_rule(CGImageGetColorSpace(self.as_ptr()))
        }
    }

    /// Returns the raw image bytes wrapped in `CFData`. Note, the returned `CFData` owns the
    /// underlying buffer.
    pub fn data(&self) -> CFData {
        let data_provider = unsafe {
            CGDataProvider::wrap_under_get_rule(CGImageGetDataProvider(self.as_ptr()))
        };
        data_provider.copy_data()
    }
}

#[link(name = "ApplicationServices", kind = "framework")]
extern {
    fn CGImageGetTypeID() -> CFTypeID;
    fn CGImageGetWidth(image: ::sys::CGImageRef) -> size_t;
    fn CGImageGetHeight(image: ::sys::CGImageRef) -> size_t;
    fn CGImageGetBitsPerComponent(image: ::sys::CGImageRef) -> size_t;
    fn CGImageGetBitsPerPixel(image: ::sys::CGImageRef) -> size_t;
    fn CGImageGetBytesPerRow(image: ::sys::CGImageRef) -> size_t;
    fn CGImageGetColorSpace(image: ::sys::CGImageRef) -> CGColorSpaceRef;
    fn CGImageGetDataProvider(image: ::sys::CGImageRef) -> CGDataProviderRef;
    fn CGImageRelease(image: ::sys::CGImageRef);

    //fn CGImageGetAlphaInfo(image: ::sys::CGImageRef) -> CGImageAlphaInfo;
    //fn CGImageCreateCopyWithColorSpace(image: ::sys::CGImageRef, space: CGColorSpaceRef) -> ::sys::CGImageRef
}
