use libc::c_void;

use base::{CFAllocatorRef, CFTypeID, CFIndex};

pub type CFDataRef = *const c_void;

extern {
    /*
     * CFData.h
     */

    pub fn CFDataCreate(allocator: CFAllocatorRef,
                        bytes: *const u8, length: CFIndex) -> CFDataRef;
    //fn CFDataFind
    pub fn CFDataGetBytePtr(theData: CFDataRef) -> *const u8;
    pub fn CFDataGetLength(theData: CFDataRef) -> CFIndex;

    pub fn CFDataGetTypeID() -> CFTypeID;
}
