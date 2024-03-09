use leptos::*;

#[component]
pub fn Section(
    cx: Scope,
    #[prop(default = "")] label: &'static str,
    #[prop(default = false)] include_section_icon: bool,

    children: Children,
) -> impl IntoView {
    return view! {
      cx, <div class="flex flex-col gap-4 col-start-2 my-2">
        {children(cx)}
      </div>
    };
}

#[component]
pub fn Paragraphs(cx: Scope, children: Children) -> impl IntoView {
    return view! {
      cx, <div class="flex flex-col gap-4 col-start-2">
        {children(cx)}
      </div>
    };
}

#[component]
pub fn Example(cx: Scope, children: Children) -> impl IntoView {
    return view! {
      cx, <div class="flex flex-col gap-4 col-start-2">
        {children(cx)}
      </div>
    };
}

#[component]
pub fn Pause(cx: Scope, amount: &'static str, children: Children) -> impl IntoView {
    return view! {
      cx, <div style=move ||format!("marginInsetBlock : {}", amount)>
        {children(cx)}
      </div>
    };
}
