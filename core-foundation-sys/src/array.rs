use libc::c_void;

use base::{CFIndex, CFAllocatorRef, CFTypeID};

/// FIXME(pcwalton): This is wrong.
pub type CFArrayRetainCallBack = *const u8;

/// FIXME(pcwalton): This is wrong.
pub type CFArrayReleaseCallBack = *const u8;

/// FIXME(pcwalton): This is wrong.
pub type CFArrayCopyDescriptionCallBack = *const u8;

/// FIXME(pcwalton): This is wrong.
pub type CFArrayEqualCallBack = *const u8;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CFArrayCallBacks {
    pub version: CFIndex,
    pub retain: CFArrayRetainCallBack,
    pub release: CFArrayReleaseCallBack,
    pub copyDescription: CFArrayCopyDescriptionCallBack,
    pub equal: CFArrayEqualCallBack,
}

pub type CFArrayRef = *const c_void;

extern {
    /*
     * CFArray.h
     */
    pub static kCFTypeArrayCallBacks: CFArrayCallBacks;

    pub fn CFArrayCreate(allocator: CFAllocatorRef, values: *const *const c_void,
                     numValues: CFIndex, callBacks: *const CFArrayCallBacks) -> CFArrayRef;
    // CFArrayCreateCopy
    // CFArrayBSearchValues
    // CFArrayContainsValue
    pub fn CFArrayGetCount(theArray: CFArrayRef) -> CFIndex;
    // CFArrayGetCountOfValue
    // CFArrayGetFirstIndexOfValue
    // CFArrayGetLastIndexOfValue
    // CFArrayGetValues
    pub fn CFArrayGetValueAtIndex(theArray: CFArrayRef, idx: CFIndex) -> *const c_void;
    // CFArrayApplyFunction
    pub fn CFArrayGetTypeID() -> CFTypeID;
}
