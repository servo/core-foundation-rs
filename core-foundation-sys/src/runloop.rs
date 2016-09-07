// Copyright 2013-2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use allocator::CFAllocator;
use base::{CFDowncast, CFHashCode, CFIndex, CFObject, CFOptionFlags};
use base::{CFType, CFTypeID};
use std::os::raw::c_void;
use string::{CFString, CFStringRef};
use sync::{CFRef, CFShared};
use time::{CFAbsoluteTime, CFTimeInterval};
use version::{CFVersion0, CFVersion1};

pub type CFRunLoopRef = CFRef<CFRunLoop>;

#[repr(C)]
pub struct CFRunLoop { obj: CFObject }

unsafe impl Send for CFRunLoop {}
unsafe impl Sync for CFRunLoop {}

unsafe impl CFType for CFRunLoop {
    #[inline]
    fn as_object(&self) -> &CFObject {
        &self.obj
    }
}

unsafe impl CFDowncast for CFRunLoop {
    #[inline]
    fn type_id() -> CFTypeID {
        unsafe { CFRunLoopGetTypeID() }
    }
}

impl CFRunLoop {
    #[inline]
    pub fn current() -> CFRunLoopRef {
        unsafe { (&*CFRunLoopGetCurrent()).retain() }
    }

    #[inline]
    pub fn main() -> CFRunLoopRef {
        unsafe { (&*CFRunLoopGetMain()).retain() }
    }

    #[inline]
    pub fn current_mode(&self) -> Option<CFStringRef> {
        unsafe { CFRef::try_from_retained(CFRunLoopCopyCurrentMode(self)).ok() }
    }

    #[inline]
    pub fn next_timer_fire_date(&self, mode: &CFString) -> CFAbsoluteTime {
        unsafe { CFRunLoopGetNextTimerFireDate(self, mode) }
    }

    #[inline]
    pub fn run() {
        unsafe { CFRunLoopRun() }
    }

    #[inline]
    pub fn run_in_mode(
            mode: &CFString,
            seconds: CFTimeInterval,
            return_after_source_handled: bool)
            -> CFRunLoopRunResult {
        unsafe {
            CFRunLoopRunInMode(mode, seconds, return_after_source_handled)
        }
    }

    #[inline]
    pub fn is_waiting(&self) -> bool {
        unsafe { CFRunLoopIsWaiting(self) }
    }

    #[inline]
    pub fn wakeup(&self) {
        unsafe { CFRunLoopWakeUp(self) }
    }

    #[inline]
    pub fn stop(&self) {
        unsafe { CFRunLoopStop(self) }
    }

    #[inline]
    pub fn contains_source(
            &self, source: &CFRunLoopSource, mode: &CFString)
            -> bool {
        unsafe { CFRunLoopContainsSource(self, source, mode) }
    }

    #[inline]
    pub fn add_source(&self, source: &CFRunLoopSource, mode: &CFString) {
        unsafe { CFRunLoopAddSource(self, source, mode) }
    }

    #[inline]
    pub fn remove_source(&self, source: &CFRunLoopSource, mode: &CFString) {
        unsafe { CFRunLoopRemoveSource(self, source, mode) }
    }

    #[inline]
    pub fn contains_observer(
            &self, observer: &CFRunLoopObserver, mode: &CFString)
            -> bool {
        unsafe { CFRunLoopContainsObserver(self, observer, mode) }
    }

    #[inline]
    pub fn add_observer(&self, observer: &CFRunLoopObserver, mode: &CFString) {
        unsafe { CFRunLoopAddObserver(self, observer, mode) }
    }

    #[inline]
    pub fn remove_observer(
            &self, observer: &CFRunLoopObserver, mode: &CFString) {
        unsafe { CFRunLoopRemoveObserver(self, observer, mode) }
    }

    #[inline]
    pub fn contains_timer(
            &self, timer: &CFRunLoopTimer, mode: &CFString)
            -> bool {
        unsafe { CFRunLoopContainsTimer(self, timer, mode) }
    }

    #[inline]
    pub fn add_timer(&self, timer: &CFRunLoopTimer, mode: &CFString) {
        unsafe { CFRunLoopAddTimer(self, timer, mode) }
    }

    #[inline]
    pub fn remove_timer(&self, timer: &CFRunLoopTimer, mode: &CFString) {
        unsafe { CFRunLoopRemoveTimer(self, timer, mode) }
    }
}

#[repr(i32)]
pub enum CFRunLoopRunResult {
    Finished = 1,
    Stopped = 2,
    TimedOut = 3,
    HandledSource = 4,
}

pub type CFRunLoopSourceRef = CFRef<CFRunLoopSource>;

#[repr(C)]
pub struct CFRunLoopSource { obj: CFObject }

unsafe impl Send for CFRunLoopSource {}
unsafe impl Sync for CFRunLoopSource {}

unsafe impl CFType for CFRunLoopSource {
    #[inline]
    fn as_object(&self) -> &CFObject {
        &self.obj
    }
}

unsafe impl CFDowncast for CFRunLoopSource {
    #[inline]
    fn type_id() -> CFTypeID {
        unsafe { CFRunLoopSourceGetTypeID() }
    }
}

impl CFRunLoopSource {
    #[inline]
    pub fn order(&self) -> CFIndex {
        unsafe { CFRunLoopSourceGetOrder(self) }
    }

    #[inline]
    pub fn invalidate(&self) {
        unsafe { CFRunLoopSourceInvalidate(self) }
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        unsafe { CFRunLoopSourceIsValid(self) }
    }

    #[inline]
    pub fn signal(&self) {
        unsafe { CFRunLoopSourceSignal(self) }
    }
}

#[repr(C)]
pub struct CFRunLoopSourceContext {
    version: CFVersion0,
    info: *mut c_void,
    retain: Option<CFRunLoopRetainCallBack>,
    release: Option<CFRunLoopReleaseCallBack>,
    copyDescription: Option<CFRunLoopCopyDescriptionCallBack>,
    equal: Option<CFRunLoopEqualCallBack>,
    hash: Option<CFRunLoopHashCallBack>,
    schedule: Option<CFRunLoopScheduleCallBack>,
    cancel: Option<CFRunLoopCancelCallBack>,
    perform: CFRunLoopPerformCallBack,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct CFRunLoopSourceContext1 {
    version: CFVersion1,
    info: *mut c_void,
    retain: Option<CFRunLoopRetainCallBack>,
    release: Option<CFRunLoopReleaseCallBack>,
    copyDescription: Option<CFRunLoopCopyDescriptionCallBack>,
    equal: Option<CFRunLoopEqualCallBack>,
    hash: Option<CFRunLoopHashCallBack>,
    getPort: Option<CFRunLoopGetPortCallBack>,
    perform: CFRunLoopPerformCallBack,
}

pub type CFRunLoopRetainCallBack =
    unsafe extern fn(info: *const c_void) -> *const c_void;

pub type CFRunLoopReleaseCallBack =
    unsafe extern fn(info: *const c_void);

pub type CFRunLoopCopyDescriptionCallBack =
    unsafe extern fn(
        info: *const c_void)
        -> Option<&'static CFShared<CFString>>;

pub type CFRunLoopEqualCallBack =
    unsafe extern fn(info1: *const c_void, info2: *const c_void) -> bool;

pub type CFRunLoopHashCallBack =
    unsafe extern fn(info: *const c_void) -> CFHashCode;

pub type CFRunLoopScheduleCallBack =
    unsafe extern fn(
        info: *mut c_void,
        rl: &CFShared<CFRunLoop>,
        mode: &CFShared<CFString>);

pub type CFRunLoopCancelCallBack =
    unsafe extern fn(
        info: *mut c_void,
        rl: &CFShared<CFRunLoop>,
        mode: &CFShared<CFString>);

pub type CFRunLoopPerformCallBack =
    unsafe extern fn(info: *const c_void);

pub type CFRunLoopGetPortCallBack =
    unsafe extern fn(info: *const c_void) -> *const c_void;

pub type CFRunLoopMachPerformCallBack =
    unsafe extern fn(
        info: *const c_void,
        size: CFIndex,
        allocator: &CFShared<CFAllocator>);

pub type CFRunLoopObserverRef = CFRef<CFRunLoopObserver>;

#[repr(C)]
pub struct CFRunLoopObserver { obj: CFObject }

unsafe impl Send for CFRunLoopObserver {}
unsafe impl Sync for CFRunLoopObserver {}

unsafe impl CFType for CFRunLoopObserver {
    #[inline]
    fn as_object(&self) -> &CFObject {
        &self.obj
    }
}

bitflags! {
    #[repr(C)]
    pub flags CFRunLoopActivity: CFOptionFlags {
        const RUN_LOOP_ENTRY = 1 << 0,
        const RUN_LOOP_BEFORE_TIMERS = 1 << 1,
        const RUN_LOOP_BEFORE_SOURCES = 1 << 2,
        const RUN_LOOP_BEFORE_WAITING = 1 << 5,
        const RUN_LOOP_AFTER_WAITING = 1 << 6,
        const RUN_LOOP_EXIT = 1 << 7,
        const RUN_LOOP_ALL_ACIVITIES = 0x0FFFFFFF,
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct CFRunLoopObserverContext {
    version: CFVersion0,
    info: *mut c_void,
    retain: Option<CFRunLoopRetainCallBack>,
    release: Option<CFRunLoopReleaseCallBack>,
    copyDescription: Option<CFRunLoopCopyDescriptionCallBack>,
}

pub type CFRunLoopObserverCallBack =
    unsafe extern fn(
        observer: &CFShared<CFRunLoopObserver>,
        activity: CFRunLoopActivity,
        info: *mut c_void);

pub type CFRunLoopTimerRef = CFRef<CFRunLoopTimer>;

#[repr(C)]
pub struct CFRunLoopTimer { obj: CFObject }

unsafe impl CFType for CFRunLoopTimer {
    #[inline]
    fn as_object(&self) -> &CFObject {
        &self.obj
    }
}

unsafe impl Send for CFRunLoopTimer {}
unsafe impl Sync for CFRunLoopTimer {}

unsafe impl CFDowncast for CFRunLoopTimer {
    #[inline]
    fn type_id() -> CFTypeID {
        unsafe { CFRunLoopTimerGetTypeID() }
    }
}

impl CFRunLoopTimer {
    #[inline]
    pub unsafe fn new(
            fire_date: CFAbsoluteTime,
            interval: CFTimeInterval,
            callout: CFRunLoopTimerCallBack,
            context: Option<&CFRunLoopTimerContext>)
            -> CFRunLoopTimerRef {
        CFRef::from_retained(
            CFRunLoopTimerCreate(
                None,
                fire_date,
                interval,
                CFRunLoopTimerFlags::default(),
                0,
                callout,
                context))
    }

    #[inline]
    pub fn next_fire_date(&self) -> CFAbsoluteTime {
        unsafe { CFRunLoopTimerGetNextFireDate(self) }
    }

    #[inline]
    pub fn set_next_fire_date(&self, fire_date: CFAbsoluteTime) {
        unsafe { CFRunLoopTimerSetNextFireDate(self, fire_date) }
    }

    #[inline]
    pub fn interval(&self) -> CFTimeInterval {
        unsafe { CFRunLoopTimerGetInterval(self) }
    }

    #[inline]
    pub fn order(&self) -> CFIndex {
        unsafe { CFRunLoopTimerGetOrder(self) }
    }

    #[inline]
    pub fn repeats(&self) -> bool {
        unsafe { CFRunLoopTimerDoesRepeat(self) }
    }

    #[inline]
    pub fn invalidate(&self) {
        unsafe { CFRunLoopTimerInvalidate(self) }
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        unsafe { CFRunLoopTimerIsValid(self) }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct CFRunLoopTimerContext {
    version: CFVersion0,
    info: *mut c_void,
    retain: Option<CFRunLoopRetainCallBack>,
    release: Option<CFRunLoopReleaseCallBack>,
    copyDescription: Option<CFRunLoopCopyDescriptionCallBack>,
}

pub type CFRunLoopTimerCallBack =
    unsafe extern fn(observer: &CFShared<CFRunLoopTimer>, info: *mut c_void);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct CFRunLoopTimerFlags { flags: CFOptionFlags }

impl CFRunLoopTimerFlags {
    #[inline]
    fn new() -> Self {
        CFRunLoopTimerFlags { flags: 0 }
    }
}

impl Default for CFRunLoopTimerFlags {
    #[inline]
    fn default() -> Self {
        CFRunLoopTimerFlags::new()
    }
}

#[cfg(test)]
mod test {
    use runloop::{CFRunLoop, CFRunLoopTimer};
    use runloop::{CFRunLoopTimerContext, kCFRunLoopDefaultMode};
    use std::os::raw::c_void;
    use sync::CFShared;
    use time::{CFAbsoluteTime, CFAbsoluteTimeGetCurrent};

    #[test]
    fn wait_200_milliseconds() {
        let run_loop = CFRunLoop::current();
        let mut now = unsafe { CFAbsoluteTimeGetCurrent() };
        let context = CFRunLoopTimerContext {
            version: Default::default(),
            info: &mut now as *mut _ as *mut _,
            retain: None,
            release: None,
            copyDescription: None,
        };

        let timer = unsafe {
            CFRunLoopTimer::new(
                now + 0.20f64, 0f64, timer_popped, Some(&context))
        };
        run_loop.add_timer(&timer, kCFRunLoopDefaultMode.unwrap());

        CFRunLoop::run();
    }

    unsafe extern fn timer_popped(
            _timer: &CFShared<CFRunLoopTimer>, info: *mut c_void) {
        let previous_now = &*(info as *const CFAbsoluteTime);
        let now = CFAbsoluteTimeGetCurrent();
        assert!(now - previous_now > 0.19 && now - previous_now < 0.21);
        CFRunLoop::current().stop();
    }
}

extern {
    pub static kCFRunLoopDefaultMode: Option<&'static CFShared<CFString>>;

    pub fn CFRunLoopGetTypeID() -> CFTypeID;

    pub fn CFRunLoopGetCurrent() -> *const CFShared<CFRunLoop>;
    pub fn CFRunLoopGetMain() -> *const CFShared<CFRunLoop>;

    pub fn CFRunLoopCopyCurrentMode(
            rl: &CFRunLoop)
            -> *const CFShared<CFString>;

    pub fn CFRunLoopGetNextTimerFireDate(
            rl: &CFRunLoop, mode: &CFString)
            -> CFAbsoluteTime;

    pub fn CFRunLoopRun();

    pub fn CFRunLoopRunInMode(
            mode: &CFString,
            seconds: CFTimeInterval,
            returnAfterSourceHandled: bool)
            -> CFRunLoopRunResult;

    pub fn CFRunLoopIsWaiting(rl: &CFRunLoop) -> bool;
    pub fn CFRunLoopWakeUp(rl: &CFRunLoop);
    pub fn CFRunLoopStop(rl: &CFRunLoop);

    pub fn CFRunLoopContainsSource(
            rl: &CFRunLoop, source: &CFRunLoopSource, mode: &CFString)
            -> bool;

    pub fn CFRunLoopAddSource(
            rl: &CFRunLoop, source: &CFRunLoopSource, mode: &CFString);

    pub fn CFRunLoopRemoveSource(
            rl: &CFRunLoop, source: &CFRunLoopSource, mode: &CFString);

    pub fn CFRunLoopContainsObserver(
            rl: &CFRunLoop, observer: &CFRunLoopObserver, mode: &CFString)
            -> bool;

    pub fn CFRunLoopAddObserver(
            rl: &CFRunLoop, observer: &CFRunLoopObserver, mode: &CFString);

    pub fn CFRunLoopRemoveObserver(
            rl: &CFRunLoop, observer: &CFRunLoopObserver, mode: &CFString);

    pub fn CFRunLoopContainsTimer(
            rl: &CFRunLoop, timer: &CFRunLoopTimer, mode: &CFString)
            -> bool;

    pub fn CFRunLoopAddTimer(
            rl: &CFRunLoop, timer: &CFRunLoopTimer, mode: &CFString);

    pub fn CFRunLoopRemoveTimer(
            rl: &CFRunLoop, timer: &CFRunLoopTimer, mode: &CFString);

    pub fn CFRunLoopSourceGetTypeID() -> CFTypeID;

    pub fn CFRunLoopSourceCreate(
            allocator: Option<&'static CFAllocator>,
            order: CFIndex,
            context: *const c_void)
            -> *const CFShared<CFRunLoopSource>;

    pub fn CFRunLoopSourceGetOrder(source: &CFRunLoopSource) -> CFIndex;
    pub fn CFRunLoopSourceInvalidate(source: &CFRunLoopSource);
    pub fn CFRunLoopSourceIsValid(source: &CFRunLoopSource) -> bool;

    pub fn CFRunLoopSourceGetContext(
            source: &CFRunLoopSource, context: *mut c_void);

    pub fn CFRunLoopSourceSignal(source: &CFRunLoopSource);

    pub fn CFRunLoopObserverGetTypeID() -> CFTypeID;

    pub fn CFRunLoopObserverCreate(
            allocator: Option<&'static CFAllocator>,
            activities: CFRunLoopActivity,
            repeats: bool,
            order: CFIndex,
            callout: CFRunLoopObserverCallBack,
            context: Option<&CFRunLoopObserverContext>)
            -> *const CFShared<CFRunLoopObserver>;

    pub fn CFRunLoopObserverGetActivities(
            observer: &CFRunLoopObserver)
            -> CFRunLoopActivity;

    pub fn CFRunLoopObserverDoesRepeat(observer: &CFRunLoopObserver) -> bool;
    pub fn CFRunLoopObserverGetOrder(observer: CFRunLoopObserver) -> CFIndex;
    pub fn CFRunLoopObserverInvalidate(observer: CFRunLoopObserver);
    pub fn CFRunLoopObserverIsValid(observer: &CFRunLoopObserver) -> bool;

    pub fn CFRunLoopObserverGetContext(
            observer: &CFRunLoopObserver,
            context: &mut CFRunLoopObserverContext);

    pub fn CFRunLoopTimerGetTypeID() -> CFTypeID;

    pub fn CFRunLoopTimerCreate(
            allocator: Option<&'static CFAllocator>,
            fireDate: CFAbsoluteTime,
            interval: CFTimeInterval,
            flags: CFRunLoopTimerFlags,
            order: CFIndex,
            callout: CFRunLoopTimerCallBack,
            context: Option<&CFRunLoopTimerContext>)
            -> *const CFShared<CFRunLoopTimer>;

    pub fn CFRunLoopTimerGetNextFireDate(
            timer: &CFRunLoopTimer)
            -> CFAbsoluteTime;

    pub fn CFRunLoopTimerSetNextFireDate(
            timer: &CFRunLoopTimer, fireDate: CFAbsoluteTime);

    pub fn CFRunLoopTimerGetInterval(timer: &CFRunLoopTimer) -> CFTimeInterval;
    pub fn CFRunLoopTimerDoesRepeat(timer: &CFRunLoopTimer) -> bool;
    pub fn CFRunLoopTimerGetOrder(timer: &CFRunLoopTimer) -> CFIndex;
    pub fn CFRunLoopTimerInvalidate(timer: &CFRunLoopTimer);
    pub fn CFRunLoopTimerIsValid(timer: &CFRunLoopTimer) -> bool;

    pub fn CFRunLoopTimerGetContext(
            timer: &CFRunLoopTimer, context: &mut CFRunLoopTimerContext);
}
