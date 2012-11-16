// Rust bindings to the IOSurface framework on Mac OS X.

use cf = core_foundation;
use cf::base::{
    AbstractCFTypeRef,
    CFType,
    CFTypeRef,
    CFWrapper,
};
use cf::dictionary::{CFDictionary, CFDictionaryRef, UntypedCFDictionary};
use cf::number::CFNumber;
use cf::string::{CFString, CFStringRef};

struct __IOSurface { private: () }
pub type IOSurfaceRef = *__IOSurface;

impl IOSurfaceRef : AbstractCFTypeRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }
}

pub type IOSurface = CFWrapper<IOSurfaceRef, (), ()>;

pub type IOSurfaceID = u32;

pub trait IOSurfaceMethods {
    fn get_id(&self) -> IOSurfaceID;
}

pub fn new(properties: &UntypedCFDictionary) -> IOSurface {
    let result = IOSurfaceCreate(*properties.borrow_ref());
    CFWrapper::wrap_owned(result)
}

pub fn lookup(csid: IOSurfaceID) -> IOSurface {
    let result = IOSurfaceLookup(csid);
    CFWrapper::wrap_owned(result)
}

pub impl IOSurface : IOSurfaceMethods {
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

