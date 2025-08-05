use core_foundation::base::{CFRelease, CFRetain, CFTypeID};
use foreign_types::{foreign_type, ForeignType};

/// Possible source states of an event source.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum CGEventSourceStateID {
    Private = -1,
    CombinedSessionState = 0,
    HIDSystemState = 1,
}

foreign_type! {
    #[doc(hidden)]
    pub unsafe type CGEventSource {
        type CType = crate::sys::CGEventSource;
        fn drop = |p| CFRelease(p as *mut _);
        fn clone = |p| CFRetain(p as *const _) as *mut _;
    }
}

impl CGEventSource {
    pub fn type_id() -> CFTypeID {
        unsafe { CGEventSourceGetTypeID() }
    }

    pub fn new(state_id: CGEventSourceStateID) -> Result<Self, ()> {
        unsafe {
            let event_source_ref = CGEventSourceCreate(state_id);
            if !event_source_ref.is_null() {
                Ok(Self::from_ptr(event_source_ref))
            } else {
                Err(())
            }
        }
    }

    pub unsafe fn get_location(&self) -> CGPoint {
        CGEventSourceGetLocation(self.as_ptr() as *mut _)
    }
}

#[cfg_attr(feature = "link", link(name = "CoreGraphics", kind = "framework"))]
extern "C" {
    /// Return the type identifier for the opaque type [`CGEventSourceRef`].
    ///
    /// [`CGEventSourceRef`]: crate::sys::CGEventSourceRef
    fn CGEventSourceGetTypeID() -> CFTypeID;

    /// Return a Quartz event source created with a specified source state.
    fn CGEventSourceCreate(stateID: CGEventSourceStateID) -> crate::sys::CGEventSourceRef;

    fn CGEventSourceGetLocation(source: *mut sys::CGEventSource) -> CGPoint;
}
