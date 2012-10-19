use libc::{c_void, c_char, size_t};

pub type CGDataProviderRef = *c_void;
pub type CGDataProviderReleaseDataCallback = *c_void;

#[nolink]
#[link_args="-framework ApplicationServices"]
extern {
    pub fn CGDataProviderCreateWithFilename(filename: *c_char) -> CGDataProviderRef;
    pub fn CGDataProviderCreateWithData(info: *c_void,
                                        data: *c_void,
                                        size: size_t,
                                        releaseData: CGDataProviderReleaseDataCallback
                                        ) -> CGDataProviderRef;
    pub fn CGDataProviderRelease(provider: CGDataProviderRef);
}
