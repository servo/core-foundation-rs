use base::{
    AbstractCFType,
    AbstractCFTypeRef,
    CFAllocatorRef,
    CFRelease,
    CFType,
    CFTypeID,
    CFTypeRef,
};
use data::{
    CFDataRef,
};
use string::{
    CFString,
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
    pure fn get_ref() -> CFURLRef { self.obj }

    static fn wrap(obj: CFURLRef) -> CFURL {
        CFURL { obj: obj }
    }

    static fn unwrap(wrapper: CFURL) -> CFURLRef {
        wrapper.obj
    }
}

impl CFURL : ToStr {
    pure fn to_str() -> ~str unsafe {
        let cfstr : CFString = base::wrap(CFURLGetString(self.obj));
        cfstr.to_str()
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
    fn CFURLGetTypeID() -> CFTypeID;

}