use crate::components::Article::Article;
use crate::components::Header::{ChapterMenu, Header, MenuButton, MenuState};
use crate::constants::MOBILE_MAX_WIDTH;
use crate::error_template::{AppError, ErrorTemplate};

use crate::global_state::GlobalState;
use crate::page::state::PageState;
use crate::svg_defs::SVGDefinitions;
use leptos::ev::resize;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use web_sys::{ScrollBehavior, ScrollToOptions};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let (page_state, set_page_state) = create_signal(cx, PageState::ShowArticle);
    provide_context(cx, set_page_state);
    provide_context(cx, page_state);

    let (menu_state, set_menu_state) = create_signal(cx, MenuState::Closed);
    provide_context(cx, set_menu_state);
    provide_context(cx, menu_state);

    let (solution_open, set_solution_open) = create_signal(cx, false);
    provide_context(cx, set_solution_open);
    provide_context(cx, solution_open);

    let (route, set_route) = create_signal(cx, "");
    provide_context(cx, set_route);
    provide_context(cx, route);

    provide_context(cx, GlobalState::new(cx));

    let GlobalState {
        route, on_mobile, ..
    } = use_context(cx).unwrap();

    create_effect(cx, move |_| {
        // execute on every route change
        route.get();

        /*  */
        let script = document().create_element("script");
        match script {
            Ok(elem) => {
                let _ = elem.set_attribute("id", "mathjax-rendered");
                let _ = elem.set_attribute("type", "text/javascript");
                let _ = elem.set_attribute("src", "/mathjax.js");
                let head = document().get_elements_by_tag_name("head").item(0);

                if let Some(_head) = head {
                    if let Some(mathEl) = document().get_element_by_id("mathjax-rendered") {
                        let _ = mathEl.remove();
                    }

                    let _ = _head.append_child(&elem);
                }
            }
            _ => {}
        }

        let mut options = ScrollToOptions::new();
        options.left(1500.0);
        options.behavior(ScrollBehavior::Instant);
        window().scroll_with_scroll_to_options(&options);
    });

    create_effect(cx, move |_| {
        on_mobile.set(window().inner_width().unwrap().as_f64().unwrap() <= MOBILE_MAX_WIDTH as f64);

        window_event_listener(resize, move |_| {
            on_mobile
                .set(window().inner_width().unwrap().as_f64().unwrap() <= MOBILE_MAX_WIDTH as f64);
        });
    });

    view! { cx,
      // injects a stylesheet into the document <head>
      // id=leptos means cargo-leptos will hot-reload this stylesheet
      <Stylesheet id="leptos" href="/pkg/little-bo-peep.css"/>
      <meta name="viewport" content="width=device-width,initial-scale=1.0,minimum-scale=1"/>
      <meta name="format-detection" content="telephone=no"/>
      // sets the document title
      <Title text="Little Bo Peep"/>
      <Link href="/images/book_favicon_sized_v2.svg" rel="icon"/>
      <Script  src="/mathjax_setup.js" defer="true"/>
      <Script
      src="https://cdnjs.cloudflare.com/ajax/libs/mathjax/2.7.9/MathJax.js?config=TeX-AMS_SVG"
      defer="true"
      />

      // content for this welcome page
      <Router fallback=|cx| {
          let mut outside_errors = Errors::default();
          outside_errors.insert_with_default_key(AppError::NotFound);
          view! { cx, <ErrorTemplate outside_errors/> }.into_view(cx)
      }>
        <main>
          <MenuButton />
          <ChapterMenu />
          <Article>
            <Header/>
            <Routes>
              <Route path="" view=crate::page::home::View/>
              <Route path="/article/ch_1" view=crate::page::article::ch_1::View/>
              <Route path="/article/ch_2" view=crate::page::article::ch_2::View/>
              <Route path="/article/ch_3" view=crate::page::article::ch_3::View/>
            </Routes>
          </Article>
        </main>
      </Router>
      <SVGDefinitions />
    }
}
