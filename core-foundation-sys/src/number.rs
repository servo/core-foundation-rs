use libc::c_void;

use base::{CFAllocatorRef, CFTypeID};

pub type CFBooleanRef = *const c_void;

pub type CFNumberType = u32;

// members of enum CFNumberType
// static kCFNumberSInt8Type:     CFNumberType = 1;
// static kCFNumberSInt16Type:    CFNumberType = 2;
pub static kCFNumberSInt32Type:    CFNumberType = 3;
pub static kCFNumberSInt64Type:    CFNumberType = 4;
// static kCFNumberFloat32Type:   CFNumberType = 5;
pub static kCFNumberFloat64Type:   CFNumberType = 6;
// static kCFNumberCharType:      CFNumberType = 7;
// static kCFNumberShortType:     CFNumberType = 8;
// static kCFNumberIntType:       CFNumberType = 9;
// static kCFNumberLongType:      CFNumberType = 10;
// static kCFNumberLongLongType:  CFNumberType = 11;
// static kCFNumberFloatType:     CFNumberType = 12;
// static kCFNumberDoubleType:    CFNumberType = 13;
// static kCFNumberCFIndexType:   CFNumberType = 14;
// static kCFNumberNSIntegerType: CFNumberType = 15;
// static kCFNumberCGFloatType:   CFNumberType = 16;
// static kCFNumberMaxType:       CFNumberType = 16;

#[repr(C)]
struct __CFNumber;

pub type CFNumberRef = *const __CFNumber;

extern {
    /*
     * CFNumber.h
     */
    pub static kCFBooleanTrue: CFBooleanRef;
    pub static kCFBooleanFalse: CFBooleanRef;

    pub fn CFBooleanGetTypeID() -> CFTypeID;
    pub fn CFNumberCreate(allocator: CFAllocatorRef, theType: CFNumberType, valuePtr: *const c_void)
                          -> CFNumberRef;
    //fn CFNumberGetByteSize
    pub fn CFNumberGetValue(number: CFNumberRef, theType: CFNumberType, valuePtr: *mut c_void) -> bool;
    //fn CFNumberCompare
    pub fn CFNumberGetTypeID() -> CFTypeID;
}
