pub mod article;
pub mod home;
pub mod state;

use leptos::*;
use state::PageState;

#[component]
pub fn Article(cx: Scope, children: Children) -> impl IntoView {
    let (page_state, set_page_state) = create_signal(cx, PageState::ShowArticle);
    provide_context(cx, set_page_state);
    let show_right = move || page_state() == PageState::ShowRight;

    view! { cx,
        <div
            class="pt-14 lg:pt-20 hidden-on-startup"
        >    {move || page_state.get().to_string()}
            <div class="absolute flex justify-center align-center w-full overflow-hidden" id="Article">
                <div
                    class="w-full md:w-192 lg:w-full transition duration-300 lg:overflow-visible lg:translate-x-0"
                    class=("-translate-x-3/4", show_right)
                    class=("md:-translate-x-[85%]", show_right)
                >
                    <div class="font-baskerville w-full">
                        {children(cx)}
                    </div>
                </div>
            </div>
        </div>
        <MathJaxTypeset/>
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
fn Columns(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="relative indent-0 text-xl sm:leading-relaxed -translate-x-[1000px] lg:translate-x-0 grid grid-cols-[1000px_100%_1000px] lg:grid lg:grid-cols-[1fr_32.5rem_1fr]">
            {children(cx)}
        </div>
    }
}

#[component]
fn Math(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="inline-block">
            {children(cx)}
        </div>
    }
}

#[component]
fn MathBlock(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="indent-0 text-xl min-h-[4rem] flex items-center justify-center col-start-2">
            {children(cx)}
        </div>
    }
}

#[component]
fn Paragraph(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <span class="col-start-2 px-4">
            {children(cx)}
        </span>
    }
}

#[component]
fn Italic(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <span class="font-baskerville-italic">{children(cx)}</span>
    }
}

#[component]
fn ImageRight(cx: Scope, translate: &'static str, src: &'static str) -> impl IntoView {
    let set_page_state =
        use_context::<WriteSignal<PageState>>(cx).expect("set_page_state context to exist");

    view! {cx,
        <div class="col-start-3 h-0 flex items-center justify-start">
            <button
                on:click=move |_| set_page_state.update(|value| *value = PageState::ShowRight)
                style=move || format!("transform: translate{}", translate)
                class="flex shrink-0 transition-opacity duration-300 lg:transition-none lg:opacity-100 lg:pointer-events-none"
            >
                <img src=src />
            </button>
        </div>
    }
}
