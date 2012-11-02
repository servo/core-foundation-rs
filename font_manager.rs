use cf = core_foundation;
use cf::array::CFArrayRef;
use cf::url::CFURLRef;

extern {
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