use libc::c_long;

pub type Boolean = u8;

pub type CFIndex = c_long;

struct __CFAllocator { private: () }
pub type CFAllocatorRef = *__CFAllocator;

struct __CFType { private: () }
pub type CFTypeRef = *__CFType;

trait CFType {
    pure fn get(&self) -> CFTypeRef;
}

trait CFTypeOps {
    fn show(&self);
}

impl<T:CFType> T : CFTypeOps {
    fn show(&self) {
        CFShow(self.get());
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

    fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    fn CFRelease(cf: CFTypeRef);

    /*
     * CFString.h
     */

    fn CFShow(obj: CFTypeRef);
}

