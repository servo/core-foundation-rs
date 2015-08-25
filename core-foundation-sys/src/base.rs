use libc::{c_uint, c_long, c_ulong, c_void};

pub type Boolean = u8;
pub type CFIndex = c_long;
pub type mach_port_t = c_uint;
pub type CFAllocatorRef = *const c_void;
pub type CFNullRef = *const c_void;
pub type CFHashCode = c_ulong;
pub type CFTypeID = c_ulong;
pub type CFTypeRef = *const c_void;
pub type CFOptionFlags = u32;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CFRange {
    pub location: CFIndex,
    pub length: CFIndex
}

extern {
    /*
     * CFBase.h
     */

    /* CFAllocator Reference */
    // N.B. Many CFAllocator functions and constants are omitted here.
    pub static kCFAllocatorDefault: CFAllocatorRef;
    pub static kCFAllocatorSystemDefault: CFAllocatorRef;
    pub static kCFAllocatorMalloc: CFAllocatorRef;
    pub static kCFAllocatorMallocZone: CFAllocatorRef;
    pub static kCFAllocatorNull: CFAllocatorRef;
    pub static kCFAllocatorUseContext: CFAllocatorRef;

    /* CFNull Reference */

    pub static kCFNull: CFNullRef;

    /* CFType Reference */

    //fn CFCopyDescription
    //fn CFCopyTypeIDDescription
    //fn CFEqual
    //fn CFGetAllocator
    pub fn CFGetRetainCount(cf: CFTypeRef) -> CFIndex;
    pub fn CFGetTypeID(cf: CFTypeRef) -> CFTypeID;
    pub fn CFHash(cf: CFTypeRef) -> CFHashCode;
    //fn CFMakeCollectable
    pub fn CFRelease(cf: CFTypeRef);
    pub fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    pub fn CFShow(obj: CFTypeRef);

    /* Base Utilities Reference */
    // N.B. Some things missing here.
}
