function detectMobileBrowser() {
  if (navigator.userAgentData) {
    return navigator.userAgentData.mobile;
  }

  return /Mobi|Android/i.test(navigator.userAgent);
}

miniquad_add_plugin({
  name: "browser_utils",
  version: 1,
  register_plugin: (importObject) => {
    importObject.env.detect_mobile_browser = detectMobileBrowser
  },
  on_init:  () => {},
});
