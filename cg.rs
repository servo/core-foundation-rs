// FIXME: Should be bindgenned

use libc::{c_void, c_char, size_t};

#[nolink]
pub extern mod cg {
    pub fn CGDataProviderCreateWithFilename(filename: *c_char) -> CGDataProviderRef;
    pub fn CGDataProviderCreateWithData(info: *c_void,
                                        data: *c_void,
                                        size: size_t,
                                        releaseData: CGDataProviderReleaseDataCallback
                                        ) -> CGDataProviderRef;
    pub fn CGDataProviderRelease(provider: CGDataProviderRef);
    pub fn CGFontCreateWithDataProvider(provider: CGDataProviderRef) -> CGFontRef;
    pub fn CGFontRelease(font: CGFontRef);
}

pub type CGFontRef = *c_void;
pub type CGDataProviderRef = *c_void;
pub type CGDataProviderReleaseDataCallback = *c_void;
