// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base::CGFloat;

pub struct CGSize {
    width: CGFloat,
    height: CGFloat,
}

pub struct CGPoint {
    x: CGFloat,
    y: CGFloat,
}

pub struct CGRect {
    origin: CGPoint,
    size: CGSize
}
