use base::{AbstractCFType, CFRelease, CFRetain, CFTypeRef};
use cast::reinterpret_cast;

struct __CFBoolean { private: () }
pub type CFBooleanRef = *__CFBoolean;

pub struct CFBoolean {
    obj: CFBooleanRef,

    drop {
        unsafe {
            CFRelease(reinterpret_cast(&self.obj));
        }
    }
}

pub impl CFBoolean {
    static fn wrap(obj: CFBooleanRef) -> CFBoolean {
        CFBoolean { obj: obj }
    }

    static fn true_value() -> CFBoolean {
        unsafe {
            let obj = kCFBooleanTrue;
            CFRetain(reinterpret_cast(&obj));
            return CFBoolean::wrap(obj);
        }
    }

    static fn false_value() -> CFBoolean {
        unsafe {
            let obj = kCFBooleanFalse;
            CFRetain(reinterpret_cast(&obj));
            return CFBoolean::wrap(obj);
        }
    }
}

impl CFBoolean : AbstractCFType {
    pure fn as_type_ref(&self) -> CFTypeRef {
        unsafe {
            reinterpret_cast(&self.obj)
        }
    }
}

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    const kCFBooleanTrue: CFBooleanRef;
    const kCFBooleanFalse: CFBooleanRef;
}

