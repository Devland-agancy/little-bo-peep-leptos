use crate::components::SectionDivider::SectionDivider;
use leptos::*;

#[component]
pub fn Section(
    cx: Scope,
    #[prop(default = "")] label: &'static str,
    #[prop(default = true)] divider: bool,

    children: Children,
) -> impl IntoView {
    return view! {
      cx, <div class="flex flex-col container col-start-2 my-2">
            {children(cx)}
          </div>
            <Show fallback=|_| () when=move || divider>
            <SectionDivider />
          </Show>
    };
}

#[component]
pub fn Paragraphs(cx: Scope, children: Children) -> impl IntoView {
    return view! {
      cx, <div class="flex flex-col container col-start-2">
        {children(cx)}
      </div>
    };
}

#[component]
pub fn Example(cx: Scope, children: Children) -> impl IntoView {
    return view! {
      cx, <div class="flex flex-col container col-start-2">
        {children(cx)}
      </div>
    };
}

#[component]
pub fn Pause(
    cx: Scope,
    #[prop(default = "0")] amount: &'static str,
    children: Children,
) -> impl IntoView {
    return view! {
      cx, <div class="pause" style=move ||format!("margin-block : {}", amount)>
        {children(cx)}
      </div>
    };
}
