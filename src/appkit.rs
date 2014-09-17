// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base::{ObjCMethodCall, id};

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

pub mod NSProcessInfo {
    use base::{id, ObjCMethodCall};

    pub unsafe fn processInfo() -> id {
        "NSProcessInfo".send("processInfo", ())
    }

    pub unsafe fn processName(this: id) -> id {
        this.send("processName", ())
    }
}

pub mod NSApplication {
    use base::{id, NSInteger, ObjCMethodCall};
    use appkit::NSApplicationActivationPolicy;

    pub unsafe fn setActivationPolicy_(this: id, policy: NSApplicationActivationPolicy) -> bool {
        this.send_bool("setActivationPolicy:", policy as NSInteger)
    }

    pub unsafe fn setMainMenu_(this: id, menu: id) {
        this.send_void("setMainMenu:", menu)
    }

    pub unsafe fn activateIgnoringOtherApps_(this: id, ignore: bool) {
        this.send_void("activateIgnoringOtherApps:", ignore);
    }

    pub unsafe fn run(this: id) {
        this.send_void("run", ());
    }
}

pub mod NSMenu {
    use base::{id, ObjCMethodCall};

    pub unsafe fn new() -> id {
        "NSMenu".send("new", ())
    }

    pub unsafe fn addItem_(this: id, menu_item: id) {
        this.send_void("addItem:", menu_item)
    }
}

pub mod NSMenuItem {
    use base::{id, ObjCMethodCall, SEL};

    pub unsafe fn alloc() -> id {
        "NSMenuItem".send("alloc", ())
    }

    pub unsafe fn new() -> id {
        "NSMenuItem".send("new", ())
    }

    pub unsafe fn initWithTitle_action_keyEquivalent_(this: id, title: id, action: SEL, key: id) -> id {
        this.send("initWithTitle:action:keyEquivalent:", (title, action, key))
    }

    pub unsafe fn setSubmenu_(this: id, submenu: id) {
        this.send_void("setSubmenu:", submenu)
    }
}

pub mod NSWindow {
    use base::{id, NSUInteger, ObjCMethodCall};
    use appkit::{NSRect, NSPoint, NSBackingStoreType};

    pub unsafe fn alloc() -> id {
        "NSWindow".send("alloc", ())
    }

    pub unsafe fn initWithContentRect_styleMask_backing_defer_(this: id,
                                                               rect: NSRect,
                                                               style: NSUInteger,
                                                               backing: NSBackingStoreType,
                                                               defer: bool) -> id {
        this.send("initWithContentRect:styleMask:backing:defer:",
                  (rect, style, backing as NSUInteger, defer))
    }

    pub unsafe fn cascadeTopLeftFromPoint_(this: id, top_left: NSPoint) -> NSPoint {
        this.send_point("cascadeTopLeftFromPoint:", top_left)
    }

    pub unsafe fn setTitle_(this: id, title: id) {
        this.send_void("setTitle:", title);
    }

    pub unsafe fn makeKeyAndOrderFront_(this: id, sender: id) {
        this.send_void("makeKeyAndOrderFront:", sender)
    }
}

pub mod NSString {
    use base::{id, ObjCMethodCall};

    pub unsafe fn alloc() -> id {
        "NSString".send("alloc", ())
    }

    pub unsafe fn initWithUTF8String_(this: id, c_string: *const u8) -> id {
        this.send("initWithUTF8String:", c_string as id)
    }

    pub unsafe fn stringByAppendingString_(this: id, other: id) -> id {
        this.send("stringByAppendingString:", other)
    }

    pub unsafe fn from_str(string: &str) -> id {
        initWithUTF8String_(alloc(), string.as_ptr())
    }
}

