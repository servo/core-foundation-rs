use base::{
    AbstractCFTypeRef,
    Boolean,
    CFAllocatorRef,
    CFIndex,
    CFTypeRef,
    CFTypeID,
    CFWrapper,
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
    static pure fn type_id() -> CFTypeID { unsafe { CFSetGetTypeID() } }
}

pub type CFSet<ElemRefType: AbstractCFTypeRef> = CFWrapper<CFSetRef, ElemRefType, ()>;

pub impl<ElemRefType : AbstractCFTypeRef>
    CFSet<ElemRefType> {
    static fn new(elems: &[ElemRefType]) -> CFSet<ElemRefType> {
        let result : CFSetRef;
        let elems_refs = do vec::map(elems) |e: &ElemRefType| { e.as_type_ref() };

        unsafe {
            result = CFSetCreate(kCFAllocatorDefault,
                                  cast::transmute(vec::raw::to_ptr(elems_refs)),
                                  elems.len() as CFIndex,
                                  ptr::to_unsafe_ptr(&kCFTypeSetCallBacks));
        }
        CFWrapper::wrap_owned(result)
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

