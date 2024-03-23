use leptos::{html::Span, *};

#[component]
pub fn Indent(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
      <span
        class="indent indent-10 block"
      >
        {children(cx)}
      </span>
    }
}
