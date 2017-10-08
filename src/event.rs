#![allow(non_upper_case_globals)]

use core_foundation::base::{CFRelease, CFRetain, CFTypeID};
use geometry::CGPoint;
use event_source::CGEventSource;

use libc;

use foreign_types::ForeignType;

pub type CGKeyCode = libc::uint16_t;

/// Flags for events
///
/// [Ref](http://opensource.apple.com/source/IOHIDFamily/IOHIDFamily-700/IOHIDSystem/IOKit/hidsystem/IOLLEvent.h)
bitflags! {
    #[repr(C)]
    pub struct CGEventFlags: u64 {
        const CGEventFlagNull = 0;

        // Device-independent modifier key bits.
        const CGEventFlagAlphaShift = 0x00010000;
        const CGEventFlagShift = 0x00020000;
        const CGEventFlagControl = 0x00040000;
        const CGEventFlagAlternate = 0x00080000;
        const CGEventFlagCommand = 0x00100000;

        // Special key identifiers.
        const CGEventFlagHelp = 0x00400000;
        const CGEventFlagSecondaryFn = 0x00800000;

        // Identifies key events from numeric keypad area on extended keyboards.
        const CGEventFlagNumericPad = 0x00200000;

        // Indicates if mouse/pen movement events are not being coalesced
        const CGEventFlagNonCoalesced = 0x00000100;
    }
}

/// Key codes for keys that are independent of keyboard layout.
///
/// [Ref](https://github.com/phracker/MacOSX-SDKs/blob/master/MacOSX10.13.sdk/System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/HIToolbox.framework/Versions/A/Headers/Events.h)
#[repr(C)]
pub struct KeyCode;
impl KeyCode {
    pub const RETURN: CGKeyCode = 0x24;
    pub const TAB: CGKeyCode = 0x30;
    pub const SPACE: CGKeyCode = 0x31;
    pub const DELETE: CGKeyCode = 0x33;
    pub const ESCAPE: CGKeyCode = 0x35;
    pub const COMMAND: CGKeyCode = 0x37;
    pub const SHIFT: CGKeyCode = 0x38;
    pub const CAPS_LOCK: CGKeyCode = 0x39;
    pub const OPTION: CGKeyCode = 0x3A;
    pub const CONTROL: CGKeyCode = 0x3B;
    pub const RIGHT_COMMAND: CGKeyCode = 0x36;
    pub const RIGHT_SHIFT: CGKeyCode = 0x3C;
    pub const RIGHT_OPTION: CGKeyCode = 0x3D;
    pub const RIGHT_CONTROL: CGKeyCode = 0x3E;
    pub const FUNCTION: CGKeyCode = 0x3F;
    pub const VOLUME_UP: CGKeyCode = 0x48;
    pub const VOLUME_DOWN: CGKeyCode = 0x49;
    pub const MUTE: CGKeyCode = 0x4A;
    pub const F1: CGKeyCode = 0x7A;
    pub const F2: CGKeyCode = 0x78;
    pub const F3: CGKeyCode = 0x63;
    pub const F4: CGKeyCode = 0x76;
    pub const F5: CGKeyCode = 0x60;
    pub const F6: CGKeyCode = 0x61;
    pub const F7: CGKeyCode = 0x62;
    pub const F8: CGKeyCode = 0x64;
    pub const F9: CGKeyCode = 0x65;
    pub const F10: CGKeyCode = 0x6D;
    pub const F11: CGKeyCode = 0x67;
    pub const F12: CGKeyCode = 0x6F;
    pub const F13: CGKeyCode = 0x69;
    pub const F14: CGKeyCode = 0x6B;
    pub const F15: CGKeyCode = 0x71;
    pub const F16: CGKeyCode = 0x6A;
    pub const F17: CGKeyCode = 0x40;
    pub const F18: CGKeyCode = 0x4F;
    pub const F19: CGKeyCode = 0x50;
    pub const F20: CGKeyCode = 0x5A;
    pub const HELP: CGKeyCode = 0x72;
    pub const HOME: CGKeyCode = 0x73;
    pub const PAGE_UP: CGKeyCode = 0x74;
    pub const FORWARD_DELETE: CGKeyCode = 0x75;
    pub const END: CGKeyCode = 0x77;
    pub const PAGE_DOWN: CGKeyCode = 0x79;
    pub const LEFT_ARROW: CGKeyCode = 0x7B;
    pub const RIGHT_ARROW: CGKeyCode = 0x7C;
    pub const DOWN_ARROW: CGKeyCode = 0x7D;
    pub const UP_ARROW: CGKeyCode = 0x7E;
}

/// Constants that specify the different types of input events.
///
/// [Ref](http://opensource.apple.com/source/IOHIDFamily/IOHIDFamily-700/IOHIDSystem/IOKit/hidsystem/IOLLEvent.h)
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum CGEventType {
    Null = 0,

    // Mouse events.
    LeftMouseDown = 1,
    LeftMouseUp = 2,
    RightMouseDown = 3,
    RightMouseUp = 4,
    MouseMoved = 5,
    LeftMouseDragged = 6,
    RightMouseDragged = 7,

    // Keyboard events.
    KeyDown = 10,
    KeyUp = 11,
    FlagsChanged = 12,

    // Specialized control devices.
    ScrollWheel = 22,
    TabletPointer = 23,
    TabletProximity = 24,
    OtherMouseDown = 25,
    OtherMouseUp = 26,
    OtherMouseDragged = 27,

    // Out of band event types. These are delivered to the event tap callback
    // to notify it of unusual conditions that disable the event tap.
    TapDisabledByTimeout = 0xFFFFFFFE,
    TapDisabledByUserInput = 0xFFFFFFFF,
}

// Constants that specify buttons on a one, two, or three-button mouse.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum CGMouseButton {
    Left,
    Right,
    Center,
}

/// Possible tapping points for events.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum CGEventTapLocation {
    HID,
    Session,
    AnnotatedSession,
}

foreign_type! {
    #[doc(hidden)]
    type CType = ::sys::CGEvent;
    fn drop = |p| CFRelease(p as *mut _);
    fn clone = |p| CFRetain(p as *const _) as *mut _;
    pub struct CGEvent;
    pub struct CGEventRef;
}

impl CGEvent {
    pub fn type_id() -> CFTypeID {
        unsafe {
            CGEventGetTypeID()
        }
    }

    pub fn new(source: CGEventSource) -> Result<CGEvent, ()> {
        unsafe {
            let event_ref = CGEventCreate(source.as_ptr());
            if !event_ref.is_null() {
                Ok(Self::from_ptr(event_ref))
            } else {
                Err(())
            }
        }
    }

    pub fn new_keyboard_event(
        source: CGEventSource,
        keycode: CGKeyCode,
        keydown: bool
    ) -> Result<CGEvent, ()> {
        unsafe {
            let event_ref = CGEventCreateKeyboardEvent(source.as_ptr(), keycode, keydown);
            if !event_ref.is_null() {
                Ok(Self::from_ptr(event_ref))
            } else {
                Err(())
            }
        }
    }

    pub fn new_mouse_event(
        source: CGEventSource,
        mouse_type: CGEventType,
        mouse_cursor_position: CGPoint,
        mouse_button: CGMouseButton
    ) -> Result<CGEvent, ()> {
        unsafe {
            let event_ref = CGEventCreateMouseEvent(source.as_ptr(), mouse_type,
                mouse_cursor_position, mouse_button);
            if !event_ref.is_null() {
                Ok(Self::from_ptr(event_ref))
            } else {
                Err(())
            }
        }
    }

    pub fn post(&self, tap_location: CGEventTapLocation) {
        unsafe {
            CGEventPost(tap_location, self.as_ptr());
        }
    }

    pub fn location(&self) -> CGPoint {
        unsafe {
            CGEventGetLocation(self.as_ptr())
        }
    }

    #[cfg(feature = "elcapitan")]
    pub fn post_to_pid(&self, pid: libc::pid_t) {
        unsafe {
            CGEventPostToPid(pid, self.as_ptr());
        }
    }

    pub fn set_flags(&self, flags: CGEventFlags) {
        unsafe {
            CGEventSetFlags(self.as_ptr(), flags);
        }
    }

    pub fn get_flags(&self) -> CGEventFlags {
        unsafe {
            CGEventGetFlags(self.as_ptr())
        }
    }

    pub fn set_type(&self, event_type: CGEventType) {
        unsafe {
            CGEventSetType(self.as_ptr(), event_type);
        }
    }

    pub fn get_type(&self) -> CGEventType {
        unsafe {
            CGEventGetType(self.as_ptr())
        }
    }

    pub fn set_string_from_utf16_unchecked(&self, buf: &[u16]) {
        let buflen = buf.len() as libc::c_ulong;
        unsafe {
            CGEventKeyboardSetUnicodeString(self.as_ptr(), buflen, buf.as_ptr());
        }
    }

    pub fn set_string(&self, string: &str) {
        let buf: Vec<u16> = string.encode_utf16().collect();
        self.set_string_from_utf16_unchecked(&buf);
    }
}

#[link(name = "ApplicationServices", kind = "framework")]
extern {
    /// Return the type identifier for the opaque type `CGEventRef'.
    fn CGEventGetTypeID() -> CFTypeID;

    /// Return a new event using the event source `source'. If `source' is NULL,
    /// the default source is used.
    fn CGEventCreate(source: ::sys::CGEventSourceRef) -> ::sys::CGEventRef;

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
    fn CGEventCreateKeyboardEvent(source: ::sys::CGEventSourceRef, keycode: CGKeyCode,
        keydown: bool) -> ::sys::CGEventRef;

    /// Return a new mouse event.
    ///
    /// The event source may be taken from another event, or may be NULL.
    /// `mouseType' should be one of the mouse event types. `mouseCursorPosition'
    /// should be the position of the mouse cursor in global coordinates.
    /// `mouseButton' should be the button that's changing state; `mouseButton'
    /// is ignored unless `mouseType' is one of `kCGEventOtherMouseDown',
    /// `kCGEventOtherMouseDragged', or `kCGEventOtherMouseUp'.
    ///
    /// The current implementation of the event system supports a maximum of
    /// thirty-two buttons. Mouse button 0 is the primary button on the mouse.
    /// Mouse button 1 is the secondary mouse button (right). Mouse button 2 is
    /// the center button, and the remaining buttons are in USB device order.
    fn CGEventCreateMouseEvent(source: ::sys::CGEventSourceRef, mouseType: CGEventType,
        mouseCursorPosition: CGPoint, mouseButton: CGMouseButton) -> ::sys::CGEventRef;

    /// Post an event into the event stream at a specified location.
    ///
    /// This function posts the specified event immediately before any event taps
    /// instantiated for that location, and the event passes through any such
    /// taps.
    fn CGEventPost(tapLocation: CGEventTapLocation, event: ::sys::CGEventRef);

    #[cfg(feature = "elcapitan")]
    /// Post an event to a specified process ID
    fn CGEventPostToPid(pid: libc::pid_t, event: ::sys::CGEventRef);

    /// Set the event flags of an event.
    fn CGEventSetFlags(event: ::sys::CGEventRef, flags: CGEventFlags);

    /// Return the event flags of an event.
    fn CGEventGetFlags(event: ::sys::CGEventRef) -> CGEventFlags;

    /// Return the location of an event in global display coordinates.
    /// CGPointZero is returned if event is not a valid ::sys::CGEventRef.
    fn CGEventGetLocation(event: ::sys::CGEventRef) -> CGPoint;

    /// Set the event type of an event.
    fn CGEventSetType(event: ::sys::CGEventRef, eventType: CGEventType);

    /// Return the event type of an event (left mouse down, for example).
    fn CGEventGetType(event: ::sys::CGEventRef) -> CGEventType;

    /// Set the Unicode string associated with a keyboard event.
    ///
    /// By default, the system translates the virtual key code in a keyboard
    /// event into a Unicode string based on the keyboard ID in the event
    /// source.  This function allows you to manually override this string.
    /// Note that application frameworks may ignore the Unicode string in a
    /// keyboard event and do their own translation based on the virtual
    /// keycode and perceived event state.
    fn CGEventKeyboardSetUnicodeString(event: ::sys::CGEventRef,
                                       length: libc::c_ulong,
                                       string: *const u16);
}
