use leptos::{html::{Img} ,*};
use crate::page::state::PageState;

#[component]
pub fn ImageLeft(
    cx: Scope,
    translate: &'static str,
    src: &'static str,
    #[prop(default = false)] hidden_in_mobile: bool,
    #[prop(default = false)] absolute: bool,
    #[prop(optional)] top: i32,
    #[prop(optional)] left: i32,
    #[prop(default = "")] children_translate: &'static str,

    #[prop(default = "-1.5rem")] squiggle_right: &'static str,
    #[prop(default = "46%")] squiggle_top: &'static str,
    
    children: Children,

) -> impl IntoView {
    let set_page_state =
        use_context::<WriteSignal<PageState>>(cx).expect("set_page_state context to exist");
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let show_left = move || page_state() == PageState::ShowLeft;

    view! {cx,
        <div class="col-start-1 h-0 flex items-center justify-end">
            <button
                on:click=move |e| {
                    e.stop_propagation();
                    set_page_state.update(|value| *value = match value {
                    PageState::ShowArticle => PageState::ShowLeft,
                    _ => PageState::ShowArticle
                    });
                }
                style=move || format!("transform: translate{}; left: {}px; top: {}px", translate, left, top)
                class="flex shrink-0 transition-opacity duration-300 lg:transition-none lg:opacity-100 lg:pointer-events-none z-10"
                class=("pointer-events-none", show_left)
                class=("absolute", move || absolute)
            >
                <div
                    style=move || format!("transform: translate{}", children_translate)
                >
                    {children(cx)}
                </div>
                <img src=src />
                
                <Show fallback=|_| () when=move || hidden_in_mobile >
                    <div class="block sm:hidden absolute"
                    style=move || format!("right: {}; top: {}", squiggle_right, squiggle_top)
                    >
                        <img src="/images/squiggle.png" class="h-11" />
                    </div>
                </Show>
            </button>
        </div>
    }
}
