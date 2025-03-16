#![allow(non_upper_case_globals)]
use crate::event_source::CGEventSource;
use crate::geometry::CGPoint;

use bitflags::bitflags;
use core::ffi::{c_ulong, c_void};
use core_foundation::{
    base::{CFRelease, CFRetain, CFTypeID, TCFType},
    mach_port::{CFMachPort, CFMachPortInvalidate, CFMachPortRef},
    runloop::{kCFRunLoopCommonModes, CFRunLoop},
};
use foreign_types::{foreign_type, ForeignType};
use std::{mem::ManuallyDrop, ptr};

pub type CGEventField = u32;
pub type CGKeyCode = u16;
pub type CGScrollEventUnit = u32;

bitflags! {
    /// Flags for events
    ///
    /// [Ref](http://opensource.apple.com/source/IOHIDFamily/IOHIDFamily-700/IOHIDSystem/IOKit/hidsystem/IOLLEvent.h)
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

/// Constants for the virtual key codes
///
/// These constants are the virtual keycodes defined originally in
/// Inside Mac Volume V, pg. V-191. They identify physical keys on a
/// keyboard. The struct contains the values of the `ANSIKeyCode`,
/// `KeyCode`, `ISOKeyCode` and `JISKeyCode` of the original Carbon headers.
///
/// Those constants with "ANSI" in the name are labeled
/// according to the key position on an ANSI-standard US keyboard.
/// For example, `ANSI_A` indicates the virtual keycode for the key
/// with the letter 'A' in the US keyboard layout. Other keyboard
/// layouts may have the 'A' key label on a different physical key;
/// in this case, pressing 'A' will generate a different virtual
/// keycode. Constants with the 'JIS_' or 'ISO_' prefix behave
/// analogously. Keys without a prefix are independent of the
/// keyboard layout.
///
/// [Ref](https://github.com/phracker/MacOSX-SDKs/blob/master/MacOSX10.13.sdk/System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/HIToolbox.framework/Versions/A/Headers/Events.h#L197-L327)
#[repr(C)]
pub struct KeyCode;
impl KeyCode {
    pub const ANSI_A: CGKeyCode = 0x00;
    pub const ANSI_S: CGKeyCode = 0x01;
    pub const ANSI_D: CGKeyCode = 0x02;
    pub const ANSI_F: CGKeyCode = 0x03;
    pub const ANSI_H: CGKeyCode = 0x04;
    pub const ANSI_G: CGKeyCode = 0x05;
    pub const ANSI_Z: CGKeyCode = 0x06;
    pub const ANSI_X: CGKeyCode = 0x07;
    pub const ANSI_C: CGKeyCode = 0x08;
    pub const ANSI_V: CGKeyCode = 0x09;
    pub const ANSI_B: CGKeyCode = 0x0B;
    pub const ANSI_Q: CGKeyCode = 0x0C;
    pub const ANSI_W: CGKeyCode = 0x0D;
    pub const ANSI_E: CGKeyCode = 0x0E;
    pub const ANSI_R: CGKeyCode = 0x0F;
    pub const ANSI_Y: CGKeyCode = 0x10;
    pub const ANSI_T: CGKeyCode = 0x11;
    pub const ANSI_1: CGKeyCode = 0x12;
    pub const ANSI_2: CGKeyCode = 0x13;
    pub const ANSI_3: CGKeyCode = 0x14;
    pub const ANSI_4: CGKeyCode = 0x15;
    pub const ANSI_6: CGKeyCode = 0x16;
    pub const ANSI_5: CGKeyCode = 0x17;
    pub const ANSI_EQUAL: CGKeyCode = 0x18;
    pub const ANSI_9: CGKeyCode = 0x19;
    pub const ANSI_7: CGKeyCode = 0x1A;
    pub const ANSI_MINUS: CGKeyCode = 0x1B;
    pub const ANSI_8: CGKeyCode = 0x1C;
    pub const ANSI_0: CGKeyCode = 0x1D;
    pub const ANSI_RIGHT_BRACKET: CGKeyCode = 0x1E;
    pub const ANSI_O: CGKeyCode = 0x1F;
    pub const ANSI_U: CGKeyCode = 0x20;
    pub const ANSI_LEFT_BRACKET: CGKeyCode = 0x21;
    pub const ANSI_I: CGKeyCode = 0x22;
    pub const ANSI_P: CGKeyCode = 0x23;
    pub const ANSI_L: CGKeyCode = 0x25;
    pub const ANSI_J: CGKeyCode = 0x26;
    pub const ANSI_QUOTE: CGKeyCode = 0x27;
    pub const ANSI_K: CGKeyCode = 0x28;
    pub const ANSI_SEMICOLON: CGKeyCode = 0x29;
    pub const ANSI_BACKSLASH: CGKeyCode = 0x2A;
    pub const ANSI_COMMA: CGKeyCode = 0x2B;
    pub const ANSI_SLASH: CGKeyCode = 0x2C;
    pub const ANSI_N: CGKeyCode = 0x2D;
    pub const ANSI_M: CGKeyCode = 0x2E;
    pub const ANSI_PERIOD: CGKeyCode = 0x2F;
    pub const ANSI_GRAVE: CGKeyCode = 0x32;
    pub const ANSI_KEYPAD_DECIMAL: CGKeyCode = 0x41;
    pub const ANSI_KEYPAD_MULTIPLY: CGKeyCode = 0x43;
    pub const ANSI_KEYPAD_PLUS: CGKeyCode = 0x45;
    pub const ANSI_KEYPAD_CLEAR: CGKeyCode = 0x47;
    pub const ANSI_KEYPAD_DIVIDE: CGKeyCode = 0x4B;
    pub const ANSI_KEYPAD_ENTER: CGKeyCode = 0x4C;
    pub const ANSI_KEYPAD_MINUS: CGKeyCode = 0x4E;
    pub const ANSI_KEYPAD_EQUAL: CGKeyCode = 0x51;
    pub const ANSI_KEYPAD_0: CGKeyCode = 0x52;
    pub const ANSI_KEYPAD_1: CGKeyCode = 0x53;
    pub const ANSI_KEYPAD_2: CGKeyCode = 0x54;
    pub const ANSI_KEYPAD_3: CGKeyCode = 0x55;
    pub const ANSI_KEYPAD_4: CGKeyCode = 0x56;
    pub const ANSI_KEYPAD_5: CGKeyCode = 0x57;
    pub const ANSI_KEYPAD_6: CGKeyCode = 0x58;
    pub const ANSI_KEYPAD_7: CGKeyCode = 0x59;
    pub const ANSI_KEYPAD_8: CGKeyCode = 0x5B;
    pub const ANSI_KEYPAD_9: CGKeyCode = 0x5C;
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
    pub const F17: CGKeyCode = 0x40;
    pub const VOLUME_UP: CGKeyCode = 0x48;
    pub const VOLUME_DOWN: CGKeyCode = 0x49;
    pub const MUTE: CGKeyCode = 0x4A;
    pub const F18: CGKeyCode = 0x4F;
    pub const F19: CGKeyCode = 0x50;
    pub const F20: CGKeyCode = 0x5A;
    pub const F5: CGKeyCode = 0x60;
    pub const F6: CGKeyCode = 0x61;
    pub const F7: CGKeyCode = 0x62;
    pub const F3: CGKeyCode = 0x63;
    pub const F8: CGKeyCode = 0x64;
    pub const F9: CGKeyCode = 0x65;
    pub const F11: CGKeyCode = 0x67;
    pub const F13: CGKeyCode = 0x69;
    pub const F16: CGKeyCode = 0x6A;
    pub const F14: CGKeyCode = 0x6B;
    pub const F10: CGKeyCode = 0x6D;
    pub const F12: CGKeyCode = 0x6F;
    pub const F15: CGKeyCode = 0x71;
    pub const HELP: CGKeyCode = 0x72;
    pub const HOME: CGKeyCode = 0x73;
    pub const PAGE_UP: CGKeyCode = 0x74;
    pub const FORWARD_DELETE: CGKeyCode = 0x75;
    pub const F4: CGKeyCode = 0x76;
    pub const END: CGKeyCode = 0x77;
    pub const F2: CGKeyCode = 0x78;
    pub const PAGE_DOWN: CGKeyCode = 0x79;
    pub const F1: CGKeyCode = 0x7A;
    pub const LEFT_ARROW: CGKeyCode = 0x7B;
    pub const RIGHT_ARROW: CGKeyCode = 0x7C;
    pub const DOWN_ARROW: CGKeyCode = 0x7D;
    pub const UP_ARROW: CGKeyCode = 0x7E;
    pub const ISO_SECTION: CGKeyCode = 0x0A;
    pub const JIS_YEN: CGKeyCode = 0x5D;
    pub const JIS_UNDERSCORE: CGKeyCode = 0x5E;
    pub const JIS_KEYPAD_COMMA: CGKeyCode = 0x5F;
    pub const JIS_EISU: CGKeyCode = 0x66;
    pub const JIS_KANA: CGKeyCode = 0x68;
}

#[repr(C)]
pub struct ScrollEventUnit {}
impl ScrollEventUnit {
    pub const PIXEL: CGScrollEventUnit = 0;
    pub const LINE: CGScrollEventUnit = 1;
}

/// Constants that specify the different types of input events.
///
/// [Ref](http://opensource.apple.com/source/IOHIDFamily/IOHIDFamily-700/IOHIDSystem/IOKit/hidsystem/IOLLEvent.h)
#[repr(u32)]
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

/// Constants used as keys to access specialized fields in low-level events.
///
/// [Ref](https://developer.apple.com/documentation/coregraphics/cgeventfield)
pub struct EventField;
impl EventField {
    /// Key to access an integer field that contains the mouse button event
    /// number. Matching mouse-down and mouse-up events will have the same
    /// event number.
    pub const MOUSE_EVENT_NUMBER: CGEventField = 0;

    /// Key to access an integer field that contains the mouse button click
    /// state. A click state of 1 represents a single click. A click state of
    /// 2 represents a double-click. A click state of 3 represents a
    /// triple-click.
    pub const MOUSE_EVENT_CLICK_STATE: CGEventField = 1;

    /// Key to access a double field that contains the mouse button pressure.
    /// The pressure value may range from 0 to 1, with 0 representing the
    /// mouse being up. This value is commonly set by tablet pens mimicking a
    /// mouse.
    pub const MOUSE_EVENT_PRESSURE: CGEventField = 2;

    /// Key to access an integer field that contains the mouse button number.
    pub const MOUSE_EVENT_BUTTON_NUMBER: CGEventField = 3;

    /// Key to access an integer field that contains the horizontal mouse
    /// delta since the last mouse movement event.
    pub const MOUSE_EVENT_DELTA_X: CGEventField = 4;

    /// Key to access an integer field that contains the vertical mouse delta
    /// since the last mouse movement event.
    pub const MOUSE_EVENT_DELTA_Y: CGEventField = 5;

    /// Key to access an integer field. The value is non-zero if the event
    /// should be ignored by the Inkwell subsystem.
    pub const MOUSE_EVENT_INSTANT_MOUSER: CGEventField = 6;

    /// Key to access an integer field that encodes the mouse event subtype as
    /// a `kCFNumberIntType`.
    pub const MOUSE_EVENT_SUB_TYPE: CGEventField = 7;

    /// Key to access an integer field, non-zero when this is an autorepeat of
    /// a key-down, and zero otherwise.
    pub const KEYBOARD_EVENT_AUTOREPEAT: CGEventField = 8;

    /// Key to access an integer field that contains the virtual keycode of the
    /// key-down or key-up event.
    pub const KEYBOARD_EVENT_KEYCODE: CGEventField = 9;

    /// Key to access an integer field that contains the keyboard type
    /// identifier.
    pub const KEYBOARD_EVENT_KEYBOARD_TYPE: CGEventField = 10;

    /// Key to access an integer field that contains scrolling data. This field
    /// typically contains the change in vertical position since the last
    /// scrolling event from a Mighty Mouse scroller or a single-wheel mouse
    /// scroller.
    pub const SCROLL_WHEEL_EVENT_DELTA_AXIS_1: CGEventField = 11;

    /// Key to access an integer field that contains scrolling data. This field
    /// typically contains the change in horizontal position since the last
    /// scrolling event from a Mighty Mouse scroller.
    pub const SCROLL_WHEEL_EVENT_DELTA_AXIS_2: CGEventField = 12;

    /// Key to access a field that contains scrolling data. The scrolling data
    /// represents a line-based or pixel-based change in vertical position
    /// since the last scrolling event from a Mighty Mouse scroller or a
    /// single-wheel mouse scroller. The scrolling data uses a fixed-point
    /// 16.16 signed integer format. If this key is passed to
    /// `CGEventGetDoubleValueField`, the fixed-point value is converted to a
    /// double value.
    pub const SCROLL_WHEEL_EVENT_FIXED_POINT_DELTA_AXIS_1: CGEventField = 93;

    /// Key to access a field that contains scrolling data. The scrolling data
    /// represents a line-based or pixel-based change in horizontal position
    /// since the last scrolling event from a Mighty Mouse scroller. The
    /// scrolling data uses a fixed-point 16.16 signed integer format. If this
    /// key is passed to `CGEventGetDoubleValueField`, the fixed-point value is
    /// converted to a double value.
    pub const SCROLL_WHEEL_EVENT_FIXED_POINT_DELTA_AXIS_2: CGEventField = 94;

    /// Key to access an integer field that contains pixel-based scrolling
    /// data. The scrolling data represents the change in vertical position
    /// since the last scrolling event from a Mighty Mouse scroller or a
    /// single-wheel mouse scroller.
    pub const SCROLL_WHEEL_EVENT_POINT_DELTA_AXIS_1: CGEventField = 96;

    /// Key to access an integer field that contains pixel-based scrolling
    /// data. The scrolling data represents the change in horizontal position
    /// since the last scrolling event from a Mighty Mouse scroller.
    pub const SCROLL_WHEEL_EVENT_POINT_DELTA_AXIS_2: CGEventField = 97;

    /// Key to access an integer field that indicates whether the event should
    /// be ignored by the Inkwell subsystem. If the value is non-zero, the
    /// event should be ignored.
    pub const SCROLL_WHEEL_EVENT_INSTANT_MOUSER: CGEventField = 14;

    /// Key to access an integer field that contains the absolute X coordinate
    /// in tablet space at full tablet resolution.
    pub const TABLET_EVENT_POINT_X: CGEventField = 15;

    /// Key to access an integer field that contains the absolute Y coordinate
    /// in tablet space at full tablet resolution.
    pub const TABLET_EVENT_POINT_Y: CGEventField = 16;

    /// Key to access an integer field that contains the absolute Z coordinate
    /// in tablet space at full tablet resolution.
    pub const TABLET_EVENT_POINT_Z: CGEventField = 17;

    /// Key to access an integer field that contains the tablet button state.
    /// Bit 0 is the first button, and a set bit represents a closed or pressed
    /// button. Up to 16 buttons are supported.
    pub const TABLET_EVENT_POINT_BUTTONS: CGEventField = 18;

    /// Key to access a double field that contains the tablet pen pressure. A
    /// value of 0.0 represents no pressure, and 1.0 represents maximum
    /// pressure.
    pub const TABLET_EVENT_POINT_PRESSURE: CGEventField = 19;

    /// Key to access a double field that contains the horizontal tablet pen
    /// tilt. A value of 0 represents no tilt, and 1 represents maximum tilt.
    pub const TABLET_EVENT_TILT_X: CGEventField = 20;

    /// Key to access a double field that contains the vertical tablet pen
    /// tilt. A value of 0 represents no tilt, and 1 represents maximum tilt.
    pub const TABLET_EVENT_TILT_Y: CGEventField = 21;

    /// Key to access a double field that contains the tablet pen rotation.
    pub const TABLET_EVENT_ROTATION: CGEventField = 22;

    /// Key to access a double field that contains the tangential pressure on
    /// the device. A value of 0.0 represents no pressure, and 1.0 represents
    /// maximum pressure.
    pub const TABLET_EVENT_TANGENTIAL_PRESSURE: CGEventField = 23;

    /// Key to access an integer field that contains the system-assigned unique
    /// device ID.
    pub const TABLET_EVENT_DEVICE_ID: CGEventField = 24;

    /// Key to access an integer field that contains a vendor-specified value.
    pub const TABLET_EVENT_VENDOR_1: CGEventField = 25;

    /// Key to access an integer field that contains a vendor-specified value.
    pub const TABLET_EVENT_VENDOR_2: CGEventField = 26;

    /// Key to access an integer field that contains a vendor-specified value.
    pub const TABLET_EVENT_VENDOR_3: CGEventField = 27;

    /// Key to access an integer field that contains the vendor-defined ID,
    /// typically the USB vendor ID.
    pub const TABLET_PROXIMITY_EVENT_VENDOR_ID: CGEventField = 28;

    /// Key to access an integer field that contains the vendor-defined tablet
    /// ID, typically the USB product ID.
    pub const TABLET_PROXIMITY_EVENT_TABLET_ID: CGEventField = 29;

    /// Key to access an integer field that contains the vendor-defined ID of
    /// the pointing device.
    pub const TABLET_PROXIMITY_EVENT_POINTER_ID: CGEventField = 30;

    /// Key to access an integer field that contains the system-assigned
    /// device ID.
    pub const TABLET_PROXIMITY_EVENT_DEVICE_ID: CGEventField = 31;

    /// Key to access an integer field that contains the system-assigned
    /// unique tablet ID.
    pub const TABLET_PROXIMITY_EVENT_SYSTEM_TABLET_ID: CGEventField = 32;

    /// Key to access an integer field that contains the vendor-assigned
    /// pointer type.
    pub const TABLET_PROXIMITY_EVENT_VENDOR_POINTER_TYPE: CGEventField = 33;

    /// Key to access an integer field that contains the vendor-defined
    /// pointer serial number.
    pub const TABLET_PROXIMITY_EVENT_VENDOR_POINTER_SERIAL_NUMBER: CGEventField = 34;

    /// Key to access an integer field that contains the vendor-defined unique
    /// ID.
    pub const TABLET_PROXIMITY_EVENT_VENDOR_UNIQUE_ID: CGEventField = 35;

    /// Key to access an integer field that contains the device capabilities
    /// mask.
    pub const TABLET_PROXIMITY_EVENT_CAPABILITY_MASK: CGEventField = 36;

    /// Key to access an integer field that contains the pointer type.
    pub const TABLET_PROXIMITY_EVENT_POINTER_TYPE: CGEventField = 37;

    /// Key to access an integer field that indicates whether the pen is in
    /// proximity to the tablet. The value is non-zero if the pen is in
    /// proximity to the tablet and zero when leaving the tablet.
    pub const TABLET_PROXIMITY_EVENT_ENTER_PROXIMITY: CGEventField = 38;

    /// Key to access a field that contains the event target process serial
    /// number. The value is a 64-bit value.
    pub const EVENT_TARGET_PROCESS_SERIAL_NUMBER: CGEventField = 39;

    /// Key to access a field that contains the event target Unix process ID.
    pub const EVENT_TARGET_UNIX_PROCESS_ID: CGEventField = 40;

    /// Key to access a field that contains the event source Unix process ID.
    pub const EVENT_SOURCE_UNIX_PROCESS_ID: CGEventField = 41;

    /// Key to access a field that contains the event source user-supplied
    /// data, up to 64 bits.
    pub const EVENT_SOURCE_USER_DATA: CGEventField = 42;

    /// Key to access a field that contains the event source Unix effective UID.
    pub const EVENT_SOURCE_USER_ID: CGEventField = 43;

    /// Key to access a field that contains the event source Unix effective
    /// GID.
    pub const EVENT_SOURCE_GROUP_ID: CGEventField = 44;

    /// Key to access a field that contains the event source state ID used to
    /// create this event.
    pub const EVENT_SOURCE_STATE_ID: CGEventField = 45;

    /// Key to access an integer field that indicates whether a scrolling event
    /// contains continuous, pixel-based scrolling data. The value is non-zero
    /// when the scrolling data is pixel-based and zero when the scrolling data
    /// is line-based.
    pub const SCROLL_WHEEL_EVENT_IS_CONTINUOUS: CGEventField = 88;

    /// Added in 10.5; made public in 10.7.
    pub const MOUSE_EVENT_WINDOW_UNDER_MOUSE_POINTER: CGEventField = 91;
    pub const MOUSE_EVENT_WINDOW_UNDER_MOUSE_POINTER_THAT_CAN_HANDLE_THIS_EVENT: CGEventField = 92;
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

// The next three enums are taken from:
// [Ref](https://github.com/phracker/MacOSX-SDKs/blob/ef9fe35d5691b6dd383c8c46d867a499817a01b6/MacOSX10.15.sdk/System/Library/Frameworks/CoreGraphics.framework/Versions/A/Headers/CGEventTypes.h)
/* Constants that specify where a new event tap is inserted into the list of
active event taps. */
#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum CGEventTapPlacement {
    HeadInsertEventTap = 0,
    TailAppendEventTap,
}

/* Constants that specify whether a new event tap is an active filter or a
passive listener. */
#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum CGEventTapOptions {
    Default = 0x00000000,
    ListenOnly = 0x00000001,
}

pub type CGEventMask = u64;
/* Generate an event mask for a single type of event. */
macro_rules! CGEventMaskBit {
    ($eventType:expr) => {
        (1 << $eventType as CGEventMask)
    };
}

pub type CGEventTapProxy = *const c_void;

/// What the system should do with the event passed to the callback.
///
/// This value is ignored if [`CGEventTapOptions::ListenOnly`] is specified.
pub enum CallbackResult {
    /// Pass the event unchanged to other consumers.
    Keep,
    /// Drop the event so it is not passed to later consumers.
    Drop,
    /// Replace the event with a different one.
    Replace(CGEvent),
}

type CGEventTapCallbackFn<'tap_life> =
    Box<dyn Fn(CGEventTapProxy, CGEventType, &CGEvent) -> CallbackResult + 'tap_life>;
type CGEventTapCallBackInternal = unsafe extern "C" fn(
    proxy: CGEventTapProxy,
    etype: CGEventType,
    event: crate::sys::CGEventRef,
    user_info: *const c_void,
) -> crate::sys::CGEventRef;

unsafe extern "C" fn cg_event_tap_callback_internal(
    proxy: CGEventTapProxy,
    etype: CGEventType,
    event: crate::sys::CGEventRef,
    user_info: *const c_void,
) -> crate::sys::CGEventRef {
    let callback = user_info as *mut CGEventTapCallbackFn;
    let event = ManuallyDrop::new(CGEvent::from_ptr(event));
    let response = (*callback)(proxy, etype, &event);
    use CallbackResult::*;
    match response {
        Keep => event.as_ptr(),
        Drop => ptr::null_mut(),
        Replace(new_event) => ManuallyDrop::new(new_event).as_ptr(),
    }
}

/// ```no_run
/// use core_foundation::runloop::{kCFRunLoopCommonModes, CFRunLoop};
/// use core_graphics::event::{CGEventTap, CGEventTapLocation, CGEventTapPlacement, CGEventTapOptions, CGEventType, CallbackResult};
/// let current = CFRunLoop::get_current();
///
/// CGEventTap::with_enabled(
///     CGEventTapLocation::HID,
///     CGEventTapPlacement::HeadInsertEventTap,
///     CGEventTapOptions::Default,
///     vec![CGEventType::MouseMoved],
///     |_proxy, _type, event| {
///         println!("{:?}", event.location());
///         CallbackResult::Keep
///     },
///     ||  CFRunLoop::run_current(),
/// ).expect("Failed to install event tap");
/// ```
#[must_use = "CGEventTap is disabled when dropped"]
pub struct CGEventTap<'tap_life> {
    mach_port: CFMachPort,
    _callback: Box<CGEventTapCallbackFn<'tap_life>>,
}

impl CGEventTap<'static> {
    pub fn new<F: Fn(CGEventTapProxy, CGEventType, &CGEvent) -> CallbackResult + Send + 'static>(
        tap: CGEventTapLocation,
        place: CGEventTapPlacement,
        options: CGEventTapOptions,
        events_of_interest: std::vec::Vec<CGEventType>,
        callback: F,
    ) -> Result<Self, ()> {
        // SAFETY: callback is 'static so even if this object is forgotten it
        // will be valid to call. F is safe to send across threads.
        unsafe { Self::new_unchecked(tap, place, options, events_of_interest, callback) }
    }
}

impl<'tap_life> CGEventTap<'tap_life> {
    /// Configures an event tap with the supplied options and callback, then
    /// calls `with_fn`.
    ///
    /// Note that the current thread run loop must run within `with_fn` for the
    /// tap to process events. The tap is destroyed when `with_fn` returns.
    pub fn with_enabled<R>(
        tap: CGEventTapLocation,
        place: CGEventTapPlacement,
        options: CGEventTapOptions,
        events_of_interest: std::vec::Vec<CGEventType>,
        callback: impl Fn(CGEventTapProxy, CGEventType, &CGEvent) -> CallbackResult + 'tap_life,
        with_fn: impl FnOnce() -> R,
    ) -> Result<R, ()> {
        // SAFETY: We are okay to bypass the 'static restriction because the
        // event tap is dropped before returning. The callback therefore cannot
        // be called after its lifetime expires. Since we only enable the tap
        // on the current thread run loop and don't hand it to user code, we
        // know that the callback will only be called from the current thread.
        let event_tap: Self =
            unsafe { Self::new_unchecked(tap, place, options, events_of_interest, callback)? };
        let loop_source = event_tap
            .mach_port()
            .create_runloop_source(0)
            .expect("Runloop source creation failed");
        CFRunLoop::get_current().add_source(&loop_source, unsafe { kCFRunLoopCommonModes });
        event_tap.enable();
        Ok(with_fn())
    }

    /// Caller is responsible for ensuring that this object is dropped before
    /// `'tap_life` expires. Either state captured by `callback` must be safe to
    /// send across threads, or the tap must only be installed on the current
    /// thread's run loop.
    pub unsafe fn new_unchecked(
        tap: CGEventTapLocation,
        place: CGEventTapPlacement,
        options: CGEventTapOptions,
        events_of_interest: std::vec::Vec<CGEventType>,
        callback: impl Fn(CGEventTapProxy, CGEventType, &CGEvent) -> CallbackResult + 'tap_life,
    ) -> Result<Self, ()> {
        let event_mask: CGEventMask = events_of_interest
            .iter()
            .fold(CGEventType::Null as CGEventMask, |mask, &etype| {
                mask | CGEventMaskBit!(etype)
            });
        let cb: Box<CGEventTapCallbackFn> = Box::new(Box::new(callback));
        let cbr = Box::into_raw(cb);
        unsafe {
            let event_tap_ref = CGEventTapCreate(
                tap,
                place,
                options,
                event_mask,
                cg_event_tap_callback_internal,
                cbr as *const c_void,
            );

            if !event_tap_ref.is_null() {
                Ok(Self {
                    mach_port: (CFMachPort::wrap_under_create_rule(event_tap_ref)),
                    _callback: Box::from_raw(cbr),
                })
            } else {
                let _ = Box::from_raw(cbr);
                Err(())
            }
        }
    }

    pub fn mach_port(&self) -> &CFMachPort {
        &self.mach_port
    }

    pub fn enable(&self) {
        unsafe { CGEventTapEnable(self.mach_port.as_concrete_TypeRef(), true) }
    }
}

impl Drop for CGEventTap<'_> {
    fn drop(&mut self) {
        unsafe { CFMachPortInvalidate(self.mach_port.as_CFTypeRef() as *mut _) };
    }
}

foreign_type! {
    #[doc(hidden)]
    pub unsafe type CGEvent {
        type CType = crate::sys::CGEvent;
        fn drop = |p| CFRelease(p as *mut _);
        fn clone = |p| CFRetain(p as *const _) as *mut _;
    }
}

impl CGEvent {
    pub fn type_id() -> CFTypeID {
        unsafe { CGEventGetTypeID() }
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
        keydown: bool,
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
        mouse_button: CGMouseButton,
    ) -> Result<CGEvent, ()> {
        unsafe {
            let event_ref = CGEventCreateMouseEvent(
                source.as_ptr(),
                mouse_type,
                mouse_cursor_position,
                mouse_button,
            );
            if !event_ref.is_null() {
                Ok(Self::from_ptr(event_ref))
            } else {
                Err(())
            }
        }
    }

    #[cfg(feature = "highsierra")]
    pub fn new_scroll_event(
        source: CGEventSource,
        units: CGScrollEventUnit,
        wheel_count: u32,
        wheel1: i32,
        wheel2: i32,
        wheel3: i32,
    ) -> Result<CGEvent, ()> {
        unsafe {
            let event_ref = CGEventCreateScrollWheelEvent2(
                source.as_ptr(),
                units,
                wheel_count,
                wheel1,
                wheel2,
                wheel3,
            );
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

    pub fn post_from_tap(&self, tap_proxy: CGEventTapProxy) {
        unsafe {
            CGEventTapPostEvent(tap_proxy, self.as_ptr());
        }
    }

    pub fn location(&self) -> CGPoint {
        unsafe { CGEventGetLocation(self.as_ptr()) }
    }

    pub fn set_location(&self, location: CGPoint) {
        unsafe {
            CGEventSetLocation(self.as_ptr(), location);
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
        unsafe { CGEventGetFlags(self.as_ptr()) }
    }

    pub fn set_type(&self, event_type: CGEventType) {
        unsafe {
            CGEventSetType(self.as_ptr(), event_type);
        }
    }

    pub fn get_type(&self) -> CGEventType {
        unsafe { CGEventGetType(self.as_ptr()) }
    }

    pub fn set_string_from_utf16_unchecked(&self, buf: &[u16]) {
        let buflen = buf.len() as c_ulong;
        unsafe {
            CGEventKeyboardSetUnicodeString(self.as_ptr(), buflen, buf.as_ptr());
        }
    }

    pub fn set_string(&self, string: &str) {
        let buf: Vec<u16> = string.encode_utf16().collect();
        self.set_string_from_utf16_unchecked(&buf);
    }

    pub fn get_integer_value_field(&self, field: CGEventField) -> i64 {
        unsafe { CGEventGetIntegerValueField(self.as_ptr(), field) }
    }

    pub fn set_integer_value_field(&self, field: CGEventField, value: i64) {
        unsafe { CGEventSetIntegerValueField(self.as_ptr(), field, value) }
    }

    pub fn get_double_value_field(&self, field: CGEventField) -> f64 {
        unsafe { CGEventGetDoubleValueField(self.as_ptr(), field) }
    }

    pub fn set_double_value_field(&self, field: CGEventField, value: f64) {
        unsafe { CGEventSetDoubleValueField(self.as_ptr(), field, value) }
    }
}

#[cfg_attr(feature = "link", link(name = "CoreGraphics", kind = "framework"))]
extern "C" {
    /// Return the type identifier for the opaque type [`CGEventRef`].
    ///
    /// [`CGEventRef`]: crate::sys::CGEventRef
    fn CGEventGetTypeID() -> CFTypeID;

    /// Return a new event using the event source `source`. If `source` is NULL,
    /// the default source is used.
    fn CGEventCreate(source: crate::sys::CGEventSourceRef) -> crate::sys::CGEventRef;

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
    fn CGEventCreateKeyboardEvent(
        source: crate::sys::CGEventSourceRef,
        keycode: CGKeyCode,
        keydown: bool,
    ) -> crate::sys::CGEventRef;

    /// Return a new mouse event.
    ///
    /// The event source may be taken from another event, or may be NULL.
    /// `mouseType` should be one of the mouse event types. `mouseCursorPosition`
    /// should be the position of the mouse cursor in global coordinates.
    /// `mouseButton` should be the button that's changing state; `mouseButton`
    /// is ignored unless `mouseType` is one of `kCGEventOtherMouseDown`,
    /// `kCGEventOtherMouseDragged`, or `kCGEventOtherMouseUp`.
    ///
    /// The current implementation of the event system supports a maximum of
    /// thirty-two buttons. Mouse button 0 is the primary button on the mouse.
    /// Mouse button 1 is the secondary mouse button (right). Mouse button 2 is
    /// the center button, and the remaining buttons are in USB device order.
    fn CGEventCreateMouseEvent(
        source: crate::sys::CGEventSourceRef,
        mouseType: CGEventType,
        mouseCursorPosition: CGPoint,
        mouseButton: CGMouseButton,
    ) -> crate::sys::CGEventRef;

    /// A non-variadic variant version of [`CGEventCreateScrollWheelEvent`].
    ///
    /// Returns a new Quartz scrolling event.
    ///
    /// This function allows you to create a scrolling event and customize the
    /// event before posting it to the event system.
    #[cfg(feature = "highsierra")]
    fn CGEventCreateScrollWheelEvent2(
        source: crate::sys::CGEventSourceRef,
        units: CGScrollEventUnit,
        wheelCount: u32,
        wheel1: i32,
        wheel2: i32,
        wheel3: i32,
    ) -> crate::sys::CGEventRef;

    /// Post an event into the event stream at a specified location.
    ///
    /// This function posts the specified event immediately before any event taps
    /// instantiated for that location, and the event passes through any such
    /// taps.
    fn CGEventPost(tapLocation: CGEventTapLocation, event: crate::sys::CGEventRef);

    fn CGEventTapPostEvent(tapProxy: CGEventTapProxy, event: crate::sys::CGEventRef);

    #[cfg(feature = "elcapitan")]
    /// Post an event to a specified process ID
    fn CGEventPostToPid(pid: libc::pid_t, event: crate::sys::CGEventRef);

    /// Set the event flags of an event.
    fn CGEventSetFlags(event: crate::sys::CGEventRef, flags: CGEventFlags);

    /// Return the event flags of an event.
    fn CGEventGetFlags(event: crate::sys::CGEventRef) -> CGEventFlags;

    /// Return the location of an event in global display coordinates.
    /// `CGPointZero` is returned if event is not a valid [`CGEventRef`].
    ///
    /// [`CGEventRef`]: crate::sys::CGEventRef
    fn CGEventGetLocation(event: crate::sys::CGEventRef) -> CGPoint;

    /// Set the event type of an event.
    fn CGEventSetType(event: crate::sys::CGEventRef, eventType: CGEventType);

    /// Return the event type of an event (left mouse down, for example).
    fn CGEventGetType(event: crate::sys::CGEventRef) -> CGEventType;

    /// Set the Unicode string associated with a keyboard event.
    ///
    /// By default, the system translates the virtual key code in a keyboard
    /// event into a Unicode string based on the keyboard ID in the event
    /// source.  This function allows you to manually override this string.
    /// Note that application frameworks may ignore the Unicode string in a
    /// keyboard event and do their own translation based on the virtual
    /// keycode and perceived event state.
    fn CGEventKeyboardSetUnicodeString(
        event: crate::sys::CGEventRef,
        length: c_ulong,
        string: *const u16,
    );

    /// Return the integer value of a field in an event.
    fn CGEventGetIntegerValueField(event: crate::sys::CGEventRef, field: CGEventField) -> i64;

    /// Set the integer value of a field in an event.
    ///
    /// Before calling this function, the event type must be set using a typed
    /// event creation function such as [`CGEventCreateMouseEvent`], or by
    /// calling [`CGEventSetType`].
    ///
    /// If you are creating a mouse event generated by a tablet, call this
    /// function and specify the field `kCGMouseEventSubtype` with a value of
    /// `kCGEventMouseSubtypeTabletPoint` or
    /// `kCGEventMouseSubtypeTabletProximity` before setting other parameters.
    fn CGEventSetIntegerValueField(event: crate::sys::CGEventRef, field: CGEventField, value: i64);

    /// Return the floating-point value of a field in an event.
    ///
    /// In cases where the field value is represented within the event by a fixed
    /// point number or an integer, the result is scaled to the appropriate range
    /// as part of creating the floating-point representation.
    fn CGEventGetDoubleValueField(event: crate::sys::CGEventRef, field: CGEventField) -> f64;

    /// Set the floating-point value of a field in an event.
    ///
    /// Before calling this function, the event type must be set using a typed
    /// event creation function such as [`CGEventCreateMouseEvent`], or by calling
    /// [`CGEventSetType`].
    ///
    /// In cases where the field’s value is represented within the event by a
    /// fixed point number or integer, the value parameter is scaled as needed
    /// and converted to the appropriate type.
    fn CGEventSetDoubleValueField(event: crate::sys::CGEventRef, field: CGEventField, value: f64);

    // ::sys::CGEventTapRef is actually an CFMachPortRef
    fn CGEventTapCreate(
        tap: CGEventTapLocation,
        place: CGEventTapPlacement,
        options: CGEventTapOptions,
        eventsOfInterest: CGEventMask,
        callback: CGEventTapCallBackInternal,
        userInfo: *const c_void,
    ) -> CFMachPortRef;

    /// Enable or disable an event tap.
    ///
    /// Event taps are normally enabled when created. If an event tap becomes
    /// unresponsive, or if a user requests that event taps be disabled, then
    /// a `kCGEventTapDisabled` event is passed to the event tap callback
    /// function. Event taps may be re-enabled by calling this function.
    fn CGEventTapEnable(tap: CFMachPortRef, enable: bool);

    /// Set the location of a mouse event.
    fn CGEventSetLocation(event: crate::sys::CGEventRef, location: CGPoint);
}
