// Rust bindings to the IOSurface framework on Mac OS X.

use cf = core_foundation;
use cf::base::{AbstractCFType, AbstractCFTypeRef, CFRelease, CFType, CFTypeOps, CFTypeRef};
use cf::dictionary::{CFDictionary, CFDictionaryRef};
use cf::number::CFNumber;
use cf::string::{CFString, CFStringRef};
use cast::transmute;

struct __IOSurface { private: () }
pub type IOSurfaceRef = *__IOSurface;

impl IOSurfaceRef : AbstractCFTypeRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }
}

pub type IOSurfaceID = u32;

pub struct IOSurface {
    obj: IOSurfaceRef,

    drop {
        unsafe {
            CFRelease(transmute(copy self.obj));
        }
    }
}

pub impl IOSurface {
    static fn new_io_surface(properties: &CFDictionary<CFStringRef, CFTypeRef, CFString, CFType>) -> IOSurface {
        cf::base::wrap(IOSurfaceCreate(properties.obj))
    }

    static fn lookup(csid: IOSurfaceID) -> IOSurface {
        cf::base::wrap(IOSurfaceLookup(csid))
    }
}

impl IOSurface : AbstractCFType<IOSurfaceRef> {
    pure fn as_type_ref(&self) -> CFTypeRef {
        unsafe {
            transmute(copy self.obj)
        }
    }

    static fn wrap(obj: IOSurfaceRef) -> IOSurface {
        assert obj != ptr::null();
        IOSurface { obj: obj }
    }

    static fn unwrap(wrapper: IOSurface) -> IOSurfaceRef {
        wrapper.obj
    }
}

impl IOSurface {
    fn get_id(&self) -> IOSurfaceID {
        IOSurfaceGetID(self.obj)
    }
}

#[link_args="-framework IOSurface"]
#[nolink]
extern {
    const kIOSurfaceAllocSize: CFStringRef;
    const kIOSurfaceWidth: CFStringRef;
    const kIOSurfaceHeight: CFStringRef;
    const kIOSurfaceBytesPerRow: CFStringRef;
    const kIOSurfaceBytesPerElement: CFStringRef;
    const kIOSurfaceElementWidth: CFStringRef;
    const kIOSurfaceElementHeight: CFStringRef;
    const kIOSurfaceOffset: CFStringRef;

    const kIOSurfacePlaneInfo: CFStringRef;
    const kIOSurfacePlaneWidth: CFStringRef;
    const kIOSurfacePlaneHeight: CFStringRef;
    const kIOSurfacePlaneBytesPerRow: CFStringRef;
    const kIOSurfacePlaneOffset: CFStringRef;
    const kIOSurfacePlaneSize: CFStringRef;

    const kIOSurfacePlaneBase: CFStringRef;
    const kIOSurfacePlaneBytesPerElement: CFStringRef;
    const kIOSurfacePlaneElementWidth: CFStringRef;
    const kIOSurfacePlaneElementHeight: CFStringRef;

    const kIOSurfaceCacheMode: CFStringRef;
    const kIOSurfaceIsGlobal: CFStringRef;
    const kIOSurfacePixelFormat: CFStringRef;

    fn IOSurfaceCreate(properties: CFDictionaryRef) -> IOSurfaceRef;
    fn IOSurfaceLookup(csid: IOSurfaceID) -> IOSurfaceRef;
    fn IOSurfaceGetID(buffer: IOSurfaceRef) -> IOSurfaceID;
}

