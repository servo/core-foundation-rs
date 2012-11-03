use base::CFTypeOps;
use boolean::{CFBoolean, CFBooleanRef};
use number::{CFNumber, CFNumberRef};
use dictionary::CFDictionary;
use string::{CFString, CFStringRef};

fn main() {
    /*let n = CFNumber::new_number(42 as i32);
    io::println(fmt!("%d", (&n).retain_count() as int));
    (&n).show();*/
    let d = CFDictionary::new([
        (CFString::new_static("Foo"), base::as_CFType::<CFStringRef, CFString>(CFString::new_static("Bar"))),
        (CFString::new_static("Baz"), base::as_CFType::<CFBooleanRef, CFBoolean>(CFBoolean::true_value())),
        (CFString::new_static("Boo"), base::as_CFType::<CFNumberRef, CFNumber>(CFNumber::new(42 as i32))),
    ]);
    (&d).show();
}

