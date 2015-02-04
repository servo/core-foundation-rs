// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base::{CFAllocatorRef, CFIndex, CFOptionFlags, CFRelease, CFRetain};
use base::{CFTypeID, CFTypeRef, TCFType};
use base::{kCFAllocatorDefault};
use base::{Boolean};
use string::{CFString, CFStringRef};
use date::{CFAbsoluteTime, CFTimeInterval, CFAbsoluteTimeGetCurrent};
use libc::c_void;
use std::mem;
use std::ptr;

#[repr(C)]
struct __CFRunLoop;

pub type CFRunLoopRef = *const __CFRunLoop;

/// FIXME(pcwalton): Should be a newtype struct, but that fails due to a Rust compiler bug.
pub struct CFRunLoop {
    obj: CFRunLoopRef,
}

impl Drop for CFRunLoop {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl TCFType<CFRunLoopRef> for CFRunLoop {
    #[inline]
    fn as_concrete_TypeRef(&self) -> CFRunLoopRef {
        self.obj
    }

    #[inline]
    unsafe fn wrap_under_get_rule(reference: CFRunLoopRef) -> CFRunLoop {
        let reference: CFRunLoopRef = mem::transmute(CFRetain(mem::transmute(reference)));
        TCFType::wrap_under_create_rule(reference)
    }

    #[inline]
    fn as_CFTypeRef(&self) -> CFTypeRef {
        unsafe {
            mem::transmute(self.as_concrete_TypeRef())
        }
    }

    #[inline]
    unsafe fn wrap_under_create_rule(obj: CFRunLoopRef) -> CFRunLoop {
        CFRunLoop {
            obj: obj,
        }
    }

    #[inline]
    fn type_id(_: Option<CFRunLoop>) -> CFTypeID {
        unsafe {
            CFRunLoopGetTypeID()
        }
    }
}

impl CFRunLoop {
    pub fn get_current() -> CFRunLoop {
        unsafe {
            let run_loop_ref = CFRunLoopGetCurrent();
            TCFType::wrap_under_get_rule(run_loop_ref)
        }
    }

    pub fn get_main() -> CFRunLoop {
        unsafe {
            let run_loop_ref = CFRunLoopGetMain();
            TCFType::wrap_under_get_rule(run_loop_ref)
        }
    }

    pub fn run_current() {
        unsafe {
            CFRunLoopRun();
        }
    }

    pub fn stop(&self) {
        unsafe {
            CFRunLoopStop(self.obj);
        }
    }

    pub fn current_mode(&self) -> Option<String> {
        unsafe {
            let string_ref = CFRunLoopCopyCurrentMode(self.obj);
            if string_ref.is_null() {
                return None;
            }

            let cf_string: CFString = TCFType::wrap_under_create_rule(string_ref);
            Some(cf_string.to_string())
        }
    }

    pub fn contains_timer(&self, timer: &CFRunLoopTimer, mode: CFStringRef) -> bool {
        unsafe {
            CFRunLoopContainsTimer(self.obj, timer.obj, mode) != 0
        }
    }

    pub fn add_timer(&self, timer: &CFRunLoopTimer, mode: CFStringRef) {
        unsafe {
            CFRunLoopAddTimer(self.obj, timer.obj, mode);
        }
    }
     
}

//typedef struct __CFRunLoopSource * CFRunLoopSourceRef;
//
//typedef struct __CFRunLoopObserver * CFRunLoopObserverRef;
//
//typedef struct CF_BRIDGED_MUTABLE_TYPE(NSTimer) __CFRunLoopTimer * CFRunLoopTimerRef;
//
///* Reasons for CFRunLoopRunInMode() to Return */
//enum {
//    kCFRunLoopRunFinished = 1,
//    kCFRunLoopRunStopped = 2,
//    kCFRunLoopRunTimedOut = 3,
//    kCFRunLoopRunHandledSource = 4
//};
//
///* Run Loop Observer Activities */
//typedef CF_OPTIONS(CFOptionFlags, CFRunLoopActivity) {
//    kCFRunLoopEntry = (1UL << 0),
//    kCFRunLoopBeforeTimers = (1UL << 1),
//    kCFRunLoopBeforeSources = (1UL << 2),
//    kCFRunLoopBeforeWaiting = (1UL << 5),
//    kCFRunLoopAfterWaiting = (1UL << 6),
//    kCFRunLoopExit = (1UL << 7),
//    kCFRunLoopAllActivities = 0x0FFFFFFFU
//};
//
//
//typedef struct {
//    CFIndex	version;
//    void *	info;
//    const void *(*retain)(const void *info);
//    void	(*release)(const void *info);
//    CFStringRef	(*copyDescription)(const void *info);
//    Boolean	(*equal)(const void *info1, const void *info2);
//    CFHashCode	(*hash)(const void *info);
//    void	(*schedule)(void *info, CFRunLoopRef rl, CFStringRef mode);
//    void	(*cancel)(void *info, CFRunLoopRef rl, CFStringRef mode);
//    void	(*perform)(void *info);
//} CFRunLoopSourceContext;
//
//typedef struct {
//    CFIndex	version;
//    void *	info;
//    const void *(*retain)(const void *info);
//    void	(*release)(const void *info);
//    CFStringRef	(*copyDescription)(const void *info);
//    Boolean	(*equal)(const void *info1, const void *info2);
//    CFHashCode	(*hash)(const void *info);
//#if (TARGET_OS_MAC && !(TARGET_OS_EMBEDDED || TARGET_OS_IPHONE)) || (TARGET_OS_EMBEDDED || TARGET_OS_IPHONE)
//    mach_port_t	(*getPort)(void *info);
//    void *	(*perform)(void *msg, CFIndex size, CFAllocatorRef allocator, void *info);
//#else
//    void *	(*getPort)(void *info);
//    void	(*perform)(void *info);
//#endif
//} CFRunLoopSourceContext1;
//
//typedef struct {
//    CFIndex	version;
//    void *	info;
//    const void *(*retain)(const void *info);
//    void	(*release)(const void *info);
//    CFStringRef	(*copyDescription)(const void *info);
//} CFRunLoopObserverContext;
//
//typedef void (*CFRunLoopObserverCallBack)(CFRunLoopObserverRef observer, CFRunLoopActivity activity, void *info);
//

#[repr(C)]
pub struct CFRunLoopTimerContext {
    version: CFIndex, 
    info: *mut c_void,
    retain: extern "C" fn (info: *const c_void) -> *const c_void,
    release: extern "C" fn (info: *const c_void),
    copyDescription: extern "C" fn (info: *const c_void) -> CFStringRef,
}

pub type CFRunLoopTimerCallBack = extern "C" fn (timer: CFRunLoopTimerRef, info: *mut c_void);

#[repr(C)]
struct __CFRunLoopTimer;

pub type CFRunLoopTimerRef = *const __CFRunLoopTimer;

/// FIXME(pcwalton): Should be a newtype struct, but that fails due to a Rust compiler bug.
pub struct CFRunLoopTimer {
    obj: CFRunLoopTimerRef,
}

impl Drop for CFRunLoopTimer {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl TCFType<CFRunLoopTimerRef> for CFRunLoopTimer {
    #[inline]
    fn as_concrete_TypeRef(&self) -> CFRunLoopTimerRef {
        self.obj
    }

    #[inline]
    unsafe fn wrap_under_get_rule(reference: CFRunLoopTimerRef) -> CFRunLoopTimer {
        let reference: CFRunLoopTimerRef = mem::transmute(CFRetain(mem::transmute(reference)));
        TCFType::wrap_under_create_rule(reference)
    }

    #[inline]
    fn as_CFTypeRef(&self) -> CFTypeRef {
        unsafe {
            mem::transmute(self.as_concrete_TypeRef())
        }
    }

    #[inline]
    unsafe fn wrap_under_create_rule(obj: CFRunLoopTimerRef) -> CFRunLoopTimer {
        CFRunLoopTimer {
            obj: obj,
        }
    }

    #[inline]
    fn type_id(_: Option<CFRunLoopTimer>) -> CFTypeID {
        unsafe {
            CFRunLoopTimerGetTypeID()
        }
    }
}

impl CFRunLoopTimer {
    pub fn new(fireDate: CFAbsoluteTime, interval: CFTimeInterval, flags: CFOptionFlags, order: CFIndex, callout: CFRunLoopTimerCallBack, context: *mut CFRunLoopTimerContext) -> CFRunLoopTimer {
        unsafe {
            let timer_ref = CFRunLoopTimerCreate(kCFAllocatorDefault, fireDate, interval, flags, order, callout, context);
            TCFType::wrap_under_create_rule(timer_ref)
        }
    }
}


#[link(name = "CoreFoundation", kind = "framework")]
extern {
    /*
     * CFRunLoop.h
     */
    pub static kCFRunLoopDefaultMode: CFStringRef;
    pub static kCFRunLoopCommonModes: CFStringRef;
    fn CFRunLoopGetTypeID() -> CFTypeID;
    fn CFRunLoopGetCurrent() -> CFRunLoopRef;
    fn CFRunLoopGetMain() -> CFRunLoopRef;
    fn CFRunLoopCopyCurrentMode(rl: CFRunLoopRef) -> CFStringRef;
    //fn CFRunLoopCopyAllModes(rl: CFRunLoopRef) -> CFArrayRef;
    //fn CFRunLoopAddCommonMode(rl: CFRunLoopRef, mode: CFStringRef);
    //fn CFRunLoopGetNextTimerFireDate(rl: CFRunLoopRef, mode: CFStringRef) -> CFAbsoluteTime;
    fn CFRunLoopRun();
    //fn CFRunLoopRunInMode(mode: CFStringRef, seconds: CFTimeInterval, returnAfterSourceHandled: Boolean) -> SInt32;
    //fn CFRunLoopIsWaiting(rl: CFRunLoopRef) -> Boolean;
    //fn CFRunLoopWakeUp(rl: CFRunLoopRef);
    fn CFRunLoopStop(rl: CFRunLoopRef);
    // fn CFRunLoopPerformBlock(rl: CFRunLoopRef, mode: CFTypeRef, block: void (^)(void));
    //fn CFRunLoopContainsSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFStringRef) -> Boolean;
    //fn CFRunLoopAddSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFStringRef);
    //fn CFRunLoopRemoveSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFStringRef);
    //fn CFRunLoopContainsObserver(rl: CFRunLoopRef, observer: CFRunLoopObserverRef, mode: CFStringRef) -> Boolean;
    //fn CFRunLoopAddObserver(rl: CFRunLoopRef, observer: CFRunLoopObserverRef, mode: CFStringRef);
    //fn CFRunLoopRemoveObserver(rl: CFRunLoopRef, observer: CFRunLoopObserverRef, mode: CFStringRef);
    //fn CFRunLoopRemoveObserver(rl: CFRunLoopRef, observer: CFRunLoopObserverRef, mode: CFStringRef);
    fn CFRunLoopContainsTimer(rl: CFRunLoopRef, timer: CFRunLoopTimerRef, mode: CFStringRef) -> Boolean;
    fn CFRunLoopAddTimer(rl: CFRunLoopRef, timer: CFRunLoopTimerRef, mode: CFStringRef);
    //fn CFRunLoopRemoveTimer(rl: CFRunLoopRef, timer: CFRunLoopTimerRef, mode: CFStringRef);

    //fn CFRunLoopSourceGetTypeID() -> CFTypeID;
    //fn CFRunLoopSourceCreate(allocator: CFAllocatorRef, order: CFIndex, context: *mut CFRunLoopSourceContext) -> CFRunLoopSourceRef;
    //fn CFRunLoopSourceGetOrder(source: CFRunLoopSourceRef) -> CFIndex;
    //fn CFRunLoopSourceInvalidate(source: CFRunLoopSourceRef);
    //fn CFRunLoopSourceIsValid(source: CFRunLoopSourceRef) -> Boolean;
    //fn CFRunLoopSourceGetContext(source: CFRunLoopSourceRef, context: *mut CFRunLoopSourceContext);
    //fn CFRunLoopSourceSignal(source: CFRunLoopSourceRef);

    //fn CFRunLoopObserverGetTypeID() -> CFTypeID;
    //fn CFRunLoopObserverCreate(allocator: CFAllocatorRef, activities: CFOptionFlags, repeats: Boolean, order: CFIndex, callout: CFRunLoopObserverCallBack, context: *mut CFRunLoopObserverContext) -> CFRunLoopObserverRef;
    // fn CFRunLoopObserverCreateWithHandler(allocator: CFAllocatorRef, activities: CFOptionFlags, repeats: Boolean, order: CFIndex, block: void (^) (CFRunLoopObserverRef observer, CFRunLoopActivity activity)) -> CFRunLoopObserverRef;
    //fn CFRunLoopObserverGetActivities(observer: CFRunLoopObserverRef) -> CFOptionFlags;
    //fn CFRunLoopObserverDoesRepeat(observer: CFRunLoopObserverRef) -> Boolean;
    //fn CFRunLoopObserverGetOrder(observer: CFRunLoopObserverRef) -> CFIndex;
    //fn CFRunLoopObserverInvalidate(observer: CFRunLoopObserverRef);
    //fn CFRunLoopObserverIsValid(observer: CFRunLoopObserverRef) -> Boolean;
    //fn CFRunLoopObserverGetContext(observer: CFRunLoopObserverRef, context: *mut CFRunLoopObserverContext);

    fn CFRunLoopTimerGetTypeID() -> CFTypeID;
    fn CFRunLoopTimerCreate(allocator: CFAllocatorRef, fireDate: CFAbsoluteTime, interval: CFTimeInterval, flags: CFOptionFlags, order: CFIndex, callout: CFRunLoopTimerCallBack, context: *mut CFRunLoopTimerContext) -> CFRunLoopTimerRef;
    // fn CFRunLoopTimerCreateWithHandler(allocator: CFAllocatorRef, fireDate: CFAbsoluteTime, interval: CFTimeInterval, flags: CFOptionFlags, order: CFIndex, block: void (^) (CFRunLoopTimerRef timer)) -> CFRunLoopTimerRef;
    //fn CFRunLoopTimerGetNextFireDate(timer: CFRunLoopTimerRef) -> CFAbsoluteTime;
    //fn CFRunLoopTimerSetNextFireDate(timer: CFRunLoopTimerRef, fireDate: CFAbsoluteTime);
    //fn CFRunLoopTimerGetInterval(timer: CFRunLoopTimerRef) -> CFTimeInterval;
    //fn CFRunLoopTimerDoesRepeat(timer: CFRunLoopTimerRef) -> Boolean;
    //fn CFRunLoopTimerGetOrder(timer: CFRunLoopTimerRef) -> CFIndex;
    //fn CFRunLoopTimerInvalidate(timer: CFRunLoopTimerRef);
    //fn CFRunLoopTimerIsValid(timer: CFRunLoopTimerRef) -> Boolean;
    //fn CFRunLoopTimerGetContext(timer: CFRunLoopTimerRef, context: *mut CFRunLoopTimerContext);
    //fn CFRunLoopTimerGetTolerance(timer: CFRunLoopTimerRef) -> CFTimeInterval;
    //fn CFRunLoopTimerSetTolerance(timer: CFRunLoopTimerRef, tolerance: CFTimeInterval);
}


#[test]
fn wait_200_milliseconds() {
        let run_loop = CFRunLoop::get_current();
        let mut now = unsafe { CFAbsoluteTimeGetCurrent() };
        let mut context = unsafe { CFRunLoopTimerContext {
            version: 0,
            info: mem::transmute(&mut now),
            retain: mem::zeroed(),
            release: mem::zeroed(),
            copyDescription: mem::zeroed(),
        } };


        let run_loop_timer = CFRunLoopTimer::new(now + 0.20f64, 0f64, 0, 0, timer_popped, &mut context);
        run_loop.add_timer(&run_loop_timer, kCFRunLoopDefaultMode);

        CFRunLoop::run_current();
}

extern "C" fn timer_popped(_timer: CFRunLoopTimerRef, _info: *mut c_void) {
    let previous_now_ptr: *const CFAbsoluteTime = unsafe { mem::transmute(_info) };
    let previous_now = unsafe { *previous_now_ptr };
    let now = unsafe { CFAbsoluteTimeGetCurrent() };
    assert!(now - previous_now > 0.19 && now - previous_now < 0.21);
    CFRunLoop::get_current().stop();
}
