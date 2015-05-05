// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// this file defines CGFloat, as well as stubbed data types.

use libc;
use base::{CGError, boolean_t};
use geometry::{CGRect, CGPoint, CGSize};

pub type CGDirectDisplayID = libc::uint32_t;

#[link(name = "ApplicationServices", kind = "framework")]
extern {
    pub fn CGMainDisplayID() -> CGDirectDisplayID;
    pub fn CGDisplayIsActive(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsAlwaysInMirrorSet(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsAsleep(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsBuiltin(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsInHWMirrorSet(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsInMirrorSet(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsMain(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsOnline(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsStereo(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayMirrorsDisplay(display: CGDirectDisplayID) -> CGDirectDisplayID;
    pub fn CGDisplayPrimaryDisplay(display: CGDirectDisplayID) -> CGDirectDisplayID;
    pub fn CGDisplayRotation(display: CGDirectDisplayID) -> libc::c_double;
    pub fn CGDisplayScreenSize(display: CGDirectDisplayID) -> CGSize;
    pub fn CGDisplaySerialNumber(display: CGDirectDisplayID) -> libc::uint32_t;
    pub fn CGDisplayUnitNumber(display: CGDirectDisplayID) -> libc::uint32_t;
    pub fn CGDisplayUsesOpenGLAcceleration(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayVendorNumber(display: CGDirectDisplayID) -> libc::uint32_t;
    pub fn CGGetActiveDisplayList(max_displays: libc::uint32_t,
                                  active_displays: *mut CGDirectDisplayID,
                                  display_count: *mut libc::uint32_t) -> CGError;
    pub fn CGDisplayModelNumber(display: CGDirectDisplayID) -> libc::uint32_t;
    pub fn CGDisplayPixelsHigh(display: CGDirectDisplayID) -> libc::size_t;
    pub fn CGDisplayPixelsWide(display: CGDirectDisplayID) -> libc::size_t;
    pub fn CGDisplayBounds(display: CGDirectDisplayID) -> CGRect;

    // mouse stuff
    pub fn CGDisplayHideCursor(display: CGDirectDisplayID) -> CGError;
    pub fn CGDisplayShowCursor(display: CGDirectDisplayID) -> CGError;
    pub fn CGDisplayMoveCursorToPoint(display: CGDirectDisplayID, point: CGPoint) -> CGError;
    pub fn CGWarpMouseCursorPosition(point: CGPoint) -> CGError;
    pub fn CGAssociateMouseAndMouseCursorPosition(connected: bool) -> CGError;

}
