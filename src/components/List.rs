use leptos::*;

#[component]
pub fn List(cx: Scope, #[prop(default = true)] indent: bool, children: Children) -> impl IntoView {
    view! { cx,
      <ol class=("ml-6", indent) class="px-4 list-decimal">
        {children(cx)}
      </ol>
    }
}

#[component]
pub fn Item(cx: Scope, children: Children) -> impl IntoView {
    view! { cx, <li>{children(cx)}</li> }
}
