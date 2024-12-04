use crate::components::SectionDivider::SectionDivider;
use leptos::*;

#[component]
pub fn Section(#[prop(default = true)] divider: bool, children: Children) -> impl IntoView {
    return view! {
      <div class="sec">{children()}</div>
      <Show fallback=|| () when=move || divider>
        <SectionDivider/>
      </Show>
    };
}

#[component]
pub fn Paragraphs(children: Children) -> impl IntoView {
    return view! { <div class="">{children()}</div> };
}

#[component]
pub fn Example(children: Children) -> impl IntoView {
    return view! { <div class="example !mt-0">{children()}</div> };
}

#[component]
pub fn Pause(
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    return view! { <div class="pause">{children.map(|c| c())}</div> };
}

#[component]
pub fn Spacer(
    #[prop(default = false)] inner: bool,
    #[prop(default = false)] before: bool,
) -> impl IntoView {
    return view! {
      <div class="spacer col-start-2" class=("inner-spacer", inner) class=("before-spacer", before)>
        ""
      </div>
    };
}

#[component]
pub fn Center(children: Children, #[prop(default = "")] style: &'static str) -> impl IntoView {
    return view! {
      <div class="block-element" align="center" style=style>{children()}</div>
      <Spacer />
    };
}
