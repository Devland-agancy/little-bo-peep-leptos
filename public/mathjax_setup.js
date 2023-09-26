window.MathJax = {
  svg: {
    mtextInheritFont: true,
  },
  chtml: {
    mtextInheritFont: true,
  },
  tex: {
    inlineMath: [
      ["$", "$"],
      ["\\(", "\\)"],
    ],
    processEscapes: true,
    macros: {
      dblcol:
        "\\!\\rt{0.1}\\mathrel{\\raise.13ex{\\substack{\\small \\circ \\\\ \\small \\circ}}}",
      hc: "\\!\\rt{0.1}\\mathrel{\\raise.13ex{\\substack{\\small \\circ \\\\ \\small \\circ}}}",
      rr: "\\mathbb{R}",
      zz: "\\mathbb{Z}",
      nn: "\\mathbb{N}",
      ww: "\\mathbb{W}",
      qq: "\\mathbb{Q}",
      te: "\\text",
      dom: "\\text{dom}\\,",
      degree: "\\text{deg}\\,",
      f: "\\Rule{0.12em}{0.8pt}{-0.8pt}f",
      fsp: "\\hspace{0.06em}\\Rule{0.12em}{0.8pt}{-0.8pt}f",
      sp: "\\Rule{0.08em}{0.8pt}{-0.8pt}",
      ra: "\\rightarrow",
      back: "\\backslash",
      sqt: "{\\color{white} *\\!\\!\\!}",
      up: ["\\rule{0pt}{#1em}", 1], // vspace doesn't seem to work / exist ?
      dn: ["\\Rule{0pt}{0em}{#1em}", 1],
      rt: ["\\hspace{#1em}", 1],
      hlfbk: "\\!\\hspace{0.1em}",
      fl: ["\\lfloor #1 \\rfloor", 1],
      cl: ["\\lceil #1 \\rceil", 1],
      FL: ["\\left\\lfloor #1 \\right\\rfloor", 1],
      CL: ["\\left\\lceil #1 \\right\\rceil", 1],
      implies: "\\Longrightarrow",
      psa: "{}\\!\\hspace{0.0691em}",
      psb: "{}\\!\\hspace{0.06901311249137em}",
      ncdot: "\\re\\cdot\\re",
      re: "\\!\\hspace{0.1em}",
      bk: "\\!\\hspace{0.1em}",
      gbk: "\\!\\hspace{0.15em}",
      fw: "\\hspace{0.1em}",
      hfbk: "\\!\\hspace{0.2em}",
      deg: "\\circ",
      km: "[\\text{km}]",
      ddx: "{d \\over dx}\\hspace{0.1em}",
      ddt: "{d \\over dt}\\hspace{0.1em}",
      ddu: "{d \\over du}\\hspace{0.1em}",
      ddz: "{d \\over dz}\\hspace{0.1em}",
      ov: ["\\overline{#1}", 1],
      floor: ["\\lfloor{#1}\\rfloor", 1],
      faketextelement: "{\\color{white}\\text{*}}\\!\\!\\!\\rt{0.1}",
    },
  },
  options: {
    enableMenu: false,
  },
  loader: {
    load: ["[tex]/color"],
  },
  startup: {
    ready: () => {
      MathJax.startup.defaultReady();
      MathJax.startup.promise.then(() => {
        document.querySelectorAll(".hidden-on-startup").forEach((elem) => {
          elem.classList.remove("hidden-on-startup");
          elem.classList.add("animate-appear");
        });
      });
    },
  },
};
