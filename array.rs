use base::{
    AbstractCFType,
    AbstractCFTypeRef,
    CFAllocatorRef,
    CFIndex,
    CFRange,
    CFRelease,
    CFTypeRef,
    kCFAllocatorDefault,
};
use libc::c_void;

pub type CFArrayRetainCallBack = *u8;
pub type CFArrayReleaseCallBack = *u8;
pub type CFArrayCopyDescriptionCallBack = *u8;
pub type CFArrayEqualCallBack = *u8;

pub struct CFArrayCallBacks {
    version: CFIndex,
    retain: CFArrayRetainCallBack,
    release: CFArrayReleaseCallBack,
    copyDescription: CFArrayCopyDescriptionCallBack,
    equal: CFArrayEqualCallBack,
}

struct __CFArray { private: () }
pub type CFArrayRef = *__CFArray;


struct CFArray<ElemRefType : AbstractCFTypeRef,
               ElemType    : AbstractCFType<ElemRefType>> {
    obj: CFArrayRef,

    drop {
        unsafe {
            CFRelease(cast::transmute(self.obj))
        }
    }
}

pub impl<ElemRefType : AbstractCFTypeRef,
         ElemType    : AbstractCFType<ElemRefType>> 
    CFArray<ElemRefType, ElemType> {
    static fn new(elems: &[ElemType]) -> CFArray<ElemRefType, ElemType> {
        let array_ref : CFArrayRef;

        let elems_refs = do vec::map(elems) |e| {
            e.as_type_ref()
        };

        unsafe {
            array_ref = CFArrayCreate(kCFAllocatorDefault,
                                      cast::transmute(vec::raw::to_ptr(elems_refs)),
                                      elems.len() as CFIndex,
                                      ptr::to_unsafe_ptr(&kCFTypeArrayCallBacks));
        }
        // return CFArray::wrap(array_ref)
        return CFArray { obj: array_ref };
    }

    pub fn each<A>(cb: fn&(ElemRefType) -> A) {
        for uint::range(0, self.len()) |i| {
            cb(self[i]);
        }
    }

    pub fn eachi<A>(cb: fn&(uint, ElemRefType) -> A) {
        for uint::range(0, self.len()) |i| {
            cb(i, self[i]);
        }
    }

    pub pure fn len() -> uint {
        unsafe { return CFArrayGetCount(self.obj) as uint; }
    }
}

pub impl<ElemRefType : AbstractCFTypeRef,
         ElemType    : AbstractCFType<ElemRefType>> 
    CFArray<ElemRefType, ElemType> : AbstractCFType<CFArrayRef> {
    static fn wrap(obj: CFArrayRef) -> CFArray<ElemRefType, ElemType> {
        CFArray { obj: obj }
    }

    static fn unwrap(wrapper: CFArray<ElemRefType, ElemType>) -> CFArrayRef {
        wrapper.obj
    }

    pure fn as_type_ref(&self) -> CFTypeRef {
        unsafe {
            cast::transmute(self.obj)
        }
    }
}

pub impl<ElemRefType : AbstractCFTypeRef,
         ElemType    : AbstractCFType<ElemRefType>> 
    CFArray<ElemRefType, ElemType> : Index<uint, ElemRefType> {
    pure fn index(idx: uint) -> ElemRefType {
        assert idx < self.len();
        unsafe { 
            let elem = CFArrayGetValueAtIndex(self.obj, idx as CFIndex);
            // Don't return a wrapped thing, since we don't know whether
            // it needs base::wrap() or base::wrap_borrowed()
            return cast::transmute(elem);
        }
    }
}

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    /*
     * CFArray.h
     */
    const kCFTypeArrayCallBacks: CFArrayCallBacks;

    fn CFArrayCreate(allocator: CFAllocatorRef, values: **c_void,
                     numValues: CFIndex, callBacks: *CFArrayCallBacks) -> CFArrayRef;

    fn CFArrayGetValueAtIndex(theArray: CFArrayRef, idx: CFIndex) -> *c_void;
    fn CFArrayGetCount(theArray: CFArrayRef) -> CFIndex;
}

#[test]
fn should_box_and_unbox() {
    use number::CFNumber;

    let arr = CFArray::new([
        CFNumber::new(1 as i32),
        CFNumber::new(2 as i32),
        CFNumber::new(3 as i32),
        CFNumber::new(4 as i32),
        CFNumber::new(5 as i32)]);

    let mut sum = 0i32;
    // TODO: not always safe to re-wrap, unless we addref the element
    for arr.each |elem| {
        sum += elem.to_i32();
    }

    assert sum == 15;
}