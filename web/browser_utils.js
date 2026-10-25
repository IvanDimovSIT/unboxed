function detectMobileBrowser() {
  if (navigator.userAgentData) {
    return navigator.userAgentData.mobile;
  }

  return /Mobi|Android/i.test(navigator.userAgent);
}

function detectFullscreen() {
  if (document.fullscreenElement || document.webkitFullscreenElement) return true;

  if (window.matchMedia?.("(display-mode: fullscreen)").matches) return true;

  const tol = 2;
  return (
    Math.abs(window.innerWidth - screen.width) <= tol &&
    Math.abs(window.innerHeight - screen.height) <= tol
  );
}

function setBrowserFullscreen(fullscreen) {
  if (fullscreen) {
    const elem = document.getElementById("glcanvas");
    if (elem.requestFullscreen) {
      elem.requestFullscreen();
    } else if (elem.webkitRequestFullscreen) {
      elem.webkitRequestFullscreen();
    } else if (elem.msRequestFullscreen) {
      elem.msRequestFullscreen();
    }
  } else {
    if (document.exitFullscreen) {
      document.exitFullscreen();
    } else if (document.webkitExitFullscreen) {
      document.webkitExitFullscreen();
    } else if (document.msExitFullscreen) {
      document.msExitFullscreen();
    }
  }
}

miniquad_add_plugin({
  name: "browser_utils",
  version: 1,
  register_plugin: (importObject) => {
    importObject.env.detect_mobile_browser = detectMobileBrowser;
    importObject.env.detect_fullscreen = detectFullscreen;
    importObject.env.set_browser_fullscreen = setBrowserFullscreen;
  },
  on_init:  () => {},
});
