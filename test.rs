// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use boolean::CFBoolean;
use number::CFNumber;
use dictionary::CFDictionary;
use string::CFString;

fn main() {
    /*let n = CFNumber::new_number(42 as i32);
    io::println(fmt!("%d", (&n).retain_count() as int));
    (&n).show();*/

    let bar = CFString::new_static("Bar");
    let baz = CFString::new_static("Baz");
    let boo = CFString::new_static("Boo");
    let foo = CFString::new_static("Foo");
    let tru = CFBoolean::true_value();
    let n42 = CFNumber::new(42 as i32);

    let d = CFDictionary::new([
        (*bar.contents.borrow_ref(), *boo.contents.borrow_type_ref()),
        (*baz.contents.borrow_ref(), *tru.contents.borrow_type_ref()),
        (*foo.contents.borrow_ref(), *n42.contents.borrow_type_ref()),
    ]);
    d.contents.show();
}
