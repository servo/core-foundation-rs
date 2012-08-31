use base::CFTypeOps;
use boolean::CFBoolean;
use dictionary::CFDictionary;
use string::CFString;

fn main() {
    let d = CFDictionary::new_dictionary([
        (CFString::new_static("Foo"), CFString::new_static("Bar").as_type()),
        (CFString::new_static("Baz"), CFBoolean::true_value().as_type())
    ]);
    (&d).show();
}

