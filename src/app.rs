use crate::article::Article;
use crate::error_template::{AppError, ErrorTemplate};
use crate::header::Header;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/little-bo-peep.css"/>

        // sets the document title
        <Title text="Little Bo Peep"/>
        <Link href="/images/book_favicon_sized_v2.png" rel="icon" />
        <Script src="/mathjax_setup.js" />
        <Script src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js" type_="text/javascript"/>

        // content for this welcome page
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx,
                <ErrorTemplate outside_errors/>
            }
            .into_view(cx)
        }>
            <main>
                <Header />
                <Routes>
                    <Route path="" view=crate::article::ch_1::view />
                </Routes>
            </main>
        </Router>
    }
}
