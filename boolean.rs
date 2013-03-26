use base::{
    AbstractCFTypeRef,
    CFTypeRef,
    CFTypeID,
    CFWrapper,
};

pub type Boolean = u32;

struct __CFBoolean { private: () }
pub type CFBooleanRef = *__CFBoolean;

impl AbstractCFTypeRef for CFBooleanRef {
    fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }

    fn type_id() -> CFTypeID {
        unsafe { CFBooleanGetTypeID() }
    }
}

pub type CFBoolean = CFWrapper<CFBooleanRef, (), ()>;

pub impl CFBoolean {
    fn true_value() -> CFBoolean {
        CFWrapper::wrap_shared(kCFBooleanTrue)
    }

    fn false_value() -> CFBoolean {
        CFWrapper::wrap_shared(kCFBooleanFalse)
    }
}

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    static kCFBooleanTrue: CFBooleanRef;
    static kCFBooleanFalse: CFBooleanRef;

    fn CFBooleanGetValue(boolean: CFBooleanRef) -> Boolean;
    fn CFBooleanGetTypeID() -> CFTypeID;
}

