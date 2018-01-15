pub use core_foundation_sys::filedescriptor::*;

use core_foundation_sys::base::{Boolean, CFIndex, CFRelease};
use core_foundation_sys::base::{kCFAllocatorDefault, CFOptionFlags};

use base::{TCFType};

use std::mem;
use std::os::unix::io::{AsRawFd, RawFd};
use std::ptr;

pub struct CFFileDescriptor(CFFileDescriptorRef);

impl Drop for CFFileDescriptor {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl_TCFType!(CFFileDescriptor, CFFileDescriptorRef, CFFileDescriptorGetTypeID);

impl CFFileDescriptor {
    pub fn new(fd: RawFd,
               closeOnInvalidate: bool,
               callout: CFFileDescriptorCallBack,
               context: Option<&CFFileDescriptorContext>) -> CFFileDescriptor {
        unsafe {
            let fd_ref = CFFileDescriptorCreate(kCFAllocatorDefault,
                                                fd,
                                                closeOnInvalidate as Boolean,
                                                callout,
                                                if let Some(context) = context {
                                                    context
                                                } else {
                                                    ptr::null()
                                                });
            TCFType::wrap_under_create_rule(fd_ref)
        }
    }

    pub fn context(&self) -> CFFileDescriptorContext {
        unsafe {
            let mut context: CFFileDescriptorContext = mem::uninitialized();
            CFFileDescriptorGetContext(self.0, &mut context);
            context
        }
    }

    pub fn enable_callbacks(&self, callback_types: CFOptionFlags) {
        unsafe {
            CFFileDescriptorEnableCallBacks(self.0, callback_types)
        }
    }

    pub fn disable_callbacks(&self, callback_types: CFOptionFlags) {
        unsafe {
            CFFileDescriptorDisableCallBacks(self.0, callback_types)
        }
    }

    pub fn valid(&self) -> bool {
        unsafe {
            CFFileDescriptorIsValid(self.0) != 0
        }
    }

    pub fn invalidate(&self) {
        unsafe {
            CFFileDescriptorInvalidate(self.0)
        }
    }
}

impl AsRawFd for CFFileDescriptor {
    fn as_raw_fd(&self) -> RawFd {
        unsafe {
            CFFileDescriptorGetNativeDescriptor(self.0)
        }
    }
}

use runloop::{CFRunLoopSource};

impl CFRunLoopSource {
    pub fn from_file_descriptor(fd: &CFFileDescriptor, order: CFIndex) -> CFRunLoopSource {
        unsafe {
            let source_ref = CFFileDescriptorCreateRunLoopSource(kCFAllocatorDefault, fd.0, order);
            TCFType::wrap_under_create_rule(source_ref)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
