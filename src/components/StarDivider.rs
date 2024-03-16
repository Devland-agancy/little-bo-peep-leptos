use crate::components::Paragraph::Align;
use crate::components::Span::*;
use leptos::*;

#[component]
pub fn StarDivider(cx: Scope, children: Children) -> impl IntoView {
    return view! {
      cx, <Span align=Align::Center>
        "* * * *"
      </Span>
    };
}
