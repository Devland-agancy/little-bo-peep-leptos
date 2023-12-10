use leptos::*;

#[component]
pub fn Math(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
      <span class="w-fit inline-flex items-baseline indent-0 hidden-on-startup">
        {children(cx)}
      </span>
    }
}
