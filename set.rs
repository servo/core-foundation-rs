use base::{
    AbstractCFType,
    AbstractCFTypeRef,
    Boolean,
    CFAllocatorRef,
    CFIndex,
    CFRelease,
    CFTypeRef,
};
use libc::c_char;

pub type CFSetRetainCallBack = *u8;
pub type CFSetReleaseCallBack = *u8;
pub type CFSetCopyDescriptionCallBack = *u8;
pub type CFSetEqualCallBack = *u8;
pub type CFSetHashCallBack = *u8;

pub struct CFSetCallBacks {
    version: CFIndex,
    retain: CFSetRetainCallBack,
    release: CFSetReleaseCallBack,
    copyDescription: CFSetCopyDescriptionCallBack,
    equal: CFSetEqualCallBack,
    hash: CFSetHashCallBack,
}

struct __CFSet { private: () }
pub type CFSetRef = *__CFSet;

impl CFSetRef : AbstractCFTypeRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }
}

pub struct CFSet {
    obj: CFSetRef,

    drop {
        unsafe {
            CFRelease(cast::transmute(self.obj));
        }
    }
}

pub impl CFSet : AbstractCFType<CFSetRef> {
    pure fn as_type_ref(&self) -> CFTypeRef {
        unsafe {
            self.obj.as_type_ref()
        }
    }

    static fn wrap(obj: CFSetRef) -> CFSet {
        CFSet { obj: obj }
    }

    static fn unwrap(wrapper: CFSet) -> CFSetRef {
        wrapper.obj
    }
}

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    /*
     * CFSet.h
     */

    const kCFTypeSetCallBacks: CFSetCallBacks;
    const kCFTypeCopyStringSetCallBacks: CFSetCallBacks;

    //fn CFSetCreate
    //fn CFSetCreateCopy
    //fn CFSetContainsValue
    //fn CFSetGetCount
    //fn CFSetGetCountOfValue
    //fn CFSetGetValue
    //fn CFSetGetValueIfPresent
    //fn CFSetGetValues
    //fn CFSetApplyFunction
    //fn CFSetGetTypeID
}

