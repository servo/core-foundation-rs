// Copyright 2013-2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use allocator::CFAllocator;
use array::CFArray;
use base::{CFDowncast, CFIndex, CFObject, CFOptionFlags, CFRange, CFType};
use base::{CFTypeID, FromCFIndex, IntoCFIndex};
use data::CFData;
use dictionary::CFDictionary;
use error::{CFError, CFErrorRef};
use std::ops::Range;
use std::os::raw::c_void;
use std::ptr;
use string::{CFString, CFStringBuiltInEncodings, CFStringEncoding, CFStringRef};
use sync::{CFShared, CFRef};

pub type CFURLRef = CFRef<CFURL>;

#[repr(C)]
pub struct CFURL { obj: CFObject }

unsafe impl Send for CFURL {}
unsafe impl Sync for CFURL {}

unsafe impl CFType for CFURL {
    #[inline]
    fn as_object(&self) -> &CFObject {
        &self.obj
    }
}

unsafe impl CFDowncast for CFURL {
    #[inline]
    fn type_id() -> CFTypeID {
        unsafe { CFURLGetTypeID() }
    }
}

impl CFURL {
    #[inline]
    pub fn from_str(
            input: &str, base: Option<&CFURL>)
            -> Result<CFURLRef, ()> {
        unsafe {
            CFRef::try_from_retained(
                CFURLCreateWithBytes(
                    None,
                    input.as_bytes().as_ptr(),
                    input.len().into_index(),
                    CFStringBuiltInEncodings::UTF8 as u32,
                    base))
        }
    }

    #[inline]
    pub fn from_string(
            input: &CFString, base: Option<&CFURL>)
            -> Result<CFURLRef, ()> {
        unsafe {
            CFRef::try_from_retained(CFURLCreateWithString(None, input, base))
        }
    }

    #[inline]
    pub fn from_file_system_path(
            path: &CFString,
            path_style: CFURLPathStyle,
            is_directory: bool)
            -> Result<CFURLRef, ()> {
        unsafe {
            CFRef::try_from_retained(
                CFURLCreateWithFileSystemPath(
                    None, path, path_style, is_directory))
        }
    }

    #[inline]
    pub fn to_absolute(&self) -> Result<CFURLRef, ()> {
        unsafe {
            CFRef::try_from_retained(CFURLCopyAbsoluteURL(self))
        }
    }

    #[inline]
    pub fn as_string(&self) -> &CFShared<CFString> {
        unsafe { CFURLGetString(self).unwrap() }
    }

    #[inline]
    pub fn base(&self) -> Option<&CFShared<CFURL>> {
        unsafe { CFURLGetBaseURL(self) }
    }

    #[inline]
    pub fn can_be_decomposed(&self) -> bool {
        unsafe { CFURLCanBeDecomposed(self) }
    }

    #[inline]
    pub fn scheme(&self) -> Option<CFStringRef> {
        unsafe { CFRef::try_from_retained(CFURLCopyScheme(self)).ok() }
    }

    #[inline]
    pub fn net_location(&self) -> Option<CFStringRef> {
        unsafe { CFRef::try_from_retained(CFURLCopyNetLocation(self)).ok() }
    }

    #[inline]
    pub fn path(&self) -> Option<CFStringRef> {
        unsafe { CFRef::try_from_retained(CFURLCopyPath(self)).ok() }
    }

    #[inline]
    pub fn strict_path(&self) -> (bool, Option<CFStringRef>) {
        unsafe {
            let mut is_absolute = false;
            let result =
                CFRef::try_from_retained(
                    CFURLCopyStrictPath(self, Some(&mut is_absolute)));
            (is_absolute, result.ok())
        }
    }

    #[inline]
    pub fn file_system_path(
            &self, path_style: CFURLPathStyle)
            -> Option<CFStringRef> {
        unsafe {
            let result = CFRef::try_from_retained(
                CFURLCopyFileSystemPath(self, path_style));
            result.ok()
        }
    }

    #[inline]
    pub fn has_directory_path(&self) -> bool {
        unsafe { CFURLHasDirectoryPath(self) }
    }

    #[inline]
    pub fn resource_specifier(&self) -> Option<CFStringRef> {
        unsafe {
            CFRef::try_from_retained(CFURLCopyResourceSpecifier(self)).ok()
        }
    }

    #[inline]
    pub fn host(&self) -> Option<CFStringRef> {
        unsafe { CFRef::try_from_retained(CFURLCopyHostName(self)).ok() }
    }

    #[inline]
    pub fn port(&self) -> Option<u16> {
        let result = unsafe { CFURLGetPortNumber(self) };
        if result >= 0 {
            assert!(result <= u16::max_value() as i32);
            Some(result as u16)
        } else {
            None
        }
    }

    #[inline]
    pub fn username(&self) -> Option<CFStringRef> {
        let result = unsafe {
            CFRef::try_from_retained(CFURLCopyUserName(self)).ok()
        };
        if result.as_ref().map_or(true, |str| str.is_empty()) {
            return None;
        }
        result
    }

    #[inline]
    pub fn password(&self) -> Option<CFStringRef> {
        let result = unsafe {
            CFRef::try_from_retained(CFURLCopyPassword(self)).ok()
        };
        if result.as_ref().map_or(true, |str| str.is_empty()) {
            return None;
        }
        result
    }

    #[inline]
    pub fn parameters(
            &self, characters_to_leave_escaped: Option<&CFString>)
            -> Option<CFStringRef> {
        unsafe {
            let result =
                CFURLCopyParameterString(self, characters_to_leave_escaped);
            CFRef::try_from_retained(result).ok()
        }
    }

    #[inline]
    pub fn query(
            &self, characters_to_leave_escaped: Option<&CFString>)
            -> Option<CFStringRef> {
        unsafe {
            let result =
                CFURLCopyQueryString(self, characters_to_leave_escaped);
            CFRef::try_from_retained(result).ok()
        }
    }

    #[inline]
    pub fn fragment(
            &self, characters_to_leave_escaped: Option<&CFString>)
            -> Option<CFStringRef> {
        unsafe {
            let result = CFURLCopyFragment(self, characters_to_leave_escaped);
            CFRef::try_from_retained(result).ok()
        }
    }

    #[inline]
    pub fn last_path_component(&self) -> CFStringRef {
        unsafe { CFRef::from_retained(CFURLCopyLastPathComponent(self)) }
    }

    #[inline]
    pub fn path_extension(&self) -> Option<CFStringRef> {
        unsafe { CFRef::try_from_retained(CFURLCopyPathExtension(self)).ok() }
    }

    #[inline]
    pub fn with_path_component(
            &self, path_component: &CFString, is_directory: bool)
            -> Result<CFURLRef, ()> {
        unsafe {
            CFRef::try_from_retained(
                CFURLCreateCopyAppendingPathComponent(
                    None, self, path_component, is_directory))
        }
    }

    #[inline]
    pub fn without_last_path_component(&self) -> Result<CFURLRef, ()> {
        unsafe {
            CFRef::try_from_retained(
                CFURLCreateCopyDeletingLastPathComponent(None, self))
        }
    }

    #[inline]
    pub fn with_path_extension(
            &self, extension: &CFString) ->
            Result<CFURLRef, ()> {
        unsafe {
            CFRef::try_from_retained(
                CFURLCreateCopyAppendingPathExtension(None, self, extension))
        }
    }

    #[inline]
    pub fn without_path_extension(&self) -> Result<CFURLRef, ()> {
        unsafe {
            CFRef::try_from_retained(
                CFURLCreateCopyDeletingPathExtension(None, self))
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        unsafe {
            let output_len = CFURLGetBytes(self, ptr::null_mut(), 0);
            let mut output = vec![0; usize::from_index(output_len)];
            let wrote = CFURLGetBytes(self, output.as_mut_ptr(), output_len);
            assert_eq!(wrote, output_len);
            output
        }
    }

    #[inline]
    pub fn component_range(
            &self, component: CFURLComponentType)
            -> Result<(Range<usize>, Range<usize>), Range<usize>> {
        let mut range_including_separators = CFRange::default();
        let range = unsafe {
            CFURLGetByteRangeForComponent(
                self, component, Some(&mut range_including_separators))
        };
        if range.location >= 0 {
            Ok((range.into(), range_including_separators.into()))
        } else {
            Err(range_including_separators.into())
        }
    }

    #[inline]
    pub fn percent_decode(
            input: &CFString, characters_to_leave_escaped: Option<&CFString>)
            -> Result<CFStringRef, ()> {
        unsafe {
            CFRef::try_from_retained(
                CFURLCreateStringByReplacingPercentEscapes(
                    None, input, characters_to_leave_escaped))
        }
    }

    #[inline]
    pub fn to_file_reference_url(
            &self)
            -> Result<CFURLRef, CFErrorRef> {
        unsafe {
            let mut error = ptr::null();
            let result = 
                CFURLCreateFileReferenceURL(None, self, Some(&mut error));
            CFRef::try_from_retained(result)
                .map_err(|()| CFRef::from_retained(error))
        }
    }

    #[inline]
    pub fn to_file_path_url(&self) -> Result<CFURLRef, CFErrorRef> {
        unsafe {
            let mut error = ptr::null();
            let result = 
                CFURLCreateFilePathURL(None, self, Some(&mut error));
            CFRef::try_from_retained(result)
                .map_err(|()| CFRef::from_retained(error))
        }
    }
}

impl<'a> From<&'a CFURL> for Vec<u8> {
    fn from(input: &'a CFURL) -> Self {
        input.to_vec()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(i64)]
pub enum CFURLPathStyle {
    POSIX = 0,
    HFS = 1,
    Windows = 2,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(i64)]
pub enum CFURLComponentType {
    Scheme = 1,
    NetLocation = 2,
    Path = 3,
    ResourceSpecifier = 4,
    User = 5,
    Password = 6,
    UserInfo = 7,
    Host = 8,
    Port = 9,
    ParameterString = 10,
    Query = 11,
    Fragment = 12,
}

bitflags! {
    #[repr(C)]
    pub flags CFURLBookmarkCreationOptions: CFOptionFlags {
        const CREATE_MINIMAL_BOOKMARK_MASK = 1 << 9,
        const CREATE_SUITABLE_FOR_BOOKMARK_FILE = 1 << 10,
        const CREATE_WITH_SECURITY_SCOPE = 1 << 11,
        const CREATE_SECURITY_SCOPE_ALLOW_ONLY_READ_ACCESS = 1 << 12,
    }
}

bitflags! {
    #[repr(C)]
    pub flags CFURLBookmarkResolutionOptions: CFOptionFlags {
        const RESOLVE_WITHOUT_UI_MASK = 1 << 8,
        const RESOLVE_WITHOUT_MOUNTING_MASK = 1 << 9,
        const RESOLVE_WITH_SECURITY_SCOPE = 1 << 10,
    }
}

pub type CFURLBookmarkFileCreationOptions = CFOptionFlags;

#[test]
fn test_file_url_from_path() {
    let path = "/usr/local/foo/";
    let url = CFURL::from_file_system_path(
        &CFString::from_static_str(path), CFURLPathStyle::POSIX, true);
    assert_eq!(
        url.unwrap().as_string().to_string(),
        Ok("file:///usr/local/foo/".to_owned()));
}

extern {
    pub fn CFURLGetTypeID() -> CFTypeID;

    pub fn CFURLCreateWithBytes(
            allocator: Option<&'static CFAllocator>,
            URLBytes: *const u8,
            length: CFIndex,
            encoding: CFStringEncoding,
            baseURL: Option<&CFURL>)
            -> *const CFShared<CFURL>;

    pub fn CFURLCreateData(
            allocator: Option<&'static CFAllocator>,
            url: &CFURL,
            encoding: CFStringEncoding,
            escapeWhitespace: bool)
            -> *const CFShared<CFData>;

    pub fn CFURLCreateWithString(
            allocator: Option<&'static CFAllocator>,
            URLString: &CFString,
            baseURL: Option<&CFURL>)
            -> *const CFShared<CFURL>;

    pub fn CFURLCreateAbsoluteURLWithBytes(
            alloc: Option<&'static CFAllocator>,
            relativeURLBytes: *const u8,
            length: CFIndex,
            encoding: CFStringEncoding,
            baseURL: Option<&CFURL>,
            useCompatibilityMode: bool)
            -> *const CFShared<CFURL>;

    pub fn CFURLCreateWithFileSystemPath(
            allocator: Option<&'static CFAllocator>,
            filePath: &CFString,
            pathStyle: CFURLPathStyle,
            isDirectory: bool)
            -> *const CFShared<CFURL>;

    pub fn CFURLCreateFromFileSystemRepresentation(
            allocator: Option<&'static CFAllocator>,
            buffer: *const u8,
            bufLen: CFIndex,
            isDirectory: bool)
            -> *const CFShared<CFURL>;

    pub fn CFURLCreateWithFileSystemPathRelativeToBase(
            allocator: Option<&'static CFAllocator>,
            filePath: &CFString,
            pathStyle: CFURLPathStyle,
            isDirectory: bool,
            baseURL: Option<&CFURL>)
            -> *const CFShared<CFURL>;

    pub fn CFURLCreateFromFileSystemRepresentationRelativeToBase(
            allocator: Option<&'static CFAllocator>,
            buffer: *const u8,
            bufLen: CFIndex,
            isDirectory: bool,
            baseURL: Option<&CFURL>)
            -> *const CFShared<CFURL>;

    pub fn CFURLGetFileSystemRepresentation(
            url: &CFURL,
            resolveAgainstBase: bool,
            buffer: *mut u8,
            maxBufLen: CFIndex)
            -> bool;

    pub fn CFURLCopyAbsoluteURL(relativeURL: &CFURL) -> *const CFShared<CFURL>;
    pub fn CFURLGetString(anURL: &CFURL) -> Option<&CFShared<CFString>>;
    pub fn CFURLGetBaseURL(anURL: &CFURL) -> Option<&CFShared<CFURL>>;
    pub fn CFURLCanBeDecomposed(anURL: &CFURL) -> bool;
    pub fn CFURLCopyScheme(anURL: &CFURL) -> *const CFShared<CFString>;
    pub fn CFURLCopyNetLocation(anURL: &CFURL) -> *const CFShared<CFString>;
    pub fn CFURLCopyPath(anURL: &CFURL) -> *const CFShared<CFString>;

    pub fn CFURLCopyStrictPath(
            anURL: &CFURL, isAbsolute: Option<&mut bool>)
            -> *const CFShared<CFString>;

    pub fn CFURLCopyFileSystemPath(
            anURL: &CFURL, pathStyle: CFURLPathStyle)
            -> *const CFShared<CFString>;

    pub fn CFURLHasDirectoryPath(anURL: &CFURL) -> bool;

    pub fn CFURLCopyResourceSpecifier(
            anURL: &CFURL)
            -> *const CFShared<CFString>;

    pub fn CFURLCopyHostName(anURL: &CFURL) -> *const CFShared<CFString>;
    pub fn CFURLGetPortNumber(anURL: &CFURL) -> i32;
    pub fn CFURLCopyUserName(anURL: &CFURL) -> *const CFShared<CFString>;
    pub fn CFURLCopyPassword(anURL: &CFURL) -> *const CFShared<CFString>;

    pub fn CFURLCopyParameterString(
            anURL: &CFURL, charactersToLeaveEscaped: Option<&CFString>)
            -> *const CFShared<CFString>;

    pub fn CFURLCopyQueryString(
            anURL: &CFURL, charactersToLeaveEscaped: Option<&CFString>)
            -> *const CFShared<CFString>;

    pub fn CFURLCopyFragment(
            anURL: &CFURL, charactersToLeaveEscaped: Option<&CFString>)
            -> *const CFShared<CFString>;

    pub fn CFURLCopyLastPathComponent(
            anURL: &CFURL)
            -> *const CFShared<CFString>;

    pub fn CFURLCopyPathExtension(anURL: &CFURL) -> *const CFShared<CFString>;

    pub fn CFURLCreateCopyAppendingPathComponent(
            allocator: Option<&'static CFAllocator>,
            url: &CFURL,
            pathComponent: &CFString,
            isDirectory: bool)
            -> *const CFShared<CFURL>;

    pub fn CFURLCreateCopyDeletingLastPathComponent(
            allocator: Option<&'static CFAllocator>, url: &CFURL)
            -> *const CFShared<CFURL>;

    pub fn CFURLCreateCopyAppendingPathExtension(
            allocator: Option<&'static CFAllocator>,
            url: &CFURL,
            extension: &CFString)
            -> *const CFShared<CFURL>;

    pub fn CFURLCreateCopyDeletingPathExtension(
            allocator: Option<&'static CFAllocator>, url: &CFURL)
            -> *const CFShared<CFURL>;

    pub fn CFURLGetBytes(
            url: &CFURL, buffer: *const u8, bufferLength: CFIndex)
            -> CFIndex;

    pub fn CFURLGetByteRangeForComponent(
            url: &CFURL,
            component: CFURLComponentType,
            rangeIncludingSeparators: Option<&mut CFRange>)
            -> CFRange;

    pub fn CFURLCreateStringByReplacingPercentEscapes(
            allocator: Option<&'static CFAllocator>,
            originalString: &CFString,
            charactersToLeaveEscaped: Option<&CFString>)
            -> *const CFShared<CFString>;

    pub fn CFURLCreateFileReferenceURL(
            allocator: Option<&'static CFAllocator>,
            url: &CFURL,
            error: Option<&mut *const CFShared<CFError>>)
            -> *const CFShared<CFURL>;

    pub fn CFURLCreateFilePathURL(
            allocator: Option<&'static CFAllocator>,
            url: &CFURL,
            error: Option<&mut *const CFShared<CFError>>)
            -> *const CFShared<CFURL>;

    pub fn CFURLCopyResourcePropertyForKey(
            url: &CFURL,
            key: &CFString,
            propertyValueTypeRefPtr: *mut c_void,
            error: Option<&mut *const CFShared<CFError>>)
            -> *const CFShared<CFURL>;

    pub fn CFURLCopyResourcePropertiesForKeys(
            url: &CFURL,
            keys: &CFArray,
            error: Option<&mut *const CFShared<CFError>>)
            -> *const CFShared<CFDictionary>;

    pub fn CFURLSetResourcePropertyForKey(
            url: &CFURL,
            key: &CFString,
            propertyValue: &CFShared<CFObject>,
            error: Option<&mut *const CFShared<CFError>>)
            -> bool;

    pub fn CFURLSetResourcePropertiesForKeys(
            url: &CFURL,
            keyedPropertyValues: &CFDictionary,
            error: Option<&mut *const CFShared<CFError>>)
            -> bool;

    pub static kCFURLKeysOfUnsetValuesKey: Option<&'static CFShared<CFString>>;

    pub fn CFURLClearResourcePropertyCacheForKey(url: &CFURL, key: &CFString);
    pub fn CFURLClearResourcePropertyCache(url: &CFURL);

    pub fn CFURLSetTemporaryResourcePropertyForKey(
            url: &CFURL, key: &CFString, propertyValue: &CFShared<CFObject>);

    pub fn CFURLResourceIsReachable(
            url: &CFURL, error: Option<&mut *const CFShared<CFError>>)
            -> bool;

    pub fn CFURLCreateBookmarkData(
            allocator: Option<&'static CFAllocator>,
            url: &CFURL,
            options: CFURLBookmarkCreationOptions,
            resourcePropertiesToInclude: &CFArray,
            relativeToURL: Option<&CFURL>,
            error: Option<&mut *const CFShared<CFError>>)
            -> *const CFShared<CFData>;

    pub fn CFURLCreateByResolvingBookmarkData(
            allocator: Option<&'static CFAllocator>,
            bookmark: &CFData,
            options: CFURLBookmarkResolutionOptions,
            relativeToURL: Option<&CFURL>,
            resourcePropertiesToInclude: Option<&CFArray>,
            isStale: &mut bool,
            error: Option<&mut *const CFShared<CFError>>)
            -> *const CFShared<CFURL>;

    pub fn CFURLCreateResourcePropertiesForKeysFromBookmarkData(
            allocator: Option<&'static CFAllocator>,
            resourcePropertiesToReturn: &CFArray,
            bookmark: &CFData)
            -> *const CFShared<CFDictionary>;

    pub fn CFURLCreateResourcePropertyForKeyFromBookmarkData(
            allocator: Option<&'static CFAllocator>,
            resourcePropertyKey: &CFString,
            bookmark: &CFData)
            -> *const CFShared<CFObject>;

    pub fn CFURLCreateBookmarkDataFromFile(
            allocator: Option<&'static CFAllocator>,
            fileURL: &CFURL,
            error: Option<&mut *const CFShared<CFError>>)
            -> *const CFShared<CFData>;

    pub fn CFURLWriteBookmarkDataToFile(
            bookmarkRef: &CFData,
            fileURL: &CFURL,
            options: CFURLBookmarkFileCreationOptions,
            error: Option<&mut *const CFShared<CFError>>)
            -> bool;

    pub fn CFURLCreateBookmarkDataFromAliasRecord(
            allocatorRef: Option<&'static CFAllocator>,
            aliasRecordDataRef: &CFData)
            -> *const CFShared<CFData>;

    pub fn CFURLStartAccessingSecurityScopedResource(url: &CFURL) -> bool;
    pub fn CFURLStopAccessingSecurityScopedResource(url: &CFURL);
}
