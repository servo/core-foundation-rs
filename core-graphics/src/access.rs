#[derive(Default)]
pub struct ScreenCaptureAccess;

impl ScreenCaptureAccess {
    /// If current app not in list, will open window.
    /// Return the same result as preflight.
    #[inline]
    pub fn request(&self) -> bool {
        unsafe { CGRequestScreenCaptureAccess() }
    }

    /// Return `true` if has access
    #[inline]
    pub fn preflight(&self) -> bool {
        unsafe { CGPreflightScreenCaptureAccess() }
    }
}

#[cfg_attr(feature = "link", link(name = "CoreGraphics", kind = "framework"))]
extern "C" {
    // Screen Capture Access
    fn CGRequestScreenCaptureAccess() -> bool;
    fn CGPreflightScreenCaptureAccess() -> bool;
}
