// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_upper_case_globals)]

use base::{ObjCMethodCall, id, SEL, NSInteger, NSUInteger};
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

#[link(name = "Foundation", kind = "framework")]
extern {
    pub static NSDefaultRunLoopMode: id;
}

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

#[repr(u64)]
pub enum NSOpenGLPixelFormatAttribute {
    NSOpenGLPFAAllRenderers             = 1,
    NSOpenGLPFATripleBuffer             = 3,
    NSOpenGLPFADoubleBuffer             = 5,
    NSOpenGLPFAStereo                   = 6,
    NSOpenGLPFAAuxBuffers               = 7,
    NSOpenGLPFAColorSize                = 8,
    NSOpenGLPFAAlphaSize                = 11,
    NSOpenGLPFADepthSize                = 12,
    NSOpenGLPFAStencilSize              = 13,
    NSOpenGLPFAAccumSize                = 14,
    NSOpenGLPFAMinimumPolicy            = 51,
    NSOpenGLPFAMaximumPolicy            = 52,
    NSOpenGLPFAOffScreen                = 53,
    NSOpenGLPFAFullScreen               = 54,
    NSOpenGLPFASampleBuffers            = 55,
    NSOpenGLPFASamples                  = 56,
    NSOpenGLPFAAuxDepthStencil          = 57,
    NSOpenGLPFAColorFloat               = 58,
    NSOpenGLPFAMultisample              = 59,
    NSOpenGLPFASupersample              = 60,
    NSOpenGLPFASampleAlpha              = 61,
    NSOpenGLPFARendererID               = 70,
    NSOpenGLPFASingleRenderer           = 71,
    NSOpenGLPFANoRecovery               = 72,
    NSOpenGLPFAAccelerated              = 73,
    NSOpenGLPFAClosestPolicy            = 74,
    NSOpenGLPFARobust                   = 75,
    NSOpenGLPFABackingStore             = 76,
    NSOpenGLPFAMPSafe                   = 78,
    NSOpenGLPFAWindow                   = 80,
    NSOpenGLPFAMultiScreen              = 81,
    NSOpenGLPFACompliant                = 83,
    NSOpenGLPFAScreenMask               = 84,
    NSOpenGLPFAPixelBuffer              = 90,
    NSOpenGLPFARemotePixelBuffer        = 91,
    NSOpenGLPFAAllowOfflineRenderers    = 96,
    NSOpenGLPFAAcceleratedCompute       = 97,
    NSOpenGLPFAOpenGLProfile            = 99,
    NSOpenGLPFAVirtualScreenCount       = 128,
}

#[repr(u64)]
pub enum NSEventType {
    NSLeftMouseDown         = 1,
    NSLeftMouseUp           = 2,
    NSRightMouseDown        = 3,
    NSRightMouseUp          = 4,
    NSMouseMoved            = 5,
    NSLeftMouseDragged      = 6,
    NSRightMouseDragged     = 7,
    NSMouseEntered          = 8,
    NSMouseExited           = 9,
    NSKeyDown               = 10,
    NSKeyUp                 = 11,
    NSFlagsChanged          = 12,
    NSAppKitDefined         = 13,
    NSSystemDefined         = 14,
    NSApplicationDefined    = 15,
    NSPeriodic              = 16,
    NSCursorUpdate          = 17,
    NSScrollWheel           = 22,
    NSTabletPoint           = 23,
    NSTabletProximity       = 24,
    NSOtherMouseDown        = 25,
    NSOtherMouseUp          = 26,
    NSOtherMouseDragged     = 27,
    NSEventTypeGesture      = 29,
    NSEventTypeMagnify      = 30,
    NSEventTypeSwipe        = 31,
    NSEventTypeRotate       = 18,
    NSEventTypeBeginGesture = 19,
    NSEventTypeEndGesture   = 20,
}

#[repr(u64)]
pub enum NSEventMask {
    NSLeftMouseDownMask         = 1 << NSLeftMouseDown as uint,
    NSLeftMouseUpMask           = 1 << NSLeftMouseUp as uint,
    NSRightMouseDownMask        = 1 << NSRightMouseDown as uint,
    NSRightMouseUpMask          = 1 << NSRightMouseUp as uint,
    NSMouseMovedMask            = 1 << NSMouseMoved as uint,
    NSLeftMouseDraggedMask      = 1 << NSLeftMouseDragged as uint,
    NSRightMouseDraggedMask     = 1 << NSRightMouseDragged as uint,
    NSMouseEnteredMask          = 1 << NSMouseEntered as uint,
    NSMouseExitedMask           = 1 << NSMouseExited as uint,
    NSKeyDownMask               = 1 << NSKeyDown as uint,
    NSKeyUpMask                 = 1 << NSKeyUp as uint,
    NSFlagsChangedMask          = 1 << NSFlagsChanged as uint,
    NSAppKitDefinedMask         = 1 << NSAppKitDefined as uint,
    NSSystemDefinedMask         = 1 << NSSystemDefined as uint,
    NSAPplicationDefinedMask    = 1 << NSApplicationDefined as uint,
    NSPeriodicMask              = 1 << NSPeriodic as uint,
    NSCursorUpdateMask          = 1 << NSCursorUpdate as uint,
    NSScrollWheelMask           = 1 << NSScrollWheel as uint,
    NSTabletPointMask           = 1 << NSTabletPoint as uint,
    NSTabletProximityMask       = 1 << NSTabletProximity as uint,
    NSOtherMouseDownMask        = 1 << NSOtherMouseDown as uint,
    NSOtherMouseUpMask          = 1 << NSOtherMouseUp as uint,
    NSOtherMouseDraggedMask     = 1 << NSOtherMouseDragged as uint,
    NSEventMaskgesture          = 1 << NSEventTypeGesture as uint,
    NSEventMaskSwipe            = 1 << NSEventTypeSwipe as uint,
    NSEventMaskRotate           = 1 << NSEventTypeRotate as uint,
    NSEventMaskBeginGesture     = 1 << NSEventTypeBeginGesture as uint,
    NSEventMaskEndGesture       = 1 << NSEventTypeEndGesture as uint,
    NSAnyEventMask              = 0xffffffff,
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
    unsafe fn finishLaunching(self);
    unsafe fn nextEventMatchingMask_untilDate_inMode_dequeue_(self,
                                                              mask: NSUInteger,
                                                              expiration: id,
                                                              in_mode: id,
                                                              dequeue: bool) -> id;
    unsafe fn sendEvent_(self, an_event: id);
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

    unsafe fn finishLaunching(self) {
        self.send_void("finishLaunching", ())
    }

    unsafe fn nextEventMatchingMask_untilDate_inMode_dequeue_(self,
                                                              mask: NSUInteger,
                                                              expiration: id,
                                                              in_mode: id,
                                                              dequeue: bool) -> id {
        self.send("nextEventMatchingMask:untilDate:inMode:dequeue:",
                  (mask, expiration, in_mode, dequeue))
    }

    unsafe fn sendEvent_(self, an_event: id) {
        self.send_void("sendEvent:", an_event)
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

    // Managing Key Status
    unsafe fn canBecomeKeyWindow(self) -> bool;
    unsafe fn makeKeyWindow(self);
    unsafe fn makeKeyAndOrderFront_(self, sender: id);
    // skipped: becomeKeyWindow (should not be invoked directly, according to Apple's documentation)
    // skipped: resignKeyWindow (should not be invoked directly, according to Apple's documentation)

    // Managing Main Status
    unsafe fn canBecomeMainWindow(self) -> bool;
    unsafe fn makeMainWindow(self);
    // skipped: becomeMainWindow (should not be invoked directly, according to Apple's documentation)
    // skipped: resignMainWindow (should not be invoked directly, according to Apple's documentation)

    // Converting Coordinates
    unsafe fn backingScaleFactor(self) -> CGFloat;
    unsafe fn backingAlignedRect_options_(self, rect: NSRect, options: NSAlignmentOptions) -> NSRect;
    unsafe fn convertRectFromBacking_(self, rect: NSRect) -> NSRect;
    unsafe fn convertRectToBacking_(self, rect: NSRect) -> NSRect;
    unsafe fn convertRectToScreen_(self, rect: NSRect) -> NSRect;
    unsafe fn convertRectFromScreen_(self, rect: NSRect) -> NSRect;

    // Accessing Edited Status
    unsafe fn setDocumentEdited_(self, documentEdited: bool);

    // Managing Titles
    unsafe fn title(self) -> id;
    unsafe fn setTitle_(self, title: id);
    unsafe fn setTitleWithRepresentedFilename_(self, filePath: id);
    unsafe fn representedFilename(self) -> id;
    unsafe fn setRepresentedFilename_(self, filePath: id);
    // skipped: representedURL
    // skipped: setRepresentedURL_

    // Moving Windows
    unsafe fn setMovableByWindowBackground_(self, movableByWindowBackground: bool);
    unsafe fn setMovable_(self, movable: bool);
    unsafe fn center(self);

    // Closing Windows
    unsafe fn performClose_(self, sender: id);
    unsafe fn close(self);
    unsafe fn setReleasedWhenClosed_(self, releasedWhenClosed: bool);

    // Minimizing Windows
    unsafe fn performMiniaturize_(self, sender: id);
    unsafe fn miniaturize_(self, sender: id);
    unsafe fn deminiaturize_(self, sender: id);
    // skipped: miniwindowImage
    // skipped: setMiniwindowImage
    unsafe fn miniwindowTitle(self) -> id;
    unsafe fn setMiniwindowTitle_(self, miniwindowTitle: id);

    unsafe fn setContentView(self, view: id);
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

    // Managing Key Status

    unsafe fn canBecomeKeyWindow(self) -> bool {
        self.send_bool("canBecomeKeyWindow", ())
    }

    unsafe fn makeKeyWindow(self) {
        self.send_void("makeKeyWindow", ());
    }

    unsafe fn makeKeyAndOrderFront_(self, sender: id) {
        self.send_void("makeKeyAndOrderFront:", sender);
    }

    // Managing Main Status

    unsafe fn canBecomeMainWindow(self) -> bool {
        self.send_bool("canBecomeMainWindow", ())
    }

    unsafe fn makeMainWindow(self) {
        self.send_void("makeMainWindow", ());
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

    // Accessing Edited Status

    unsafe fn setDocumentEdited_(self, documentEdited: bool) {
        self.send_void("setDocumentEdited:", documentEdited);
    }

    // Managing Titles

    unsafe fn title(self) -> id {
        self.send("title", ())
    }

    unsafe fn setTitle_(self, title: id) {
        self.send_void("setTitle:", title);
    }

    unsafe fn setTitleWithRepresentedFilename_(self, filePath: id) {
        self.send_void("setTitleWithRepresentedFilename:", filePath);
    }

    unsafe fn representedFilename(self) -> id {
        self.send("representedFilename", ())
    }

    unsafe fn setRepresentedFilename_(self, filePath: id) {
        self.send_void("setRepresentedFilename:", filePath);
    }

    // Moving Windows

    unsafe fn setMovableByWindowBackground_(self, movableByWindowBackground: bool) {
        self.send_void("setMovableByWindowBackground:", movableByWindowBackground);
    }

    unsafe fn setMovable_(self, movable: bool) {
        self.send_void("setMovable:", movable);
    }

    unsafe fn center(self) {
        self.send_void("center", ());
    }

    // Closing Windows

    unsafe fn performClose_(self, sender: id) {
        self.send_void("performClose:", sender);
    }

    unsafe fn close(self) {
        self.send_void("close", ());
    }

    unsafe fn setReleasedWhenClosed_(self, releasedWhenClosed: bool) {
        self.send_void("setReleasedWhenClosed:", releasedWhenClosed);
    }

    // Minimizing Windows

    unsafe fn performMiniaturize_(self, sender: id) {
        self.send_void("performMiniaturize:", sender);
    }

    unsafe fn miniaturize_(self, sender: id) {
        self.send_void("miniaturize:", sender);
    }

    unsafe fn deminiaturize_(self, sender: id) {
        self.send_void("deminiaturize:", sender);
    }

    unsafe fn miniwindowTitle(self) -> id {
        self.send("miniwindowTitle", ())
    }

    unsafe fn setMiniwindowTitle_(self, miniwindowTitle: id) {
        self.send_void("setMiniwindowTitle:", miniwindowTitle);
    }

    unsafe fn setContentView(self, view: id) {
        self.send_void("setContentView:", view)
    }

    unsafe fn setContentView(self, view: id) {
        self.send_void("setContentView:", view)
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

pub trait NSView {
    unsafe fn alloc(_: Self) -> id {
        "NSView".send("alloc", ())
    }

    unsafe fn init(self) -> id;
    unsafe fn initWithFrame_(self, frameRect: NSRect) -> id;
    unsafe fn display_(self);
    unsafe fn setWantsBestResolutionOpenGLSurface_(self, flag: bool);
}

impl NSView for id {
    unsafe fn init(self) -> id {
        self.send("init", ())
    }

    unsafe fn initWithFrame_(self, frameRect: NSRect) -> id {
        self.send("initWithFrame:", frameRect)
    }

    unsafe fn display_(self) {
        self.send_void("display", ())
    }

    unsafe fn setWantsBestResolutionOpenGLSurface_(self, flag: bool) {
        self.send_void("setWantsBestResolutionOpenGLSurface:", flag)
    }
}

pub trait NSOpenGLView {
    unsafe fn alloc(_: Self) -> id {
        "NSOpenGLView".send("alloc", ())
    }

    unsafe fn initWithFrame_pixelFormat_(self, frameRect: NSRect, format: id) -> id;
    unsafe fn display_(self);
}

impl NSOpenGLView for id {
    unsafe fn initWithFrame_pixelFormat_(self,  frameRect: NSRect, format: id) -> id {
        self.send("initWithFrame:pixelFormat:", (frameRect, format))
    }

    unsafe fn display_(self) {
        self.send_void("display", ())
    }
}

pub trait NSOpenGLPixelFormat {
    unsafe fn alloc(_: Self) -> id {
        "NSOpenGLPixelFormat".send("alloc", ())
    }

    unsafe fn initWithAttributes_(self, attributes: &[uint]) -> id;
}

impl NSOpenGLPixelFormat for id {
    unsafe fn initWithAttributes_(self, attributes: &[uint]) -> id {
        self.send("initWithAttributes:", attributes)
    }
}

pub trait NSOpenGLContext {
    unsafe fn alloc(_: Self) -> id {
        "NSOpenGLContext".send("alloc", ())
    }

    unsafe fn initWithFormat_shareContext_(self, format: id, shareContext: id) -> id;
    unsafe fn setView_(self, view: id);
    unsafe fn makeCurrentContext(self);
    unsafe fn flushBuffer(self);
}

impl NSOpenGLContext for id {
    unsafe fn initWithFormat_shareContext_(self, format: id, shareContext: id) -> id {
        self.send("initWithFormat:shareContext:", (format, shareContext))
    }

    unsafe fn setView_(self, view: id) {
        self.send_void("setView:", view)
    }

    unsafe fn makeCurrentContext(self) {
        self.send_void("makeCurrentContext", ())
    }

    unsafe fn flushBuffer(self) {
        self.send_void("flushBuffer", ())
    }
}

pub trait NSDate {
    unsafe fn distantPast(_: Self) -> id {
        "NSDate".send("distantPast", ())
    }

    unsafe fn distantFuture(_: Self) -> id {
        "NSDate".send("distantFuture", ())
    }
}

impl NSDate for id {

}

pub trait NSEvent {
    unsafe fn get_type(self) -> NSEventType;
}

impl NSEvent for id {
    unsafe fn get_type(self) -> NSEventType {
        self.send_event("type", ())
    }
}
