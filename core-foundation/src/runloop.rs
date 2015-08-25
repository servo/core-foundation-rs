// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_upper_case_globals)]


use core_foundation_sys::base::{CFAllocatorRef, CFIndex, CFRelease};
use core_foundation_sys::base::{CFTypeID, CFHashCode, mach_port_t};
use core_foundation_sys::base::{kCFAllocatorDefault, Boolean, CFOptionFlags};
use core_foundation_sys::array::{CFArrayRef};
use core_foundation_sys::string::CFStringRef;
use libc::c_void;
use std::mem;

use base::{TCFType};
use string::{CFString};
use date::{CFAbsoluteTime, CFTimeInterval};

pub struct CFRunLoop(CFRunLoopRef);

impl Drop for CFRunLoop {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl_TCFType!(CFRunLoop, CFRunLoopRef, CFRunLoopGetTypeID);

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
            CFRunLoopStop(self.0);
        }
    }

    pub fn current_mode(&self) -> Option<String> {
        unsafe {
            let string_ref = CFRunLoopCopyCurrentMode(self.0);
            if string_ref.is_null() {
                return None;
            }

            let cf_string: CFString = TCFType::wrap_under_create_rule(string_ref);
            Some(cf_string.to_string())
        }
    }

    pub fn contains_timer(&self, timer: &CFRunLoopTimer, mode: CFStringRef) -> bool {
        unsafe {
            CFRunLoopContainsTimer(self.0, timer.0, mode) != 0
        }
    }

    pub fn add_timer(&self, timer: &CFRunLoopTimer, mode: CFStringRef) {
        unsafe {
            CFRunLoopAddTimer(self.0, timer.0, mode);
        }
    }

}

#[repr(C)]
struct __CFRunLoop;

pub type CFRunLoopRef = *const __CFRunLoop;

#[repr(C)]
struct __CFRunLoopSource;

pub type CFRunLoopSourceRef = *const __CFRunLoopSource;

#[repr(C)]
struct __CFRunLoopObserver;

pub type CFRunLoopObserverRef = *const __CFRunLoopObserver;

// Reasons for CFRunLoopRunInMode() to Return
pub const kCFRunLoopRunFinished: i32      = 1;
pub const kCFRunLoopRunStopped: i32       = 2;
pub const kCFRunLoopRunTimedOut: i32      = 3;
pub const kCFRunLoopRunHandledSource: i32 = 4;

// Run Loop Observer Activities
//typedef CF_OPTIONS(CFOptionFlags, CFRunLoopActivity) {
pub type CFRunLoopActivity = CFOptionFlags;
pub const kCFRunLoopEntry: CFOptionFlags         = 1 << 0;
pub const kCFRunLoopBeforeTimers: CFOptionFlags  = 1 << 1;
pub const kCFRunLoopBeforeSources: CFOptionFlags = 1 << 2;
pub const kCFRunLoopBeforeWaiting: CFOptionFlags = 1 << 5;
pub const kCFRunLoopAfterWaiting: CFOptionFlags  = 1 << 6;
pub const kCFRunLoopExit: CFOptionFlags          = 1 << 7;
pub const kCFRunLoopAllActivities: CFOptionFlags = 0x0FFFFFFF;

#[repr(C)]
pub struct CFRunLoopSourceContext {
    version: CFIndex,
    info: *mut c_void,
    retain: extern "C" fn (info: *const c_void) -> *const c_void,
    release: extern "C" fn (info: *const c_void),
    copyDescription: extern "C" fn (info: *const c_void) -> CFStringRef,
    equal: extern "C" fn (info1: *const c_void, info2: *const c_void) -> Boolean,
    hash: extern "C" fn (info: *const c_void) -> CFHashCode,
    schedule: extern "C" fn (info: *const c_void, rl: CFRunLoopRef, mode: CFStringRef),
    cancel: extern "C" fn (info: *const c_void, rl: CFRunLoopRef, mode: CFStringRef),
    perform: extern "C" fn (info: *const c_void),
}

#[repr(C)]
pub struct CFRunLoopSourceContext1 {
    version: CFIndex,
    info: *mut c_void,
    retain: extern "C" fn (info: *const c_void) -> *const c_void,
    release: extern "C" fn (info: *const c_void),
    copyDescription: extern "C" fn (info: *const c_void) -> CFStringRef,
    equal: extern "C" fn (info1: *const c_void, info2: *const c_void) -> Boolean,
    hash: extern "C" fn (info: *const c_void) -> CFHashCode,
    // note that the following two fields are platform dependent in the C header, the ones here are for OS X
    getPort: extern "C" fn (info: *mut c_void) -> mach_port_t,
    perform: extern "C" fn (msg: *mut c_void, size: CFIndex, allocator: CFAllocatorRef, info: *mut c_void) -> *mut c_void,
}

#[repr(C)]
pub struct CFRunLoopObserverContext {
    version: CFIndex,
    info: *mut c_void,
    retain: extern "C" fn (info: *const c_void) -> *const c_void,
    release: extern "C" fn (info: *const c_void),
    copyDescription: extern "C" fn (info: *const c_void) -> CFStringRef,
}

pub type CFRunLoopObserverCallBack = extern "C" fn (observer: CFRunLoopObserverRef, activity: CFRunLoopActivity, info: *mut c_void);

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

pub struct CFRunLoopTimer(CFRunLoopTimerRef);

impl Drop for CFRunLoopTimer {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl_TCFType!(CFRunLoopTimer, CFRunLoopTimerRef, CFRunLoopTimerGetTypeID);

impl CFRunLoopTimer {
    pub fn new(fireDate: CFAbsoluteTime, interval: CFTimeInterval, flags: CFOptionFlags, order: CFIndex, callout: CFRunLoopTimerCallBack, context: *mut CFRunLoopTimerContext) -> CFRunLoopTimer {
        unsafe {
            let timer_ref = CFRunLoopTimerCreate(kCFAllocatorDefault, fireDate, interval, flags, order, callout, context);
            TCFType::wrap_under_create_rule(timer_ref)
        }
    }
}


#[allow(dead_code)]
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
    fn CFRunLoopCopyAllModes(rl: CFRunLoopRef) -> CFArrayRef;
    fn CFRunLoopAddCommonMode(rl: CFRunLoopRef, mode: CFStringRef);
    fn CFRunLoopGetNextTimerFireDate(rl: CFRunLoopRef, mode: CFStringRef) -> CFAbsoluteTime;
    fn CFRunLoopRun();
    fn CFRunLoopRunInMode(mode: CFStringRef, seconds: CFTimeInterval, returnAfterSourceHandled: Boolean) -> i32;
    fn CFRunLoopIsWaiting(rl: CFRunLoopRef) -> Boolean;
    fn CFRunLoopWakeUp(rl: CFRunLoopRef);
    fn CFRunLoopStop(rl: CFRunLoopRef);
    // fn CFRunLoopPerformBlock(rl: CFRunLoopRef, mode: CFTypeRef, block: void (^)(void));
    fn CFRunLoopContainsSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFStringRef) -> Boolean;
    fn CFRunLoopAddSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFStringRef);
    fn CFRunLoopRemoveSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFStringRef);
    fn CFRunLoopContainsObserver(rl: CFRunLoopRef, observer: CFRunLoopObserverRef, mode: CFStringRef) -> Boolean;
    fn CFRunLoopAddObserver(rl: CFRunLoopRef, observer: CFRunLoopObserverRef, mode: CFStringRef);
    fn CFRunLoopRemoveObserver(rl: CFRunLoopRef, observer: CFRunLoopObserverRef, mode: CFStringRef);
    fn CFRunLoopContainsTimer(rl: CFRunLoopRef, timer: CFRunLoopTimerRef, mode: CFStringRef) -> Boolean;
    fn CFRunLoopAddTimer(rl: CFRunLoopRef, timer: CFRunLoopTimerRef, mode: CFStringRef);
    fn CFRunLoopRemoveTimer(rl: CFRunLoopRef, timer: CFRunLoopTimerRef, mode: CFStringRef);

    fn CFRunLoopSourceGetTypeID() -> CFTypeID;
    fn CFRunLoopSourceCreate(allocator: CFAllocatorRef, order: CFIndex, context: *mut CFRunLoopSourceContext) -> CFRunLoopSourceRef;
    fn CFRunLoopSourceGetOrder(source: CFRunLoopSourceRef) -> CFIndex;
    fn CFRunLoopSourceInvalidate(source: CFRunLoopSourceRef);
    fn CFRunLoopSourceIsValid(source: CFRunLoopSourceRef) -> Boolean;
    fn CFRunLoopSourceGetContext(source: CFRunLoopSourceRef, context: *mut CFRunLoopSourceContext);
    fn CFRunLoopSourceSignal(source: CFRunLoopSourceRef);

    fn CFRunLoopObserverGetTypeID() -> CFTypeID;
    fn CFRunLoopObserverCreate(allocator: CFAllocatorRef, activities: CFOptionFlags, repeats: Boolean, order: CFIndex, callout: CFRunLoopObserverCallBack, context: *mut CFRunLoopObserverContext) -> CFRunLoopObserverRef;
    // fn CFRunLoopObserverCreateWithHandler(allocator: CFAllocatorRef, activities: CFOptionFlags, repeats: Boolean, order: CFIndex, block: void (^) (CFRunLoopObserverRef observer, CFRunLoopActivity activity)) -> CFRunLoopObserverRef;
    fn CFRunLoopObserverGetActivities(observer: CFRunLoopObserverRef) -> CFOptionFlags;
    fn CFRunLoopObserverDoesRepeat(observer: CFRunLoopObserverRef) -> Boolean;
    fn CFRunLoopObserverGetOrder(observer: CFRunLoopObserverRef) -> CFIndex;
    fn CFRunLoopObserverInvalidate(observer: CFRunLoopObserverRef);
    fn CFRunLoopObserverIsValid(observer: CFRunLoopObserverRef) -> Boolean;
    fn CFRunLoopObserverGetContext(observer: CFRunLoopObserverRef, context: *mut CFRunLoopObserverContext);

    fn CFRunLoopTimerGetTypeID() -> CFTypeID;
    fn CFRunLoopTimerCreate(allocator: CFAllocatorRef, fireDate: CFAbsoluteTime, interval: CFTimeInterval, flags: CFOptionFlags, order: CFIndex, callout: CFRunLoopTimerCallBack, context: *mut CFRunLoopTimerContext) -> CFRunLoopTimerRef;
    // fn CFRunLoopTimerCreateWithHandler(allocator: CFAllocatorRef, fireDate: CFAbsoluteTime, interval: CFTimeInterval, flags: CFOptionFlags, order: CFIndex, block: void (^) (CFRunLoopTimerRef timer)) -> CFRunLoopTimerRef;
    fn CFRunLoopTimerGetNextFireDate(timer: CFRunLoopTimerRef) -> CFAbsoluteTime;
    fn CFRunLoopTimerSetNextFireDate(timer: CFRunLoopTimerRef, fireDate: CFAbsoluteTime);
    fn CFRunLoopTimerGetInterval(timer: CFRunLoopTimerRef) -> CFTimeInterval;
    fn CFRunLoopTimerDoesRepeat(timer: CFRunLoopTimerRef) -> Boolean;
    fn CFRunLoopTimerGetOrder(timer: CFRunLoopTimerRef) -> CFIndex;
    fn CFRunLoopTimerInvalidate(timer: CFRunLoopTimerRef);
    fn CFRunLoopTimerIsValid(timer: CFRunLoopTimerRef) -> Boolean;
    fn CFRunLoopTimerGetContext(timer: CFRunLoopTimerRef, context: *mut CFRunLoopTimerContext);
    fn CFRunLoopTimerGetTolerance(timer: CFRunLoopTimerRef) -> CFTimeInterval;
    fn CFRunLoopTimerSetTolerance(timer: CFRunLoopTimerRef, tolerance: CFTimeInterval);
}

#[cfg(test)]
mod test {
    use super::*;
    use date::{CFAbsoluteTime, CFAbsoluteTimeGetCurrent};
    use std::mem;
    use libc::c_void;

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
}
