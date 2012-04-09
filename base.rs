use std;

type id = libc::intptr_t;
type SEL = libc::intptr_t;

const nil : id = 0 as id;

native mod objc {
    fn objc_getClass(name : *libc::c_char) -> id;
    fn objc_msgSend(theReceiver : id, theSelector : SEL) -> id;
    fn sel_registerName(name : *libc::c_char) -> SEL;
}

#[test]
fn test_nsapp() {
    let klass = str::as_c_str("NSApplication") { |s|
        objc::objc_getClass(s)
    };

    let sel = str::as_c_str("sharedApplication") { |s|
        objc::sel_registerName(s)
    };

    let nsapp = objc::objc_msgSend(klass, sel);

    io::println(#fmt("nsapp: %d", (nsapp as int)));
}

