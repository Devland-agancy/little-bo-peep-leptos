use leptos::*;

#[component]
pub fn Columns(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
      <div class="leading-[28px] sm:leading-[32.5px]">
        {children(cx)}
      </div>
    }
}
