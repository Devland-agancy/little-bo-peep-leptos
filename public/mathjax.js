localStorage.setItem("activate_scroll", "false");
if (localStorage.getItem(`${location.pathname.split("/")[2]}_scroll`)) {
  setTimeout((e) => {
    window.scroll({
      top: localStorage.getItem(`${location.pathname.split("/")[2]}_scroll`),
    });
  }, 100);
}

MathJax.Hub.Queue(["Typeset", MathJax.Hub]);

MathJax.Hub.Register.StartupHook("End", function () {
  document.querySelectorAll(".hidden-on-startup").forEach((elem) => {
    elem.classList.remove("hidden-on-startup");
    elem.classList.add("animate-appear");

    setTimeout((e) => {
      if (localStorage.getItem(`${location.pathname.split("/")[2]}_scroll`)) {
        window.scroll({
          top: localStorage.getItem(
            `${location.pathname.split("/")[2]}_scroll`
          ),
        });
      }
      setTimeout((e) => {
        localStorage.setItem("activate_scroll", "true");
        window.addEventListener("scroll", () => {
          if (
            localStorage.getItem("activate_scroll") == "true" &&
            window.scrollY > 0
          )
            localStorage.setItem(
              `${location.pathname.split("/")[2]}_scroll`,
              window.scrollY
            );
        });
      }, 100);
    }, 700);
  });
});
