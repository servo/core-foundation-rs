use base::{AbstractCFType, CFRelease, CFRetain, CFTypeRef};
use unsafe::reinterpret_cast;

struct __CFBoolean { private: () }
pub type CFBooleanRef = *__CFBoolean;

struct CFBoolean {
    obj: CFBooleanRef,

    drop {
        unsafe {
            CFRelease(reinterpret_cast(&self.obj));
        }
    }
}

mod CFBoolean {
    fn wrap(obj: CFBooleanRef) -> CFBoolean {
        CFBoolean { obj: obj }
    }

    fn true_value() -> CFBoolean {
        unsafe {
            let obj = kCFBooleanTrue;
            CFRetain(reinterpret_cast(&obj));
            return wrap(obj);
        }
    }

    fn false_value() -> CFBoolean {
        unsafe {
            let obj = kCFBooleanFalse;
            CFRetain(reinterpret_cast(&obj));
            return wrap(obj);
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

