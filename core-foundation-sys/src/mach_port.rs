use base::{CFAllocatorRef, CFIndex, CFTypeID, Boolean, mach_port_t};
use string::CFStringRef;
use runloop::CFRunLoopSourceRef;
use std::os::raw::c_void;

#[repr(C)]
pub struct __CFMachPort(c_void);
pub type CFMachPortRef = *mut __CFMachPort;

pub type CFMachPortCallBack = extern "C" fn (port: CFMachPortRef, msg: *mut c_void, size: CFIndex, info: *mut c_void);
pub type CFMachPortInvalidationCallBack = extern "C" fn (port: CFMachPortRef, info: *mut c_void);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CFMachPortContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: extern "C" fn (info: *const c_void) -> *const c_void,
    pub release: extern "C" fn (info: *const c_void),
    pub copyDescription: extern "C" fn (info: *const c_void) -> CFStringRef,
}

extern "C" {
    /*
     * CFMachPort.h
     */

    /* Creating a CFMachPort Object */
    pub fn CFMachPortCreate(allocator: CFAllocatorRef, callout: CFMachPortCallBack, context: *mut CFMachPortContext, shouldFreeInfo: *mut Boolean) -> CFMachPortRef;
    pub fn CFMachPortCreateWithPort(allocator: CFAllocatorRef, portNum: mach_port_t, callout: CFMachPortCallBack, context: *mut CFMachPortContext, shouldFreeInfo: *mut Boolean) -> CFMachPortRef;

    /* Configuring a CFMachPort Object */
    pub fn CFMachPortInvalidate(port: CFMachPortRef);
    pub fn CFMachPortCreateRunLoopSource(allocator: CFAllocatorRef, port: CFMachPortRef, order: CFIndex) -> CFRunLoopSourceRef;
    pub fn CFMachPortSetInvalidationCallBack(port: CFMachPortRef, callout: CFMachPortInvalidationCallBack);

    /* Examining a CFMachPort Object */
    pub fn CFMachPortGetContext(port: CFMachPortRef, context: *mut CFMachPortContext);
    pub fn CFMachPortGetInvalidationCallBack(port: CFMachPortRef) -> CFMachPortInvalidationCallBack;
    pub fn CFMachPortGetPort(port: CFMachPortRef) -> mach_port_t;
    pub fn CFMachPortIsValid(port: CFMachPortRef) -> Boolean;

    /* Getting the CFMachPort Type ID */
    pub fn CFMachPortGetTypeID() -> CFTypeID;
}
