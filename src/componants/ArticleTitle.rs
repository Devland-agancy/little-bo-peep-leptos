use leptos::*;

#[component]
pub fn ArticleTitle(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
      <div class="sm:grid sm:grid-cols-[1fr_30.5rem_1fr]">
        <h1 class="sm:col-start-2 text-3xl sm:text-4xl p-4">{children(cx)}</h1>
      </div>
    }
}