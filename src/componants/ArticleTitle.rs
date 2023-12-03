use leptos::*;

#[component]
pub fn ArticleTitle(
    cx: Scope,
    #[prop(optional)] on_mobile: &'static str,
    children: Children,
) -> impl IntoView {
    view! { cx,
      <div class="sm:grid sm:grid-cols-[1fr_456px_1fr]">
        <h1 class="sm:col-start-2 text-3xl sm:text-4xl p-4">
          <span class="sm:hidden">{on_mobile}</span>
          <span class="hidden sm:block">{children(cx)}</span>
        </h1>
      </div>
    }
}
