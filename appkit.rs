use base::objc;
use base;

#[nolink]
#[link_args="-framework AppKit"]
pub extern mod appkit {
    fn NSBeep();
}

pub fn NSApp() -> base::id {
    unsafe {
        let klass = str::as_c_str(~"NSApplication", |s| objc::objc_getClass(s));
        let sel = str::as_c_str(~"sharedApplication", |s| objc::sel_registerName(s));
        objc::objc_msgSend(klass, sel)
    }
}

