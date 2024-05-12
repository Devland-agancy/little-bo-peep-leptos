use crate::components::SectionDivider::SectionDivider;
use leptos::*;

#[component]
pub fn Section(
    cx: Scope,
    #[prop(default = true)] divider: bool,

    children: Children,
) -> impl IntoView {
    return view! {
      cx, <div class="sec col-start-2">
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
      cx, <div class="col-start-2">
        {children(cx)}
      </div>
    };
}

#[component]
pub fn Example(cx: Scope, children: Children) -> impl IntoView {
    return view! {
      cx, <div class="example col-start-2 !mt-0">
        {children(cx)}
      </div>
    };
}

#[component]
pub fn Pause(cx: Scope, children: Children) -> impl IntoView {
    return view! {
      cx, <div class="pause">
        {children(cx)}
      </div>
    };
}

#[component]
pub fn Spacer(cx: Scope) -> impl IntoView {
    return view! {
      cx, <div class="spacer">
        ""
      </div>
    };
}
