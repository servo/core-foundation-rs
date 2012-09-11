// FIXME: Should be bindgenned

use libc::{c_void, c_char, size_t};

#[nolink]
extern mod cg {
    fn CGDataProviderCreateWithFilename(filename: *c_char) -> CGDataProviderRef;
    fn CGDataProviderCreateWithData(info: *c_void,
                                    data: *c_void,
                                    size: size_t,
                                    releaseData: CGDataProviderReleaseDataCallback
                                   ) -> CGDataProviderRef;
    fn CGDataProviderRelease(provider: CGDataProviderRef);
    fn CGFontCreateWithDataProvider(provider: CGDataProviderRef) -> CGFontRef;
    fn CGFontRelease(font: CGFontRef);
}

type CGFontRef = *c_void;
type CGDataProviderRef = *c_void;
type CGDataProviderReleaseDataCallback = *c_void;