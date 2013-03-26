use base::{
    AbstractCFTypeRef,
    CFAllocatorRef,
    CFIndex,
    CFRange,
    CFTypeID,
    CFTypeRef,
    CFWrapper,
    kCFAllocatorDefault,
};
use core::libc::c_void;

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

impl AbstractCFTypeRef for CFArrayRef {
    fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }
    fn type_id() -> CFTypeID { unsafe { CFArrayGetTypeID() } }
}

pub type CFArray<ElemRefType> = CFWrapper<CFArrayRef, ElemRefType, ()>;

pub impl<ElemRefType:AbstractCFTypeRef> CFArray<ElemRefType> {
    fn new(elems: &[ElemRefType]) -> CFArray<ElemRefType> {
        let array_ref: CFArrayRef;
        let elems_refs = do vec::map(elems) |e: &ElemRefType| { e.as_type_ref() };

        unsafe {
            array_ref = CFArrayCreate(kCFAllocatorDefault,
                                      cast::transmute::<*CFTypeRef, **c_void>(vec::raw::to_ptr(elems_refs)),
                                      elems.len() as CFIndex,
                                      ptr::to_unsafe_ptr(&kCFTypeArrayCallBacks));
        }

        CFWrapper::wrap_owned(array_ref)
    }

    pub fn each_ref<A>(&self, cb: &fn(ElemRefType) -> A) {
        for uint::range(0, self.len()) |i| { cb(self[i]); }
    }

    pub fn eachi_ref<A>(&self, cb: &fn(uint, ElemRefType) -> A) {
        for uint::range(0, self.len()) |i| { cb(i, self[i]); }
    }

    // Careful; the callback must wrap the reference properly.
    // Generally, when array elements are Core Foundation objects (not
    // always true), they need to be wrapped with CFWrapper::wrap_shared.
    pub fn each<A>(&self, cb: &fn(&ElemRefType) -> A) {
        for uint::range(0, self.len()) |i| { cb(&self[i]); }
    }

    // Careful; the callback must wrap the reference properly.
    // Generally, when array elements are Core Foundation objects (not
    // always true), they need to be wrapped with CFWrapper::wrap_shared.
    pub fn eachi<A>(&self, cb: &fn(uint, &ElemRefType) -> A) {
        for uint::range(0, self.len()) |i| { cb(i, &self[i]); }
    }

    pub fn len(&self) -> uint {
        unsafe { return CFArrayGetCount(*self.borrow_ref()) as uint; }
    }
}

impl<ElemRefType:AbstractCFTypeRef> Index<uint,ElemRefType> for CFArray<ElemRefType> {
    // Careful; the caller must wrap any returned reference properly.
    // Generally, when array elements are Core Foundation objects (not
    // always true), they need to be wrapped with CFWrapper::wrap_shared.
    fn index(&self, idx: uint) -> ElemRefType {
        fail_unless!(idx < self.len());
        unsafe { 
            let elem = CFArrayGetValueAtIndex(*self.borrow_ref(), idx as CFIndex);
            // Don't return a wrapped thing, since we don't know whether
            // it needs base::wrap_shared() or base::wrap_owned()
            cast::transmute::<*c_void,ElemRefType>(elem)
        }
    }
}

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    /*
     * CFArray.h
     */
    static kCFTypeArrayCallBacks: CFArrayCallBacks;

    fn CFArrayCreate(allocator: CFAllocatorRef, values: **c_void,
                     numValues: CFIndex, callBacks: *CFArrayCallBacks) -> CFArrayRef;
    // CFArrayCreateCopy
    // CFArrayBSearchValues
    // CFArrayContainsValue
    fn CFArrayGetCount(theArray: CFArrayRef) -> CFIndex;
    // CFArrayGetCountOfValue
    // CFArrayGetFirstIndexOfValue
    // CFArrayGetLastIndexOfValue
    // CFArrayGetValues
    fn CFArrayGetValueAtIndex(theArray: CFArrayRef, idx: CFIndex) -> *c_void;
    // CFArrayApplyFunction
    fn CFArrayGetTypeID() -> CFTypeID;
}

#[test]
fn should_box_and_unbox() {
    use number::{CFNumber, CFNumberRef};

    let one = CFNumber::new(1 as i32);
    let two = CFNumber::new(2 as i32);
    let thr = CFNumber::new(3 as i32);
    let fou = CFNumber::new(4 as i32);
    let fiv = CFNumber::new(5 as i32);

    let arr = CFArray::new([
        *one.borrow_ref(),
        *two.borrow_ref(),
        *thr.borrow_ref(),
        *fou.borrow_ref(),
        *fiv.borrow_ref()]);

    let mut sum = 0i32;

    for arr.each |elem: &CFNumberRef| {
        sum += CFWrapper::wrap_shared(*elem).to_i32();
    }

    fail_unless!(sum == 15);

    for arr.each |elem: &CFNumberRef| {
        sum += CFWrapper::wrap_shared(*elem).to_i32();
    }

    fail_unless!(sum == 30);
}
