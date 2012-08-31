// Rust bindings to the IOSurface framework on Mac OS X.

use core_foundation::base::{AbstractCFType, CFType, CFTypeOps, CFTypeRef};
use core_foundation::base::__foreign_mod__::CFRelease;  // FIXME
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::number::CFNumber;
use core_foundation::string::{CFString, CFStringRef};
use unsafe::reinterpret_cast;

struct __IOSurface { private: () }
pub type IOSurfaceRef = *__IOSurface;

pub type IOSurfaceID = u32;

pub struct IOSurface {
    obj: IOSurfaceRef,

    drop {
        unsafe {
            CFRelease(reinterpret_cast(self.obj));
        }
    }
}

pub mod IOSurface {
    fn wrap(obj: IOSurfaceRef) -> IOSurface {
        assert obj != ptr::null();
        IOSurface { obj: obj }
    }

    fn new_io_surface(properties: &CFDictionary<CFString,CFType>) -> IOSurface {
        wrap(IOSurfaceCreate(properties.obj))
    }

    fn lookup(csid: IOSurfaceID) -> IOSurface {
        wrap(IOSurfaceLookup(csid))
    }
}

impl IOSurface : AbstractCFType {
    pure fn as_type_ref(&self) -> CFTypeRef {
        unsafe {
            reinterpret_cast(self.obj)
        }
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

