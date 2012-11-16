use base::{
    AbstractCFTypeRef,
    Boolean,
    CFAllocatorRef,
    CFIndex,
    CFTypeID,
    CFTypeRef,
    CFWrapper,
    kCFAllocatorDefault
};
use libc::{c_int, c_void};

pub type CFNumberType = u32;

// members of enum CFNumberType
const kCFNumberSInt8Type:     CFNumberType = 1;
const kCFNumberSInt16Type:    CFNumberType = 2;
const kCFNumberSInt32Type:    CFNumberType = 3;
const kCFNumberSInt64Type:    CFNumberType = 4;
const kCFNumberFloat32Type:   CFNumberType = 5;
const kCFNumberFloat64Type:   CFNumberType = 6;
const kCFNumberCharType:      CFNumberType = 7;
const kCFNumberShortType:     CFNumberType = 8;
const kCFNumberIntType:       CFNumberType = 9;
const kCFNumberLongType:      CFNumberType = 10;
const kCFNumberLongLongType:  CFNumberType = 11;
const kCFNumberFloatType:     CFNumberType = 12;
const kCFNumberDoubleType:    CFNumberType = 13;
const kCFNumberCFIndexType:   CFNumberType = 14;
const kCFNumberNSIntegerType: CFNumberType = 15;
const kCFNumberCGFloatType:   CFNumberType = 16;
const kCFNumberMaxType:       CFNumberType = 16;

struct __CFNumber { private: () }
pub type CFNumberRef = *__CFNumber;

pub impl CFNumberRef : AbstractCFTypeRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }
    static pure fn type_id() -> CFTypeID unsafe { CFNumberGetTypeID() }
}

pub type CFNumber = CFWrapper<CFNumberRef, (), ()>;

pub impl CFNumber {
    static fn new<T:Copy ConvertibleToCFNumber>(n: T) -> CFNumber {
        let objref = unsafe { CFNumberCreate(kCFAllocatorDefault, n.cf_number_type(), cast::transmute(&n)) };
        CFWrapper::wrap_owned(objref)
    }

    pure fn to_i8() -> i8 {
        let ty = kCFNumberSInt8Type;
        assert self.has_number_type(ty);
        unsafe {
            let val: i8 = 0i8;
            if !CFNumberGetValue(self.obj, ty, cast::transmute(&val)) {
                fail ~"Error in unwrapping CFNumber to i8";
            }
            return val;
        }
    }

    pure fn to_i16() -> i16 {
        let ty = kCFNumberSInt16Type;
        assert self.has_number_type(ty);
        unsafe {
            let val: i16 = 0i16;
            if !CFNumberGetValue(self.obj, ty, cast::transmute(&val)) {
                fail ~"Error in unwrapping CFNumber to i16";
            }
            return val;
        }
    }

    pure fn to_i32() -> i32 {
        let ty = kCFNumberSInt32Type;
        assert self.has_number_type(ty);
        unsafe {
            let val: i32 = 0i32;
            if !CFNumberGetValue(self.obj, ty, cast::transmute(&val)) {
                fail ~"Error in unwrapping CFNumber to i32";
            }
            return val;
        }
    }

    pure fn to_float() -> float unsafe {
        assert self.has_float_type();
        let ty = CFNumberGetType(self.obj);
        if ty == kCFNumberFloat32Type || ty == kCFNumberFloatType {
            let mut val: libc::c_float = 0.0f as libc::c_float;
            if !CFNumberGetValue(self.obj, ty, cast::transmute(&val)) {
                fail ~"Error in unwrapping CFNumber to libc::c_float";
            }
            return val as float;
        }
        else if ty == kCFNumberFloat64Type || ty == kCFNumberDoubleType {
            let mut val: libc::c_double = 0.0f as libc::c_double;
            if !CFNumberGetValue(self.obj, ty, cast::transmute(&val)) {
                    fail ~"Error in unwrapping CFNumber to libc::c_double";
                }
            return val as float;
        }

        fail fmt!("Unable to wrap CFNumber into float: with type tag=%?", ty)
    }

    priv pure fn has_float_type() -> bool unsafe { CFNumberIsFloatType(self.obj) as bool }

    priv pure fn has_number_type(ty: CFNumberType) -> bool {
        unsafe { CFNumberGetType(self.obj) == ty }
    }
}

pub trait ConvertibleToCFNumber {
    // FIXME: Should be static, but that breaks.
    pure fn cf_number_type(&self) -> CFNumberType;
}

impl i8 : ConvertibleToCFNumber {
    pure fn cf_number_type(&self) -> CFNumberType { kCFNumberSInt8Type as CFNumberType }
}

impl i16 : ConvertibleToCFNumber {
    pure fn cf_number_type(&self) -> CFNumberType { kCFNumberSInt16Type as CFNumberType }
}

impl i32 : ConvertibleToCFNumber {
    pure fn cf_number_type(&self) -> CFNumberType { kCFNumberSInt32Type as CFNumberType }
}

impl i64 : ConvertibleToCFNumber {
    pure fn cf_number_type(&self) -> CFNumberType { kCFNumberSInt64Type as CFNumberType }
}

impl float : ConvertibleToCFNumber {
    pure fn cf_number_type(&self) -> CFNumberType { kCFNumberFloatType as CFNumberType }
}

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    /*
     * CFNumber.h
     */

    const kCFNumberNaN: CFNumberRef;
    const kCFNumberNegativeInfinity: CFNumberRef;
    const kCFNumberPositiveInfinity: CFNumberRef;

    fn CFNumberCreate(allocator: CFAllocatorRef, theType: CFNumberType, valuePtr: *c_void)
                   -> CFNumberRef;
    //fn CFNumberGetByteSize
    fn CFNumberGetType(number: CFNumberRef) -> CFNumberType;
    fn CFNumberGetValue(number: CFNumberRef, theType: CFNumberType, valuePtr: *c_void) -> bool;
    fn CFNumberIsFloatType(number: CFNumberRef) -> Boolean;
    //fn CFNumberCompare
    fn CFNumberGetTypeID() -> CFTypeID;
}

fn should_fail_on_bad_downcast() {
    #[test];
    #[should_fail];

    use boolean::CFBooleanRef;

    let one = CFWrapper::to_CFType(CFNumber::new(1_i32));
    let casted = base::downcast::<CFBooleanRef>(*one.borrow_ref());
}