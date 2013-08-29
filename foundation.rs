// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base;

#[nolink]
#[link_args="-framework Foundation"]
extern {
}

enum NSAutoreleasePool {
    NSAutoreleasePool_priv(base::id)
}

#[fixed_stack_segment]
fn NSAutoreleasePool() -> base::id {
    unsafe {
        let klass = do "NSAutoreleasePool".to_c_str().with_ref |s| {
            base::objc_getClass(s)
        };

        let alloc_sel = do "alloc".to_c_str().with_ref |s| { base::sel_registerName(s) };
        let init_sel = do "init".to_c_str().with_ref |s| { base::sel_registerName(s) };

        let pool = base::objc_msgSend(klass, alloc_sel);
        base::objc_msgSend(pool, init_sel)
    }
}

