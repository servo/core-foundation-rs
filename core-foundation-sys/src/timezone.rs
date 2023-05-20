// Copyright 2013-2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

use base::{CFAllocatorRef, CFTypeID, Boolean};
use date::{CFTimeInterval, CFAbsoluteTime};
use string::CFStringRef;
use array::CFArrayRef;
use dictionary::CFDictionaryRef;
use data::CFDataRef;

#[repr(C)]
pub struct __CFTimeZone(c_void);

pub type CFTimeZoneRef = *const __CFTimeZone;

extern {
    //pub static kCFTimeZoneSystemTimeZoneDidChangeNotification: CFNotificationName;

    /* Creating a Time Zone */
    pub fn CFTimeZoneCreate(allocator: CFAllocatorRef, name: CFStringRef, data: CFDataRef) -> CFTimeZoneRef;
    pub fn CFTimeZoneCreateWithName(allocator: CFAllocatorRef, name: CFStringRef, tryAbbrev: Boolean) -> CFTimeZoneRef;
    pub fn CFTimeZoneCreateWithTimeIntervalFromGMT(allocator: CFAllocatorRef, interval: CFTimeInterval) -> CFTimeZoneRef;

    /* System and Default Time Zones and Information */
    pub fn CFTimeZoneCopyAbbreviationDictionary() -> CFDictionaryRef;
    pub fn CFTimeZoneCopyAbbreviation(tz: CFTimeZoneRef, at: CFAbsoluteTime) -> CFStringRef;
    pub fn CFTimeZoneCopyDefault() -> CFTimeZoneRef;
    pub fn CFTimeZoneCopySystem() -> CFTimeZoneRef;
    pub fn CFTimeZoneSetDefault(tz: CFTimeZoneRef);
    pub fn CFTimeZoneCopyKnownNames() -> CFArrayRef;
    pub fn CFTimeZoneResetSystem();
    pub fn CFTimeZoneSetAbbreviationDictionary(dict: CFDictionaryRef);

    /* Getting Information About Time Zones */
    pub fn CFTimeZoneGetName(tz: CFTimeZoneRef) -> CFStringRef;
    //pub fn CFTimeZoneCopyLocalizedName(tz: CFTimeZoneRef, style: CFTimeZoneNameStyle, locale: CFLocaleRef) -> CFStringRef;
    pub fn CFTimeZoneGetSecondsFromGMT(tz: CFTimeZoneRef, time: CFAbsoluteTime) -> CFTimeInterval;
    pub fn CFTimeZoneGetData(tz: CFTimeZoneRef) -> CFDataRef;

    /* Getting Daylight Savings Time Information */
    pub fn CFTimeZoneIsDaylightSavingTime(tz: CFTimeZoneRef, at: CFAbsoluteTime) -> Boolean;
    pub fn CFTimeZoneGetDaylightSavingTimeOffset(tz: CFTimeZoneRef, at: CFAbsoluteTime) -> CFTimeInterval;
    pub fn CFTimeZoneGetNextDaylightSavingTimeTransition(tz: CFTimeZoneRef, at: CFAbsoluteTime) -> CFAbsoluteTime;

    /* Getting the CFTimeZone Type ID */
    pub fn CFTimeZoneGetTypeID() -> CFTypeID;
}
