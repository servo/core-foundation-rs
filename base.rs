use libc::c_long;

// a raw Core Foundation reference. It may or may not have been
// CFRetain'ed, depending on whether it was obtained via ownership or
// borrow semantics. 
trait AbstractCFTypeRef {
    pure fn as_type_ref(&self) -> CFTypeRef;
    static pure fn type_id() -> CFTypeID;
}

pub type Boolean = u8;

pub type CFIndex = c_long;
pub type CFOptionFlags = u32;
pub struct CFRange {
    location: CFIndex,
    length: CFIndex
}

fn CFRangeMake(off: CFIndex, len: CFIndex) -> CFRange {
    CFRange { location: off, length: len }
}

struct __CFAllocator { private: () }
pub type CFAllocatorRef = *__CFAllocator;

struct __CFNull { private: () }
pub type CFNullRef = *__CFNull;

pub type CFHashCode = libc::c_ulong;
pub type CFTypeID = libc::c_ulong;

struct __CFType { private: () }
pub type CFTypeRef = *__CFType;


pub impl CFTypeRef : AbstractCFTypeRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self }
    // this can't be used, because CFType is the supertype and has no type id.
    static pure fn type_id() -> CFTypeID { fail; }
}

pub pure fn downcast<T:AbstractCFTypeRef>(r: CFTypeRef) -> T unsafe {
    assert CFGetTypeID(r) == type_id::<T>();
    cast::transmute(r)
}

struct CFWrapper<T:Copy AbstractCFTypeRef, PlaceholderType1, PlaceholderType2> {
    obj: T,

    drop {
        unsafe {
            assert CFGetRetainCount(cast::transmute(self.obj)) > 0 as CFIndex;
            // sadly, cannot use obj.as_type_ref() here, because drop
            // cannot make virtual method calls using trait
            // types. Instead, just transmute the bugger.
            CFRelease(cast::transmute(self.obj))
        }
    }
}

pub type CFType = CFWrapper<CFTypeRef, (), ()>;

pub impl<T:Copy AbstractCFTypeRef, E1, E2>
    CFWrapper<T, E1, E2> {
    pure fn borrow_ref(&self) -> &self/T {
        &self.obj
    }

    pure fn borrow_type_ref() -> &self/CFTypeRef unsafe {
        cast::transmute(&self.obj)
    }

    // Use this when following Core Foundation's "Create" rule; i.e., the wrapper assumes ownership.
    // The object has already been retained, so we need not increment the retain count ourself.
    static pure fn wrap_owned(some_ref: T) -> CFWrapper<T,E1,E2> {
        // N.B. we can't make any assertions about retain count here,
        // because returned things are only guaranteed to be already
        // retained. Strings, for example, could be interned.
        CFWrapper { obj: some_ref }
    }

    // Use this when following Core Foundation's "Get" rule. The wrapper does not have ownership.
    // Twe need to increment object's the retain count so it isn't freed out from under our noses.
    static pure fn wrap_shared(some_ref: T) -> CFWrapper<T,E1,E2> {
        unsafe { CFRetain(some_ref.as_type_ref()); }
        CFWrapper { obj: some_ref }
    }

    // Unwraps the wrapper, returning the underlying AbstractCFType.
    static fn unwrap(wrapper: CFWrapper<T,E1,E2>) -> T {
        wrapper.obj
    }

    static fn to_CFType(wrapper: CFWrapper<T,E1,E2>) -> CFType unsafe {
        cast::transmute(move wrapper)
    }

    static fn from_CFType(wrapper: CFType) -> CFWrapper<T,E1,E2> unsafe {
        assert wrapper.type_id() == type_id::<T>();
        cast::transmute(move wrapper)
    }

    static fn clone(wrapper: &CFWrapper<T,E1,E2>) -> CFWrapper<T,E1,E2> {
        CFWrapper::wrap_shared(*wrapper.borrow_ref())
    }

    pure fn retain_count() -> CFIndex unsafe {
        CFGetRetainCount(*self.borrow_type_ref())
    }

    pure fn type_id() -> CFTypeID unsafe {
        CFGetTypeID(*self.borrow_type_ref())
    }

    pure fn show() unsafe {
        CFShow(*self.borrow_type_ref());
    }
}

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    /*
     * CFBase.h
     */

    /* CFAllocator Reference */
    // N.B. Many CFAllocator functions and constants are omitted here.
    const kCFAllocatorDefault: CFAllocatorRef;
    const kCFAllocatorSystemDefault: CFAllocatorRef;
    const kCFAllocatorMalloc: CFAllocatorRef;
    const kCFAllocatorMallocZone: CFAllocatorRef;
    const kCFAllocatorNull: CFAllocatorRef;
    const kCFAllocatorUseContext: CFAllocatorRef;

    /* CFNull Reference */

    const kCFNull: CFNullRef;

    /* CFType Reference */

    //fn CFCopyDescription
    //fn CFCopyTypeIDDescription
    //fn CFEqual
    //fn CFGetAllocator
    fn CFGetRetainCount(cf: CFTypeRef) -> CFIndex;
    fn CFGetTypeID(cf: CFTypeRef) -> CFTypeID;
    fn CFHash(cf: CFTypeRef) -> CFHashCode;
    //fn CFMakeCollectable
    fn CFRelease(cf: CFTypeRef);
    fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    fn CFShow(obj: CFTypeRef);

    /* Base Utilities Reference */
    // N.B. Some things missing here.
}
