use crate::components::Paragraph::*;
use leptos::*;

#[component]
pub fn StarDivider(cx: Scope, children: Children) -> impl IntoView {
    return view! {
      cx, <Paragraph align=Align::Center>
        "* * * *"
      </Paragraph>
    };
}
