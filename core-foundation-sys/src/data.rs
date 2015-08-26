use base::{CFAllocatorRef, CFTypeID, CFIndex};

#[repr(C)]
struct __CFData;

pub type CFDataRef = *const __CFData;

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
