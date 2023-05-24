pub use base::boolean_t;

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    // Screen Capture Access
    pub fn CGRequestScreenCaptureAccess() -> boolean_t;
    pub fn CGPreflightScreenCaptureAccess() -> boolean_t;
}
