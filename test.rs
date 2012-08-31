use base::CFTypeOps;
use dictionary::CFDictionary;
use string::CFString;

fn main() {
    let d = CFDictionary::new_dictionary([
        (CFString::new_static("Foo"), CFString::new_static("Bar")),
        (CFString::new_static("Baz"), CFString::new_static("Boo"))
    ]);
    (&d).show();
}

