use base::CFTypeOps;
use boolean::CFBoolean;
use number::CFNumber;
use dictionary::CFDictionary;
use string::CFString;

fn main() {
    /*let n = CFNumber::new_number(42 as i32);
    io::println(fmt!("%d", (&n).retain_count() as int));
    (&n).show();*/
    let d = CFDictionary::new([
        (CFString::new_static("Foo"), (&CFString::new_static("Bar")).as_type()),
        (CFString::new_static("Baz"), (&CFBoolean::true_value()).as_type()),
        (CFString::new_static("Boo"), (&CFNumber::new(42 as i32)).as_type()),
    ]);
    (&d).show();
}

