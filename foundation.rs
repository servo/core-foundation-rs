// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
    unsafe {
        let klass = str::as_c_str(~"NSAutoreleasePool", |s|
            objc::objc_getClass(s)
        );

        let alloc_sel = str::as_c_str(~"alloc", |s| objc::sel_registerName(s));
        let init_sel = str::as_c_str(~"init", |s| objc::sel_registerName(s));

        let pool = objc::objc_msgSend(klass, alloc_sel);
        objc::objc_msgSend(pool, init_sel)
    }
}

