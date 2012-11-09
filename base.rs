use libc::c_long;

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

struct __CFType { private: () }
pub type CFTypeRef = *__CFType;

pub type CFHashCode = libc::c_ulong;
pub type CFTypeID = libc::c_ulong;

struct CFType {
    obj: CFTypeRef,

    drop {
        unsafe {
            CFRelease(self.obj)
        }
    }
}

// a raw Core Foundation reference. It may or may not have been
// CFRetain'ed, depending on whether it was obtained via ownership or
// borrow semantics. See 
trait AbstractCFTypeRef {
    pure fn as_type_ref(&self) -> CFTypeRef;
}

trait AbstractCFType<T: AbstractCFTypeRef> {
    pure fn get_ref() -> T;
    static fn wrap(T) -> self;
    static fn unwrap(wrapper: self) -> T;
}

impl CFTypeRef : AbstractCFTypeRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self }
}

impl CFType : AbstractCFType<CFTypeRef> {
    pure fn get_ref() -> CFTypeRef { self.obj }

    static fn wrap(obj: CFTypeRef) -> CFType {
        CFType { obj: obj }
    }

    static fn unwrap(wrapper: CFType) -> CFTypeRef {
        wrapper.obj
    }
}

trait CFTypeOps<T:AbstractCFTypeRef> {
    pure fn get_type_ref(&self) -> CFTypeRef;
    static fn as_CFType(o: self) -> CFType;
    static fn convert_from_CFType(o: CFType) -> self;
    fn clone_as_CFType(&self) -> CFType;
    static fn clone(&T) -> self;
    fn retain_count(&self) -> CFIndex;
    pure fn type_id() -> CFTypeID;
    fn show(&self);
}

impl<T:Copy AbstractCFTypeRef,S:AbstractCFType<T>> S : CFTypeOps<T> {
    // not actually unsafe
    pure fn get_type_ref(&self) -> CFTypeRef unsafe {
        self.get_ref().as_type_ref()
    }

    static fn as_CFType(obj: S) -> CFType {
        let tyref : CFTypeRef = base::unwrap(move obj).as_type_ref();
        CFType { obj: tyref }
    }

    static fn convert_from_CFType(obj: CFType) -> S unsafe {
        // so we don't deallocate while transferring out of CFType.
        // the call will be balanced by the dtor of the returned wrapper.
        CFRetain(obj.get_ref());
        let tyref : CFTypeRef = base::unwrap(move obj);
        let convref : T = cast::transmute(tyref); 
        base::wrap(convref)
    }

    fn clone_as_CFType(&self) -> CFType {
        let tyref = self.get_ref().as_type_ref();
        CFRetain(tyref);
        CFType { obj: tyref }
    }

    static fn clone(cfref: &T) -> S {
        CFRetain(cfref.as_type_ref());
        base::wrap(copy *cfref)
    }

    fn retain_count(&self) -> CFIndex {
        CFGetRetainCount(self.get_type_ref())
    }

    pure fn type_id() -> CFTypeID unsafe {
        CFGetTypeID(self.get_type_ref())
    }

    fn show(&self) {
        CFShow(self.get_type_ref());
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
