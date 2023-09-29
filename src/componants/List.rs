use leptos::*;

#[component]
pub fn List(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <ol class="p-4 list-decimal">
            {children(cx)}
        </ol>
    }
}

#[component]
pub fn Item(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <li>
            {children(cx)}
        </li>
    }
}