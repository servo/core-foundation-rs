use cf = core_foundation;
use cf::base::{
    AbstractCFType,
    AbstractCFTypeRef,
    CFIndex,
    CFRelease,
    CFTypeID,
    CFTypeRef,
};

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

pub impl CGDataProviderRef : AbstractCFTypeRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }
}

pub struct CGDataProvider {
    obj: CGDataProviderRef,

    drop {
        unsafe {
            CFRelease(cast::transmute(self.obj))
        }
    }
}

pub impl CGDataProvider : AbstractCFType<CGDataProviderRef> {
    pure fn get_ref() -> CGDataProviderRef { self.obj }

    static fn wrap(obj: CGDataProviderRef) -> CGDataProvider {
        CGDataProvider { obj: obj }
    }

    static fn unwrap(wrapper: CGDataProvider) -> CGDataProviderRef {
        wrapper.obj
    }
}

pub impl CGDataProvider {
    static fn new_from_buffer(buf: *u8, len: uint) -> CGDataProvider unsafe {
        let obj = CGDataProviderCreateWithData(
            ptr::null(),
            cast::transmute(buf),
            len as size_t,
            ptr::null());

        return cf::base::wrap(obj);
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
