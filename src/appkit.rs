// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_upper_case_globals)]

use base::{id, msg_send, msg_send_fpret, msg_send_stret, class, selector};
use base::{BOOL, SEL, NSInteger, NSUInteger};
use libc;

pub use self::NSApplicationActivationPolicy::*;
pub use self::NSWindowMask::*;
pub use self::NSBackingStoreType::*;
pub use self::NSOpenGLPixelFormatAttribute::*;
pub use self::NSOpenGLPFAOpenGLProfiles::*;
pub use self::NSEventType::*;

use std::ffi::CString;

#[cfg(target_pointer_width = "32")]
pub type CGFloat = f32;
#[cfg(target_pointer_width = "64")]
pub type CGFloat = f64;

pub type CGLContextObj = *mut libc::c_void;

pub type GLint = libc::int32_t;

#[repr(C)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

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

// Same as CGRectEdge
#[repr(u32)]
pub enum NSRectEdge {
    NSRectMinXEdge,
    NSRectMinYEdge,
    NSRectMaxXEdge,
    NSRectMaxYEdge,
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
pub enum NSApplicationTerminateReply {
    NSTerminateCancel = 0,
    NSTerminateNow = 1,
    NSTerminateLater = 2,
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
#[allow(non_camel_case_types)]
pub enum NSOpenGLPFAOpenGLProfiles {
    NSOpenGLProfileVersionLegacy = 0x1000,
    NSOpenGLProfileVersion3_2Core = 0x3200,
    NSOpenGLProfileVersion4_1Core = 0x4100,
}

#[repr(u64)]
pub enum NSOpenGLContextParameter {
    NSOpenGLCPSwapInterval          = 222,
    NSOpenGLCPSurfaceOrder          = 235,
    NSOpenGLCPSurfaceOpacity        = 236,
    NSOpenGLCPSurfaceBackingSize    = 304,
    NSOpenGLCPReclaimResources      = 308,
    NSOpenGLCPCurrentRendererID     = 309,
    NSOpenGLCPGPUVertexProcessing   = 310,
    NSOpenGLCPGPUFragmentProcessing = 311,
    NSOpenGLCPHasDrawable           = 314,
    NSOpenGLCPMPSwapsInFlight       = 315,
}

pub static NSMainMenuWindowLevel: libc::int32_t = 24;

pub trait NSAutoreleasePool {
    unsafe fn new(_: Self) -> id {
        msg_send()(class("NSAutoreleasePool"), selector("new"))
    }

    unsafe fn autorelease(self) -> Self;
    unsafe fn drain(self);
}

impl NSAutoreleasePool for id {
    unsafe fn autorelease(self) -> id {
        msg_send()(self, selector("autorelease"))
    }

    unsafe fn drain(self) {
        msg_send()(self, selector("drain"))
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

pub type NSTimeInterval = libc::c_double;

pub trait NSApplication {
    unsafe fn sharedApplication(_: Self) -> id {
        msg_send()(class("NSApplication"), selector("sharedApplication"))
    }

    unsafe fn setActivationPolicy_(self, policy: NSApplicationActivationPolicy) -> BOOL;
    unsafe fn setMainMenu_(self, menu: id);
    unsafe fn activateIgnoringOtherApps_(self, ignore: BOOL);
    unsafe fn run(self);
    unsafe fn finishLaunching(self);
    unsafe fn nextEventMatchingMask_untilDate_inMode_dequeue_(self,
                                                              mask: NSUInteger,
                                                              expiration: id,
                                                              in_mode: id,
                                                              dequeue: BOOL) -> id;
    unsafe fn sendEvent_(self, an_event: id);
    unsafe fn postEvent_atStart_(self, anEvent: id, flag: BOOL);
    unsafe fn stop_(self, sender: id);
}

impl NSApplication for id {
    unsafe fn setActivationPolicy_(self, policy: NSApplicationActivationPolicy) -> BOOL {
        msg_send()(self, selector("setActivationPolicy:"), policy as NSInteger)
    }

    unsafe fn setMainMenu_(self, menu: id) {
        msg_send()(self, selector("setMainMenu:"), menu)
    }

    unsafe fn activateIgnoringOtherApps_(self, ignore: BOOL) {
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
                                                              dequeue: BOOL) -> id {
        msg_send()(self, selector("nextEventMatchingMask:untilDate:inMode:dequeue:"),
                   mask, expiration, in_mode, dequeue as libc::c_int)
    }

    unsafe fn sendEvent_(self, an_event: id) {
        msg_send()(self, selector("sendEvent:"), an_event)
    }

    unsafe fn postEvent_atStart_(self, anEvent: id, flag: BOOL) {
        msg_send()(self, selector("postEvent:atStart:"), anEvent, flag as libc::c_int)
    }

    unsafe fn stop_(self, sender: id) {
        msg_send()(self, selector("stop:"), sender)
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

pub type NSWindowDepth = libc::c_int;

bitflags! {
    flags NSWindowCollectionBehavior: NSUInteger {
        const NSWindowCollectionBehaviorDefault = 0,
        const NSWindowCollectionBehaviorCanJoinAllSpaces = 1 << 0,
        const NSWindowCollectionBehaviorMoveToActiveSpace = 1 << 1,

        const NSWindowCollectionBehaviorManaged = 1 << 2,
        const NSWindowCollectionBehaviorTransient = 1 << 3,
        const NSWindowCollectionBehaviorStationary = 1 << 4,

        const NSWindowCollectionBehaviorParticipatesInCycle = 1 << 5,
        const NSWindowCollectionBehaviorIgnoresCycle = 1 << 6,

        const NSWindowCollectionBehaviorFullScreenPrimary = 1 << 7,
        const NSWindowCollectionBehaviorFullScreenAuxiliary = 1 << 8,
    }
}

bitflags! {
    flags NSWindowOcclusionState: NSUInteger {
        const NSWindowOcclusionStateVisible = 1 << 1
    }
}

pub trait NSWindow {
    unsafe fn alloc(_: Self) -> id {
        msg_send()(class("NSWindow"), selector("alloc"))
    }

    // Creating Windows
    unsafe fn initWithContentRect_styleMask_backing_defer_(self,
                                                           rect: NSRect,
                                                           style: NSUInteger,
                                                           backing: NSBackingStoreType,
                                                           defer: BOOL) -> id;
    unsafe fn initWithContentRect_styleMask_backing_defer_screen_(self,
                                                                  rect: NSRect,
                                                                  style: NSUInteger,
                                                                  backing: NSBackingStoreType,
                                                                  defer: BOOL,
                                                                  screen: id) -> id;

    // Configuring Windows
    unsafe fn styleMask(self) -> NSUInteger;
    unsafe fn setStyleMask_(self, styleMask: NSUInteger);
    unsafe fn toggleFullScreen_(self, sender: id);
    unsafe fn worksWhenModal(self) -> BOOL;
    unsafe fn alphaValue(self) -> CGFloat;
    unsafe fn setAlphaValue_(self, windowAlpha: CGFloat);
    unsafe fn backgroundColor(self) -> id;
    unsafe fn setBackgroundColor_(self, color: id);
    unsafe fn colorSpace(self) -> id;
    unsafe fn setColorSpace_(self, colorSpace: id);
    unsafe fn contentView(self) -> id;
    unsafe fn setContentView_(self, view: id);
    unsafe fn canHide(self) -> BOOL;
    unsafe fn setCanHide_(self, canHide: BOOL);
    unsafe fn hidesOnDeactivate(self) -> BOOL;
    unsafe fn setHidesOnDeactivate_(self, hideOnDeactivate: BOOL);
    unsafe fn collectionBehavior(self) -> NSWindowCollectionBehavior;
    unsafe fn setCollectionBehavior_(self, collectionBehavior: NSWindowCollectionBehavior);
    unsafe fn setOpaque_(self, opaque: BOOL);
    unsafe fn hasShadow(self) -> BOOL;
    unsafe fn setHasShadow_(self, hasShadow: BOOL);
    unsafe fn invalidateShadow(self);
    unsafe fn autorecalculatesContentBorderThicknessForEdge_(self, edge: NSRectEdge) -> BOOL;
    unsafe fn setAutorecalculatesContentBorderThickness_forEdge_(self,
                                                                 autorecalculateContentBorderThickness: BOOL,
                                                                 edge: NSRectEdge) -> BOOL;
    unsafe fn contentBorderThicknessForEdge_(self, edge: NSRectEdge) -> CGFloat;
    unsafe fn setContentBorderThickness_forEdge_(self, borderThickness: CGFloat, edge: NSRectEdge);
    unsafe fn delegate(self) -> id;
    unsafe fn setDelegate_(self, delegate: id);
    unsafe fn preventsApplicationTerminationWhenModal(self) -> BOOL;
    unsafe fn setPreventsApplicationTerminationWhenModal_(self, flag: BOOL);

    // TODO: Accessing Window Information

    // Getting Layout Information
    unsafe fn contentRectForFrameRect_styleMask_(self, windowFrame: NSRect, windowStyle: NSUInteger) -> NSRect;
    unsafe fn frameRectForContentRect_styleMask_(self, windowContentRect: NSRect, windowStyle: NSUInteger) -> NSRect;
    unsafe fn minFrameWidthWithTitle_styleMask_(self, windowTitle: id, windowStyle: NSUInteger) -> CGFloat;
    unsafe fn contentRectForFrameRect_(self, windowFrame: NSRect) -> NSRect;
    unsafe fn frameRectForContentRect_(self, windowContent: NSRect) -> NSRect;

    // Managing Windows
    unsafe fn drawers(self) -> id;
    unsafe fn windowController(self) -> id;
    unsafe fn setWindowController_(self, windowController: id);

    // TODO: Managing Sheets

    // Sizing Windows
    unsafe fn frame(self) -> NSRect;
    unsafe fn setFrameOrigin_(self, point: NSPoint);
    unsafe fn setFrameTopLeftPoint_(self, point: NSPoint);
    unsafe fn constrainFrameRect_toScreen_(self, frameRect: NSRect, screen: id);
    unsafe fn cascadeTopLeftFromPoint_(self, topLeft: NSPoint) -> NSPoint;
    unsafe fn setFrame_display_(self, windowFrame: NSRect, display: BOOL);
    unsafe fn setFrame_displayViews_(self, windowFrame: NSRect, display: BOOL);
    unsafe fn aspectRatio(self) -> NSSize;
    unsafe fn setAspectRatio_(self, aspectRatio: NSSize);
    unsafe fn minSize(self) -> NSSize;
    unsafe fn setMinSize_(self, minSize: NSSize);
    unsafe fn maxSize(self) -> NSSize;
    unsafe fn setMaxSize_(self, maxSize: NSSize);
    unsafe fn performZoom_(self, sender: id);
    unsafe fn zoom_(self, sender: id);
    unsafe fn resizeFlags(self) -> NSInteger;
    unsafe fn showsResizeIndicator(self) -> BOOL;
    unsafe fn setShowsResizeIndicator_(self, showsResizeIndicator: BOOL);
    unsafe fn resizeIncrements(self) -> NSSize;
    unsafe fn setResizeIncrements_(self, resizeIncrements: NSSize);
    unsafe fn preservesContentDuringLiveResize(self) -> BOOL;
    unsafe fn setPreservesContentDuringLiveResize_(self, preservesContentDuringLiveResize: BOOL);
    unsafe fn inLiveResize(self) -> BOOL;

    // Sizing Content
    unsafe fn contentAspectRatio(self) -> NSSize;
    unsafe fn setContentAspectRatio_(self, contentAspectRatio: NSSize);
    unsafe fn contentMinSize(self) -> NSSize;
    unsafe fn setContentMinSize_(self, contentMinSize: NSSize);
    unsafe fn contentSize(self) -> NSSize;
    unsafe fn setContentSize_(self, contentSize: NSSize);
    unsafe fn contentMaxSize(self) -> NSSize;
    unsafe fn setContentMaxSize_(self, contentMaxSize: NSSize);
    unsafe fn contentResizeIncrements(self) -> NSSize;
    unsafe fn setContentResizeIncrements_(self, contentResizeIncrements: NSSize);

    // Managing Window Visibility and Occlusion State
    unsafe fn isVisible(self) -> BOOL; // NOTE: Deprecated in 10.9
    unsafe fn occlusionState(self) -> NSWindowOcclusionState;

    // Managing Window Layers
    unsafe fn orderOut_(self, sender: id);
    unsafe fn orderBack_(self, sender: id);
    unsafe fn orderFront_(self, sender: id);
    unsafe fn orderFrontRegardless(self);
    unsafe fn orderFrontWindow_relativeTo_(self, orderingMode: NSWindowOrderingMode, otherWindowNumber: NSInteger);
    unsafe fn level(self) -> NSInteger;
    unsafe fn setLevel_(self, level: NSInteger);

    // Managing Key Status
    unsafe fn canBecomeKeyWindow(self) -> BOOL;
    unsafe fn makeKeyWindow(self);
    unsafe fn makeKeyAndOrderFront_(self, sender: id);
    // skipped: becomeKeyWindow (should not be invoked directly, according to Apple's documentation)
    // skipped: resignKeyWindow (should not be invoked directly, according to Apple's documentation)

    // Managing Main Status
    unsafe fn canBecomeMainWindow(self) -> BOOL;
    unsafe fn makeMainWindow(self);
    // skipped: becomeMainWindow (should not be invoked directly, according to Apple's documentation)
    // skipped: resignMainWindow (should not be invoked directly, according to Apple's documentation)

    // TODO: Managing Toolbars
    // TODO: Managing Attached Windows
    // TODO: Managing Window Buffers
    // TODO: Managing Default Buttons
    // TODO: Managing Field Editors
    // TODO: Managing the Window Menu
    // TODO: Managing Cursor Rectangles
    // TODO: Managing Title Bars
    // TODO: Managing Tooltips
    // TODO: Handling Events
    // TODO: Managing Responders
    // TODO: Managing the Key View Loop

    // Handling Keyboard Events
    unsafe fn keyDown_(self, event: id);

    // Handling Mouse Events
    unsafe fn acceptsMouseMovedEvents(self) -> BOOL;
    unsafe fn ignoresMouseEvents(self) -> BOOL;
    unsafe fn setIgnoresMouseEvents_(self, ignoreMouseEvents: BOOL);
    unsafe fn mouseLocationOutsideOfEventStream(self) -> NSPoint;
    unsafe fn setAcceptsMouseMovedEvents_(self, acceptMouseMovedEvents: BOOL);
    unsafe fn windowNumberAtPoint_belowWindowWithWindowNumber_(self,
                                                               point: NSPoint,
                                                               windowNumber: NSInteger) -> NSInteger;

    // TODO: Handling Window Restoration
    // TODO: Bracketing Drawing Operations
    // TODO: Drawing Windows
    // TODO: Window Animation
    // TODO: Updating Windows
    // TODO: Dragging Items

    // Converting Coordinates
    unsafe fn backingScaleFactor(self) -> CGFloat;
    unsafe fn backingAlignedRect_options_(self, rect: NSRect, options: NSAlignmentOptions) -> NSRect;
    unsafe fn convertRectFromBacking_(self, rect: NSRect) -> NSRect;
    unsafe fn convertRectToBacking_(self, rect: NSRect) -> NSRect;
    unsafe fn convertRectToScreen_(self, rect: NSRect) -> NSRect;
    unsafe fn convertRectFromScreen_(self, rect: NSRect) -> NSRect;

    // Accessing Edited Status
    unsafe fn setDocumentEdited_(self, documentEdited: BOOL);

    // Managing Titles
    unsafe fn title(self) -> id;
    unsafe fn setTitle_(self, title: id);
    unsafe fn setTitleWithRepresentedFilename_(self, filePath: id);
    unsafe fn representedFilename(self) -> id;
    unsafe fn setRepresentedFilename_(self, filePath: id);
    unsafe fn representedURL(self) -> id;
    unsafe fn setRepresentedURL_(self, representedURL: id);

    // Accessing Screen Information
    unsafe fn screen(self) -> id;
    unsafe fn deepestScreen(self) -> id;
    unsafe fn displaysWhenScreenProfileChanges(self) -> BOOL;
    unsafe fn setDisplaysWhenScreenProfileChanges_(self, displaysWhenScreenProfileChanges: BOOL);

    // Moving Windows
    unsafe fn setMovableByWindowBackground_(self, movableByWindowBackground: BOOL);
    unsafe fn setMovable_(self, movable: BOOL);
    unsafe fn center(self);

    // Closing Windows
    unsafe fn performClose_(self, sender: id);
    unsafe fn close(self);
    unsafe fn setReleasedWhenClosed_(self, releasedWhenClosed: BOOL);

    // Minimizing Windows
    unsafe fn performMiniaturize_(self, sender: id);
    unsafe fn miniaturize_(self, sender: id);
    unsafe fn deminiaturize_(self, sender: id);
    unsafe fn miniwindowImage(self) -> id;
    unsafe fn setMiniwindowImage_(self, miniwindowImage: id);
    unsafe fn miniwindowTitle(self) -> id;
    unsafe fn setMiniwindowTitle_(self, miniwindowTitle: id);

    // TODO: Getting the Dock Tile
    // TODO: Printing Windows
    // TODO: Providing Services
    // TODO: Working with Carbon
    // TODO: Triggering Constraint-Based Layout
    // TODO: Debugging Constraint-Based Layout
    // TODO: Constraint-Based Layouts
}

impl NSWindow for id {
    // Creating Windows

    unsafe fn initWithContentRect_styleMask_backing_defer_(self,
                                                           rect: NSRect,
                                                           style: NSUInteger,
                                                           backing: NSBackingStoreType,
                                                           defer: BOOL) -> id {
        msg_send()(self, selector("initWithContentRect:styleMask:backing:defer:"),
                   rect, style, backing as NSUInteger, defer as libc::c_int)
    }

    unsafe fn initWithContentRect_styleMask_backing_defer_screen_(self,
                                                                  rect: NSRect,
                                                                  style: NSUInteger,
                                                                  backing: NSBackingStoreType,
                                                                  defer: BOOL,
                                                                  screen: id) -> id {
        msg_send()(self, selector("initWithContentRect:styleMask:backing:defer:"),
                   rect, style, backing as NSUInteger, defer as libc::c_int, screen)
    }

    // Configuring Windows

    unsafe fn styleMask(self) -> NSUInteger {
        msg_send()(self, selector("styleMask"))
    }

    unsafe fn setStyleMask_(self, styleMask: NSUInteger) {
        msg_send()(self, selector("setStyleMask:"), styleMask)
    }

    unsafe fn toggleFullScreen_(self, sender: id) {
        msg_send()(self, selector("toggleFullScreen:"), sender)
    }

    unsafe fn worksWhenModal(self) -> BOOL {
        msg_send()(self, selector("worksWhenModal"))
    }

    unsafe fn alphaValue(self) -> CGFloat {
        msg_send_fpret()(self, selector("alphaValue"))
    }

    unsafe fn setAlphaValue_(self, windowAlpha: CGFloat) {
        msg_send()(self, selector("setAlphaValue:"), windowAlpha)
    }

    unsafe fn backgroundColor(self) -> id {
        msg_send()(self, selector("backgroundColor"))
    }

    unsafe fn setBackgroundColor_(self, color: id) {
        msg_send()(self, selector("setBackgroundColor:"), color)
    }

    unsafe fn colorSpace(self) -> id {
        msg_send()(self, selector("colorSpace"))
    }

    unsafe fn setColorSpace_(self, colorSpace: id) {
        msg_send()(self, selector("setColorSpace:"), colorSpace)
    }

    unsafe fn contentView(self) -> id {
        msg_send()(self, selector("contentView"))
    }

    unsafe fn setContentView_(self, view: id) {
        msg_send()(self, selector("setContentView:"), view)
    }

    unsafe fn canHide(self) -> BOOL {
        msg_send()(self, selector("canHide"))
    }

    unsafe fn setCanHide_(self, canHide: BOOL) {
        msg_send()(self, selector("setCanHide:"), canHide as libc::c_int)
    }

    unsafe fn hidesOnDeactivate(self) -> BOOL {
        msg_send()(self, selector("hidesOnDeactivate"))
    }

    unsafe fn setHidesOnDeactivate_(self, hideOnDeactivate: BOOL) {
        msg_send()(self, selector("setHidesOnDeactivate:"), hideOnDeactivate as libc::c_int)
    }

    unsafe fn collectionBehavior(self) -> NSWindowCollectionBehavior {
        msg_send()(self, selector("collectionBehavior"))
    }

    unsafe fn setCollectionBehavior_(self, collectionBehavior: NSWindowCollectionBehavior) {
        msg_send()(self, selector("setCollectionBehavior:"), collectionBehavior)
    }

    unsafe fn setOpaque_(self, opaque: BOOL) {
        msg_send()(self, selector("setOpaque:"), opaque as libc::c_int)
    }

    unsafe fn hasShadow(self) -> BOOL {
        msg_send()(self, selector("hasShadow"))
    }

    unsafe fn setHasShadow_(self, hasShadow: BOOL) {
        msg_send()(self, selector("setHasShadow:"), hasShadow as libc::c_int)
    }

    unsafe fn invalidateShadow(self) {
        msg_send()(self, selector("invalidateShadow"))
    }

    unsafe fn autorecalculatesContentBorderThicknessForEdge_(self, edge: NSRectEdge) -> BOOL {
        msg_send()(self, selector("autorecalculatesContentBorderThicknessForEdge:"), edge)
    }

    unsafe fn setAutorecalculatesContentBorderThickness_forEdge_(self,
                                                                 autorecalculateContentBorderThickness: BOOL,
                                                                 edge: NSRectEdge) -> BOOL {
        msg_send()(self, selector("setAutorecalculatesContentBorderThickness:forEdge:"),
                   autorecalculateContentBorderThickness as libc::c_int, edge)
    }

    unsafe fn contentBorderThicknessForEdge_(self, edge: NSRectEdge) -> CGFloat {
        msg_send_fpret()(self, selector("contentBorderThicknessForEdge:"), edge)
    }

    unsafe fn setContentBorderThickness_forEdge_(self, borderThickness: CGFloat, edge: NSRectEdge) {
        msg_send()(self, selector("setContentBorderThickness:forEdge:"), borderThickness, edge)
    }

    unsafe fn delegate(self) -> id {
        msg_send()(self, selector("delegate"))
    }

    unsafe fn setDelegate_(self, delegate: id) {
        msg_send()(self, selector("setDelegate:"), delegate)
    }

    unsafe fn preventsApplicationTerminationWhenModal(self) -> BOOL {
        msg_send()(self, selector("preventsApplicationTerminationWhenModal"))
    }

    unsafe fn setPreventsApplicationTerminationWhenModal_(self, flag: BOOL) {
        msg_send()(self, selector("setPreventsApplicationTerminationWhenModal:"), flag as libc::c_int)
    }

    // TODO: Accessing Window Information

    // Getting Layout Information

    unsafe fn contentRectForFrameRect_styleMask_(self, windowFrame: NSRect, windowStyle: NSUInteger) -> NSRect {
        msg_send_stret()(self, selector("contentRectForFrameRect:styleMask:"), windowFrame, windowStyle)
    }

    unsafe fn frameRectForContentRect_styleMask_(self, windowContentRect: NSRect, windowStyle: NSUInteger) -> NSRect {
        msg_send_stret()(self, selector("frameRectForContentRect:styleMask:"), windowContentRect, windowStyle)
    }

    unsafe fn minFrameWidthWithTitle_styleMask_(self, windowTitle: id, windowStyle: NSUInteger) -> CGFloat {
        msg_send_fpret()(self, selector("minFrameWidthWithTitle:styleMask:"), windowTitle, windowStyle)
    }

    unsafe fn contentRectForFrameRect_(self, windowFrame: NSRect) -> NSRect {
        msg_send_stret()(self, selector("contentRectForFrameRect:"), windowFrame)
    }

    unsafe fn frameRectForContentRect_(self, windowContent: NSRect) -> NSRect {
        msg_send_stret()(self, selector("frameRectForContentRect:"), windowContent)
    }

    // Managing Windows

    unsafe fn drawers(self) -> id {
        msg_send()(self, selector("drawers"))
    }

    unsafe fn windowController(self) -> id {
        msg_send()(self, selector("windowController"))
    }

    unsafe fn setWindowController_(self, windowController: id) {
        msg_send()(self, selector("setWindowController:"), windowController)
    }

    // TODO: Managing Sheets

    // Sizing Windows

    unsafe fn frame(self) -> NSRect {
        msg_send_stret()(self, selector("frame"))
    }

    unsafe fn setFrameOrigin_(self, point: NSPoint) {
        msg_send()(self, selector("setFrameOrigin:"), point)
    }

    unsafe fn setFrameTopLeftPoint_(self, point: NSPoint) {
        msg_send()(self, selector("setFrameTopLeftPoint:"), point)
    }

    unsafe fn constrainFrameRect_toScreen_(self, frameRect: NSRect, screen: id) {
        msg_send()(self, selector("constrainFrameRect:toScreen:"), frameRect, screen)
    }

    unsafe fn cascadeTopLeftFromPoint_(self, topLeft: NSPoint) -> NSPoint {
        msg_send()(self, selector("cascadeTopLeftFromPoint:"), topLeft)
    }

    unsafe fn setFrame_display_(self, windowFrame: NSRect, display: BOOL) {
        msg_send()(self, selector("setFrame:display:"), windowFrame, display as libc::c_int)
    }

    unsafe fn setFrame_displayViews_(self, windowFrame: NSRect, display: BOOL) {
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

    unsafe fn resizeFlags(self) -> NSInteger {
        msg_send()(self, selector("resizeFlags"))
    }

    unsafe fn showsResizeIndicator(self) -> BOOL {
        msg_send()(self, selector("showsResizeIndicator"))
    }

    unsafe fn setShowsResizeIndicator_(self, showsResizeIndicator: BOOL) {
        msg_send()(self, selector("setShowsResizeIndicator:"), showsResizeIndicator as libc::c_int)
    }

    unsafe fn resizeIncrements(self) -> NSSize {
        msg_send()(self, selector("resizeIncrements"))
    }

    unsafe fn setResizeIncrements_(self, resizeIncrements: NSSize) {
        msg_send()(self, selector("setResizeIncrements:"), resizeIncrements)
    }

    unsafe fn preservesContentDuringLiveResize(self) -> BOOL {
        msg_send()(self, selector("preservesContentDuringLiveResize"))
    }

    unsafe fn setPreservesContentDuringLiveResize_(self, preservesContentDuringLiveResize: BOOL) {
        msg_send()(self, selector("setPreservesContentDuringLiveResize:"), preservesContentDuringLiveResize as libc::c_int)
    }

    unsafe fn inLiveResize(self) -> BOOL {
        msg_send()(self, selector("inLiveResize"))
    }

    // Sizing Content

    unsafe fn contentAspectRatio(self) -> NSSize {
        msg_send_stret()(self, selector("contentAspectRatio"))
    }

    unsafe fn setContentAspectRatio_(self, contentAspectRatio: NSSize) {
        msg_send()(self, selector("setContentAspectRatio:"), contentAspectRatio)
    }

    unsafe fn contentMinSize(self) -> NSSize {
        msg_send_stret()(self, selector("contentMinSize"))
    }

    unsafe fn setContentMinSize_(self, contentMinSize: NSSize) {
        msg_send()(self, selector("setContentMinSize:"), contentMinSize)
    }

    unsafe fn contentSize(self) -> NSSize {
        msg_send_stret()(self, selector("contentSize"))
    }

    unsafe fn setContentSize_(self, contentSize: NSSize) {
        msg_send()(self, selector("setContentSize:"), contentSize)
    }

    unsafe fn contentMaxSize(self) -> NSSize {
        msg_send_stret()(self, selector("contentMaxSize"))
    }

    unsafe fn setContentMaxSize_(self, contentMaxSize: NSSize) {
        msg_send()(self, selector("setContentMaxSize:"), contentMaxSize)
    }

    unsafe fn contentResizeIncrements(self) -> NSSize {
        msg_send_stret()(self, selector("contentResizeIncrements"))
    }

    unsafe fn setContentResizeIncrements_(self, contentResizeIncrements: NSSize) {
        msg_send()(self, selector("setContentResizeIncrements:"), contentResizeIncrements)
    }

    // Managing Window Visibility and Occlusion State

    unsafe fn isVisible(self) -> BOOL {
        msg_send()(self, selector("isVisible"))
    }

    unsafe fn occlusionState(self) -> NSWindowOcclusionState {
        msg_send()(self, selector("occlusionState"))
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

    unsafe fn canBecomeKeyWindow(self) -> BOOL {
        msg_send()(self, selector("canBecomeKeyWindow"))
    }

    unsafe fn makeKeyWindow(self) {
        msg_send()(self, selector("makeKeyWindow"))
    }

    unsafe fn makeKeyAndOrderFront_(self, sender: id) {
        msg_send()(self, selector("makeKeyAndOrderFront:"), sender)
    }

    // Managing Main Status

    unsafe fn canBecomeMainWindow(self) -> BOOL {
        msg_send()(self, selector("canBecomeMainWindow"))
    }

    unsafe fn makeMainWindow(self) {
        msg_send()(self, selector("makeMainWindow"))
    }

    // TODO: Managing Toolbars
    // TODO: Managing Attached Windows
    // TODO: Managing Window Buffers
    // TODO: Managing Default Buttons
    // TODO: Managing Field Editors
    // TODO: Managing the Window Menu
    // TODO: Managing Cursor Rectangles
    // TODO: Managing Title Bars
    // TODO: Managing Tooltips
    // TODO: Handling Events
    // TODO: Managing Responders
    // TODO: Managing the Key View Loop

    // Handling Keyboard Events

    unsafe fn keyDown_(self, event: id) {
        msg_send()(self, selector("keyDown:"), event)
    }

    // Handling Mouse Events

    unsafe fn acceptsMouseMovedEvents(self) -> BOOL {
        msg_send()(self, selector("acceptsMouseMovedEvents"))
    }

    unsafe fn ignoresMouseEvents(self) -> BOOL {
        msg_send()(self, selector("ignoresMouseEvents"))
    }

    unsafe fn setIgnoresMouseEvents_(self, ignoreMouseEvents: BOOL) {
        msg_send()(self, selector("setIgnoresMouseEvents:"), ignoreMouseEvents as libc::c_int)
    }

    unsafe fn mouseLocationOutsideOfEventStream(self) -> NSPoint {
        msg_send()(self, selector("mouseLocationOutsideOfEventStream"))
    }

    unsafe fn setAcceptsMouseMovedEvents_(self, acceptMouseMovedEvents: BOOL) {
        msg_send()(self, selector("setAcceptsMouseMovedEvents:"), acceptMouseMovedEvents as libc::c_int)
    }

    unsafe fn windowNumberAtPoint_belowWindowWithWindowNumber_(self,
                                                               point: NSPoint,
                                                               windowNumber: NSInteger) -> NSInteger {
        msg_send()(self, selector("windowNumberAtPoint:belowWindowWithWindowNumber:"),
                   point, windowNumber)
    }

    // Converting Coordinates

    unsafe fn backingScaleFactor(self) -> CGFloat {
        msg_send_fpret()(self, selector("backingScaleFactor"))
    }

    unsafe fn backingAlignedRect_options_(self, rect: NSRect, options: NSAlignmentOptions) -> NSRect {
        msg_send_stret()(self, selector("backingAlignedRect:options:"), rect, options)
    }

    unsafe fn convertRectFromBacking_(self, rect: NSRect) -> NSRect {
        msg_send_stret()(self, selector("convertRectFromBacking:"), rect)
    }

    unsafe fn convertRectToBacking_(self, rect: NSRect) -> NSRect {
        msg_send_stret()(self, selector("convertRectToBacking:"), rect)
    }

    unsafe fn convertRectToScreen_(self, rect: NSRect) -> NSRect {
        msg_send_stret()(self, selector("convertRectToScreen:"), rect)
    }

    unsafe fn convertRectFromScreen_(self, rect: NSRect) -> NSRect {
        msg_send_stret()(self, selector("convertRectFromScreen:"), rect)
    }

    // Accessing Edited Status

    unsafe fn setDocumentEdited_(self, documentEdited: BOOL) {
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

    unsafe fn representedURL(self) -> id {
        msg_send()(self, selector("representedURL"))
    }

    unsafe fn setRepresentedURL_(self, representedURL: id) {
        msg_send()(self, selector("setRepresentedURL:"), representedURL)
    }

    // Accessing Screen Information

    unsafe fn screen(self) -> id {
        msg_send()(self, selector("screen"))
    }

    unsafe fn deepestScreen(self) -> id {
        msg_send()(self, selector("deepestScreen"))
    }

    unsafe fn displaysWhenScreenProfileChanges(self) -> BOOL {
        msg_send()(self, selector("displaysWhenScreenProfileChanges"))
    }

    unsafe fn setDisplaysWhenScreenProfileChanges_(self, displaysWhenScreenProfileChanges: BOOL) {
        msg_send()(self, selector("setDisplaysWhenScreenProfileChanges:"), displaysWhenScreenProfileChanges as libc::c_int)
    }

    // Moving Windows

    unsafe fn setMovableByWindowBackground_(self, movableByWindowBackground: BOOL) {
        msg_send()(self, selector("setMovableByWindowBackground:"), movableByWindowBackground as libc::c_int)
    }

    unsafe fn setMovable_(self, movable: BOOL) {
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

    unsafe fn setReleasedWhenClosed_(self, releasedWhenClosed: BOOL) {
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

    unsafe fn miniwindowImage(self) -> id {
        msg_send()(self, selector("miniwindowImage"))
    }

    unsafe fn setMiniwindowImage_(self, miniwindowImage: id) {
        msg_send()(self, selector("setMiniwindowImage:"), miniwindowImage)
    }

    unsafe fn miniwindowTitle(self) -> id {
        msg_send()(self, selector("miniwindowTitle"))
    }

    unsafe fn setMiniwindowTitle_(self, miniwindowTitle: id) {
        msg_send()(self, selector("setMiniwindowTitle:"), miniwindowTitle)
    }

    // TODO: Getting the Dock Tile
    // TODO: Printing Windows
    // TODO: Providing Services
    // TODO: Working with Carbon
    // TODO: Triggering Constraint-Based Layout
    // TODO: Debugging Constraint-Based Layout
    // TODO: Constraint-Based Layouts
}

pub trait NSString {
    unsafe fn alloc(_: Self) -> id {
        msg_send()(class("NSString"), selector("alloc"))
    }

    unsafe fn initWithUTF8String_(self, c_string: *const i8) -> id;
    unsafe fn stringByAppendingString_(self, other: id) -> id;
    unsafe fn init_str(self, string: &str) -> Self;
    unsafe fn UTF8String(self) -> *const libc::c_char;
}

impl NSString for id {
    unsafe fn initWithUTF8String_(self, c_string: *const i8) -> id {
        msg_send()(self, selector("initWithUTF8String:"), c_string as id)
    }

    unsafe fn stringByAppendingString_(self, other: id) -> id {
        msg_send()(self, selector("stringByAppendingString:"), other)
    }

    unsafe fn init_str(self, string: &str) -> id {
        let cstring = CString::new(string).unwrap();
        self.initWithUTF8String_(cstring.as_ptr())
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
    unsafe fn bounds(self) -> NSRect;
    unsafe fn frame(self) -> NSRect;
    unsafe fn display_(self);
    unsafe fn setWantsBestResolutionOpenGLSurface_(self, flag: BOOL);
    unsafe fn convertPoint_fromView_(self, point: NSPoint, view: id) -> NSPoint;
}

impl NSView for id {
    unsafe fn init(self) -> id {
        msg_send()(self, selector("init"))
    }

    unsafe fn initWithFrame_(self, frameRect: NSRect) -> id {
        msg_send()(self, selector("initWithFrame:"), frameRect)
    }

    unsafe fn bounds(self) -> NSRect {
        msg_send_stret()(self, selector("bounds"))
    }

    unsafe fn frame(self) -> NSRect {
        msg_send_stret()(self, selector("frame"))
    }

    unsafe fn display_(self) {
        msg_send()(self, selector("display"))
    }

    unsafe fn setWantsBestResolutionOpenGLSurface_(self, flag: BOOL) {
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
    unsafe fn setOpenGLContext_(self, context: id);
    unsafe fn setPixelFormat_(self, pixelformat: id);
}

impl NSOpenGLView for id {
    unsafe fn initWithFrame_pixelFormat_(self,  frameRect: NSRect, format: id) -> id {
        msg_send()(self, selector("initWithFrame:pixelFormat:"), frameRect, format)
    }

    unsafe fn display_(self) {
        msg_send()(self, selector("display"))
    }

    unsafe fn setOpenGLContext_(self, context: id) {
        msg_send()(self, selector("setOpenGLContext:"), context)
    }

    unsafe fn setPixelFormat_(self, pixelformat: id) {
        msg_send()(self, selector("setPixelFormat:"), pixelformat)
    }
}

pub trait NSOpenGLPixelFormat {
    unsafe fn alloc(_: Self) -> id {
        msg_send()(class("NSOpenGLPixelFormat"), selector("alloc"))
    }

    unsafe fn initWithAttributes_(self, attributes: &[u32]) -> id;
}

impl NSOpenGLPixelFormat for id {
    unsafe fn initWithAttributes_(self, attributes: &[u32]) -> id {
        msg_send()(self, selector("initWithAttributes:"), attributes)
    }
}

pub trait NSOpenGLContext {
    unsafe fn alloc(_: Self) -> id {
        msg_send()(class("NSOpenGLContext"), selector("alloc"))
    }

    // Context Creation
    unsafe fn initWithFormat_shareContext_(self, format: id /* (NSOpenGLPixelFormat *) */, shareContext: id /* (NSOpenGLContext *) */) -> id /* (instancetype) */;
    unsafe fn initWithCGLContextObj_(self, context: CGLContextObj) -> id /* (instancetype) */;

    // Managing the Current Context
    unsafe fn clearCurrentContext(_: Self);
    unsafe fn currentContext(_: Self) -> id /* (NSOpenGLContext *) */;
    unsafe fn makeCurrentContext(self);

    // Drawable Object Management
    unsafe fn setView_(self, view: id /* (NSView *) */);
    unsafe fn view(self) -> id /* (NSView *) */;
    unsafe fn clearDrawable(self);
    unsafe fn update(self);

    // Flushing the Drawing Buffer
    unsafe fn flushBuffer(self);

    // Context Parameter Handling
    unsafe fn setValues_forParameter_(self, vals: *const GLint, param: NSOpenGLContextParameter);
    unsafe fn getValues_forParameter_(self, vals: *mut GLint, param: NSOpenGLContextParameter);

    // Working with Virtual Screens
    unsafe fn setCurrentVirtualScreen_(self, screen: GLint);
    unsafe fn currentVirtualScreen(self) -> GLint;

    // Getting the CGL Context Object
    unsafe fn CGLContextObj(self) -> CGLContextObj;
}

impl NSOpenGLContext for id {
    // Context Creation

    unsafe fn initWithFormat_shareContext_(self, format: id /* (NSOpenGLPixelFormat *) */, shareContext: id /* (NSOpenGLContext *) */) -> id /* (instancetype) */ {
        msg_send()(self, selector("initWithFormat:shareContext:"), format, shareContext)
    }

    unsafe fn initWithCGLContextObj_(self, context: CGLContextObj) -> id /* (instancetype) */ {
        msg_send()(self, selector("initWithCGLContextObj:"), context)
    }

    // Managing the Current Context

    unsafe fn clearCurrentContext(_: Self) {
        msg_send()(class("NSOpenGLContext"), selector("clearCurrentContext"))
    }

    unsafe fn currentContext(_: Self) -> id /* (NSOpenGLContext *) */ {
        msg_send()(class("NSOpenGLContext"), selector("currentContext"))
    }

    unsafe fn makeCurrentContext(self) {
        msg_send()(self, selector("makeCurrentContext"))
    }

    // Drawable Object Management

    unsafe fn setView_(self, view: id /* (NSView *) */) {
        msg_send()(self, selector("setView:"), view)
    }

    unsafe fn view(self) -> id /* (NSView *) */ {
        msg_send()(self, selector("view"))
    }

    unsafe fn clearDrawable(self) {
        msg_send()(self, selector("clearDrawable"))
    }

    unsafe fn update(self) {
        msg_send()(self, selector("update"))
    }

    // Flushing the Drawing Buffer

    unsafe fn flushBuffer(self) {
        msg_send()(self, selector("flushBuffer"))
    }

    // Context Parameter Handling

    unsafe fn setValues_forParameter_(self, vals: *const GLint, param: NSOpenGLContextParameter) {
        msg_send()(self, selector("setValues:forParameter:"), vals, param)
    }

    unsafe fn getValues_forParameter_(self, vals: *mut GLint, param: NSOpenGLContextParameter) {
        msg_send()(self, selector("getValues:forParameter:"), vals, param)
    }

    // Working with Virtual Screens

    unsafe fn setCurrentVirtualScreen_(self, screen: GLint) {
        msg_send()(self, selector("setCurrentVirtualScreen:"), screen)
    }

    unsafe fn currentVirtualScreen(self) -> GLint {
        msg_send()(self, selector("currentVirtualScreen"))
    }

    // Getting the CGL Context Object

    unsafe fn CGLContextObj(self) -> CGLContextObj {
        msg_send()(self, selector("CGLContextObj"))
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

bitflags! {
    flags NSEventSwipeTrackingOptions: NSUInteger {
        const NSEventSwipeTrackingLockDirection         = 0x1 << 0,
        const NSEventSwipeTrackingClampGestureAmount    = 0x1 << 1,
    }
}

#[repr(i64)] // NSInteger
pub enum NSEventGestureAxis {
    NSEventGestureAxisNone = 0,
    NSEventGestureAxisHorizontal,
    NSEventGestureAxisVertical,
}

bitflags! {
    flags NSEventPhase: NSUInteger {
       const NSEventPhaseNone        = 0,
       const NSEventPhaseBegan       = 0x1 << 0,
       const NSEventPhaseStationary  = 0x1 << 1,
       const NSEventPhaseChanged     = 0x1 << 2,
       const NSEventPhaseEnded       = 0x1 << 3,
       const NSEventPhaseCancelled   = 0x1 << 4,
       const NSEventPhaseMayBegin    = 0x1 << 5,
    }
}

bitflags! {
    flags NSTouchPhase: NSUInteger {
        const NSTouchPhaseBegan         = 1 << 0,
        const NSTouchPhaseMoved         = 1 << 1,
        const NSTouchPhaseStationary    = 1 << 2,
        const NSTouchPhaseEnded         = 1 << 3,
        const NSTouchPhaseCancelled     = 1 << 4,
        const NSTouchPhaseTouching      = NSTouchPhaseBegan.bits
                                        | NSTouchPhaseMoved.bits
                                        | NSTouchPhaseStationary.bits,
        const NSTouchPhaseAny           = 0 - 1, // NSUIntegerMax
    }
}

#[repr(u64)] // NSUInteger
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

bitflags! {
    flags NSEventMask: libc::c_ulonglong {
        const NSLeftMouseDownMask         = 1 << NSLeftMouseDown as libc::c_ulonglong,
        const NSLeftMouseUpMask           = 1 << NSLeftMouseUp as libc::c_ulonglong,
        const NSRightMouseDownMask        = 1 << NSRightMouseDown as libc::c_ulonglong,
        const NSRightMouseUpMask          = 1 << NSRightMouseUp as libc::c_ulonglong,
        const NSMouseMovedMask            = 1 << NSMouseMoved as libc::c_ulonglong,
        const NSLeftMouseDraggedMask      = 1 << NSLeftMouseDragged as libc::c_ulonglong,
        const NSRightMouseDraggedMask     = 1 << NSRightMouseDragged as libc::c_ulonglong,
        const NSMouseEnteredMask          = 1 << NSMouseEntered as libc::c_ulonglong,
        const NSMouseExitedMask           = 1 << NSMouseExited as libc::c_ulonglong,
        const NSKeyDownMask               = 1 << NSKeyDown as libc::c_ulonglong,
        const NSKeyUpMask                 = 1 << NSKeyUp as libc::c_ulonglong,
        const NSFlagsChangedMask          = 1 << NSFlagsChanged as libc::c_ulonglong,
        const NSAppKitDefinedMask         = 1 << NSAppKitDefined as libc::c_ulonglong,
        const NSSystemDefinedMask         = 1 << NSSystemDefined as libc::c_ulonglong,
        const NSApplicationDefinedMask    = 1 << NSApplicationDefined as libc::c_ulonglong,
        const NSPeriodicMask              = 1 << NSPeriodic as libc::c_ulonglong,
        const NSCursorUpdateMask          = 1 << NSCursorUpdate as libc::c_ulonglong,
        const NSScrollWheelMask           = 1 << NSScrollWheel as libc::c_ulonglong,
        const NSTabletPointMask           = 1 << NSTabletPoint as libc::c_ulonglong,
        const NSTabletProximityMask       = 1 << NSTabletProximity as libc::c_ulonglong,
        const NSOtherMouseDownMask        = 1 << NSOtherMouseDown as libc::c_ulonglong,
        const NSOtherMouseUpMask          = 1 << NSOtherMouseUp as libc::c_ulonglong,
        const NSOtherMouseDraggedMask     = 1 << NSOtherMouseDragged as libc::c_ulonglong,
        const NSEventMaskGesture          = 1 << NSEventTypeGesture as libc::c_ulonglong,
        const NSEventMaskSwipe            = 1 << NSEventTypeSwipe as libc::c_ulonglong,
        const NSEventMaskRotate           = 1 << NSEventTypeRotate as libc::c_ulonglong,
        const NSEventMaskBeginGesture     = 1 << NSEventTypeBeginGesture as libc::c_ulonglong,
        const NSEventMaskEndGesture       = 1 << NSEventTypeEndGesture as libc::c_ulonglong,
        const NSAnyEventMask              = 0xffffffff,
    }
}

impl NSEventMask {
    pub fn from_type(ty: NSEventType) -> NSEventMask {
        NSEventMask { bits: 1 << ty as libc::c_ulonglong }
    }
}

bitflags! {
    flags NSEventModifierFlags: NSUInteger {
        const NSAlphaShiftKeyMask                     = 1 << 16,
        const NSShiftKeyMask                          = 1 << 17,
        const NSControlKeyMask                        = 1 << 18,
        const NSAlternateKeyMask                      = 1 << 19,
        const NSCommandKeyMask                        = 1 << 20,
        const NSNumericPadKeyMask                     = 1 << 21,
        const NSHelpKeyMask                           = 1 << 22,
        const NSFunctionKeyMask                       = 1 << 23,
        const NSDeviceIndependentModifierFlagsMask    = 0xffff0000,
    }
}

// Not sure of the type here
pub enum NSPointingDeviceType {
    // TODO: Not sure what these values are
    // NSUnknownPointingDevice = NX_TABLET_POINTER_UNKNOWN,
    // NSPenPointingDevice     = NX_TABLET_POINTER_PEN,
    // NSCursorPointingDevice  = NX_TABLET_POINTER_CURSOR,
    // NSEraserPointingDevice  = NX_TABLET_POINTER_ERASER,
}

// Not sure of the type here
pub enum NSEventButtonMask {
    // TODO: Not sure what these values are
    // NSPenTipMask =       NX_TABLET_BUTTON_PENTIPMASK,
    // NSPenLowerSideMask = NX_TABLET_BUTTON_PENLOWERSIDEMASK,
    // NSPenUpperSideMask = NX_TABLET_BUTTON_PENUPPERSIDEMASK,
}

#[repr(i16)]
pub enum NSEventSubtype {
    // TODO: Not sure what these values are
    // NSMouseEventSubtype           = NX_SUBTYPE_DEFAULT,
    // NSTabletPointEventSubtype     = NX_SUBTYPE_TABLET_POINT,
    // NSTabletProximityEventSubtype = NX_SUBTYPE_TABLET_PROXIMITY
    // NSTouchEventSubtype           = NX_SUBTYPE_MOUSE_TOUCH,
    NSWindowExposedEventType = 0,
    NSApplicationActivatedEventType = 1,
    NSApplicationDeactivatedEventType = 2,
    NSWindowMovedEventType = 4,
    NSScreenChangedEventType = 8,
    NSAWTEventType = 16,
}

pub const NSUpArrowFunctionKey: libc::c_ushort = 0xF700;
pub const NSDownArrowFunctionKey: libc::c_ushort = 0xF701;
pub const NSLeftArrowFunctionKey: libc::c_ushort = 0xF702;
pub const NSRightArrowFunctionKey: libc::c_ushort = 0xF703;
pub const NSF1FunctionKey: libc::c_ushort = 0xF704;
pub const NSF2FunctionKey: libc::c_ushort = 0xF705;
pub const NSF3FunctionKey: libc::c_ushort = 0xF706;
pub const NSF4FunctionKey: libc::c_ushort = 0xF707;
pub const NSF5FunctionKey: libc::c_ushort = 0xF708;
pub const NSF6FunctionKey: libc::c_ushort = 0xF709;
pub const NSF7FunctionKey: libc::c_ushort = 0xF70A;
pub const NSF8FunctionKey: libc::c_ushort = 0xF70B;
pub const NSF9FunctionKey: libc::c_ushort = 0xF70C;
pub const NSF10FunctionKey: libc::c_ushort = 0xF70D;
pub const NSF11FunctionKey: libc::c_ushort = 0xF70E;
pub const NSF12FunctionKey: libc::c_ushort = 0xF70F;
pub const NSF13FunctionKey: libc::c_ushort = 0xF710;
pub const NSF14FunctionKey: libc::c_ushort = 0xF711;
pub const NSF15FunctionKey: libc::c_ushort = 0xF712;
pub const NSF16FunctionKey: libc::c_ushort = 0xF713;
pub const NSF17FunctionKey: libc::c_ushort = 0xF714;
pub const NSF18FunctionKey: libc::c_ushort = 0xF715;
pub const NSF19FunctionKey: libc::c_ushort = 0xF716;
pub const NSF20FunctionKey: libc::c_ushort = 0xF717;
pub const NSF21FunctionKey: libc::c_ushort = 0xF718;
pub const NSF22FunctionKey: libc::c_ushort = 0xF719;
pub const NSF23FunctionKey: libc::c_ushort = 0xF71A;
pub const NSF24FunctionKey: libc::c_ushort = 0xF71B;
pub const NSF25FunctionKey: libc::c_ushort = 0xF71C;
pub const NSF26FunctionKey: libc::c_ushort = 0xF71D;
pub const NSF27FunctionKey: libc::c_ushort = 0xF71E;
pub const NSF28FunctionKey: libc::c_ushort = 0xF71F;
pub const NSF29FunctionKey: libc::c_ushort = 0xF720;
pub const NSF30FunctionKey: libc::c_ushort = 0xF721;
pub const NSF31FunctionKey: libc::c_ushort = 0xF722;
pub const NSF32FunctionKey: libc::c_ushort = 0xF723;
pub const NSF33FunctionKey: libc::c_ushort = 0xF724;
pub const NSF34FunctionKey: libc::c_ushort = 0xF725;
pub const NSF35FunctionKey: libc::c_ushort = 0xF726;
pub const NSInsertFunctionKey: libc::c_ushort = 0xF727;
pub const NSDeleteFunctionKey: libc::c_ushort = 0xF728;
pub const NSHomeFunctionKey: libc::c_ushort = 0xF729;
pub const NSBeginFunctionKey: libc::c_ushort = 0xF72A;
pub const NSEndFunctionKey: libc::c_ushort = 0xF72B;
pub const NSPageUpFunctionKey: libc::c_ushort = 0xF72C;
pub const NSPageDownFunctionKey: libc::c_ushort = 0xF72D;
pub const NSPrintScreenFunctionKey: libc::c_ushort = 0xF72E;
pub const NSScrollLockFunctionKey: libc::c_ushort = 0xF72F;
pub const NSPauseFunctionKey: libc::c_ushort = 0xF730;
pub const NSSysReqFunctionKey: libc::c_ushort = 0xF731;
pub const NSBreakFunctionKey: libc::c_ushort = 0xF732;
pub const NSResetFunctionKey: libc::c_ushort = 0xF733;
pub const NSStopFunctionKey: libc::c_ushort = 0xF734;
pub const NSMenuFunctionKey: libc::c_ushort = 0xF735;
pub const NSUserFunctionKey: libc::c_ushort = 0xF736;
pub const NSSystemFunctionKey: libc::c_ushort = 0xF737;
pub const NSPrintFunctionKey: libc::c_ushort = 0xF738;
pub const NSClearLineFunctionKey: libc::c_ushort = 0xF739;
pub const NSClearDisplayFunctionKey: libc::c_ushort = 0xF73A;
pub const NSInsertLineFunctionKey: libc::c_ushort = 0xF73B;
pub const NSDeleteLineFunctionKey: libc::c_ushort = 0xF73C;
pub const NSInsertCharFunctionKey: libc::c_ushort = 0xF73D;
pub const NSDeleteCharFunctionKey: libc::c_ushort = 0xF73E;
pub const NSPrevFunctionKey: libc::c_ushort = 0xF73F;
pub const NSNextFunctionKey: libc::c_ushort = 0xF740;
pub const NSSelectFunctionKey: libc::c_ushort = 0xF741;
pub const NSExecuteFunctionKey: libc::c_ushort = 0xF742;
pub const NSUndoFunctionKey: libc::c_ushort = 0xF743;
pub const NSRedoFunctionKey: libc::c_ushort = 0xF744;
pub const NSFindFunctionKey: libc::c_ushort = 0xF745;
pub const NSHelpFunctionKey: libc::c_ushort = 0xF746;
pub const NSModeSwitchFunctionKey: libc::c_ushort = 0xF747;

pub trait NSEvent {
    // Creating Events
    unsafe fn keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        characters: id /* (NSString *) */,
        unmodCharacters: id /* (NSString *) */,
        repeatKey: BOOL,
        code: libc::c_ushort) -> id /* (NSEvent *) */;
    unsafe fn mouseEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_clickCount_pressure_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: id /* (NSGraphicsContext *) */,
        eventNumber: NSInteger,
        clickCount: NSInteger,
        pressure: libc::c_float) -> id /* (NSEvent *) */;
    unsafe fn enterExitEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_trackingNumber_userData_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: id /* (NSGraphicsContext *) */,
        eventNumber: NSInteger,
        trackingNumber: NSInteger,
        userData: *mut libc::c_void) -> id /* (NSEvent *) */;
    unsafe fn otherEventWithType_location_modifierFlags_timestamp_windowNumber_context_subtype_data1_data2_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: id /* (NSGraphicsContext *) */,
        subtype: NSEventSubtype,
        data1: NSInteger,
        data2: NSInteger) -> id /* (NSEvent *) */;
    unsafe fn eventWithEventRef_(_: Self, eventRef: *const libc::c_void) -> id;
    unsafe fn eventWithCGEvent_(_: Self, cgEvent: *mut libc::c_void /* CGEventRef */) -> id;

    // Getting General Event Information
    unsafe fn context(self) -> id /* (NSGraphicsContext *) */;
    unsafe fn locationInWindow(self) -> NSPoint;
    unsafe fn modifierFlags(self) -> NSEventModifierFlags;
    unsafe fn timestamp(self) -> NSTimeInterval;
    // NOTE: renamed from `- type` due to Rust keyword collision
    unsafe fn eventType(self) -> NSEventType;
    unsafe fn window(self) -> id /* (NSWindow *) */;
    unsafe fn windowNumber(self) -> NSInteger;
    unsafe fn eventRef(self) -> *const libc::c_void;
    unsafe fn CGEvent(self) -> *mut libc::c_void /* CGEventRef */;

    // Getting Key Event Information
    // NOTE: renamed from `+ modifierFlags` due to conflict with `- modifierFlags`
    unsafe fn currentModifierFlags(_: Self) -> NSEventModifierFlags;
    unsafe fn keyRepeatDelay(_: Self) -> NSTimeInterval;
    unsafe fn keyRepeatInterval(_: Self) -> NSTimeInterval;
    unsafe fn characters(self) -> id /* (NSString *) */;
    unsafe fn charactersIgnoringModifiers(self) -> id /* (NSString *) */;
    unsafe fn keyCode(self) -> libc::c_ushort;

    // Getting Mouse Event Information
    unsafe fn pressedMouseButtons(_: Self) -> NSUInteger;
    unsafe fn doubleClickInterval(_: Self) -> NSTimeInterval;
    unsafe fn mouseLocation(_: Self) -> NSPoint;
    unsafe fn buttonNumber(self) -> NSInteger;
    unsafe fn clickCount(self) -> NSInteger;
    unsafe fn pressure(self) -> libc::c_float;
    unsafe fn setMouseCoalescingEnabled_(_: Self, flag: BOOL);
    unsafe fn isMouseCoalescingEnabled(_: Self) -> BOOL;

    // Getting Mouse-Tracking Event Information
    unsafe fn eventNumber(self) -> NSInteger;
    unsafe fn trackingNumber(self) -> NSInteger;
    unsafe fn trackingArea(self) -> id /* (NSTrackingArea *) */;
    unsafe fn userData(self) -> *const libc::c_void;

    // Getting Custom Event Information
    unsafe fn data1(self) -> NSInteger;
    unsafe fn data2(self) -> NSInteger;
    unsafe fn subtype(self) -> NSEventSubtype;

    // Getting Scroll Wheel Event Information
    unsafe fn deltaX(self) -> CGFloat;
    unsafe fn deltaY(self) -> CGFloat;
    unsafe fn deltaZ(self) -> CGFloat;

    // Getting Tablet Proximity Information
    unsafe fn capabilityMask(self) -> NSUInteger;
    unsafe fn deviceID(self) -> NSUInteger;
    unsafe fn pointingDeviceID(self) -> NSUInteger;
    unsafe fn pointingDeviceSerialNumber(self) -> NSUInteger;
    unsafe fn pointingDeviceType(self) -> NSPointingDeviceType;
    unsafe fn systemTabletID(self) -> NSUInteger;
    unsafe fn tabletID(self) -> NSUInteger;
    unsafe fn uniqueID(self) -> libc::c_ulonglong;
    unsafe fn vendorID(self) -> NSUInteger;
    unsafe fn vendorPointingDeviceType(self) -> NSUInteger;

    // Getting Tablet Pointing Information
    unsafe fn absoluteX(self) -> NSInteger;
    unsafe fn absoluteY(self) -> NSInteger;
    unsafe fn absoluteZ(self) -> NSInteger;
    unsafe fn buttonMask(self) -> NSEventButtonMask;
    unsafe fn rotation(self) -> libc::c_float;
    unsafe fn tangentialPressure(self) -> libc::c_float;
    unsafe fn tilt(self) -> NSPoint;
    unsafe fn vendorDefined(self) -> id;

    // Requesting and Stopping Periodic Events
    unsafe fn startPeriodicEventsAfterDelay_withPeriod_(_: Self, delaySeconds: NSTimeInterval, periodSeconds: NSTimeInterval);
    unsafe fn stopPeriodicEvents(_: Self);

    // Getting Touch and Gesture Information
    unsafe fn magnification(self) -> CGFloat;
    unsafe fn touchesMatchingPhase_inView_(self, phase: NSTouchPhase, view: id /* (NSView *) */) -> id /* (NSSet *) */;
    unsafe fn isSwipeTrackingFromScrollEventsEnabled(_: Self) -> BOOL;

    // Monitoring Application Events
    // TODO: addGlobalMonitorForEventsMatchingMask_handler_ (unsure how to bind to blocks)
    // TODO: addLocalMonitorForEventsMatchingMask_handler_ (unsure how to bind to blocks)
    unsafe fn removeMonitor_(_: Self, eventMonitor: id);

    // Scroll Wheel and Flick Events
    unsafe fn hasPreciseScrollingDeltas(self) -> BOOL;
    unsafe fn scrollingDeltaX(self) -> CGFloat;
    unsafe fn scrollingDeltaY(self) -> CGFloat;
    unsafe fn momentumPhase(self) -> NSEventPhase;
    unsafe fn phase(self) -> NSEventPhase;
    // TODO: trackSwipeEventWithOptions_dampenAmountThresholdMin_max_usingHandler_ (unsure how to bind to blocks)

    // Converting a Mouse Event’s Position into a Sprite Kit Node’s Coordinate Space
    unsafe fn locationInNode_(self, node: id /* (SKNode *) */) -> CGPoint;
}

impl NSEvent for id {
    // Creating Events

    unsafe fn keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        characters: id /* (NSString *) */,
        unmodCharacters: id /* (NSString *) */,
        repeatKey: BOOL,
        code: libc::c_ushort) -> id /* (NSEvent *) */
    {
        msg_send()(class("NSEvent"),
                   selector("keyEventWithType:location:modifierFlags:timestamp:windowNumber:\
                             context:characters:charactersIgnoringModifiers:isARepeat:keyCode:"),
                   eventType,
                   location,
                   modifierFlags,
                   timestamp,
                   windowNumber,
                   characters,
                   unmodCharacters,
                   repeatKey as libc::c_int,
                   code as libc::c_int)
    }

    unsafe fn mouseEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_clickCount_pressure_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: id /* (NSGraphicsContext *) */,
        eventNumber: NSInteger,
        clickCount: NSInteger,
        pressure: libc::c_float) -> id /* (NSEvent *) */
    {
        msg_send()(class("NSEvent"),
                   selector("mouseEventWithType:location:modifierFlags:timestamp:windowNumber:\
                             context:eventNumber:clickCount:pressure:"),
                   eventType,
                   location,
                   modifierFlags,
                   timestamp,
                   windowNumber,
                   context,
                   eventNumber,
                   clickCount,
                   pressure as libc::c_double)
    }

    unsafe fn enterExitEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_trackingNumber_userData_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: id /* (NSGraphicsContext *) */,
        eventNumber: NSInteger,
        trackingNumber: NSInteger,
        userData: *mut libc::c_void) -> id /* (NSEvent *) */
    {
        msg_send()(class("NSEvent"),
                   selector("enterExitEventWithType:location:modifierFlags:timestamp:windowNumber:\
                             context:eventNumber:trackingNumber:userData:"),
                   eventType,
                   location,
                   modifierFlags,
                   timestamp,
                   windowNumber,
                   context,
                   eventNumber,
                   trackingNumber,
                   userData)
    }

    unsafe fn otherEventWithType_location_modifierFlags_timestamp_windowNumber_context_subtype_data1_data2_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: id /* (NSGraphicsContext *) */,
        subtype: NSEventSubtype,
        data1: NSInteger,
        data2: NSInteger) -> id /* (NSEvent *) */
    {
        msg_send()(class("NSEvent"),
                   selector("otherEventWithType:location:modifierFlags:timestamp:windowNumber:\
                             context:subtype:data1:data2:"),
                   eventType,
                   location,
                   modifierFlags,
                   modifierFlags,
                   timestamp,
                   windowNumber,
                   context,
                   subtype,
                   data1,
                   data2)
    }

    unsafe fn eventWithEventRef_(_: Self, eventRef: *const libc::c_void) -> id {
        msg_send()(class("NSEvent"), selector("eventWithEventRef:"), eventRef)
    }

    unsafe fn eventWithCGEvent_(_: Self, cgEvent: *mut libc::c_void /* CGEventRef */) -> id {
        msg_send()(class("NSEvent"), selector("eventWithCGEvent:"), cgEvent)
    }

    // Getting General Event Information

    unsafe fn context(self) -> id /* (NSGraphicsContext *) */ {
        msg_send()(self, selector("context"))
    }

    unsafe fn locationInWindow(self) -> NSPoint {
        msg_send()(self, selector("locationInWindow"))
    }

    unsafe fn modifierFlags(self) -> NSEventModifierFlags {
        msg_send()(self, selector("modifierFlags"))
    }

    unsafe fn timestamp(self) -> NSTimeInterval {
        msg_send()(self, selector("timestamp"))
    }
    // NOTE: renamed from `- type` due to Rust keyword collision

    unsafe fn eventType(self) -> NSEventType {
        msg_send()(self, selector("type"))
    }

    unsafe fn window(self) -> id /* (NSWindow *) */ {
        msg_send()(self, selector("window"))
    }

    unsafe fn windowNumber(self) -> NSInteger {
        msg_send()(self, selector("windowNumber"))
    }

    unsafe fn eventRef(self) -> *const libc::c_void {
        msg_send()(self, selector("eventRef"))
    }

    unsafe fn CGEvent(self) -> *mut libc::c_void /* CGEventRef */ {
        msg_send()(self, selector("CGEvent"))
    }

    // Getting Key Event Information

    // NOTE: renamed from `+ modifierFlags` due to conflict with `- modifierFlags`

    unsafe fn currentModifierFlags(_: Self) -> NSEventModifierFlags {
        msg_send()(class("NSEvent"), selector("currentModifierFlags"))
    }

    unsafe fn keyRepeatDelay(_: Self) -> NSTimeInterval {
        msg_send()(class("NSEvent"), selector("keyRepeatDelay"))
    }

    unsafe fn keyRepeatInterval(_: Self) -> NSTimeInterval {
        msg_send()(class("NSEvent"), selector("keyRepeatInterval"))
    }

    unsafe fn characters(self) -> id /* (NSString *) */ {
        msg_send()(self, selector("characters"))
    }

    unsafe fn charactersIgnoringModifiers(self) -> id /* (NSString *) */ {
        msg_send()(self, selector("charactersIgnoringModifiers"))
    }

    unsafe fn keyCode(self) -> libc::c_ushort {
        msg_send()(self, selector("keyCode"))
    }

    // Getting Mouse Event Information

    unsafe fn pressedMouseButtons(_: Self) -> NSUInteger {
        msg_send()(class("NSEvent"), selector("pressedMouseButtons"))
    }

    unsafe fn doubleClickInterval(_: Self) -> NSTimeInterval {
        msg_send()(class("NSEvent"), selector("doubleClickInterval"))
    }

    unsafe fn mouseLocation(_: Self) -> NSPoint {
        msg_send()(class("NSEvent"), selector("mouseLocation"))
    }

    unsafe fn buttonNumber(self) -> NSInteger {
        msg_send()(self, selector("buttonNumber"))
    }

    unsafe fn clickCount(self) -> NSInteger {
        msg_send()(self, selector("clickCount"))
    }

    unsafe fn pressure(self) -> libc::c_float {
        msg_send()(self, selector("pressure"))
    }

    unsafe fn setMouseCoalescingEnabled_(_: Self, flag: BOOL) {
        msg_send()(class("NSEvent"), selector("setMouseCoalescingEnabled:"), flag as libc::c_int)
    }

    unsafe fn isMouseCoalescingEnabled(_: Self) -> BOOL {
        msg_send()(class("NSEvent"), selector("isMouseCoalescingEnabled"))
    }

    // Getting Mouse-Tracking Event Information

    unsafe fn eventNumber(self) -> NSInteger {
        msg_send()(self, selector("eventNumber"))
    }

    unsafe fn trackingNumber(self) -> NSInteger {
        msg_send()(self, selector("trackingNumber"))
    }

    unsafe fn trackingArea(self) -> id /* (NSTrackingArea *) */ {
        msg_send()(self, selector("trackingArea"))
    }

    unsafe fn userData(self) -> *const libc::c_void {
        msg_send()(self, selector("userData"))
    }

    // Getting Custom Event Information

    unsafe fn data1(self) -> NSInteger {
        msg_send()(self, selector("data1"))
    }

    unsafe fn data2(self) -> NSInteger {
        msg_send()(self, selector("data2"))
    }

    unsafe fn subtype(self) -> NSEventSubtype {
        msg_send()(self, selector("subtype"))
    }

    // Getting Scroll Wheel Event Information

    unsafe fn deltaX(self) -> CGFloat {
        msg_send_fpret()(self, selector("deltaX"))
    }

    unsafe fn deltaY(self) -> CGFloat {
        msg_send_fpret()(self, selector("deltaY"))
    }

    unsafe fn deltaZ(self) -> CGFloat {
        msg_send_fpret()(self, selector("deltaZ"))
    }

    // Getting Tablet Proximity Information

    unsafe fn capabilityMask(self) -> NSUInteger {
        msg_send()(self, selector("capabilityMask"))
    }

    unsafe fn deviceID(self) -> NSUInteger {
        msg_send()(self, selector("deviceID"))
    }

    unsafe fn pointingDeviceID(self) -> NSUInteger {
        msg_send()(self, selector("pointingDeviceID"))
    }

    unsafe fn pointingDeviceSerialNumber(self) -> NSUInteger {
        msg_send()(self, selector("pointingDeviceSerialNumber"))
    }

    unsafe fn pointingDeviceType(self) -> NSPointingDeviceType {
        msg_send()(self, selector("pointingDeviceType"))
    }

    unsafe fn systemTabletID(self) -> NSUInteger {
        msg_send()(self, selector("systemTabletID"))
    }

    unsafe fn tabletID(self) -> NSUInteger {
        msg_send()(self, selector("tabletID"))
    }

    unsafe fn uniqueID(self) -> libc::c_ulonglong {
        msg_send()(self, selector("uniqueID"))
    }

    unsafe fn vendorID(self) -> NSUInteger {
        msg_send()(self, selector("vendorID"))
    }

    unsafe fn vendorPointingDeviceType(self) -> NSUInteger {
        msg_send()(self, selector("vendorPointingDeviceType"))
    }

    // Getting Tablet Pointing Information

    unsafe fn absoluteX(self) -> NSInteger {
        msg_send()(self, selector("absoluteX"))
    }

    unsafe fn absoluteY(self) -> NSInteger {
        msg_send()(self, selector("absoluteY"))
    }

    unsafe fn absoluteZ(self) -> NSInteger {
        msg_send()(self, selector("absoluteZ"))
    }

    unsafe fn buttonMask(self) -> NSEventButtonMask {
        msg_send()(self, selector("buttonMask"))
    }

    unsafe fn rotation(self) -> libc::c_float {
        msg_send()(self, selector("rotation"))
    }

    unsafe fn tangentialPressure(self) -> libc::c_float {
        msg_send()(self, selector("tangentialPressure"))
    }

    unsafe fn tilt(self) -> NSPoint {
        msg_send()(self, selector("tilt"))
    }

    unsafe fn vendorDefined(self) -> id {
        msg_send()(self, selector("vendorDefined"))
    }

    // Requesting and Stopping Periodic Events

    unsafe fn startPeriodicEventsAfterDelay_withPeriod_(_: Self, delaySeconds: NSTimeInterval, periodSeconds: NSTimeInterval) {
        msg_send()(class("NSEvent"), selector("startPeriodicEventsAfterDelay:withPeriod:"), delaySeconds, periodSeconds)
    }

    unsafe fn stopPeriodicEvents(_: Self) {
        msg_send()(class("NSEvent"), selector("stopPeriodicEvents"))
    }

    // Getting Touch and Gesture Information

    unsafe fn magnification(self) -> CGFloat {
        msg_send_fpret()(self, selector("magnification"))
    }

    unsafe fn touchesMatchingPhase_inView_(self, phase: NSTouchPhase, view: id /* (NSView *) */) -> id /* (NSSet *) */ {
        msg_send()(self, selector("touchesMatchingPhase:inView:"), phase, view)
    }

    unsafe fn isSwipeTrackingFromScrollEventsEnabled(_: Self) -> BOOL {
        msg_send()(class("NSEvent"), selector("isSwipeTrackingFromScrollEventsEnabled"))
    }

    // Monitoring Application Events

    // TODO: addGlobalMonitorForEventsMatchingMask_handler_ (unsure how to bind to blocks)
    // TODO: addLocalMonitorForEventsMatchingMask_handler_ (unsure how to bind to blocks)

    unsafe fn removeMonitor_(_: Self, eventMonitor: id) {
        msg_send()(class("NSEvent"), selector("removeMonitor:"), eventMonitor)
    }

    // Scroll Wheel and Flick Events

    unsafe fn hasPreciseScrollingDeltas(self) -> BOOL {
        msg_send()(self, selector("hasPreciseScrollingDeltas"))
    }

    unsafe fn scrollingDeltaX(self) -> CGFloat {
        msg_send_fpret()(self, selector("scrollingDeltaX"))
    }

    unsafe fn scrollingDeltaY(self) -> CGFloat {
        msg_send_fpret()(self, selector("scrollingDeltaY"))
    }

    unsafe fn momentumPhase(self) -> NSEventPhase {
        msg_send()(self, selector("momentumPhase"))
    }

    unsafe fn phase(self) -> NSEventPhase {
        msg_send()(self, selector("phase"))
    }

    // TODO: trackSwipeEventWithOptions_dampenAmountThresholdMin_max_usingHandler_ (unsure how to bind to blocks)

    // Converting a Mouse Event’s Position into a Sprite Kit Node’s Coordinate Space
    unsafe fn locationInNode_(self, node: id /* (SKNode *) */) -> CGPoint {
        msg_send_stret()(self, selector("locationInNode:"), node)
    }
}

pub trait NSScreen {
    // Getting NSScreen Objects
    unsafe fn mainScreen(_: Self) -> id /* (NSScreen *) */;
    unsafe fn deepestScreen(_: Self) -> id /* (NSScreen *) */;
    unsafe fn screens(_: Self) -> id /* (NSArray *) */;

    // Getting Screen Information
    unsafe fn depth(self) -> NSWindowDepth;
    unsafe fn frame(self) -> NSRect;
    unsafe fn supportedWindowDepths(self) -> *const NSWindowDepth;
    unsafe fn deviceDescription(self) -> id /* (NSDictionary *) */;
    unsafe fn visibleFrame(self) -> NSRect;
    unsafe fn colorSpace(self) -> id /* (NSColorSpace *) */;
    unsafe fn screensHaveSeparateSpaces(_: Self) -> BOOL;

    // Screen Backing Coordinate Conversion
    unsafe fn backingAlignedRect_options_(self, aRect: NSRect, options: NSAlignmentOptions) -> NSRect;
    unsafe fn backingScaleFactor(self) -> CGFloat;
    unsafe fn convertRectFromBacking_(self, aRect: NSRect) -> NSRect;
    unsafe fn convertRectToBacking_(self, aRect: NSRect) -> NSRect;
}

impl NSScreen for id {
    // Getting NSScreen Objects

    unsafe fn mainScreen(_: Self) -> id /* (NSScreen *) */ {
        msg_send()(class("NSScreen"), selector("mainScreen"))
    }

    unsafe fn deepestScreen(_: Self) -> id /* (NSScreen *) */ {
        msg_send()(class("NSScreen"), selector("deepestScreen"))
    }

    unsafe fn screens(_: Self) -> id /* (NSArray *) */ {
        msg_send()(class("NSScreen"), selector("screens"))
    }

    // Getting Screen Information

    unsafe fn depth(self) -> NSWindowDepth {
        msg_send()(self, selector("depth"))
    }

    unsafe fn frame(self) -> NSRect {
        msg_send_stret()(self, selector("frame"))
    }

    unsafe fn supportedWindowDepths(self) -> *const NSWindowDepth {
        msg_send()(self, selector("supportedWindowDepths"))
    }

    unsafe fn deviceDescription(self) -> id /* (NSDictionary *) */ {
        msg_send()(self, selector("deviceDescription"))
    }

    unsafe fn visibleFrame(self) -> NSRect {
        msg_send_stret()(self, selector("visibleFrame"))
    }

    unsafe fn colorSpace(self) -> id /* (NSColorSpace *) */ {
        msg_send()(self, selector("colorSpace"))
    }

    unsafe fn screensHaveSeparateSpaces(_: Self) -> BOOL {
        msg_send()(class("NSScreen"), selector("screensHaveSeparateSpaces"))
    }

    // Screen Backing Coordinate Conversion

    unsafe fn backingAlignedRect_options_(self, aRect: NSRect, options: NSAlignmentOptions) -> NSRect {
        msg_send_stret()(self, selector("backingAlignedRect:options:"), aRect, options)
    }

    unsafe fn backingScaleFactor(self) -> CGFloat {
        msg_send_fpret()(self, selector("backingScaleFactor"))
    }

    unsafe fn convertRectFromBacking_(self, aRect: NSRect) -> NSRect {
        msg_send_stret()(self, selector("convertRectFromBacking:"), aRect)
    }

    unsafe fn convertRectToBacking_(self, aRect: NSRect) -> NSRect {
        msg_send_stret()(self, selector("convertRectToBacking:"), aRect)
    }
}
