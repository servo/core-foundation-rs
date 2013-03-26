use base::{
    AbstractCFTypeRef,
    CFAllocatorRef,
    CFIndex,
    CFRange,
    CFTypeRef,
    CFTypeID,
    CFWrapper,
    kCFAllocatorDefault,
};

use core::vec;

struct __CFData { private: () }
pub type CFDataRef = *__CFData;

impl AbstractCFTypeRef for CFDataRef {
    fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }

    fn type_id() -> CFTypeID {
        unsafe {
            CFDataGetTypeID()
        }
    }
}

type CFData = CFWrapper<CFDataRef, (), ()>;

impl CFData {
    fn new_from_buf(buf: &[u8]) -> CFData {
        let result;
        unsafe {
            result = CFDataCreate(kCFAllocatorDefault, 
                                  vec::raw::to_ptr(buf), buf.len() as CFIndex);
        }

        CFWrapper::wrap_owned(result)
    }

    // tread with caution; read-only
    fn bytes(&self) -> *u8 {
        unsafe {
            CFDataGetBytePtr(self.obj)
        }
    }

    fn len(&self) -> uint {
        unsafe {
            CFDataGetLength(self.obj) as uint
        }
    }

    fn copy_to_buf(&self) -> ~[u8] {
        unsafe {
            vec::from_buf(self.bytes(), self.len())
        }
    }

    fn with_buf<U>(&self, blk: &fn(v: &[u8]) -> U) -> U {
        unsafe {
            vec::raw::buf_as_slice(self.bytes(), self.len(), blk)
        }
    }
}

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    /*
     * CFData.h
     */

    fn CFDataCreate(allocator: CFAllocatorRef, 
                    bytes: *u8, length: CFIndex) -> CFDataRef;
    fn CFDataCreateCopy(allocator: CFAllocatorRef, theData: CFDataRef) -> CFDataRef;
    fn CFDataCreateWithBytesNoCopy(allocator: CFAllocatorRef, 
                                   bytes: *u8, length: CFIndex, 
                                   bytesDeallocator: CFAllocatorRef) -> CFDataRef;
    //fn CFDataFind
    fn CFDataGetBytePtr(theData: CFDataRef) -> *u8;
    fn CFDataGetBytes(theData: CFDataRef, range: CFRange, buffer: *u8);
    fn CFDataGetLength(theData: CFDataRef) -> CFIndex;

    fn CFDataGetTypeID() -> CFTypeID;
}
