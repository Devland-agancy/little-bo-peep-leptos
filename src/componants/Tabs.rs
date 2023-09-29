use leptos::{*};
use web_sys::{ScrollBehavior,  ScrollIntoViewOptions};

#[component]
pub fn Tabs(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div
            class="text-xl flex items-center justify-center gap-10 col-start-2 hidden-on-startup"
        >
            {children(cx)}
        </div>
    }
}

#[component]
pub fn TabElement(cx: Scope, label: &'static str, scroll_to: &'static str) -> impl IntoView {

    view! {cx,
        <div
            on:click= move |e| {
                    let scroll_to_element = document().query_selector(scroll_to).unwrap_or(None);
                    if scroll_to_element.is_some() {
                        let mut options = ScrollIntoViewOptions::new();
                        options.behavior(ScrollBehavior::Smooth);
                        scroll_to_element.unwrap().scroll_into_view_with_scroll_into_view_options(&options) 
                    }
            }
            class="cursor-pointer hover:font-bold"
        >
            {label}
        </div>
    }
}