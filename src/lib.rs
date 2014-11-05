// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "core_foundation"]
#![crate_type = "rlib"]

#![allow(non_snake_case)]

extern crate libc;

#[cfg(target_os="macos")]
pub mod array;
#[cfg(target_os="macos")]
pub mod base;
#[cfg(target_os="macos")]
pub mod boolean;
#[cfg(target_os="macos")]
pub mod data;
#[cfg(target_os="macos")]
pub mod dictionary;
#[cfg(target_os="macos")]
pub mod number;
#[cfg(target_os="macos")]
pub mod set;
#[cfg(target_os="macos")]
pub mod string;
#[cfg(target_os="macos")]
pub mod url;
#[cfg(target_os="macos")]
pub mod bundle;

#[cfg(all(target_os="macos", test))]
pub mod test {
    #[test]
    fn test_stuff() {
        use base::TCFType;
        use boolean::CFBoolean;
        use number::number;
        use dictionary::CFDictionary;
        use string::CFString;

        /*let n = CFNumber::new_number(42 as i32);
        io::println(format!("%d", (&n).retain_count() as int));
        (&n).show();*/

        let bar = CFString::from_static_string("Bar");
        let baz = CFString::from_static_string("Baz");
        let boo = CFString::from_static_string("Boo");
        let foo = CFString::from_static_string("Foo");
        let tru = CFBoolean::true_value();
        let n42 = number(42);

        let _d = CFDictionary::from_CFType_pairs([
            (bar.as_CFType(), boo.as_CFType()),
            (baz.as_CFType(), tru.as_CFType()),
            (foo.as_CFType(), n42.as_CFType()),
        ]);
    }
}
