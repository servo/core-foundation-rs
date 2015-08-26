use libc::c_void;

use base::{CFAllocatorRef, CFIndex, CFTypeID};

pub type CFSetRetainCallBack = *const u8;
pub type CFSetReleaseCallBack = *const u8;
pub type CFSetCopyDescriptionCallBack = *const u8;
pub type CFSetEqualCallBack = *const u8;
pub type CFSetHashCallBack = *const u8;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CFSetCallBacks {
    pub version: CFIndex,
    pub retain: CFSetRetainCallBack,
    pub release: CFSetReleaseCallBack,
    pub copyDescription: CFSetCopyDescriptionCallBack,
    pub equal: CFSetEqualCallBack,
    pub hash: CFSetHashCallBack,
}

#[repr(C)]
struct __CFSet;

pub type CFSetRef = *const __CFSet;

extern {
    /*
     * CFSet.h
     */

    pub static kCFTypeSetCallBacks: CFSetCallBacks;

    /* Creating Sets */
    pub fn CFSetCreate(allocator: CFAllocatorRef, values: *const *const c_void, numValues: CFIndex,
                       callBacks: *const CFSetCallBacks) -> CFSetRef;

    /* Applying a Function to Set Members */
    //fn CFSetApplyFunction

    pub fn CFSetGetTypeID() -> CFTypeID;
}

