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
        window.addEventListener("scroll", () => {
          if (window.scrollY > 0)
            localStorage.setItem(
              `${location.pathname.split("/")[2]}_scroll`,
              window.scrollY
            );
        });
      }, 100);
    }, 400);
  });
});
