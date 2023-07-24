pub mod article;
pub mod home;
pub mod state;

use leptos::*;
use state::PageState;

#[component]
pub fn Article(cx: Scope, children: Children) -> impl IntoView {
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let show_right = move || page_state() == PageState::ShowRight;
    let show_article = move || page_state() == PageState::ShowArticle;

    view! { cx,
        <div class="pt-14 lg:pt-20 overscroll-none">
        <div
            class="absolute flex justify-center align-center w-full pb-14"
            class=("overflow-hidden", show_article)
            id="Article"
        >
        <div
            class="w-full md:w-192 lg:w-full transition duration-300 lg:overflow-visible lg:translate-x-0"
            class=("-translate-x-3/4", show_right)
            class=("md:-translate-x-[85%]", show_right)
        >
        <div class="font-baskerville w-full">
            {children(cx)}
        </div>
        </div>
            <ColumnButton/>
        </div>
        </div>
        <MathJaxTypeset/>
    }
}

#[component]
pub fn Paragraph(
    cx: Scope,
    children: Children,
    #[prop(default = false)] indent: bool,
) -> impl IntoView {
    view! {cx,
        <span
            class="col-start-2 px-4"
            class=("indent-10", indent)
        >
            {children(cx)}
        </span>
    }
}

#[component]
pub fn Columns(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="relative text-xl sm:leading-relaxed -translate-x-[1000px] lg:translate-x-0 grid grid-cols-[1000px_100%_1000px] lg:grid lg:grid-cols-[1fr_32.5rem_1fr]">
            {children(cx)}
        </div>
    }
}

#[component]
fn ColumnButton(cx: Scope) -> impl IntoView {
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let set_page_state = use_context::<WriteSignal<PageState>>(cx).unwrap();

    let show_right = move || page_state() == PageState::ShowRight;
    let show_article = move || page_state() == PageState::ShowArticle;

    view! {cx,
        <button
            on:click=move |_| set_page_state.update(|value| *value = PageState::ShowArticle)
            class="z-40 bg-stone-300/50 hover:bg-stone-400/50 transition duration-300 lg:hidden absolute grid grid-cols-4 justify-end items-center w-full md:w-192 lg:w-full h-full lg:translate-0"
            style="-webkit-tap-highlight-color: transparent;"
            class=("opacity-0", show_article)
            class=("pointer-events-none", show_article)
            class=("opacity-100", show_right)
            class=("-translate-x-3/4", show_right)
            class=("md:-translate-x-[85%]", show_right)
        >
        </button>
    }
}

#[component]
fn ArticleTitle(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="lg:grid lg:grid-cols-[1fr_32.5rem_1fr]">
            <h1 class="lg:col-start-2 text-3xl lg:text-4xl p-4">
                {children(cx)}
            </h1>
        </div>
    }
}

#[component]
fn MathJaxTypeset(cx: Scope) -> impl IntoView {
    view! { cx,
        <script>
            window.MathJax.typesetPromise().then(() => {
                document.querySelectorAll(".hidden-on-startup").forEach((elem) => {
                  elem.classList.remove("hidden-on-startup");
                  elem.classList.add("animate-appear");
                });
            });
        </script>
    }
}

#[component]
fn Math(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <span class="w-fit h-0 inline-flex items-baseline indent-0 hidden-on-startup">
            {children(cx)}
        </span>
    }
}

#[component]
fn MathBlock(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="indent-0 text-xl h-20 flex items-center justify-center col-start-2 hidden-on-startup">
            {children(cx)}
        </div>
    }
}

#[component]
fn Italic(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <span class="font-baskerville-italic">{children(cx)}</span>
    }
}

#[component]
fn Bold(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <span class="font-baskerville-bold">{children(cx)}</span>
    }
}

#[component]
fn ImageRight(cx: Scope, translate: &'static str, src: &'static str) -> impl IntoView {
    let set_page_state =
        use_context::<WriteSignal<PageState>>(cx).expect("set_page_state context to exist");

    view! {cx,
        <div class="col-start-3 h-0 flex items-center justify-start">
            <button
                on:click=move |_| set_page_state.update(|value| *value = match value {
                    PageState::ShowArticle => PageState::ShowRight,
                    PageState::ShowRight => PageState::ShowArticle
                })
                style=move || format!("transform: translate{}", translate)
                class="flex shrink-0 transition-opacity duration-300 lg:transition-none lg:opacity-100 lg:pointer-events-none"
            >
                <img src=src />
            </button>
        </div>
    }
}

#[component]
fn Link(cx: Scope, href: &'static str, children: Children) -> impl IntoView {
    view! {cx,
        <a
            href=href
            class="text-stone-900 hover:text-sky-800"
        >
            {children(cx)}
        </a>
    }
}
