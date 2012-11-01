use base::{AbstractCFType, AbstractCFTypeRef, Boolean, CFAllocatorRef, CFIndex, CFRelease, CFTypeRef};
use base::{kCFAllocatorDefault, kCFAllocatorNull};
use cast::reinterpret_cast;
use libc::c_char;

pub type UniChar = libc::c_ushort;
pub type CFStringEncoding = u32;

const kCFStringEncodingMacRoman: u32 = 0;
const kCFStringEncodingWindowsLatin1: u32 = 0x0500;
const kCFStringEncodingUTF8: u32 = 0x08000100;

struct __CFString { private: () }
pub type CFStringRef = *__CFString;

impl CFStringRef : AbstractCFTypeRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }
}

pub struct CFString {
    obj: CFStringRef,

    drop {
        unsafe {
            CFRelease(reinterpret_cast(&self.obj));
        }
    }
}

pub impl CFString {
    static fn wrap(obj: CFStringRef) -> CFString {
        CFString { obj: obj }
    }

    static fn new_static(string: &static/str) -> CFString {
        let string_ref = do str::as_buf(string) |bytes, len| {
            CFStringCreateWithBytesNoCopy(kCFAllocatorDefault,
                                          bytes,
                                          len as CFIndex,
                                          kCFStringEncodingUTF8,
                                          false as Boolean,
                                          kCFAllocatorNull)
        };
        CFString::wrap(string_ref)
    }
}

pub impl CFString : AbstractCFType<CFStringRef> {
    pure fn as_type_ref(&self) -> CFTypeRef {
        unsafe {
            reinterpret_cast(&self.obj)
        }
    }

    static fn wrap(obj: CFStringRef) -> CFString {
        CFString { obj: obj }
    }

    static fn unwrap(wrapper: CFString) -> CFStringRef {
        wrapper.obj
    }
}

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    /*
     * CFString.h
     */

    fn CFStringCreateWithBytesNoCopy(alloc: CFAllocatorRef,
                                     bytes: *u8,
                                     numBytes: CFIndex,
                                     encoding: CFStringEncoding,
                                     isExternalRepresentation: Boolean,
                                     contentsDeallocator: CFAllocatorRef)
                                  -> CFStringRef;
}

