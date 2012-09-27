use base::{AbstractCFType, Boolean, CFAllocatorRef, CFIndex, CFRelease, CFTypeRef};
use base::{kCFAllocatorDefault, kCFAllocatorNull};
use cast::reinterpret_cast;
use libc::c_char;

type CFStringEncoding = u32;

const kCFStringEncodingMacRoman: u32 = 0;
const kCFStringEncodingWindowsLatin1: u32 = 0x0500;
const kCFStringEncodingUTF8: u32 = 0x08000100;

struct __CFString { private: () }
pub type CFStringRef = *__CFString;

pub struct CFString {
    obj: CFStringRef,

    drop {
        unsafe {
            CFRelease(reinterpret_cast(&self.obj));
        }
    }
}

pub mod CFString {
    pub fn wrap(obj: CFStringRef) -> CFString {
        CFString { obj: obj }
    }

    pub fn new_static(string: &static/str) -> CFString {
        let string_ref = do str::as_buf(string) |bytes, len| {
            CFStringCreateWithBytesNoCopy(kCFAllocatorDefault,
                                          bytes,
                                          len as CFIndex,
                                          kCFStringEncodingUTF8,
                                          false as Boolean,
                                          kCFAllocatorNull)
        };
        wrap(string_ref)
    }
}

impl CFString : AbstractCFType {
    pure fn as_type_ref(&self) -> CFTypeRef {
        unsafe {
            reinterpret_cast(&self.obj)
        }
    }
}

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    fn CFStringCreateWithBytesNoCopy(alloc: CFAllocatorRef,
                                     bytes: *u8,
                                     numBytes: CFIndex,
                                     encoding: CFStringEncoding,
                                     isExternalRepresentation: Boolean,
                                     contentsDeallocator: CFAllocatorRef)
                                  -> CFStringRef;
}

