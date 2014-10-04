// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base::{ObjCMethodCall, id, SEL, nil, NSInteger, NSUInteger};

pub type CGFloat = f32;

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
    unsafe fn cascadeTopLeftFromPoint_(self, top_left: NSPoint) -> NSPoint;
    unsafe fn setTitle_(self, title: id);
    unsafe fn makeKeyAndOrderFront_(self, sender: id);
    unsafe fn center(self);
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

    unsafe fn cascadeTopLeftFromPoint_(self, top_left: NSPoint) -> NSPoint {
        self.send_point("cascadeTopLeftFromPoint:", top_left)
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

