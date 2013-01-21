use base::objc;
use base;

#[nolink]
#[link_args="-framework Foundation"]
extern mod foundation {
}

enum NSAutoreleasePool {
    NSAutoreleasePool_priv(base::id)
}

fn NSAutoreleasePool() -> base::id {
    let klass = str::as_c_str(~"NSAutoreleasePool", |s|
        objc::objc_getClass(s)
    );

    let alloc_sel = str::as_c_str(~"alloc", |s| objc::sel_registerName(s));
    let init_sel = str::as_c_str(~"init", |s| objc::sel_registerName(s));

    let pool = objc::objc_msgSend(klass, alloc_sel);
    return objc::objc_msgSend(pool, init_sel);
}

