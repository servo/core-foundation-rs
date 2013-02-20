use core_foundation::base::{AbstractCFTypeRef, CFIndex, CFTypeID, CFTypeRef, CFWrapper};

use libc::{c_void, c_char, size_t};

pub type CGDataProviderGetBytesCallback = *u8;
pub type CGDataProviderReleaseInfoCallback = *u8;
pub type CGDataProviderRewindCallback = *u8;
pub type CGDataProviderSkipBytesCallback = *u8;
pub type CGDataProviderSkipForwardCallback = *u8;

pub type CGDataProviderGetBytePointerCallback = *u8;
pub type CGDataProviderGetBytesAtOffsetCallback = *u8;
pub type CGDataProviderReleaseBytePointerCallback = *u8;
pub type CGDataProviderReleaseDataCallback = *u8;
pub type CGDataProviderGetBytesAtPositionCallback = *u8;

struct __CGDataProvider { private: () }
pub type CGDataProviderRef = *__CGDataProvider;

pub impl AbstractCFTypeRef for CGDataProviderRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }

    static pure fn type_id() -> CFTypeID {
        unsafe {
            CGDataProviderGetTypeID()
        }
    }
}

pub type CGDataProvider = CFWrapper<CGDataProviderRef, (), ()>;

pub fn new_from_buffer(buf: *u8, len: uint) -> CGDataProvider {
    unsafe {
        let result = CGDataProviderCreateWithData(
            ptr::null(),
            cast::transmute(buf),
            len as size_t,
            ptr::null());

        CFWrapper::wrap_owned(result)
    }
}

#[nolink]
#[link_args="-framework ApplicationServices"]
pub extern {
    //fn CGDataProviderCopyData
    //fn CGDataProviderCreateDirect
    //fn CGDataProviderCreateSequential
    //fn CGDataProviderCreateWithCFData
    fn CGDataProviderCreateWithData(info: *c_void,
                                    data: *c_void,
                                    size: size_t,
                                    releaseData: CGDataProviderReleaseDataCallback
                                   ) -> CGDataProviderRef;
    fn CGDataProviderCreateWithFilename(filename: *c_char) -> CGDataProviderRef;
    //fn CGDataProviderCreateWithURL
    fn CGDataProviderGetTypeID() -> CFTypeID;
    fn CGDataProviderRelease(provider: CGDataProviderRef);
    fn CGDataProviderRetain(provider: CGDataProviderRef) -> CGDataProviderRef;
}
