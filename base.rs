// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// this file defines CGFloat, as well as stubbed data types.

use std::libc;

// TODO: this is actually a libc::c_float on 32bit
pub type CGFloat = libc::c_double;

pub type CGAffineTransform = ();

