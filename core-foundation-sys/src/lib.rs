// Copyright 2013-2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Bindings to the Core Foundation framework.

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[macro_use]
extern crate bitflags;

pub mod allocator;
pub mod array;
pub mod bag;
pub mod base;
pub mod boolean;
pub mod bundle;
pub mod characterset;
pub mod data;
pub mod dictionary;
pub mod error;
pub mod locale;
pub mod null;
pub mod number;
pub mod plugin;
pub mod runloop;
pub mod set;
pub mod string;
pub mod sync;
pub mod time;
pub mod url;
pub mod version;

pub use allocator::{CFAllocator, CFAllocatorRef};
pub use array::{CFArray, CFArrayRef};
pub use bag::{CFBag, CFBagRef};
pub use base::{CFObject, CFObjectRef, CFDowncast, CFType, CFTypeID};
pub use boolean::{CFBoolean, CFBooleanRef};
pub use bundle::{CFBundle, CFBundleRef};
pub use characterset::{CFCharacterSet, CFCharacterSetRef};
pub use data::{CFData, CFDataRef};
pub use dictionary::{CFDictionary, CFDictionaryRef};
pub use error::{CFError, CFErrorRef};
pub use locale::{CFLocale, CFLocaleRef};
pub use null::{CFNull, CFNullRef};
pub use number::{CFNumber, CFNumberRef};
pub use plugin::{CFPlugIn, CFPlugInRef};
pub use runloop::{CFRunLoop, CFRunLoopRef};
pub use runloop::{CFRunLoopObserver, CFRunLoopObserverRef};
pub use runloop::{CFRunLoopSource, CFRunLoopSourceRef};
pub use runloop::{CFRunLoopTimer, CFRunLoopTimerRef};
pub use set::{CFSet, CFSetRef};
pub use string::{CFString, CFStringCompareFlags};
pub use sync::{CFRef, CFShared};
pub use time::{CFAbsoluteTime, CFTimeInterval};
pub use url::{CFURL, CFURLRef};

#[cfg(test)]
mod test {
    #[test]
    fn test_stuff() {
        use boolean::CFBoolean;
        use number::CFNumber;
        use dictionary::CFDictionary;
        use string::CFString;

        let bar = CFString::from_static_str("Bar");
        let baz = CFString::from_static_str("Baz");
        let boo = CFString::from_static_str("Boo");
        let foo = CFString::from_static_str("Foo");
        let tru = CFBoolean::new(true);
        let n42 = CFNumber::from_i64(42);

        let keys = &[
            bar.as_shared_object(),
            baz.as_shared_object(),
            foo.as_shared_object(),
        ];

        let values = &[
            boo.as_shared_object(),
            tru.as_shared_object(),
            n42.as_shared_object(),
        ];

        let _d = CFDictionary::from_objects(keys, values);
    }
}
