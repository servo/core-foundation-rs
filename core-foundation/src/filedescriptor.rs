pub use core_foundation_sys::filedescriptor::*;

use core_foundation_sys::base::{Boolean, CFIndex, CFRelease};
use core_foundation_sys::base::{kCFAllocatorDefault, CFOptionFlags};

use base::{TCFType};

use std::os::unix::io::{AsRawFd, RawFd};

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
    pub unsafe fn new(fd: RawFd,
                      closeOnInvalidate: bool,
                      callout: CFFileDescriptorCallBack,
                      context: *const CFFileDescriptorContext) -> CFFileDescriptor {
        let fd_ref = CFFileDescriptorCreate(kCFAllocatorDefault,
                                            fd,
                                            closeOnInvalidate as Boolean,
                                            callout,
                                            context);
        TCFType::wrap_under_create_rule(fd_ref)
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

#[cfg(test)]
mod test {
    use super::*;
}
