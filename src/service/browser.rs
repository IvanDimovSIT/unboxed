#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn detect_mobile_browser() -> bool;
}

/// Returns true if running on a mobile browser
pub fn is_mobile() -> bool {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        detect_mobile_browser()
    }

    #[cfg(not(target_arch = "wasm32"))]
    false
}
