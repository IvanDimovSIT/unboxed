use macroquad::prelude::info;

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn detect_mobile_browser() -> bool;
    fn detect_fullscreen() -> bool;
    fn set_browser_fullscreen(fullscreen: bool);
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

#[cfg(not(target_arch = "wasm32"))]
static IS_FULLSCREEN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn is_fullscreen() -> bool {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        detect_fullscreen()
    }

    #[cfg(not(target_arch = "wasm32"))]
    IS_FULLSCREEN.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn set_fullscreen(fullscreen: bool) {
    info!("Setting fullscreen: {}", fullscreen);

    #[cfg(target_arch = "wasm32")]
    unsafe {
        set_browser_fullscreen(fullscreen);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        IS_FULLSCREEN.store(fullscreen, std::sync::atomic::Ordering::Relaxed);
        macroquad::window::set_fullscreen(fullscreen);
    }
}
