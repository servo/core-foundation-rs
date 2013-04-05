// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core_foundation::array::CFArrayRef;
use core_foundation::url::CFURLRef;

pub extern {
    /*
     * CTFontManager.h
     */

    // Incomplete function bindings are mostly related to CoreText font matching, which
    // we implement in a platform-independent manner using FontMatcher.

    //fn CTFontManagerCompareFontFamilyNames
    fn CTFontManagerCopyAvailableFontURLs() -> CFArrayRef;
    fn CTFontManagerCopyAvailableFontFamilyNames() -> CFArrayRef;
    fn CTFontManagerCopyAvailablePostScriptNames() -> CFArrayRef;
    fn CTFontManagerCreateFontDescriptorsFromURL(fileURL: CFURLRef) -> CFArrayRef;
    //fn CTFontManagerCreateFontRequestRunLoopSource
    //fn CTFontManagerEnableFontDescriptors
    //fn CTFontManagerGetAutoActivationSetting
    //fn CTFontManagerGetScopeForURL
    //fn CTFontManagerGetAutoActivationSetting
    //fn CTFontManagerGetScopeForURL
    fn CTFontManagerIsSupportedFont(fontURL: CFURLRef) -> bool;
    //fn CTFontManagerRegisterFontsForURL
    //fn CTFontManagerRegisterFontsForURLs
    //fn CTFontManagerRegisterGraphicsFont
    //fn CTFontManagerSetAutoActivationSetting
    //fn CTFontManagerUnregisterFontsForURL
    //fn CTFontManagerUnregisterFontsForURLs
    //fn CTFontManagerUnregisterGraphicsFont
}
