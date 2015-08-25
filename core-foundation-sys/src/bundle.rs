use libc::c_void;

use base::CFTypeID;
use string::CFStringRef;

pub type CFBundleRef = *const c_void;

extern {
    /*
     * CFBundle.h
     */
    pub fn CFBundleGetBundleWithIdentifier(bundleID: CFStringRef) -> CFBundleRef;
    pub fn CFBundleGetFunctionPointerForName(bundle: CFBundleRef, function_name: CFStringRef) -> *const c_void;

    pub fn CFBundleGetTypeID() -> CFTypeID;
}
