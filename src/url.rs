// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A URL type for Core Foundation.

#![allow(non_upper_case_globals)]

use base::{CFOptionFlags, CFRelease, CFRetain, CFTypeID, CFTypeRef, TCFType};
use string::{CFString, CFStringRef};

use std::fmt;
use std::mem;

#[repr(C)]
struct __CFURL;

pub type CFURLRef = *const __CFURL;

pub struct CFURL {
    obj: CFURLRef,
}

impl Drop for CFURL {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl TCFType<CFURLRef> for CFURL {
    #[inline]
    fn as_concrete_TypeRef(&self) -> CFURLRef {
        self.obj
    }

    #[inline]
    unsafe fn wrap_under_get_rule(reference: CFURLRef) -> CFURL {
        let reference: CFURLRef = mem::transmute(CFRetain(mem::transmute(reference)));
        TCFType::wrap_under_create_rule(reference)
    }

    #[inline]
    fn as_CFTypeRef(&self) -> CFTypeRef {
        unsafe {
            mem::transmute(self.as_concrete_TypeRef())
        }
    }

    unsafe fn wrap_under_create_rule(obj: CFURLRef) -> CFURL {
        CFURL {
            obj: obj,
        }
    }

    #[inline]
    fn type_id(_: Option<CFURL>) -> CFTypeID {
        unsafe {
            CFURLGetTypeID()
        }
    }
}

impl fmt::Debug for CFURL {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let string: CFString = TCFType::wrap_under_get_rule(CFURLGetString(self.obj));
            write!(f, "{}", string.to_string())
        }
    }
}

type CFURLBookmarkCreationOptions = CFOptionFlags;

// static kCFURLBookmarkCreationPreferFileIDResolutionMask: CFURLBookmarkCreationOptions =
//     (1 << 8) as u32;
// static kCFURLBookmarkCreationMinimalBookmarkMask: CFURLBookmarkCreationOptions =
//     (1 << 9) as u32;
// static kCFURLBookmarkCreationSuitableForBookmarkFile: CFURLBookmarkCreationOptions =
//     (1 << 10) as u32;
// static kCFURLBookmarkCreationWithSecurityScope: CFURLBookmarkCreationOptions =
//     (1 << 11) as u32;
// static kCFURLBookmarkCreationSecurityScopeAllowOnlyReadAccess: CFURLBookmarkCreationOptions =
//     (1 << 12) as u32;

// TODO: there are a lot of missing keys and constants. Add if you are bored or need them.

#[link(name = "CoreFoundation", kind = "framework")]
extern {
    /*
     * CFURL.h
     */

    /* Common File System Resource Keys */
    // static kCFURLAttributeModificationDateKey: CFStringRef;
    // static kCFURLContentAccessDateKey: CFStringRef;
    // static kCFURLContentModificationDateKey: CFStringRef;
    // static kCFURLCreationDateKey: CFStringRef;
    // static kCFURLCustomIconKey: CFStringRef;
    // static kCFURLEffectiveIconKey: CFStringRef;
    // static kCFURLFileResourceIdentifierKey: CFStringRef;
    // static kCFURLFileSecurityKey: CFStringRef;
    // static kCFURLHasHiddenExtensionKey: CFStringRef;
    // static kCFURLIsDirectoryKey: CFStringRef;
    // static kCFURLIsExecutableKey: CFStringRef;
    // static kCFURLIsHiddenKey: CFStringRef;
    // static kCFURLIsPackageKey: CFStringRef;
    // static kCFURLIsReadableKey: CFStringRef;
    // static kCFURLIsRegularFileKey: CFStringRef;
    // static kCFURLIsSymbolicLinkKey: CFStringRef;
    // static kCFURLIsSystemImmutableKey: CFStringRef;
    // static kCFURLIsUserImmutableKey: CFStringRef;
    // static kCFURLIsVolumeKey: CFStringRef;
    // static kCFURLIsWritableKey: CFStringRef;
    // static kCFURLLabelColorKey: CFStringRef;
    // static kCFURLLabelNumberKey: CFStringRef;
    // static kCFURLLinkCountKey: CFStringRef;
    // static kCFURLLocalizedLabelKey: CFStringRef;
    // static kCFURLLocalizedNameKey: CFStringRef;
    // static kCFURLLocalizedTypeDescriptionKey: CFStringRef;
    // static kCFURLNameKey: CFStringRef;
    // static kCFURLParentDirectoryURLKey: CFStringRef;
    // static kCFURLPreferredIOBlockSizeKey: CFStringRef;
    // static kCFURLTypeIdentifierKey: CFStringRef;
    // static kCFURLVolumeIdentifierKey: CFStringRef;
    // static kCFURLVolumeURLKey: CFStringRef;
    // static kCFURLIsExcludedFromBackupKey: CFStringRef;
    // static kCFURLFileResourceTypeKey: CFStringRef;

    /* Creating a CFURL */
    //fn CFURLCopyAbsoluteURL
    //fn CFURLCreateAbsoluteURLWithBytes
    //fn CFURLCreateByResolvingBookmarkData
    //fn CFURLCreateCopyAppendingPathComponent
    //fn CFURLCreateCopyAppendingPathExtension
    //fn CFURLCreateCopyDeletingLastPathComponent
    //fn CFURLCreateCopyDeletingPathExtension
    //fn CFURLCreateFilePathURL
    //fn CFURLCreateFileReferenceURL
    //fn CFURLCreateFromFileSystemRepresentation
    //fn CFURLCreateFromFileSystemRepresentationRelativeToBase
    //fn CFURLCreateFromFSRef
    //fn CFURLCreateWithBytes
    //fn CFURLCreateWithFileSystemPath
    //fn CFURLCreateWithFileSystemPathRelativeToBase
    //fn CFURLCreateWithString(allocator: CFAllocatorRef, urlString: CFStringRef,
    //                         baseURL: CFURLRef) -> CFURLRef;

    /* Accessing the Parts of a URL */
    //fn CFURLCanBeDecomposed
    //fn CFURLCopyFileSystemPath
    //fn CFURLCopyFragment
    //fn CFURLCopyHostName
    //fn CFURLCopyLastPathComponent
    //fn CFURLCopyNetLocation
    //fn CFURLCopyParameterString
    //fn CFURLCopyPassword
    //fn CFURLCopyPath
    //fn CFURLCopyPathExtension
    //fn CFURLCopyQueryString
    //fn CFURLCopyResourceSpecifier
    //fn CFURLCopyScheme
    //fn CFURLCopyStrictPath
    //fn CFURLCopyUserName
    //fn CFURLGetPortNumber
    //fn CFURLHasDirectoryPath

    /* Converting URLs to Other Representations */
    //fn CFURLCreateData(allocator: CFAllocatorRef, url: CFURLRef,
    //                   encoding: CFStringEncoding, escapeWhitespace: bool) -> CFDataRef;
    //fn CFURLCreateStringByAddingPercentEscapes
    //fn CFURLCreateStringByReplacingPercentEscapes
    //fn CFURLCreateStringByReplacingPercentEscapesUsingEncoding
    //fn CFURLGetFileSystemRepresentation
    //fn CFURLGetFSRef
    fn CFURLGetString(anURL: CFURLRef) -> CFStringRef;

    /* Getting URL Properties */
    //fn CFURLGetBaseURL(anURL: CFURLRef) -> CFURLRef;
    //fn CFURLGetBytes
    //fn CFURLGetByteRangeForComponent
    fn CFURLGetTypeID() -> CFTypeID;
    //fn CFURLResourceIsReachable

    /* Getting and Setting File System Resource Properties */
    //fn CFURLClearResourcePropertyCache
    //fn CFURLClearResourcePropertyCacheForKey
    //fn CFURLCopyResourcePropertiesForKeys
    //fn CFURLCopyResourcePropertyForKey
    //fn CFURLCreateResourcePropertiesForKeysFromBookmarkData
    //fn CFURLCreateResourcePropertyForKeyFromBookmarkData
    //fn CFURLSetResourcePropertiesForKeys
    //fn CFURLSetResourcePropertyForKey
    //fn CFURLSetTemporaryResourcePropertyForKey

    /* Working with Bookmark Data */
    //fn CFURLCreateBookmarkData
    //fn CFURLCreateBookmarkDataFromAliasRecord
    //fn CFURLCreateBookmarkDataFromFile
    //fn CFURLWriteBookmarkDataToFile
    //fn CFURLStartAccessingSecurityScopedResource
    //fn CFURLStopAccessingSecurityScopedResource
}
