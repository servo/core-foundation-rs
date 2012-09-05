use base::{AbstractCFType, CFAllocatorRef, CFIndex, CFRelease, CFTypeRef, kCFAllocatorDefault};
use libc::{c_int, c_void};
use unsafe::reinterpret_cast;

struct __CFNumber { private: () }
pub type CFNumberRef = *__CFNumber;

const kCFNumberSInt32Type: c_int = 3;

struct CFNumber {
    obj: CFNumberRef,

    drop {
        unsafe {
            CFRelease(reinterpret_cast(&self.obj));
        }
    }
}

mod CFNumber {
    fn wrap(obj: CFNumberRef) -> CFNumber {
        CFNumber { obj: obj }
    }

    fn new_number<T:copy ConvertibleToCFNumber>(n: T) -> CFNumber {
        unsafe {
            CFNumber {
                obj: CFNumberCreate(kCFAllocatorDefault, n.cf_number_type(), reinterpret_cast(&&n))
            }
        }
    }
}

impl CFNumber : AbstractCFType {
    pure fn as_type_ref(&self) -> CFTypeRef {
        unsafe {
            reinterpret_cast(&self.obj)
        }
    }
}

trait ConvertibleToCFNumber {
    // FIXME: Should be static, but that breaks.
    pure fn cf_number_type(self) -> CFNumberType;
}

impl i32 : ConvertibleToCFNumber {
    pure fn cf_number_type(self) -> CFNumberType { kCFNumberSInt32Type as CFNumberType }
}

type CFNumberType = CFIndex;

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    fn CFNumberCreate(allocator: CFAllocatorRef, theType: CFNumberType, valuePtr: *c_void)
                   -> CFNumberRef;
}

