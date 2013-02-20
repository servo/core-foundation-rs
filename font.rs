use core_foundation::base::{AbstractCFTypeRef, CFIndex, CFTypeID, CFTypeRef, CFWrapper};

use data_provider::{CGDataProvider, CGDataProviderRef};

use libc::{c_void, c_char, size_t};

pub type CGGlyph = libc::c_ushort;

struct __CGFont { private: () }
pub type CGFontRef = *__CGFont;

pub impl AbstractCFTypeRef for CGFontRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }

    static pure fn type_id() -> CFTypeID {
        unsafe {
            CGFontGetTypeID()
        }
    }
}

pub type CGFont = CFWrapper<CGFontRef, (), ()>;

pub fn create_with_data_provider(provider: &CGDataProvider) -> CGFont {
    // TODO: error handling
    unsafe {
        let value = CGFontCreateWithDataProvider(*provider.borrow_ref());
        CFWrapper::wrap_owned(value)
    }
}

#[nolink]
#[link_args="-framework ApplicationServices"]
extern {

    // TODO: basically nothing has bindings (even commented-out)  besides what we use.
    fn CGFontCreateWithDataProvider(provider: CGDataProviderRef) -> CGFontRef;
    fn CGFontGetTypeID() -> CFTypeID;

    // These do the same thing as CFRetain/CFRelease, except
    // gracefully handle a NULL argument. We don't use them.
    fn CGFontRetain(font: CGFontRef);
    fn CGFontRelease(font: CGFontRef);
}
