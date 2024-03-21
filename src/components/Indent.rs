use leptos::*;

#[component]
pub fn Indent(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
      <span
        class="indent-10 inline-table -mt-4"
      >
        {children(cx)}
      </span>
    }
}
