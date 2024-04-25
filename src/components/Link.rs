use leptos::*;
use leptos_router::A;

#[component]
pub fn Link(cx: Scope, href: &'static str, children: Children) -> impl IntoView {
    view! { cx,
      <A
        on:click=move |e| { e.stop_propagation() }
        href=href
        class="text-stone-900 hover:text-sky-800"
      >
        {children(cx)}
      </A>
    }
}
