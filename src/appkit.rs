// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_upper_case_globals)]

use base::{ObjCMethodCall, id, SEL, nil, NSInteger, NSUInteger};
use libc;

#[cfg(target_word_size = "32")]
pub type CGFloat = f32;
#[cfg(target_word_size = "64")]
pub type CGFloat = f64;

#[repr(C)]
pub struct NSPoint {
    pub x: f64,
    pub y: f64,
}

impl NSPoint {
    #[inline]
    pub fn new(x: f64, y: f64) -> NSPoint {
        NSPoint {
            x: x,
            y: y,
        }
    }
}

#[repr(C)]
pub struct NSSize {
    pub width: f64,
    pub height: f64,
}

impl NSSize {
    #[inline]
    pub fn new(width: f64, height: f64) -> NSSize {
        NSSize {
            width: width,
            height: height,
        }
    }
}

#[repr(C)]
pub struct NSRect {
    pub origin: NSPoint,
    pub size: NSSize,
}

impl NSRect {
    #[inline]
    pub fn new(origin: NSPoint, size: NSSize) -> NSRect {
        NSRect {
            origin: origin,
            size: size
        }
    }
}

#[link(name = "AppKit", kind = "framework")]
extern {}

pub unsafe fn NSApp() -> id {
    "NSApplication".send("sharedApplication", ())
}

#[repr(i64)]
pub enum NSApplicationActivationPolicy {
    NSApplicationActivationPolicyRegular = 0,
    NSApplicationActivationPolicyERROR = -1
}

#[repr(u64)]
pub enum NSWindowMask {
    NSBorderlessWindowMask      = 0,
    NSTitledWindowMask          = 1 << 0,
    NSClosableWindowMask        = 1 << 1,
    NSMiniaturizableWindowMask  = 1 << 2,
    NSResizableWindowMask       = 1 << 3,

    NSTexturedBackgroundWindowMask  = 1 << 8,

    NSUnifiedTitleAndToolbarWindowMask  = 1 << 12,

    NSFullScreenWindowMask      = 1 << 14
}

#[repr(u64)]
pub enum NSBackingStoreType {
    NSBackingStoreRetained      = 0,
    NSBackingStoreNonretained   = 1,
    NSBackingStoreBuffered      = 2
}

bitflags! {
    flags NSWindowOrderingMode: NSInteger {
        const NSWindowAbove =  1,
        const NSWindowBelow = -1,
        const NSWindowOut   =  0,
    }
}

bitflags! {
    flags NSAlignmentOptions: libc::c_ulonglong {
        const NSAlignMinXInward         = 1 << 0,
        const NSAlignMinYInward         = 1 << 1,
        const NSAlignMaxXInward         = 1 << 2,
        const NSAlignMaxYInward         = 1 << 3,
        const NSAlignWidthInward        = 1 << 4,
        const NSAlignHeightInward       = 1 << 5,
        const NSAlignMinXOutward        = 1 << 8,
        const NSAlignMinYOutward        = 1 << 9,
        const NSAlignMaxXOutward        = 1 << 10,
        const NSAlignMaxYOutward        = 1 << 11,
        const NSAlignWidthOutward       = 1 << 12,
        const NSAlignHeightOutward      = 1 << 13,
        const NSAlignMinXNearest        = 1 << 16,
        const NSAlignMinYNearest        = 1 << 17,
        const NSAlignMaxXNearest        = 1 << 18,
        const NSAlignMaxYNearest        = 1 << 19,
        const NSAlignWidthNearest       = 1 << 20,
        const NSAlignHeightNearest      = 1 << 21,
        const NSAlignRectFlipped        = 1 << 63,
        const NSAlignAllEdgesInward     = NSAlignMinXInward.bits
                                        | NSAlignMaxXInward.bits
                                        | NSAlignMinYInward.bits
                                        | NSAlignMaxYInward.bits,
        const NSAlignAllEdgesOutward    = NSAlignMinXOutward.bits
                                        | NSAlignMaxXOutward.bits
                                        | NSAlignMinYOutward.bits
                                        | NSAlignMaxYOutward.bits,
        const NSAlignAllEdgesNearest    = NSAlignMinXNearest.bits
                                        | NSAlignMaxXNearest.bits
                                        | NSAlignMinYNearest.bits
                                        | NSAlignMaxYNearest.bits,
    }
}

pub trait NSAutoreleasePool {
    unsafe fn new(_: Self) -> id {
        "NSAutoreleasePool".send("new", ())
    }

    unsafe fn autorelease(self) -> Self;
}

impl NSAutoreleasePool for id {
    unsafe fn autorelease(self) -> id {
        self.send("autorelease", ())
    }
}

pub trait NSProcessInfo {
    unsafe fn processInfo(_: Self) -> id {
        "NSProcessInfo".send("processInfo", ())
    }

    unsafe fn processName(self) -> id;
}

impl NSProcessInfo for id {
    unsafe fn processName(self) -> id {
        self.send("processName", ())
    }
}

pub trait NSApplication {
    unsafe fn sharedApplication(_: Self) -> id {
        "NSApplication".send("sharedApplication", ())
    }

    unsafe fn setActivationPolicy_(self, policy: NSApplicationActivationPolicy) -> bool;
    unsafe fn setMainMenu_(self, menu: id);
    unsafe fn activateIgnoringOtherApps_(self, ignore: bool);
    unsafe fn run(self);
}

impl NSApplication for id {
    unsafe fn setActivationPolicy_(self, policy: NSApplicationActivationPolicy) -> bool {
        self.send_bool("setActivationPolicy:", policy as NSInteger)
    }

    unsafe fn setMainMenu_(self, menu: id) {
        self.send_void("setMainMenu:", menu)
    }

    unsafe fn activateIgnoringOtherApps_(self, ignore: bool) {
        self.send_void("activateIgnoringOtherApps:", ignore);
    }

    unsafe fn run(self) {
        self.send_void("run", ());
    }
}

pub trait NSMenu {
    unsafe fn new(_: Self) -> id {
        "NSMenu".send("new", ())
    }

    unsafe fn addItem_(self, menu_item: id);
}

impl NSMenu for id {
    unsafe fn addItem_(self, menu_item: id) {
        self.send_void("addItem:", menu_item)
    }
}

pub trait NSMenuItem {
    unsafe fn alloc(_: Self) -> id {
        "NSMenuItem".send("alloc", ())
    }

    unsafe fn new(_: Self) -> id {
        "NSMenuItem".send("new", ())
    }

    unsafe fn initWithTitle_action_keyEquivalent_(self, title: id, action: SEL, key: id) -> id;
    unsafe fn setSubmenu_(self, submenu: id);
}

impl NSMenuItem for id {
    unsafe fn initWithTitle_action_keyEquivalent_(self, title: id, action: SEL, key: id) -> id {
        self.send("initWithTitle:action:keyEquivalent:", (title, action, key))
    }

    unsafe fn setSubmenu_(self, submenu: id) {
        self.send_void("setSubmenu:", submenu)
    }
}

pub trait NSWindow {
    unsafe fn alloc(_: Self) -> id {
        "NSWindow".send("alloc", ())
    }

    unsafe fn initWithContentRect_styleMask_backing_defer_(self,
                                                           rect: NSRect,
                                                           style: NSUInteger,
                                                           backing: NSBackingStoreType,
                                                           defer: bool) -> id;
    unsafe fn setTitle_(self, title: id);
    unsafe fn makeKeyAndOrderFront_(self, sender: id);
    unsafe fn center(self);

    // Sizing Windows
    unsafe fn frame(self) -> NSRect;
    unsafe fn setFrameOrigin_(self, point: NSPoint);
    unsafe fn setFrameTopLeftPoint_(self, point: NSPoint);
    // skipped: constrainFrameRect_toScreen_
    unsafe fn cascadeTopLeftFromPoint_(self, topLeft: NSPoint) -> NSPoint;
    unsafe fn setFrame_displayViews_(self, windowFrame: NSRect, display: bool);
    unsafe fn aspectRatio(self) -> NSSize;
    unsafe fn setAspectRatio_(self, aspectRatio: NSSize);
    unsafe fn minSize(self) -> NSSize;
    unsafe fn setMinSize_(self, minSize: NSSize);
    unsafe fn maxSize(self) -> NSSize;
    unsafe fn setMaxSize_(self, maxSize: NSSize);
    unsafe fn performZoom_(self, sender: id);
    unsafe fn zoom_(self, sender: id);
    // skipped: resizeFlags
    unsafe fn showsResizeIndicator(self) -> bool;
    unsafe fn setShowsResizeIndicator_(self, showsResizeIndicator: bool);
    unsafe fn resizeIncrements(self) -> NSSize;
    unsafe fn setResizeIncrements_(self, resizeIncrements: NSSize);
    unsafe fn preservesContentDuringLiveResize(self) -> bool;
    unsafe fn setPreservesContentDuringLiveResize_(self, preservesContentDuringLiveResize: bool);
    unsafe fn inLiveResize(self) -> bool;

    // Managing Window Layers
    unsafe fn orderOut_(self, sender: id);
    unsafe fn orderBack_(self, sender: id);
    unsafe fn orderFront_(self, sender: id);
    unsafe fn orderFrontRegardless(self);
    unsafe fn orderFrontWindow_relativeTo_(self, orderingMode: NSWindowOrderingMode, otherWindowNumber: NSInteger);
    unsafe fn level(self) -> NSInteger;
    unsafe fn setLevel_(self, level: NSInteger);

    // Converting Coordinates
    unsafe fn backingScaleFactor(self) -> CGFloat;
    unsafe fn backingAlignedRect_options_(self, rect: NSRect, options: NSAlignmentOptions) -> NSRect;
    unsafe fn convertRectFromBacking_(self, rect: NSRect) -> NSRect;
    unsafe fn convertRectToBacking_(self, rect: NSRect) -> NSRect;
    unsafe fn convertRectToScreen_(self, rect: NSRect) -> NSRect;
    unsafe fn convertRectFromScreen_(self, rect: NSRect) -> NSRect;
}

impl NSWindow for id {
    unsafe fn initWithContentRect_styleMask_backing_defer_(self,
                                                           rect: NSRect,
                                                           style: NSUInteger,
                                                           backing: NSBackingStoreType,
                                                           defer: bool) -> id {
        self.send("initWithContentRect:styleMask:backing:defer:",
                  (rect, style, backing as NSUInteger, defer))
    }

    unsafe fn setTitle_(self, title: id) {
        self.send_void("setTitle:", title);
    }

    unsafe fn makeKeyAndOrderFront_(self, sender: id) {
        self.send_void("makeKeyAndOrderFront:", sender)
    }

    unsafe fn center(self) {
        self.send_void("center", ())
    }

    // Sizing Windows

    unsafe fn frame(self) -> NSRect {
        self.send_rect("frame", ())
    }

    unsafe fn setFrameOrigin_(self, point: NSPoint) {
        self.send_void("setFrameOrigin:", point);
    }

    unsafe fn setFrameTopLeftPoint_(self, point: NSPoint) {
        self.send_void("setFrameTopLeftPoint:", point);
    }

    unsafe fn cascadeTopLeftFromPoint_(self, topLeft: NSPoint) -> NSPoint {
        self.send_point("cascadeTopLeftFromPoint:", topLeft)
    }

    unsafe fn setFrame_displayViews_(self, windowFrame: NSRect, display: bool) {
        self.send_void("setFrame:displayViews:", (windowFrame, display));
    }

    unsafe fn aspectRatio(self) -> NSSize {
        self.send_size("aspectRatio", ())
    }

    unsafe fn setAspectRatio_(self, aspectRatio: NSSize) {
        self.send_void("setAspectRatio:", aspectRatio);
    }

    unsafe fn minSize(self) -> NSSize {
        self.send_size("minSize", ())
    }

    unsafe fn setMinSize_(self, minSize: NSSize) {
        self.send_void("setMinSize:", minSize);
    }

    unsafe fn maxSize(self) -> NSSize {
        self.send_size("maxSize", ())
    }

    unsafe fn setMaxSize_(self, maxSize: NSSize) {
        self.send_void("setMaxSize:", maxSize);
    }

    unsafe fn performZoom_(self, sender: id) {
        self.send_void("performZoom:", sender);
    }

    unsafe fn zoom_(self, sender: id) {
        self.send_void("zoom:", sender);
    }

    unsafe fn showsResizeIndicator(self) -> bool {
        self.send_bool("showsResizeIndicator", ())
    }

    unsafe fn setShowsResizeIndicator_(self, showsResizeIndicator: bool) {
        self.send_void("setShowsResizeIndicator:", showsResizeIndicator)
    }

    unsafe fn resizeIncrements(self) -> NSSize {
        self.send_size("resizeIncrements", ())
    }

    unsafe fn setResizeIncrements_(self, resizeIncrements: NSSize) {
        self.send_void("setResizeIncrements:", resizeIncrements);
    }

    unsafe fn preservesContentDuringLiveResize(self) -> bool {
        self.send_bool("preservesContentDuringLiveResize", ())
    }

    unsafe fn setPreservesContentDuringLiveResize_(self, preservesContentDuringLiveResize: bool) {
        self.send_void("setPreservesContentDuringLiveResize:", preservesContentDuringLiveResize)
    }

    unsafe fn inLiveResize(self) -> bool {
        self.send_bool("inLiveResize", ())
    }

    // Managing Window Layers

    unsafe fn orderOut_(self, sender: id) {
        self.send_void("orderOut:", sender);
    }

    unsafe fn orderBack_(self, sender: id) {
        self.send_void("orderBack:", sender);
    }

    unsafe fn orderFront_(self, sender: id) {
        self.send_void("orderFront:", sender);
    }

    unsafe fn orderFrontRegardless(self) {
        self.send_void("orderFrontRegardless", ());
    }

    unsafe fn orderFrontWindow_relativeTo_(self, ordering_mode: NSWindowOrderingMode, other_window_number: NSInteger) {
        self.send_void("orderWindow:relativeTo:", (ordering_mode, other_window_number));
    }

    unsafe fn level(self) -> NSInteger {
        self.send_integer("level", ())
    }

    unsafe fn setLevel_(self, level: NSInteger) {
        self.send_void("setLevel:", level);
    }

    // Converting Coordinates

    unsafe fn backingScaleFactor(self) -> CGFloat {
        self.send_float("backingScaleFactor", ())
    }

    unsafe fn backingAlignedRect_options_(self, rect: NSRect, options: NSAlignmentOptions) -> NSRect {
        self.send_rect("backingAlignedRect:options:", (rect, options))
    }

    unsafe fn convertRectFromBacking_(self, rect: NSRect) -> NSRect {
        self.send_rect("convertRectFromBacking:", rect)
    }

    unsafe fn convertRectToBacking_(self, rect: NSRect) -> NSRect {
        self.send_rect("convertRectToBacking:", rect)
    }

    unsafe fn convertRectToScreen_(self, rect: NSRect) -> NSRect {
        self.send_rect("convertRectToScreen:", rect)
    }

    unsafe fn convertRectFromScreen_(self, rect: NSRect) -> NSRect {
        self.send_rect("convertRectFromScreen:", rect)
    }
}

pub trait NSString {
    unsafe fn alloc(_: Self) -> id {
        "NSString".send("alloc", ())
    }

    unsafe fn initWithUTF8String_(self, c_string: *const u8) -> id;
    unsafe fn stringByAppendingString_(self, other: id) -> id;
    unsafe fn init_str(self, string: &str) -> Self;
}

impl NSString for id {
    unsafe fn initWithUTF8String_(self, c_string: *const u8) -> id {
        self.send("initWithUTF8String:", c_string as id)
    }

    unsafe fn stringByAppendingString_(self, other: id) -> id {
        self.send("stringByAppendingString:", other)
    }

    unsafe fn init_str(self, string: &str) -> id {
        self.initWithUTF8String_(string.as_ptr())
    }
}

