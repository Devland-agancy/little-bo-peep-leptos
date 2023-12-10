use leptos::*;

#[component]
pub fn ArticleTitle(
    cx: Scope,
    label: &'static str,
    #[prop(default = "")] on_mobile: &'static str,
) -> impl IntoView {
    view! { cx,
      <div class="sm:grid gridColsWidth">
        <h1 class="sm:col-start-2 text-3xl sm:text-4xl p-4 mt-2 sm:mt-0">
          <span class="sm:hidden">{if on_mobile == "" {label} else {on_mobile}}</span>
          <span class="hidden sm:block">{label}</span>
        </h1>
      </div>
    }
}
