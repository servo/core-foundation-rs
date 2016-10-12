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
use base::{CFDowncast, CFObject, CFType, CFTypeID};
use dictionary::CFDictionary;
use error::CFError;
use std::os::raw::c_void;
use string::CFString;
use sync::{CFShared, CFRef};
use plugin::CFPlugIn;
use url::CFURL;

pub type CFBundleRef = CFRef<CFBundle>;

#[repr(C)]
pub struct CFBundle { obj: CFObject }

unsafe impl Send for CFBundle {}
unsafe impl Sync for CFBundle {}

unsafe impl CFType for CFBundle {
    #[inline]
    fn as_object(&self) -> &CFObject {
        &self.obj
    }
}

unsafe impl CFDowncast for CFBundle {
    #[inline]
    fn type_id() -> CFTypeID {
        unsafe { CFBundleGetTypeID() }
    }
}

impl CFBundle {
    #[inline]
    pub fn main_bundle() -> Option<&'static CFShared<Self>> {
        unsafe { CFBundleGetMainBundle() }
    }

    #[inline]
    pub fn info_dictionary(&self) -> Option<&CFShared<CFDictionary>> {
        unsafe { CFBundleGetInfoDictionary(self) }
    }
}

extern {
    pub fn CFBundleGetMainBundle() -> Option<&'static CFShared<CFBundle>>;

    pub fn CFBundleGetBundleWithIdentifier<'a>(
            bundleID: &CFString)
            -> Option<&'a CFShared<CFBundle>>;

    pub fn CFBundleGetAllBundles() -> Option<&'static CFShared<CFArray>>;

    pub fn CFBundleGetTypeID() -> CFTypeID;

    pub fn CFBundleCreate(
            allocator: Option<&'static CFAllocator>, bundleURL: &CFURL)
            -> *const CFShared<CFBundle>;

    pub fn CFBundleCreateBundlesFromDirectory(
            allocator: Option<&'static CFAllocator>,
            directoryURL: &CFURL,
            bundleType: Option<&CFString>)
            -> *const CFShared<CFBundle>;

    pub fn CFBundleCopyBundleURL(bundle: &CFBundle) -> *const CFURL;

    pub fn CFBundleGetValueForInfoDictionaryKey(
            bundle: &CFBundle, key: &CFString)
            -> *const CFShared<CFObject>;

    pub fn CFBundleGetInfoDictionary(
            bundle: &CFBundle)
            -> Option<&CFShared<CFDictionary>>;

    pub fn CFBundleGetLocalInfoDictionary(
            bundle: &CFBundle)
            -> Option<&CFShared<CFDictionary>>;

    pub fn CFBundleGetPackageInfo(
            bundle: &CFBundle, packageType: &mut u32, packageCreator: &mut u32);

    pub fn CFBundleGetIdentifier(
            bundle: &CFBundle)
            -> Option<&CFShared<CFString>>;

    pub fn CFBundleGetVersionNumber(bundle: &CFBundle) -> u32;

    pub fn CFBundleCopyInfoDictionaryInDirectory(
            bundleURL: &CFURL)
            -> *const CFShared<CFDictionary>;

    pub fn CFBundleGetPackageInfoInDirectory(
            bundleURL: &CFURL, packageType: &mut u32, packageCreator: &mut u32)
            -> bool;

    pub fn CFBundleCopyResourceURL(
            bundle: &CFBundle,
            resourceName: &CFString,
            resourceType: Option<&CFString>,
            subDirName: Option<&CFString>)
            -> *const CFShared<CFURL>;

    pub fn CFBundleCopyResourceURLsOfType(
            bundle: &CFBundle,
            resourceType: &CFString,
            subDirName: Option<&CFString>)
            -> *const CFShared<CFArray>;

    pub fn CFBundleCopyLocalizedString(
            bundle: &CFBundle,
            key: &CFShared<CFString>,
            value: Option<&CFShared<CFString>>,
            tableName: &CFString)
            -> *const CFShared<CFString>;

    pub fn CFBundleCopyResourceURLInDirectory(
            bundleURL: &CFURL,
            resourceName: &CFString,
            resourceType: Option<&CFString>,
            subDirName: Option<&CFString>)
            -> *const CFShared<CFURL>;

    pub fn CFBundleCopyResourceURLsOfTypeInDirectory(
            bundleURL: &CFURL,
            resourceType: &CFString,
            subDirName: Option<&CFString>)
            -> *const CFShared<CFArray>;

    pub fn CFBundleCopyBundleLocalizations(
            bundle: &CFBundle)
            -> *const CFShared<CFArray>;

    pub fn CFBundleCopyPreferredLocalizationsFromArray(
            array: &CFArray)
            -> *const CFShared<CFArray>;

    pub fn CFBundleCopyLocalizationsForPreferences(
            locArray: &CFArray, prefArray: Option<&CFArray>)
            -> *const CFShared<CFArray>;

    pub fn CFBundleCopyResourceURLForLocalization(
            bundle: &CFBundle,
            resourceName: &CFString,
            resourceType: &CFString,
            subDirName: Option<&CFString>,
            localizationName: &CFString)
            -> *const CFShared<CFURL>;

    pub fn CFBundleCopyResourceURLsOfTypeForLocalization(
            bundle: &CFBundle,
            resourceType: &CFString,
            subDirName: Option<&CFString>,
            localizationName: &CFString)
            -> *const CFShared<CFArray>;

    pub fn CFBundleCopyInfoDictionaryForURL(url: &CFURL) -> *const CFDictionary;

    pub fn CFBundleCopyExecutableArchitecturesForURL(
            url: &CFURL)
            -> *const CFShared<CFArray>;

    pub fn CFBundleCopyExecutableURL(
            bundle: &CFBundle)
            -> *const CFShared<CFURL>;

    pub fn CFBundleCopyExecutableArchitectures(
            bundle: &CFBundle)
            -> *const CFShared<CFArray>;

    pub fn CFBundlePreflightExecutable(
            bundle: &CFBundle, error: &mut *const CFShared<CFError>)
            -> bool;

    pub fn CFBundleLoadExecutableAndReturnError(
            bundle: &CFBundle, error: &mut *const CFShared<CFError>)
            -> bool;

    pub fn CFBundleLoadExecutable(bundle: &CFBundle) -> bool;
    pub fn CFBundleIsExecutableLoaded(bundle: &CFBundle) -> bool;
    pub fn CFBundleUnloadExecutable(bundle: &CFBundle) -> bool;

    pub fn CFBundleGetFunctionPointerForName(
            bundle: &CFBundle, functionName: &CFString)
            -> *mut c_void;

    pub fn CFBundleGetFunctionPointersForNames(
            bundle: &CFBundle,
            functionNames: &CFArray,
            ftbl: *mut *mut c_void);

    pub fn CFBundleGetDataPointerForName(
            bundle: &CFBundle, symbolName: &CFString)
            -> *mut c_void;

    pub fn CFBundleGetDataPointersForNames(
            bundle: &CFBundle,
            symbolNames: &CFArray,
            stbl: *mut *mut c_void);

    pub fn CFBundleCopyAuxiliaryExecutableURL(
            bundle: &CFBundle,
            executableName: &CFString)
            -> *const CFShared<CFURL>;

    pub fn CFBundleGetPlugIn(bundle: &CFBundle) -> Option<&CFShared<CFPlugIn>>;
}
