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

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/little-bo-peep.css"/>
        <meta name="viewport" content="width=device-width,initial-scale=1.0,minimum-scale=1" />
        // sets the document title
        <Title text="Little Bo Peep"/>
        <Link href="/images/book_favicon_sized_v2.png" rel="icon" />
        <Script src="/mathjax_setup.js" />
        <Script src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js" type_="text/javascript"/>

        // content for this welcome page
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx, <ErrorTemplate outside_errors/> }
            .into_view(cx)
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
