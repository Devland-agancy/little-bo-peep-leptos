#![allow(unused_imports)]
#![allow(dead_code)]

use crate::components::Article::Article;
use crate::components::Footer::Footer;
use crate::components::Header::{Header, MenuButton};
use crate::components::Panel::{MenuState, Panel};
use crate::components::TapToRecenter::TapToRecenter;
use crate::constants::MOBILE_SCREEN_MAX_WIDTH;
use crate::global_state::GlobalState;
use crate::page::article::*;
use crate::page::state::PageState;
use crate::svg_defs::SVGDefinitions;
use ev::Event;
use lazy_static::lazy_static;
use leptos::ev::resize;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use render_chapters::{render_article_routes, render_based_on_env};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{ScrollBehavior, ScrollToOptions};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (menu_state, set_menu_state) = create_signal(MenuState::Closed);
    provide_context(set_menu_state);
    provide_context(menu_state);

    let (solution_open, set_solution_open) = create_signal(false);
    provide_context(set_solution_open);
    provide_context(solution_open);

    let (route, set_route) = create_signal("");
    provide_context(set_route);
    provide_context(route);

    provide_context(GlobalState::new());
    let GlobalState {
        route,
        on_mobile,
        math_rendered,
        ..
    } = use_context().unwrap();
    create_effect(move |_| {
        // execute on every route change
        route.get();
        math_rendered.set(false);

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

        let options = ScrollToOptions::new();
        options.set_left(1500.0);
        options.set_behavior(ScrollBehavior::Instant);
        window().scroll_with_scroll_to_options(&options);
    });

    create_effect(move |_| {
        let cb = Closure::wrap(Box::new(move |_: Event| {
            math_rendered.set(true);
        }) as Box<dyn FnMut(_)>);

        let _ = document()
            .add_event_listener_with_callback("math-rendered", &cb.as_ref().unchecked_ref());

        cb.forget();
    });

    create_effect(move |_| {
        on_mobile.set(
            window().inner_width().unwrap().as_f64().unwrap() <= MOBILE_SCREEN_MAX_WIDTH as f64,
        );

        window_event_listener(resize, move |_| {
            on_mobile.set(
                window().inner_width().unwrap().as_f64().unwrap() <= MOBILE_SCREEN_MAX_WIDTH as f64,
            );
        });
    });

    view! {
      // injects a stylesheet into the document <head>
      // id=leptos means cargo-leptos will hot-reload this stylesheet
      <Stylesheet id="leptos" href="/pkg/little-bo-peep.css"/>
      <meta name="viewport" content="width=device-width,initial-scale=1.0,minimum-scale=1"/>
      <meta name="format-detection" content="telephone=no"/>
      // sets the document title
      <Title text="Little Bo Peep"/>
      <Script src="/mathjax_setup.js" defer="true"/>
      <Script src="/extras.js" defer="true"/>
      <Script
        src="https://cdnjs.cloudflare.com/ajax/libs/mathjax/2.7.9/MathJax.js?config=TeX-AMS_SVG"
        defer="true"
      />

      {render_based_on_env!(
        r#"
            <Link href="/images/book_favicon_sized_v2_dev.svg" rel="icon"/>
            <Script defer="true" src="https://www.googletagmanager.com/gtag/js?id=G-MYGJLZN3X3" />
            <Script src="/dev-analytics.js" defer="true"/>
        "#,
        r#"
            <Link href="/images/book_favicon_sized_v2.png" rel="icon"/>
            <Link href="/images/book_favicon_sized_v2_dev.svg" rel="icon"/>
            <Script defer="true" src="https://www.googletagmanager.com/gtag/js?id=G-CHV0VCT4XH" />
            <Script src="/analytics.js" defer="true"/>
        "#
      )}

      // content for this welcome page

    <Router fallback=|| view! { "Page not found." }.into_view()>
        <main>
            <MenuButton/>
            <Panel />
            <Article>
                <Header/>
                {render_article_routes!("chapters bootcamps")}
                <TapToRecenter />
            </Article>
            <Footer ></Footer>
        </main>
    </Router>
    <SVGDefinitions/>
    }
}








































































































































































































































