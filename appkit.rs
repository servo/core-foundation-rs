use base::objc;
use base;

#[nolink]
#[link_args="-framework AppKit"]
pub extern mod appkit {
    fn NSBeep();
}

pub fn NSApp() -> base::id {
    let klass = str::as_c_str(~"NSApplication", |s| unsafe { objc::objc_getClass(s) } );
    let sel = str::as_c_str(~"sharedApplication", |s| unsafe { objc::sel_registerName(s) } );
    return unsafe { objc::objc_msgSend(klass, sel) };
}

