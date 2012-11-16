use base::{
    AbstractCFTypeRef,
    CFTypeRef,
    CFTypeID,
    CFWrapper,
};

pub type Boolean = u32;

struct __CFBoolean { private: () }
pub type CFBooleanRef = *__CFBoolean;

pub impl CFBooleanRef : AbstractCFTypeRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }
    static pure fn type_id() -> CFTypeID unsafe { CFBooleanGetTypeID() }
}

pub type CFBoolean = CFWrapper<CFBooleanRef, (), ()>;

pub impl CFBoolean {
    static fn true_value() -> CFBoolean {
        CFWrapper::wrap_shared(kCFBooleanTrue)
    }

    static fn false_value() -> CFBoolean {
        CFWrapper::wrap_shared(kCFBooleanFalse)
    }
}

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    const kCFBooleanTrue: CFBooleanRef;
    const kCFBooleanFalse: CFBooleanRef;

    fn CFBooleanGetValue(boolean: CFBooleanRef) -> Boolean;
    fn CFBooleanGetTypeID() -> CFTypeID;
}

