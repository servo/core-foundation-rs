// Copyright 2013-2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Binds the `CFString` type.

use allocator::CFAllocator;
use array::CFArray;
use base::{CFComparisonResult, CFDowncast, CFIndex, CFObject, CFOptionFlags};
use base::{CFRange, CFType, CFTypeID, FromCFIndex, IntoCFIndex};
use characterset::CFCharacterSet;
use data::CFData;
use dictionary::CFDictionary;
use locale::CFLocale;
use std::borrow::Cow;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::ops::Range;
use std::os::raw::{c_char, c_ulong};
use std::ptr;
use std::slice;
use std::str;
use std::string::ParseError;
use sync::{CFRef, CFShared};

pub type CFStringRef = CFRef<CFString>;

/// Encapsulates string values.
///
/// Unless stated otherwise, all lengths and ranges taken and returned by
/// methods on this type are expressed in terms of WTF-16 code units.
#[repr(C)]
pub struct CFString { obj: CFObject }

unsafe impl Send for CFString {}
unsafe impl Sync for CFString {}

unsafe impl CFType for CFString {
    #[inline]
    fn as_object(&self) -> &CFObject {
        &self.obj
    }
}

unsafe impl CFDowncast for CFString {
    #[inline]
    fn type_id() -> CFTypeID {
        unsafe { CFStringGetTypeID() }
    }
}

/// ## Creating a `CFString`
impl CFString {
    /// Creates a `CFString` from a WTF-16 slice.
    #[inline]
    pub fn from_slice(input: &[u16]) -> CFStringRef {
        unsafe {
            CFRef::from_retained(
                CFStringCreateWithCharacters(
                    None, input.as_ptr(), input.len().into_index()))
        }
    }

    /// Creates a `CFString` from a static WTF-16 slice.
    #[inline]
    pub fn from_static_slice(input: &'static [u16]) -> CFStringRef {
        unsafe {
            CFRef::from_retained(
                CFStringCreateWithCharactersNoCopy(
                    None,
                    input.as_ptr(),
                    input.len().into_index(),
                    Some(CFAllocator::null_allocator())))
        }
    }

    /// Creates a `CFString` from a `str` slice.
    #[inline]
    pub fn from_str(input: &str) -> CFStringRef {
        unsafe {
            CFRef::from_retained(
                CFStringCreateWithBytes(
                    None,
                    input.as_ptr(),
                    input.len().into_index(),
                    CFStringBuiltInEncodings::UTF8 as u32,
                    false))
        }
    }

    /// Creates a `CFString` from a static `str` slice.
    #[inline]
    pub fn from_static_str(input: &'static str) -> CFStringRef {
        unsafe {
            CFRef::from_retained(
                CFStringCreateWithBytesNoCopy(
                    None,
                    input.as_ptr(),
                    input.len().into_index(),
                    CFStringBuiltInEncodings::UTF8 as u32,
                    false,
                    Some(CFAllocator::null_allocator())))
        }
    }

    /// Duplicates this string.
    #[inline]
    pub fn duplicate(&self) -> CFStringRef {
        unsafe { CFRef::from_retained(CFStringCreateCopy(None, self)) }
    }

    /// Creates a substring of this string.
    #[inline]
    pub fn substring(&self, range: Range<usize>) -> CFStringRef {
        assert!(range.end <= self.len());
        unsafe {
            CFRef::from_retained(
                CFStringCreateWithSubstring(None, self, range.into()))
        }
    }
}

/// ## Accessing characters
impl CFString {
    /// Returns the length of the string in WTF-16 code units.
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { usize::from_index(CFStringGetLength(self)) }
    }

    /// Returns whether the string is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the string as a WTF-16 slice efficiently.
    ///
    /// This may return `None` if the string isn't internally stored as WTF-16.
    #[inline]
    pub fn to_slice(&self) -> Option<&[u16]> {
        let len = self.len();
        if len == 0 {
            return Some(&[]);
        }
        unsafe {
            let ptr = CFStringGetCharactersPtr(self);
            if !ptr.is_null() {
                Some(slice::from_raw_parts(ptr, len))
            } else {
                None
            }
        }
    }

    /// Returns the string as a WTF-16 `Cow` slice.
    ///
    /// This may return `Cow::Owned(vec)` if the string isn't internally
    /// stored as WTF-16.
    #[inline]
    pub fn to_cow(&self) -> Cow<[u16]> {
        if let Some(slice) = self.to_slice() {
            return slice.into();
        }
        unsafe {
            let len = self.len();
            let mut output = vec![0; len];
            CFStringGetCharacters(self, (0..len).into(), output.as_mut_ptr());
            output.into()
        }
    }

    /// Convenience method around `self.to_cow().into_owned()`.
    #[inline]
    pub fn to_vec(&self) -> Vec<u16> {
        self.to_cow().into_owned()
    }

    /// Returns the string as a `str` slice efficiently.
    ///
    /// This may return `None` if the string isn't internally stored
    /// as a Pascal string or if it doesn't actually represent valid UTF-8
    /// text to begin with.
    pub fn to_str(&self) -> Option<&str> {
        if self.len() == 0 {
            return Some("");
        }
        unsafe {
            // CFStringGetCStringPtr cannot be used here because it returns a
            // result even if the string itself includes a NUL, thus truncating
            // the result.
            let pstr_ptr = CFStringGetPascalStringPtr(
                self, CFStringBuiltInEncodings::UTF8 as u32);
            if !pstr_ptr.is_null() {
                let slice = slice::from_raw_parts(
                    pstr_ptr.offset(1), *pstr_ptr as usize);
                return Some(str::from_utf8_unchecked(slice));
            }
            None
        }
    }

    /// Returns the string as a `str` `Cow` slice.
    ///
    /// This may return `Ok(Cow::Owned(string))` if the string represents valid
    /// UTF-8 text but isn't internally stored as a Pascal string.
    ///
    /// The value `Err(())` is returned if the string couldn't be converted
    /// to UTF-8.
    pub fn to_cow_str(&self) -> Result<Cow<str>, ()> {
        if let Some(str) = self.to_str() {
            return Ok(str.into());
        }

        unsafe {
            let len = CFStringGetLength(self);
            let full_range = CFRange {
                location: 0,
                length: len,
            };

            let mut output_len = 0;
            let read_chars_count = CFStringGetBytes(
                self,
                full_range,
                CFStringBuiltInEncodings::UTF8 as u32,
                0,
                false,
                ptr::null_mut(),
                0,
                Some(&mut output_len));
            assert!(output_len >= 0);
            if read_chars_count != len {
                return Err(());
            }

            let mut output = vec![0; usize::from_index(output_len)];
            let mut used_output_len = 0;
            let write_chars_count = CFStringGetBytes(
                self,
                full_range,
                CFStringBuiltInEncodings::UTF8 as u32,
                0,
                false,
                output.as_mut_ptr(),
                output_len,
                Some(&mut used_output_len));
            assert_eq!(used_output_len, output_len);
            assert_eq!(write_chars_count, read_chars_count);

            Ok(String::from_utf8_unchecked(output).into())
        }
    }

    /// Convenience method around `self.to_cow_str().map(Cow::into_owned)`.
    #[inline]
    pub fn to_string(&self) -> Result<String, ()> {
        self.to_cow_str().map(Cow::into_owned)
    } 
}

/// ## Comparing strings
impl CFString {
    /// Compares this string with another.
    ///
    /// Also available as `<CFString as Ord>::cmp`.
    #[inline]
    pub fn cmp_with_options(
            &self, other: &Self, options: CFStringCompareFlags)
            -> Ordering {
        unsafe {
            CFStringCompare(self, other, options).into()
        }
    }

    /// Compares a range of the WTF-16 code units in this string with another.
    #[inline]
    pub fn cmp_with_options_in_range(
            &self,
            other: &Self,
            options: CFStringCompareFlags,
            range: Range<usize>)
            -> Ordering {
        assert!(range.end <= self.len());
        let result = unsafe {
            CFStringCompareWithOptions(self, other, range.into(), options)
        };
        result.into()
    }
}

/// ## Searching strings
impl CFString {
    /// Returns whether `needle` is contained in this string.
    #[inline]
    pub fn contains(
            &self, needle: &Self, options: CFStringCompareFlags)
            -> bool {
        self.find(needle, options).is_some()
    }

    /// Returns whether `needle` is contained in as specific range.
    #[inline]
    pub fn contains_in_range(
            &self,
            needle: &Self,
            options: CFStringCompareFlags,
            range: Range<usize>)
            -> bool {
        assert!(range.end <= self.len());
        unsafe {
            CFStringFindWithOptions(self, needle, range.into(), options, None)
        }
    }

    /// Finds `needle` in this string.
    #[inline]
    pub fn find(
            &self, needle: &Self, options: CFStringCompareFlags)
            -> Option<Range<usize>> {
        let result = unsafe { CFStringFind(self, needle, options) };
        if result.location >= 0 {
            Some(result.into())
        } else {
            None
        }
    }

    /// Finds `needle` in a specific range.
    #[inline]
    pub fn find_in_range(
            &self,
            needle: &CFString,
            options: CFStringCompareFlags,
            range: Range<usize>)
            -> Option<Range<usize>> {
        assert!(range.end <= self.len());
        let mut result = CFRange::default();
        let found = unsafe {
            CFStringFindWithOptions(
                self, needle, range.into(), options, Some(&mut result))
        };
        if found {
            Some(result.into())
        } else {
            None
        }
    }

    /// Returns whether this string starts with `needle`.
    #[inline]
    pub fn starts_with(&self, needle: &CFString) -> bool {
        unsafe { CFStringHasPrefix(self, needle) }
    }

    /// Returns whether this string ends with `needle`.
    #[inline]
    pub fn ends_with(&self, needle: &CFString) -> bool {
        unsafe { CFStringHasSuffix(self, needle) }
    }
}

/// ## Getting numeric values
impl CFString {
    /// Converts this string as a `i32` value.
    ///
    /// This method just returns 0 on a conversion error.
    #[inline]
    pub fn to_i32(&self) -> i32 {
        unsafe { CFStringGetIntValue(self) }
    }

    /// Converts this string as a `f64` value.
    ///
    /// This method just returns 0.0 on a conversion error.
    #[inline]
    pub fn to_f64(&self) -> f64 {
        unsafe { CFStringGetDoubleValue(self) }
    }
}

impl fmt::Debug for CFString {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut tuple = formatter.debug_tuple("CFString");
        if let Some(str) = self.to_str() {
            tuple.field(&str);
        } else if let Some(slice) = self.to_slice() {
            tuple.field(&slice);
        } else {
            tuple.field(&format_args!("[_; {}]", self.len()));
        }
        tuple.finish()
    }
}

impl<'a> From<&'a str> for CFStringRef {
    #[inline]
    fn from(input: &'a str) -> Self {
        CFString::from_str(input)
    }
}

impl str::FromStr for CFStringRef {
    type Err = ParseError;

    #[inline]
    fn from_str(input: &str) -> Result<Self, ParseError> {
        Ok(CFString::from_str(input))
    }
}

impl<'a> From<&'a [u16]> for CFStringRef {
    #[inline]
    fn from(input: &'a [u16]) -> Self {
        CFString::from_slice(input)
    }
}

impl<'a> From<&'a CFString> for Cow<'a, [u16]> {
    #[inline]
    fn from(input: &'a CFString) -> Self {
        input.to_cow()
    }
}

impl<'a> From<&'a CFString> for Vec<u16> {
    #[inline]
    fn from(input: &'a CFString) -> Self {
        input.to_vec()
    }
}

impl Eq for CFString {}

impl PartialEq for CFString {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Ord for CFString {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        unsafe {
            CFStringCompare(self, other, COMPARE_FORCED_ORDERING).into()
        }
    }
}

impl PartialOrd for CFString {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

bitflags! {
    #[repr(C)]
    pub flags CFStringCompareFlags: CFOptionFlags {
        const COMPARE_CASE_INSENSITIVE = 1,
        const COMPARE_BACKWARDS = 4,
        const COMPARE_ANCHORED = 8,
        const COMPARE_NON_LITERAL = 16,
        const COMPARE_LOCALIZED = 32,
        const COMPARE_NUMERICALLY = 64,
        const COMPARE_DIACRITIC_INSENSITIVE = 128,
        const COMPARE_WIDTH_INSENSITIVE = 256,
        const COMPARE_FORCED_ORDERING = 512,
    }
}

pub type CFStringEncoding = u32;

pub const kCFStringEncodingInvalidId: CFStringEncoding = 0xffffffff;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u32)]
pub enum CFStringBuiltInEncodings {
    MacRoman = 0,
    WindowsLatin1 = 0x0500,
    ISOLatin1 = 0x0201,
    NextStepLatin = 0x0B01,
    ASCII = 0x0600,
    UTF8 = 0x08000100,
    NonLossyASCII = 0x0BFF,
    UTF16 = 0x0100,
    UTF16BE = 0x10000100,
    UTF16LE = 0x14000100,
    UTF32 = 0x0c000100,
    UTF32BE = 0x18000100,
    UTF32LE = 0x1c000100,
}

#[test]
fn test_to_slice() {
    let empty = CFString::from_slice(&[] as &[_]);
    assert_eq!(empty.to_slice(), Some(&[] as &[_]));
}

#[test]
fn test_to_str() {
    let empty = CFString::from_str("");
    assert_eq!(empty.to_str(), Some(""));

    let fromage = CFString::from_str("fromage");
    assert_eq!(fromage.to_str(), Some("fromage"));

    // We don't use CFStringGetCStringPtr so that works.
    let nul = CFString::from_str("\0");
    assert_eq!(nul.to_str(), Some("\0"));

    // This doesn't fit a Pascal string with a byte length tag, so this is None.
    let nul_256_times = CFString::from_str(str::from_utf8(&[0; 256]).unwrap());
    assert_eq!(nul_256_times.to_str(), None);
}

#[test]
fn test_string_and_back() {
    let original = "The quick brown fox jumped over the slow lazy dog.";
    let cf = CFString::from_static_str(original);
    assert_eq!(cf.to_string(), Ok(original.to_owned()));
}

extern {
    pub fn CFStringGetTypeID() -> CFTypeID;

    pub fn CFStringCreateWithPascalString(
            allocator: Option<&'static CFAllocator>,
            pStr: *const u8,
            encoding: CFStringEncoding)
            -> *const CFShared<CFString>;

    pub fn CFStringCreateWithCString(
            allocator: Option<&'static CFAllocator>,
            cStr: *const c_char,
            encoding: CFStringEncoding)
            -> *const CFShared<CFString>;

    pub fn CFStringCreateWithBytes(
            allocator: Option<&'static CFAllocator>,
            bytes: *const u8,
            numBytes: CFIndex,
            encoding: CFStringEncoding,
            isExternalRepresentation: bool)
            -> *const CFShared<CFString>;

    pub fn CFStringCreateWithCharacters(
            allocator: Option<&'static CFAllocator>,
            chars: *const u16,
            numChars: CFIndex)
            -> *const CFShared<CFString>;

    pub fn CFStringCreateWithPascalStringNoCopy(
            allocator: Option<&'static CFAllocator>,
            pStr: *const u8,
            encoding: CFStringEncoding,
            contentsDeallocator: Option<&'static CFAllocator>)
            -> *const CFShared<CFString>;

    pub fn CFStringCreateWithCStringNoCopy(
            allocator: Option<&'static CFAllocator>,
            cStr: *const c_char,
            encoding: CFStringEncoding,
            contentsDeallocator: Option<&'static CFAllocator>)
            -> *const CFShared<CFString>;

    pub fn CFStringCreateWithBytesNoCopy(
            allocator: Option<&'static CFAllocator>,
            bytes: *const u8,
            numBytes: CFIndex,
            encoding: CFStringEncoding,
            isExternalRepresentation: bool,
            contentsDeallocator: Option<&'static CFAllocator>)
            -> *const CFShared<CFString>;

    pub fn CFStringCreateWithCharactersNoCopy(
            allocator: Option<&'static CFAllocator>,
            chars: *const u16,
            numChars: CFIndex,
            contentsDeallocator: Option<&'static CFAllocator>)
            -> *const CFShared<CFString>;

    pub fn CFStringCreateWithSubstring(
            allocator: Option<&'static CFAllocator>,
            str: &CFString,
            range: CFRange)
            -> *const CFShared<CFString>;

    pub fn CFStringCreateCopy(
            allocator: Option<&'static CFAllocator>, theString: &CFString)
            -> *const CFShared<CFString>;

    pub fn CFStringCreateWithFormat(
            allocator: Option<&'static CFAllocator>,
            formatOptions: &CFDictionary,
            format: &CFString,
            ...)
            -> *const CFShared<CFString>;

    pub fn CFStringGetLength(theString: &CFString) -> CFIndex;

    pub fn CFStringGetCharacterAtIndex(
            theString: &CFString, idx: CFIndex)
            -> u16;

    pub fn CFStringGetCharacters(
            theString: &CFString, range: CFRange, buffer: *mut u16);

    pub fn CFStringGetPascalString(
            theString: &CFString,
            buffer: *mut u8,
            bufferSize: CFIndex,
            encoding: CFStringEncoding)
            -> bool;

    pub fn CFStringGetCString(
            theString: &CFString,
            buffer: *mut c_char,
            bufferSize: CFIndex,
            encoding: CFStringEncoding)
            -> bool;

    pub fn CFStringGetPascalStringPtr(
            theString: &CFString, encoding: CFStringEncoding)
            -> *const u8;

    pub fn CFStringGetCStringPtr(
            theString: &CFString, encoding: CFStringEncoding)
            -> *const c_char;

    pub fn CFStringGetCharactersPtr(theString: &CFString) -> *const u16;

    pub fn CFStringGetBytes(
            theString: &CFString,
            range: CFRange,
            encoding: CFStringEncoding,
            lossByte: u8,
            isExternalRepresentation: bool,
            buffer: *const u8,
            maxBufLen: CFIndex,
            usedBufLen: Option<&mut CFIndex>)
            -> CFIndex;

    pub fn CFStringCreateFromExternalRepresentation(
            alloc: Option<&'static CFAllocator>,
            data: &CFData,
            encoding: CFStringEncoding)
            -> *const CFShared<CFString>;

    pub fn CFStringCreateExternalRepresentation(
            alloc: Option<&'static CFAllocator>,
            theString: &CFString,
            encoding: CFStringEncoding,
            lossByte: u8)
            -> *const CFShared<CFData>;

    pub fn CFStringGetSmallestEncoding(
            theString: &CFString)
            -> CFStringEncoding;

    pub fn CFStringGetFastestEncoding(theString: &CFString) -> CFStringEncoding;
    pub fn CFStringGetSystemEncoding() -> CFStringEncoding;

    pub fn CFStringGetMaximumSizeForEncoding(
            length: CFIndex, encoding: CFStringEncoding)
            -> CFIndex;

    pub fn CFStringGetFileSystemRepresentation(
            string: &CFString, buffer: *mut c_char, maxBufLen: CFIndex)
            -> bool;

    pub fn CFStringGetMaximumSizeOfFileSystemRepresentation(
            string: &CFString)
            -> CFIndex;

    pub fn CFStringCreateWithFileSystemRepresentation(
            alloc: Option<&'static CFAllocator>,
            buffer: *const c_char)
            -> *const CFShared<CFString>;

    pub fn CFStringCompareWithOptionsAndLocale(
            theString1: &CFString,
            theString2: &CFString,
            rangeToCompare: CFRange,
            compareOptions: CFStringCompareFlags,
            locale: Option<&CFLocale>)
            -> CFComparisonResult;

    pub fn CFStringCompareWithOptions(
            theString1: &CFString,
            theString2: &CFString,
            rangeToCompare: CFRange,
            compareOptions: CFStringCompareFlags)
            -> CFComparisonResult;

    pub fn CFStringCompare(
            theString1: &CFString,
            theString2: &CFString,
            compareOptions: CFStringCompareFlags)
            -> CFComparisonResult;

    pub fn CFStringFindWithOptionsAndLocale(
            theString: &CFString,
            stringToFind: &CFString,
            rangeToSearch: CFRange,
            searchOptions: CFStringCompareFlags,
            locale: &CFLocale,
            result: Option<&mut CFRange>)
            -> bool;

    pub fn CFStringFindWithOptions(
            theString: &CFString,
            stringToFind: &CFString,
            rangeToSearch: CFRange,
            searchOptions: CFStringCompareFlags,
            result: Option<&mut CFRange>)
            -> bool;

    pub fn CFStringCreateArrayWithFindResults(
            allocator: Option<&'static CFAllocator>,
            theString: &CFString,
            stringToFind: &CFString,
            rangeToSearch: CFRange,
            compareOptions: CFStringCompareFlags)
            -> *const CFShared<CFArray>;

    pub fn CFStringFind(
            theString: &CFString,
            stringToFind: &CFString,
            compareOptions: CFStringCompareFlags)
            -> CFRange;

    pub fn CFStringHasPrefix(theString: &CFString, prefix: &CFString) -> bool;
    pub fn CFStringHasSuffix(theString: &CFString, suffix: &CFString) -> bool;

    pub fn CFStringGetRangeOfComposedCharactersAtIndex(
            theString: &CFString,
            theIndex: CFIndex)
            -> CFRange;

    pub fn CFStringFindCharacterFromSet(
            theString: &CFString,
            theSet: &CFCharacterSet,
            rangeToSearch: CFRange,
            searchOptions: CFStringCompareFlags,
            result: &mut CFRange)
            -> bool;

    pub fn CFStringGetLineBounds(
            theString: &CFString,
            range: CFRange,
            lineBeginIndex: &mut CFIndex,
            lineEndIndex: &mut CFIndex,
            contentsEndIndex: &mut CFIndex);

    pub fn CFStringGetParagraphBounds(
            theString: &CFString,
            range: CFRange,
            parBeginIndex: &mut CFIndex,
            parEndIndex: &mut CFIndex,
            contentsEndIndex: &mut CFIndex);

    pub fn CFStringGetHyphenationLocationBeforeIndex(
            string: &CFString,
            location: CFIndex,
            limitRange: CFRange,
            options: CFOptionFlags,
            locale: &CFLocale,
            character: Option<&mut u32>)
            -> CFIndex;

    pub fn CFStringIsHyphenationAvailableForLocale(locale: &CFLocale) -> bool;

    pub fn CFStringCreateByCombiningStrings(
            alloc: Option<&'static CFAllocator>,
            theArray: &CFArray,
            separatorString: &CFString)
            -> *const CFShared<CFString>;

    pub fn CFStringCreateArrayBySeparatingStrings(
            alloc: Option<&'static CFAllocator>,
            theString: &CFString,
            separatorString: &CFString)
            -> *const CFShared<CFArray>;

    pub fn CFStringGetIntValue(str: &CFString) -> i32;
    pub fn CFStringGetDoubleValue(str: &CFString) -> f64;
    pub fn CFStringIsEncodingAvailable(encoding: CFStringEncoding) -> bool;
    pub fn CFStringGetListOfAvailableEncodings() -> *const CFStringEncoding;

    pub fn CFStringGetNameOfEncoding(
            encoding: CFStringEncoding)
            -> *const CFShared<CFString>;

    pub fn CFStringConvertEncodingToNSStringEncoding(
            encoding: CFStringEncoding)
            -> c_ulong;

    pub fn CFStringConvertNSStringEncodingToEncoding(
            encoding: c_ulong)
            -> CFStringEncoding;

    pub fn CFStringConvertEncodingToWindowsCodepage(
            encoding: CFStringEncoding)
            -> u32;

    pub fn CFStringConvertWindowsCodepageToEncoding(
            codepage: u32)
            -> CFStringEncoding;

    pub fn CFStringConvertIANACharSetNameToEncoding(
            theString: &CFString)
            -> CFStringEncoding;

    pub fn CFStringConvertEncodingToIANACharSetName(
            encoding: CFStringEncoding)
            -> *const CFShared<CFString>;

    pub fn CFStringGetMostCompatibleMacStringEncoding(
            encoding: CFStringEncoding)
            -> CFStringEncoding;
}
