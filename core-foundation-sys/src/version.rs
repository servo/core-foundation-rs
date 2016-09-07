// Copyright 2013-2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Structures for the various versioned types throughout CF.

use base::CFIndex;

/// Represents version 0.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct CFVersion0 { version: CFIndex }

impl CFVersion0 {
	/// Creates a new `CFVersion0`.
	#[inline]
	pub fn new() -> Self {
		CF_VERSION_0
	}
}

impl Default for CFVersion0 {
	#[inline]
    fn default() -> Self {
        CF_VERSION_0
    }
}

/// Constant for version 0, to be used in constant expressions.
pub const CF_VERSION_0: CFVersion0 = CFVersion0 { version: 0 };

/// Represents version 1.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct CFVersion1 { version: CFIndex }

impl CFVersion1 {
	/// Creates a new `CFVersion1`.
	#[inline]
	pub fn new() -> Self {
		CF_VERSION_1
	}
}

/// Constant for version 1, to be used in constant expressions.
pub const CF_VERSION_1: CFVersion1 = CFVersion1 { version: 1 };

impl Default for CFVersion1 {
	#[inline]
    fn default() -> Self {
        CF_VERSION_1
    }
}
