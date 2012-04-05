import base::objc;

#[nolink]
#[link_args="-framework AppKit"]
native mod appkit {
    fn NSBeep();
}

fn NSApp() -> base::id {
    let klass = str::as_c_str("NSApplication") { |s| objc::objc_getClass(s) };
    let sel = str::as_c_str("sharedApplication") { |s| objc::sel_registerName(s) };
    ret objc::objc_msgSend(klass, sel);
}

