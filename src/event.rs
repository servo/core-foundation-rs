use core_foundation::base::{CFRelease, CFRetain, CFTypeID, CFTypeRef, TCFType};
use event_source::{CGEventSource,CGEventSourceRef};

use libc;
use std::mem;
use std::ptr;

pub type CGKeyCode = libc::uint16_t;

/// Flags for events
///
/// [Ref] (http://opensource.apple.com//source/IOHIDFamily/IOHIDFamily-308/IOHIDSystem/IOKit/hidsystem/IOLLEvent.h)
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum CGEventFlags {
  // Device-independent modifier key bits.
  AlphaShift = 0x00010000,
  Shift = 0x00020000,
  Control = 0x00040000,
  Alternate = 0x00080000,
  Command = 0x00100000,

  // Special key identifiers.
  Help = 0x00400000,
  SecondaryFn = 0x00800000,

  // Identifies key events from numeric keypad area on extended keyboards.
  NumericPad = 0x00200000,

  // Indicates if mouse/pen movement events are not being coalesced
  NonCoalesced = 0x00000100,
}

/// Possible tapping points for events.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum CGEventTapLocation {
    HIDEventTap,
    SessionEventTap,
    AnnotatedSessionEventTap,
}

#[repr(C)]
pub struct __CGEvent;

pub type CGEventRef = *const __CGEvent;

pub struct CGEvent {
    obj: CGEventRef,
}

impl Clone for CGEvent {
    #[inline]
    fn clone(&self) -> CGEvent {
        unsafe {
            TCFType::wrap_under_get_rule(self.obj)
        }
    }
}

impl Drop for CGEvent {
    fn drop(&mut self) {
        unsafe {
            let ptr = self.as_CFTypeRef();
            assert!(ptr != ptr::null());
            CFRelease(ptr);
        }
    }
}

impl TCFType<CGEventRef> for CGEvent {
    #[inline]
    fn as_concrete_TypeRef(&self) -> CGEventRef {
        self.obj
    }

    #[inline]
    unsafe fn wrap_under_get_rule(reference: CGEventRef) -> CGEvent {
        let reference: CGEventRef = mem::transmute(CFRetain(mem::transmute(reference)));
        TCFType::wrap_under_create_rule(reference)
    }

    #[inline]
    fn as_CFTypeRef(&self) -> CFTypeRef {
        unsafe {
            mem::transmute(self.as_concrete_TypeRef())
        }
    }

    #[inline]
    unsafe fn wrap_under_create_rule(obj: CGEventRef) -> CGEvent {
        CGEvent {
            obj: obj,
        }
    }

    #[inline]
    fn type_id() -> CFTypeID {
        unsafe {
            CGEventGetTypeID()
        }
    }
}

impl CGEvent {
    pub fn new(source: CGEventSource, keycode: CGKeyCode, keydown: bool) -> Result<CGEvent, ()> {
        unsafe {
            let event_ref = CGEventCreateKeyboardEvent(source.as_concrete_TypeRef(), keycode, keydown);
            if event_ref != ptr::null() {
                Ok(TCFType::wrap_under_create_rule(event_ref))
            } else {
                Err(())
            }
        }
    }

    pub fn post(&self, tapLocation: CGEventTapLocation) {
        unsafe {
            CGEventPost(tapLocation, self.as_concrete_TypeRef());
        }
    }

    pub fn post_to_pid(&self, pid: libc::pid_t) {
        unsafe {
            CGEventPostToPid(pid, self.as_concrete_TypeRef());
        }
    }

    pub fn set_flags(&self, flags: CGEventFlags) {
        unsafe {
            CGEventSetFlags(self.as_concrete_TypeRef(), flags);
        }
    }

    pub fn get_flags(&self) -> CGEventFlags {
        unsafe {
            CGEventGetFlags(self.as_concrete_TypeRef())
        }
    }
}

#[link(name = "ApplicationServices", kind = "framework")]
extern {
    /// Return the type identifier for the opaque type `CGEventRef'.
    fn CGEventGetTypeID() -> CFTypeID;

    /// Return a new keyboard event.
    ///
    /// The event source may be taken from another event, or may be NULL. Based
    /// on the virtual key code values entered, the appropriate key down, key up,
    /// or flags changed events are generated.
    ///
    /// All keystrokes needed to generate a character must be entered, including
    /// SHIFT, CONTROL, OPTION, and COMMAND keys. For example, to produce a 'Z',
    /// the SHIFT key must be down, the 'z' key must go down, and then the SHIFT
    /// and 'z' key must be released:
    fn CGEventCreateKeyboardEvent(source: CGEventSourceRef, keycode: CGKeyCode,
        keydown: bool) -> CGEventRef;

    /// Post an event into the event stream at a specified location.
    ///
    /// This function posts the specified event immediately before any event taps
    /// instantiated for that location, and the event passes through any such
    /// taps.
    fn CGEventPost(tapLocation: CGEventTapLocation, event: CGEventRef);

    /// Post an event to a specified process ID
    fn CGEventPostToPid(pid: libc::pid_t, event: CGEventRef);

    /// Set the event flags of an event.
    fn CGEventSetFlags(event: CGEventRef, flags: CGEventFlags);

    /// Return the event flags of an event.
    fn CGEventGetFlags(event: CGEventRef) -> CGEventFlags;
}
