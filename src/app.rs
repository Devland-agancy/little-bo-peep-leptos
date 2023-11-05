use crate::error_template::{AppError, ErrorTemplate};
use crate::header::{Header, MenuState};
use crate::page::state::PageState;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let (page_state, set_page_state) = create_signal(cx, PageState::ShowArticle);
    provide_context(cx, set_page_state);
    provide_context(cx, page_state);

    let (right_image_x_pos, set_right_image_x_pos) = create_signal(cx, 0_f64);
    provide_context(cx, right_image_x_pos);
    provide_context(cx, set_right_image_x_pos);

    let (menu_state, set_menu_state) = create_signal(cx, MenuState::Closed);
    provide_context(cx, set_menu_state);
    provide_context(cx, menu_state);

    let (solution_open, set_solution_open) = create_signal(cx, false);
    provide_context(cx, set_solution_open);
    provide_context(cx, solution_open);

    view! { cx,
      // injects a stylesheet into the document <head>
      // id=leptos means cargo-leptos will hot-reload this stylesheet
      <Stylesheet id="leptos" href="/pkg/little-bo-peep.css"/>
      <meta name="viewport" content="width=device-width,initial-scale=1.0,minimum-scale=1"/>
      <meta name="format-detection" content="telephone=no"/>
      // sets the document title
      <Title text="Little Bo Peep"/>
      <Link href="/images/book_favicon_sized_v2.png" rel="icon"/>
      <Script
      src="https://cdnjs.cloudflare.com/ajax/libs/mathjax/2.7.7/MathJax.js?config=TeX-AMS_SVG"
      type_="text/javascript"
      />
      <script type_="text/x-mathjax-config">
        r#"
        MathJax.Hub.Config({
          showProcessingMessages: false, // Disables the display of "Processing Math: x%" messages
          messageStyle: "none", // No message display
          SVG: {
            mtextFontInherit: true, // Ensures mtext tags use the surrounding font
          },
          jax: ["input/TeX", "output/SVG"],
          tex2jax: {
            inlineMath: [
              ["$", "$"],
              ["\\(", "\\)"]
            ],
            processEscapes: true
          },
          TeX: {
            Macros: {
              dblcol: "\\!\\rt{0.1}\\mathrel{\\raise.13ex{\\substack{\\small \\circ \\\\ \\small \\circ}}}",
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
              up: ["\\rule{0pt}{#1em}", 1],
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
              ncdot: "\\cdot",
              re: "\\!",
              bk: "\\!",
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
            }
          }
        });
        "#
      </script>

      // content for this welcome page
      <Router fallback=|cx| {
          let mut outside_errors = Errors::default();
          outside_errors.insert_with_default_key(AppError::NotFound);
          view! { cx, <ErrorTemplate outside_errors/> }.into_view(cx)
      }>
        <main>
          <Header/>
          <Routes>
            <Route path="" view=crate::page::home::View/>
            <Route path="/article/ch_1" view=crate::page::article::ch_1::View/>
            <Route path="/article/ch_2" view=crate::page::article::ch_2::View/>
            <Route path="/article/ch_3" view=crate::page::article::ch_3::View/>
          </Routes>
        </main>
      </Router>
    }
}
