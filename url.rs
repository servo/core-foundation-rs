use base::{
    AbstractCFType,
    AbstractCFTypeRef,
    CFAllocatorRef,
    CFRelease,
    CFType,
    CFTypeRef,
};
use data::{
    CFDataRef,
};
use string::{
    CFStringRef,
    CFStringEncoding,
};

struct __CFURL { private: () }
pub type CFURLRef = *__CFURL;

impl CFURLRef : AbstractCFTypeRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }
}

struct CFURL {
    obj: CFURLRef,

    drop {
        unsafe {
            CFRelease(self.obj.as_type_ref())
        }
    }
}

impl CFURL : AbstractCFType<CFURLRef> {
    pure fn as_type_ref(&self) -> CFTypeRef {
        self.obj.as_type_ref()
    }
    static fn wrap(obj: CFURLRef) -> CFURL {
        CFURL { obj: obj }
    }

    static fn unwrap(wrapper: CFURL) -> CFURLRef {
        wrapper.obj
    }
}


#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    /*
     * CFURL.h
     */

    fn CFURLCreateWithString(allocator: CFAllocatorRef, urlString: CFStringRef,
                             baseURL: CFURLRef) -> CFURLRef;
    fn CFURLCreateData(allocator: CFAllocatorRef, url: CFURLRef, 
                       encoding: CFStringEncoding, escapeWhitespace: bool) -> CFDataRef;
    fn CFURLGetString(anURL: CFURLRef) -> CFStringRef;
    fn CFURLGetBaseURL(anURL: CFURLRef) -> CFURLRef;

}