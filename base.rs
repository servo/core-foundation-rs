use libc::c_long;

pub type Boolean = u8;

pub type CFIndex = c_long;

pub struct CFRange {
    location: CFIndex,
    length: CFIndex
}

struct __CFAllocator { private: () }
pub type CFAllocatorRef = *__CFAllocator;

struct __CFNull { private: () }
pub type CFNullRef = *__CFNull;

struct __CFType { private: () }
pub type CFTypeRef = *__CFType;

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
    pure fn as_type_ref(&self) -> CFTypeRef;
    static fn wrap(T) -> self;
    static fn unwrap(wrapper: self) -> T;
}

impl CFTypeRef : AbstractCFTypeRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self }
}

impl CFType : AbstractCFType<CFTypeRef> {
    pure fn as_type_ref(&self) -> CFTypeRef {
        self.obj
    }
    static fn wrap(obj: CFTypeRef) -> CFType {
        CFType { obj: obj }
    }

    static fn unwrap(wrapper: CFType) -> CFTypeRef {
        wrapper.obj
    }
}

trait CFTypeOps<T:AbstractCFTypeRef> {
    fn as_type(&self) -> CFType;
    static fn wrap_borrowed(T) -> self;
    fn retain_count(&self) -> CFIndex;
    fn show(&self);
}

impl<T:AbstractCFTypeRef,S:AbstractCFType<T>> S : CFTypeOps<T> {
    // FIXME: Should move, but there's a linearity bug.
    fn as_type(&self) -> CFType {
        CFRetain(self.as_type_ref());
        CFType { obj: self.as_type_ref() }
    }

    static fn wrap_borrowed(cfref: T) -> S {
        CFRetain(cfref.as_type_ref());
        base::wrap(move cfref)
    }

    fn retain_count(&self) -> CFIndex {
        CFGetRetainCount(self.as_type_ref())
    }

    fn show(&self) {
        CFShow(self.as_type_ref());
    }
}

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    /*
     * CFBase.h
     */

    const kCFAllocatorDefault: CFAllocatorRef;
    const kCFAllocatorSystemDefault: CFAllocatorRef;
    const kCFAllocatorMalloc: CFAllocatorRef;
    const kCFAllocatorMallocZone: CFAllocatorRef;
    const kCFAllocatorNull: CFAllocatorRef;
    const kCFAllocatorUseContext: CFAllocatorRef;

    const kCFNull: CFNullRef;

    fn CFGetRetainCount(cf: CFTypeRef) -> CFIndex;
    fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    fn CFRelease(cf: CFTypeRef);

    /*
     * CFString.h
     */

    fn CFShow(obj: CFTypeRef);
}

    