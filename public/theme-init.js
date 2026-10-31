// Apply the stored theme before first paint so the window never flashes
// the wrong background colour.
(function () {
  try {
    var stored = JSON.parse(localStorage.getItem("yuki-theme"));
    document.documentElement.dataset.theme = stored === "light" ? "light" : "dark";
  } catch (_error) {
    document.documentElement.dataset.theme = "dark";
  }
})();
