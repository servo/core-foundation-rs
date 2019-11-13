// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Core Foundation byte buffers.

pub use core_foundation_sys::data::*;
use core_foundation_sys::base::CFIndex;
use core_foundation_sys::base::{kCFAllocatorDefault};
use std::iter::{Extend, FromIterator};
use std::ops::{Deref, DerefMut};
use std::slice;

use base::{CFIndexConvertible, CFRange, TCFType};

declare_TCFType!{
    /// An immutable byte buffer.
    CFData, CFDataRef
}
impl_TCFType!(CFData, CFDataRef, CFDataGetTypeID);
impl_CFTypeDescription!(CFData);

impl CFData {
    /// Create a `CFData` object with data copied from a specified byte buffer.
    pub fn from_buffer(buffer: &[u8]) -> CFData {
        unsafe {
            let data_ref = CFDataCreate(kCFAllocatorDefault,
                                        buffer.as_ptr(),
                                        buffer.len().to_CFIndex());
            TCFType::wrap_under_create_rule(data_ref)
        }
    }

    /// Returns an immutable pointer to the underlying bytes of this byte buffer.
    #[inline]
    pub fn bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(CFDataGetBytePtr(self.0), self.len() as usize)
        }
    }

    /// Returns the length of this byte buffer.
    #[inline]
    pub fn len(&self) -> CFIndex {
        unsafe {
            CFDataGetLength(self.0)
        }
    }
}

impl Deref for CFData {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.bytes()
    }
}

pub struct IntoIter {
    data: CFData,
    index: usize,
    len: usize
}

impl Iterator for IntoIter {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<u8> {
        let index = self.index;

        if index<self.len {
            self.index += 1;
            Some(self.data[index])
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remain_len = self.len-self.index;
        (remain_len, Some(remain_len))
    }
}

impl ExactSizeIterator for IntoIter {}

impl IntoIterator for CFData {
    type Item = u8;
    type IntoIter = IntoIter;

    fn into_iter(self) -> IntoIter {
        let data_len = self.len() as usize;

        IntoIter {
            data: self,
            index: 0,
            len: data_len
        }
    }
}

declare_TCFType!{
    /// A mutable byte buffer.
    CFMutableData, CFMutableDataRef
}
impl_TCFType!(CFMutableData, CFMutableDataRef, CFDataGetTypeID);
impl_CFTypeDescription!(CFMutableData);

impl CFMutableData {
    /// Create an empty, growable `CFMutableData` object.
    pub fn new() -> CFMutableData {
        Self::with_capacity(0)
    }

    /// Create an empty `CFMutableData` object with given capacity.
    /// If capacity is 0, the new `CFMutableData` is growable and has infinite capacity,
    /// otherwise the length of `CFMutableData` is limited by the capacity specified here.
    pub fn with_capacity(capacity: CFIndex) -> CFMutableData {
        unsafe {
            let data_ref = CFDataCreateMutable(kCFAllocatorDefault, capacity);
            TCFType::wrap_under_create_rule(data_ref)
        }
    }

    /// Create a `CFMutableData` object with data copied from a specified byte buffer.
    pub fn from_buffer(buffer: &[u8], fixed: bool) -> CFMutableData {
        let capacity = if fixed { buffer.len().to_CFIndex() } else { 0 };
        let mut data = Self::with_capacity(capacity);
        data.append(buffer);

        data
    }

    /// Returns a `CFData` pointing to the same underlying data as this mutable one.
    #[inline]
    pub fn to_immutable(&self) -> CFData {
        unsafe { CFData::wrap_under_get_rule(self.0) }
    }

    // Immutable interface

    /// Returns an immutable pointer to the underlying bytes of this byte buffer.
    #[inline]
    pub fn bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(CFDataGetBytePtr(self.0), self.len() as usize)
        }
    }

    /// Returns the length of this byte buffer.
    #[inline]
    pub fn len(&self) -> CFIndex {
        unsafe { CFDataGetLength(self.0) }
    }

    // Mutable interface
    /// Returns a mutable pointer to the underlying bytes of this byte buffer.
    #[inline]
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(CFDataGetMutableBytePtr(self.0),
                                      self.len() as usize)
        }
    }

    /// Increases the length of a `CFMutableData`'s internal byte buffer,
    /// zero-filling the extension to the buffer.
    #[inline]
    pub fn increase_len(&mut self, extra_len: CFIndex) {
        unsafe { CFDataIncreaseLength(self.0, extra_len) }
    }

    /// Resets the length of a `CFMutableData`'s internal byte buffer.
    #[inline]
    pub fn set_len(&mut self, len: CFIndex) {
        unsafe { CFDataSetLength(self.0, len) }
    }

    /// Appends the bytes from a byte buffer to the contents of a `CFMutableData`.
    #[inline]
    pub fn append(&mut self, other: &[u8]) {
        unsafe { CFDataAppendBytes(self.0, other.as_ptr(), other.len().to_CFIndex()) }
    }

    // TODO: Consider replacing panic with some kind of Error?
    fn check_range(&self, range: CFRange) {
        let data_len = self.len();
        let range_start = range.location;
        let range_end = range_start+range.length;

        if range_start>=data_len || range_end>data_len {
            panic!("invalid range ({}, {}) for CFData of length {}",
                   range_start, range_end, data_len);
        }
    }

    /// Deletes the bytes in a `CFMutableData` within a specified range.
    #[inline]
    pub fn delete(&mut self, range: CFRange) {
        self.check_range(range);
        unsafe { CFDataDeleteBytes(self.0, range) }
    }

    /// Replaces bytes in a `CFMutableData` within a specified range with other bytes.
    #[inline]
    pub fn replace(&mut self, range: CFRange, replace_with: &[u8]) {
        self.check_range(range);
        unsafe {
            CFDataReplaceBytes(self.0,
                               range,
                               replace_with.as_ptr(),
                               replace_with.len().to_CFIndex())
        }
    }
}

impl Deref for CFMutableData {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.bytes()
    }
}

impl DerefMut for CFMutableData {
    #[inline]
    fn deref_mut(&mut self) -> &mut [u8] {
        self.bytes_mut()
    }
}

impl IntoIterator for CFMutableData {
    type Item = u8;
    type IntoIter = IntoIter;

    fn into_iter(self) -> IntoIter {
        let data_len = self.len() as usize;

        IntoIter {
            data: self.to_immutable(),
            index: 0,
            len: data_len
        }
    }
}

impl Extend<u8> for CFMutableData {
    fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        let mut iter = iter.into_iter();
        let mut prev_len = self.len() as usize;
        let mut final_len: Option<usize> = None;

        loop {
            // TODO: Proper use of size hint
            // Hint on remaining length of the iterator
            let (mut size_hint, _) = iter.size_hint();
            if size_hint==0 {
                size_hint = 1;
            }
            // Increase the length of the buffer to hold new bytes
            self.increase_len(size_hint.to_CFIndex());

            let bytes = self.bytes_mut();
            // Insert new bytes into the buffer
            for i in 0..size_hint {
                match iter.next() {
                    Some(byte) => {
                        bytes[prev_len+i] = byte;
                    }
                    // Iterator is fully consumed
                    None => {
                        final_len = Some(prev_len+i);
                        break;
                    }
                }
            }

            match final_len {
                Some(_) => { break; }
                // Update previous buffer length if iterator isn't fully consumed
                None => {
                    prev_len += size_hint;
                }
            }
        }

        // Set the length of the buffer
        self.set_len(final_len.unwrap() as CFIndex);
    }
}

impl FromIterator<u8> for CFMutableData {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut data = Self::new();
        data.extend(iter);

        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::from_fn;

    #[test]
    fn data_basics() {
        let raw_data = [2, 5, 11, 13];
        let d = CFData::from_buffer(&raw_data);

        assert_eq!(d.len() as usize, raw_data.len());
        assert_eq!(d.bytes(), &raw_data);
        assert_eq!(&d[..2], &[2, 5]);
        assert_eq!(d[3], 13);
    }

    #[test]
    fn mutable_data_basics() {
        let raw_data = [3, 17, 100, 255, 202];
        let mut d1 = CFMutableData::from_buffer(&raw_data, true);

        assert_eq!(d1.len() as usize, raw_data.len());
        assert_eq!(d1.bytes(), &raw_data);
        assert_eq!(&d1[1..4], &[17, 100, 255]);
        assert_eq!(d1[0], 3);

        d1[0] += 5;                                 // d1: [8, 17, 100, 255, 202]
        assert_eq!(d1[0], 8);

        let mut_slice = d1.bytes_mut();
        mut_slice[3..].copy_from_slice(&[1, 2]);    // d1: [8, 17, 100, 1, 2]
        let d2 = d1.to_immutable();
        assert_eq!(d2[4], 2);
    }

    #[test]
    fn mutable_data_adv_mutation() {
        let mut d1 = CFMutableData::new();
        let mut d2 = CFMutableData::with_capacity(4);
        let mut d3 = CFMutableData::from_buffer(&[2, 4, 6, 8], true);
        let mut d4 = CFMutableData::from_buffer(&[1, 3, 5, 7], false);

        // CFMutableData::increase_len
        d1.increase_len(5);
        d4.increase_len(2);
        assert_eq!(d1.bytes(), &[0; 5]);
        assert_eq!(d4.bytes(), &[1, 3, 5, 7, 0, 0]);

        // CFMutableData::append
        d2.append(&d3);
        d4.append(&[9, 11]);
        assert_eq!(d2.bytes(), &[2, 4, 6, 8]);
        assert_eq!(d4.bytes(), &[1, 3, 5, 7, 0, 0, 9, 11]);

        // CFMutableData::set_len
        d1[4] = 1;
        d1.set_len(7);
        d3.set_len(3); 
        assert_eq!(d1.bytes(), &[0, 0, 0, 0, 1, 0, 0]);
        assert_eq!(d3.bytes(), &[2, 4, 6]);

        // CFMutableData::delete
        d3.delete(CFRange::init(1, 1));
        d4.delete(CFRange::init(4, 2));
        assert_eq!(d3.bytes(), &[2, 6]);
        assert_eq!(d4.bytes(), &[1, 3, 5, 7, 9, 11]);

        // CFMutableData::replace
        d1.replace(CFRange::init(0, 4), &[8]);
        d2.replace(CFRange::init(1, 2), &[3, 5]);
        d3.replace(CFRange::init(1, 1), &[5, 7, 9]);
        assert_eq!(d1.bytes(), &[8, 1, 0, 0]);
        assert_eq!(d2.bytes(), &[2, 3, 5, 8]);
        assert_eq!(d3.bytes(), &[2, 5, 7, 9]);
    }

    #[test]
    fn data_into_iter() {
        let d1 = CFData::from_buffer(&[98, 97, 100]);
        let mut d2 = CFMutableData::from_buffer(&[1, 4, 9], false);
        d2.increase_len(2);

        let s: String = d1.into_iter().map(char::from).collect();
        let v: Vec<u8> = d2.into_iter().collect();
        assert_eq!(&s, "bad");
        assert_eq!(&v, &[1, 4, 9, 0, 0]);
    }

    #[test]
    fn mutable_data_extend_from_iter() {
        let mut n = 1;
        let mut d1 = CFMutableData::with_capacity(6);
        let mut d2 = CFMutableData::from_buffer(&[1, 3, 5], false);

        // <CFMutableData as Extend<u8>>::extend
        d1.extend(from_fn(move || {
            n *= 2;

            if n<40 {
                Some(n)
            } else {
                None
            }
        }));                                    // d1: [2, 4, 8, 16, 32]
        d2.extend((1..6).map(|x| x*2));         // d2: [1, 3, 5, 2, 4, 6, 8, 10]

        assert_eq!(d1.bytes(), &[2, 4, 8, 16, 32]);
        assert_eq!(d2.bytes(), &[1, 3, 5, 2, 4, 6, 8, 10]);

        // <CFMutableData as FromIterator<u8>>::from_iter
        let d3: CFMutableData = (4..26).filter_map(|x| {
            if x%5==0 {
                Some(x-1)
            } else {
                None
            }
        }).collect();                           // d3: [4, 9, 14, 19, 24]

        assert_eq!(d3.bytes(), &[4, 9, 14, 19, 24]);
    }
}
