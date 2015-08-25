use libc::c_void;

use base::CFTypeID;

pub type CFBooleanRef = *const c_void;

extern {
    pub static kCFBooleanTrue: CFBooleanRef;
    pub static kCFBooleanFalse: CFBooleanRef;

    pub fn CFBooleanGetTypeID() -> CFTypeID;
}
