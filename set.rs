use base::{
    AbstractCFType,
    AbstractCFTypeRef,
    Boolean,
    CFAllocatorRef,
    CFIndex,
    CFRelease,
    CFTypeRef,
    CFTypeID,
    kCFAllocatorDefault,
};
use libc::{c_char, c_void};

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

struct CFSet<ElemRefType : AbstractCFTypeRef,
             ElemType    : AbstractCFType<ElemRefType>> {
    obj: CFSetRef,

    drop {
        unsafe {
            CFRelease(cast::transmute(self.obj));
        }
    }
}

pub impl<ElemRefType : AbstractCFTypeRef,
         ElemType    : AbstractCFType<ElemRefType>>
    CFSet<ElemRefType, ElemType> {
    static fn new(elems: &[ElemType]) -> CFSet<ElemRefType, ElemType> {
        let set_ref : CFSetRef;
        let elems_refs = do vec::map(elems) |e: &ElemType| { e.get_ref().as_type_ref() };

        unsafe {
            set_ref = CFSetCreate(kCFAllocatorDefault,
                                  cast::transmute(vec::raw::to_ptr(elems_refs)),
                                  elems.len() as CFIndex,
                                  ptr::to_unsafe_ptr(&kCFTypeSetCallBacks));
        }
        // return CFSet::wrap(set_ref)
        return CFSet { obj: set_ref };
    }
}

pub impl<ElemRefType : AbstractCFTypeRef,
         ElemType    : AbstractCFType<ElemRefType>>
    CFSet<ElemRefType, ElemType> : AbstractCFType<CFSetRef> {
    pure fn get_ref() -> CFSetRef { self.obj }

    static fn wrap(obj: CFSetRef) -> CFSet<ElemRefType, ElemType> {
        CFSet { obj: obj }
    }

    static fn unwrap(wrapper: CFSet<ElemRefType, ElemType>) -> CFSetRef {
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

    /* Creating Sets */
    fn CFSetCreate(allocator: CFAllocatorRef, values: **c_void, numValues: CFIndex, 
                   callBacks: *CFSetCallBacks) -> CFSetRef;
    fn CFSetCreateCopy(allocator: CFAllocatorRef, theSet: CFSetRef) -> CFSetRef;

    /* Examining a Set */
    fn CFSetContainsValue(theSet: CFSetRef, value: *c_void) -> Boolean;
    fn CFSetGetCount(theSet: CFSetRef) -> CFIndex;
    fn CFSetGetCountOfValue(theSet: CFSetRef, value: *c_void) -> CFIndex;
    fn CFSetGetValue(theSet: CFSetRef, value: *c_void) -> *c_void;
    fn CFSetGetValueIfPresent(theSet: CFSetRef, candidate: *c_void, value: **c_void) -> Boolean;
    fn CFSetGetValues(theSet: CFSetRef, values: **c_void);

    /* Applying a Function to Set Members */
    //fn CFSetApplyFunction

    fn CFSetGetTypeID() -> CFTypeID;
}

