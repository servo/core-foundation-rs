// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_upper_case_globals)]

use base::{id, msg_send, msg_send_stret, class, selector};
use base::{SEL, NSInteger, NSUInteger};
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
    msg_send()(class("NSApplication"), selector("sharedApplication"))
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

#[repr(u64)]
pub enum NSEventModifierFlags {
    NSAlphaShiftKeyMask                     = 1 << 16,
    NSShiftKeyMask                          = 1 << 17,
    NSControlKeyMask                        = 1 << 18,
    NSAlternateKeyMask                      = 1 << 19,
    NSCommandKeyMask                        = 1 << 20,
    NSNumericPadKeyMask                     = 1 << 21,
    NSHelpKeyMask                           = 1 << 22,
    NSFunctionKeyMask                       = 1 << 23,
    NSDeviceIndependentModifierFlagsMask    = 0xffff0000,
}

pub static NSMainMenuWindowLevel: libc::int32_t = 24;

pub trait NSAutoreleasePool {
    unsafe fn new(_: Self) -> id {
        msg_send()(class("NSAutoreleasePool"), selector("new"))
    }

    unsafe fn autorelease(self) -> Self;
}

impl NSAutoreleasePool for id {
    unsafe fn autorelease(self) -> id {
        msg_send()(self, selector("autorelease"))
    }
}

pub trait NSProcessInfo {
    unsafe fn processInfo(_: Self) -> id {
        msg_send()(class("NSProcessInfo"), selector("processInfo"))
    }

    unsafe fn processName(self) -> id;
}

impl NSProcessInfo for id {
    unsafe fn processName(self) -> id {
        msg_send()(self, selector("processName"))
    }
}

pub trait NSApplication {
    unsafe fn sharedApplication(_: Self) -> id {
        msg_send()(class("NSApplication"), selector("sharedApplication"))
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
        msg_send()(self, selector("setActivationPolicy:"), policy as NSInteger)
    }

    unsafe fn setMainMenu_(self, menu: id) {
        msg_send()(self, selector("setMainMenu:"), menu)
    }

    unsafe fn activateIgnoringOtherApps_(self, ignore: bool) {
        msg_send()(self, selector("activateIgnoringOtherApps:"), ignore as libc::c_int)
    }

    unsafe fn run(self) {
        msg_send()(self, selector("run"))
    }

    unsafe fn finishLaunching(self) {
        msg_send()(self, selector("finishLaunching"))
    }

    unsafe fn nextEventMatchingMask_untilDate_inMode_dequeue_(self,
                                                              mask: NSUInteger,
                                                              expiration: id,
                                                              in_mode: id,
                                                              dequeue: bool) -> id {
        msg_send()(self, selector("nextEventMatchingMask:untilDate:inMode:dequeue:"),
                   mask, expiration, in_mode, dequeue as libc::c_int)
    }

    unsafe fn sendEvent_(self, an_event: id) {
        msg_send()(self, selector("sendEvent:"), an_event)
    }
}

pub trait NSMenu {
    unsafe fn new(_: Self) -> id {
        msg_send()(class("NSMenu"), selector("new"))
    }

    unsafe fn addItem_(self, menu_item: id);
}

impl NSMenu for id {
    unsafe fn addItem_(self, menu_item: id) {
        msg_send()(self, selector("addItem:"), menu_item)
    }
}

pub trait NSMenuItem {
    unsafe fn alloc(_: Self) -> id {
        msg_send()(class("NSMenuItem"), selector("alloc"))
    }

    unsafe fn new(_: Self) -> id {
        msg_send()(class("NSMenuItem"), selector("new"))
    }

    unsafe fn initWithTitle_action_keyEquivalent_(self, title: id, action: SEL, key: id) -> id;
    unsafe fn setSubmenu_(self, submenu: id);
}

impl NSMenuItem for id {
    unsafe fn initWithTitle_action_keyEquivalent_(self, title: id, action: SEL, key: id) -> id {
        msg_send()(self, selector("initWithTitle:action:keyEquivalent:"), title, action, key)
    }

    unsafe fn setSubmenu_(self, submenu: id) {
        msg_send()(self, selector("setSubmenu:"), submenu)
    }
}

pub trait NSWindow {
    unsafe fn alloc(_: Self) -> id {
        msg_send()(class("NSWindow"), selector("alloc"))
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

    unsafe fn setContentView_(self, view: id);
    unsafe fn setAcceptsMouseMovedEvents_(self, accept: bool);
    unsafe fn isVisible(self) -> bool;
}

impl NSWindow for id {
    unsafe fn initWithContentRect_styleMask_backing_defer_(self,
                                                           rect: NSRect,
                                                           style: NSUInteger,
                                                           backing: NSBackingStoreType,
                                                           defer: bool) -> id {
        msg_send()(self, selector("initWithContentRect:styleMask:backing:defer:"),
                   rect, style, backing as NSUInteger, defer as libc::c_int)
    }

    // Sizing Windows

    unsafe fn frame(self) -> NSRect {
        msg_send()(self, selector("frame"))
    }

    unsafe fn setFrameOrigin_(self, point: NSPoint) {
        msg_send()(self, selector("setFrameOrigin:"), point)
    }

    unsafe fn setFrameTopLeftPoint_(self, point: NSPoint) {
        msg_send()(self, selector("setFrameTopLeftPoint:"), point)
    }

    unsafe fn cascadeTopLeftFromPoint_(self, topLeft: NSPoint) -> NSPoint {
        msg_send()(self, selector("cascadeTopLeftFromPoint:"), topLeft)
    }

    unsafe fn setFrame_displayViews_(self, windowFrame: NSRect, display: bool) {
        msg_send()(self, selector("setFrame:displayViews:"), windowFrame, display as libc::c_int)
    }

    unsafe fn aspectRatio(self) -> NSSize {
        msg_send()(self, selector("aspectRatio"))
    }

    unsafe fn setAspectRatio_(self, aspectRatio: NSSize) {
        msg_send()(self, selector("setAspectRatio:"), aspectRatio)
    }

    unsafe fn minSize(self) -> NSSize {
        msg_send()(self, selector("minSize"))
    }

    unsafe fn setMinSize_(self, minSize: NSSize) {
        msg_send()(self, selector("setMinSize:"), minSize)
    }

    unsafe fn maxSize(self) -> NSSize {
        msg_send()(self, selector("maxSize"))
    }

    unsafe fn setMaxSize_(self, maxSize: NSSize) {
        msg_send()(self, selector("setMaxSize:"), maxSize)
    }

    unsafe fn performZoom_(self, sender: id) {
        msg_send()(self, selector("performZoom:"), sender)
    }

    unsafe fn zoom_(self, sender: id) {
        msg_send()(self, selector("zoom:"), sender)
    }

    unsafe fn showsResizeIndicator(self) -> bool {
        msg_send()(self, selector("showsResizeIndicator"))
    }

    unsafe fn setShowsResizeIndicator_(self, showsResizeIndicator: bool) {
        msg_send()(self, selector("setShowsResizeIndicator:"), showsResizeIndicator as libc::c_int)
    }

    unsafe fn resizeIncrements(self) -> NSSize {
        msg_send()(self, selector("resizeIncrements"))
    }

    unsafe fn setResizeIncrements_(self, resizeIncrements: NSSize) {
        msg_send()(self, selector("setResizeIncrements:"), resizeIncrements)
    }

    unsafe fn preservesContentDuringLiveResize(self) -> bool {
        msg_send()(self, selector("preservesContentDuringLiveResize"))
    }

    unsafe fn setPreservesContentDuringLiveResize_(self, preservesContentDuringLiveResize: bool) {
        msg_send()(self, selector("setPreservesContentDuringLiveResize:"), preservesContentDuringLiveResize as libc::c_int)
    }

    unsafe fn inLiveResize(self) -> bool {
        msg_send()(self, selector("inLiveResize"))
    }

    // Managing Window Layers

    unsafe fn orderOut_(self, sender: id) {
        msg_send()(self, selector("orderOut:"), sender)
    }

    unsafe fn orderBack_(self, sender: id) {
        msg_send()(self, selector("orderBack:"), sender)
    }

    unsafe fn orderFront_(self, sender: id) {
        msg_send()(self, selector("orderFront:"), sender)
    }

    unsafe fn orderFrontRegardless(self) {
        msg_send()(self, selector("orderFrontRegardless"))
    }

    unsafe fn orderFrontWindow_relativeTo_(self, ordering_mode: NSWindowOrderingMode, other_window_number: NSInteger) {
        msg_send()(self, selector("orderWindow:relativeTo:"), ordering_mode, other_window_number)
    }

    unsafe fn level(self) -> NSInteger {
        msg_send()(self, selector("level"))
    }

    unsafe fn setLevel_(self, level: NSInteger) {
        msg_send()(self, selector("setLevel:"), level)
    }

    // Managing Key Status

    unsafe fn canBecomeKeyWindow(self) -> bool {
        msg_send()(self, selector("canBecomeKeyWindow"))
    }

    unsafe fn makeKeyWindow(self) {
        msg_send()(self, selector("makeKeyWindow"))
    }

    unsafe fn makeKeyAndOrderFront_(self, sender: id) {
        msg_send()(self, selector("makeKeyAndOrderFront:"), sender)
    }

    // Managing Main Status

    unsafe fn canBecomeMainWindow(self) -> bool {
        msg_send()(self, selector("canBecomeMainWindow"))
    }

    unsafe fn makeMainWindow(self) {
        msg_send()(self, selector("makeMainWindow"))
    }

    // Converting Coordinates

    unsafe fn backingScaleFactor(self) -> CGFloat {
        msg_send()(self, selector("backingScaleFactor"))
    }

    unsafe fn backingAlignedRect_options_(self, rect: NSRect, options: NSAlignmentOptions) -> NSRect {
        msg_send()(self, selector("backingAlignedRect:options:"), rect, options)
    }

    unsafe fn convertRectFromBacking_(self, rect: NSRect) -> NSRect {
        msg_send()(self, selector("convertRectFromBacking:"), rect)
    }

    unsafe fn convertRectToBacking_(self, rect: NSRect) -> NSRect {
        msg_send()(self, selector("convertRectToBacking:"), rect)
    }

    unsafe fn convertRectToScreen_(self, rect: NSRect) -> NSRect {
        msg_send()(self, selector("convertRectToScreen:"), rect)
    }

    unsafe fn convertRectFromScreen_(self, rect: NSRect) -> NSRect {
        msg_send()(self, selector("convertRectFromScreen:"), rect)
    }

    // Accessing Edited Status

    unsafe fn setDocumentEdited_(self, documentEdited: bool) {
        msg_send()(self, selector("setDocumentEdited:"), documentEdited as libc::c_int)
    }

    // Managing Titles

    unsafe fn title(self) -> id {
        msg_send()(self, selector("title"))
    }

    unsafe fn setTitle_(self, title: id) {
        msg_send()(self, selector("setTitle:"), title)
    }

    unsafe fn setTitleWithRepresentedFilename_(self, filePath: id) {
        msg_send()(self, selector("setTitleWithRepresentedFilename:"), filePath)
    }

    unsafe fn representedFilename(self) -> id {
        msg_send()(self, selector("representedFilename"))
    }

    unsafe fn setRepresentedFilename_(self, filePath: id) {
        msg_send()(self, selector("setRepresentedFilename:"), filePath)
    }

    // Moving Windows

    unsafe fn setMovableByWindowBackground_(self, movableByWindowBackground: bool) {
        msg_send()(self, selector("setMovableByWindowBackground:"), movableByWindowBackground as libc::c_int)
    }

    unsafe fn setMovable_(self, movable: bool) {
        msg_send()(self, selector("setMovable:"), movable as libc::c_int)
    }

    unsafe fn center(self) {
        msg_send()(self, selector("center"))
    }

    // Closing Windows

    unsafe fn performClose_(self, sender: id) {
        msg_send()(self, selector("performClose:"), sender)
    }

    unsafe fn close(self) {
        msg_send()(self, selector("close"))
    }

    unsafe fn setReleasedWhenClosed_(self, releasedWhenClosed: bool) {
        msg_send()(self, selector("setReleasedWhenClosed:"), releasedWhenClosed as libc::c_int)
    }

    // Minimizing Windows

    unsafe fn performMiniaturize_(self, sender: id) {
        msg_send()(self, selector("performMiniaturize:"), sender)
    }

    unsafe fn miniaturize_(self, sender: id) {
        msg_send()(self, selector("miniaturize:"), sender)
    }

    unsafe fn deminiaturize_(self, sender: id) {
        msg_send()(self, selector("deminiaturize:"), sender)
    }

    unsafe fn miniwindowTitle(self) -> id {
        msg_send()(self, selector("miniwindowTitle"))
    }

    unsafe fn setMiniwindowTitle_(self, miniwindowTitle: id) {
        msg_send()(self, selector("setMiniwindowTitle:"), miniwindowTitle)
    }

    unsafe fn setContentView_(self, view: id) {
        msg_send()(self, selector("setContentView:"), view)
    }

    unsafe fn setAcceptsMouseMovedEvents_(self, accept: bool) {
        msg_send()(self, selector("setAcceptsMouseMovedEvents:"), accept as libc::c_int)
    }

    unsafe fn isVisible(self) -> bool {
        msg_send()(self, selector("isVisible"))
    }
}

pub trait NSString {
    unsafe fn alloc(_: Self) -> id {
        msg_send()(class("NSString"), selector("alloc"))
    }

    unsafe fn initWithUTF8String_(self, c_string: *const u8) -> id;
    unsafe fn stringByAppendingString_(self, other: id) -> id;
    unsafe fn init_str(self, string: &str) -> Self;
    unsafe fn UTF8String(self) -> *const libc::c_char;
}

impl NSString for id {
    unsafe fn initWithUTF8String_(self, c_string: *const u8) -> id {
        msg_send()(self, selector("initWithUTF8String:"), c_string as id)
    }

    unsafe fn stringByAppendingString_(self, other: id) -> id {
        msg_send()(self, selector("stringByAppendingString:"), other)
    }

    unsafe fn init_str(self, string: &str) -> id {
        self.initWithUTF8String_(string.as_ptr())
    }

    unsafe fn UTF8String(self) -> *const libc::c_char {
        msg_send()(self, selector("UTF8String"))
    }
}

pub trait NSView {
    unsafe fn alloc(_: Self) -> id {
        msg_send()(class("NSView"), selector("alloc"))
    }

    unsafe fn init(self) -> id;
    unsafe fn initWithFrame_(self, frameRect: NSRect) -> id;
    unsafe fn display_(self);
    unsafe fn setWantsBestResolutionOpenGLSurface_(self, flag: bool);
    unsafe fn convertPoint_fromView_(self, point: NSPoint, view: id) -> NSPoint;
}

impl NSView for id {
    unsafe fn init(self) -> id {
        msg_send()(self, selector("init"))
    }

    unsafe fn initWithFrame_(self, frameRect: NSRect) -> id {
        msg_send()(self, selector("initWithFrame:"), frameRect)
    }

    unsafe fn display_(self) {
        msg_send()(self, selector("display"))
    }

    unsafe fn setWantsBestResolutionOpenGLSurface_(self, flag: bool) {
        msg_send()(self, selector("setWantsBestResolutionOpenGLSurface:"), flag as libc::c_int)
    }

    unsafe fn convertPoint_fromView_(self, point: NSPoint, view: id) -> NSPoint {
        msg_send()(self, selector("convertPoint:fromView:"), point, view)
    }
}

pub trait NSOpenGLView {
    unsafe fn alloc(_: Self) -> id {
        msg_send()(class("NSOpenGLView"), selector("alloc"))
    }

    unsafe fn initWithFrame_pixelFormat_(self, frameRect: NSRect, format: id) -> id;
    unsafe fn display_(self);
}

impl NSOpenGLView for id {
    unsafe fn initWithFrame_pixelFormat_(self,  frameRect: NSRect, format: id) -> id {
        msg_send()(self, selector("initWithFrame:pixelFormat:"), frameRect, format)
    }

    unsafe fn display_(self) {
        msg_send()(self, selector("display"))
    }
}

pub trait NSOpenGLPixelFormat {
    unsafe fn alloc(_: Self) -> id {
        msg_send()(class("NSOpenGLPixelFormat"), selector("alloc"))
    }

    unsafe fn initWithAttributes_(self, attributes: &[uint]) -> id;
}

impl NSOpenGLPixelFormat for id {
    unsafe fn initWithAttributes_(self, attributes: &[uint]) -> id {
        msg_send()(self, selector("initWithAttributes:"), attributes)
    }
}

pub trait NSOpenGLContext {
    unsafe fn alloc(_: Self) -> id {
        msg_send()(class("NSOpenGLContext"), selector("alloc"))
    }

    unsafe fn initWithFormat_shareContext_(self, format: id, shareContext: id) -> id;
    unsafe fn setView_(self, view: id);
    unsafe fn makeCurrentContext(self);
    unsafe fn flushBuffer(self);
}

impl NSOpenGLContext for id {
    unsafe fn initWithFormat_shareContext_(self, format: id, shareContext: id) -> id {
        msg_send()(self, selector("initWithFormat:shareContext:"), format, shareContext)
    }

    unsafe fn setView_(self, view: id) {
        msg_send()(self, selector("setView:"), view)
    }

    unsafe fn makeCurrentContext(self) {
        msg_send()(self, selector("makeCurrentContext"))
    }

    unsafe fn flushBuffer(self) {
        msg_send()(self, selector("flushBuffer"))
    }
}

pub trait NSDate {
    unsafe fn distantPast(_: Self) -> id {
        msg_send()(class("NSDate"), selector("distantPast"))
    }

    unsafe fn distantFuture(_: Self) -> id {
        msg_send()(class("NSDate"), selector("distantFuture"))
    }
}

impl NSDate for id {

}

pub trait NSEvent {
    unsafe fn get_type(self) -> NSEventType;
    unsafe fn locationInWindow(self) -> NSPoint;
    unsafe fn characters(self) -> id;
    unsafe fn charactersIgnoringModifiers(self) -> id;
    unsafe fn keycode(self) -> libc::c_ushort;
    unsafe fn modifierFlags(self) -> NSUInteger;
}

impl NSEvent for id {
    unsafe fn get_type(self) -> NSEventType {
        msg_send()(self, selector("type"))
    }

    unsafe fn locationInWindow(self) -> NSPoint {
        msg_send()(self, selector("locationInWindow"))
    }

    unsafe fn characters(self) -> id {
        msg_send()(self, selector("characters"))
    }

    unsafe fn charactersIgnoringModifiers(self) -> id {
        msg_send()(self, selector("charactersIgnoringModifiers"))
    }

    unsafe fn keycode(self) -> libc::c_ushort {
        msg_send()(self, selector("keyCode"))
    }

    unsafe fn modifierFlags(self) -> NSUInteger {
        msg_send()(self, selector("modifierFlags"))
    }
}

pub trait NSScreen {
    unsafe fn mainScreen(_: Self) -> id {
        msg_send()(class("NSScreen"), selector("mainScreen"))
    }
    unsafe fn frame(self) -> NSRect;
}

impl NSScreen for id {
    unsafe fn frame(self) -> NSRect {
        msg_send_stret()(self, selector("frame"))
    }
}
