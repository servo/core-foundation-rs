use base::{
    AbstractCFTypeRef,
    CFAllocatorRef,
    CFIndex,
    CFRange,
    CFTypeRef,
    CFWrapper,
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

pub impl CFArrayRef : AbstractCFTypeRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }
}

pub type CFArray<ElemRefType: AbstractCFTypeRef> = CFWrapper<CFArrayRef, ElemRefType, ()>;

pub impl<ElemRefType:AbstractCFTypeRef> CFArray<ElemRefType> {
    static fn new(elems: &[ElemRefType]) -> CFArray<ElemRefType> {
        let array_ref : CFArrayRef;
        let elems_refs = do vec::map(elems) |e: &ElemRefType| { e.as_type_ref() };

        unsafe {
            array_ref = CFArrayCreate(kCFAllocatorDefault,
                                      cast::transmute(vec::raw::to_ptr(elems_refs)),
                                      elems.len() as CFIndex,
                                      ptr::to_unsafe_ptr(&kCFTypeArrayCallBacks));
        }
        return move CFWrapper::wrap_owned(array_ref);
    }

    pub fn each_ref<A>(cb: fn&(ElemRefType) -> A) {
        for uint::range(0, self.len()) |i| { cb(self[i]); }
    }

    pub fn eachi_ref<A>(cb: fn&(uint, ElemRefType) -> A) {
        for uint::range(0, self.len()) |i| { cb(i, self[i]); }
    }

    // Careful; the callback must wrap the reference properly.
    // Generally, when array elements are Core Foundation objects (not
    // always true), they need to be wrapped with CFWrapper::wrap_shared.
    pub fn each<A>(cb: fn&(&ElemRefType) -> A) {
        for uint::range(0, self.len()) |i| { cb(&self[i]); }
    }

    // Careful; the callback must wrap the reference properly.
    // Generally, when array elements are Core Foundation objects (not
    // always true), they need to be wrapped with CFWrapper::wrap_shared.
    pub fn eachi<A>(cb: fn&(uint, &ElemRefType) -> A) {
        for uint::range(0, self.len()) |i| { cb(i, &self[i]); }
    }

    pub pure fn len() -> uint {
        unsafe { return CFArrayGetCount(*self.borrow_ref()) as uint; }
    }

    // Careful; the caller must wrap any returned reference properly.
    // Generally, when array elements are Core Foundation objects (not
    // always true), they need to be wrapped with CFWrapper::wrap_shared.
    pure fn index(idx: uint) -> ElemRefType {
        assert idx < self.len();
        unsafe { 
            let elem = CFArrayGetValueAtIndex(*self.borrow_ref(), idx as CFIndex);
            // Don't return a wrapped thing, since we don't know whether
            // it needs base::wrap_shared() or base::wrap_owned()
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
    use number::{CFNumber, CFNumberRef};

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

    for arr.each_ref |elem: CFNumberRef| {
        let n: CFNumber = base::wrap(elem);
        sum += n.to_i32();
    }

    assert sum == 30;
}
