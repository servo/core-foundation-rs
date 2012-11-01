use base::{AbstractCFType, AbstractCFTypeRef, CFRelease, CFRetain, CFTypeRef};
use cast::reinterpret_cast;

struct __CFBoolean { private: () }
pub type CFBooleanRef = *__CFBoolean;

impl CFBooleanRef : AbstractCFTypeRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }
}

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

impl CFBoolean : AbstractCFType<CFBooleanRef> {
    pure fn as_type_ref(&self) -> CFTypeRef {
        unsafe { cast::transmute(self.obj) }
    }

    static fn wrap(obj: CFBooleanRef) -> CFBoolean {
        CFBoolean { obj: obj }
    }

    static fn unwrap(wrapper: CFBoolean) -> CFBooleanRef {
        wrapper.obj
    }
}

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    const kCFBooleanTrue: CFBooleanRef;
    const kCFBooleanFalse: CFBooleanRef;
}

