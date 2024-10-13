use leptos::*;

#[component]
pub fn Math( children: Children) -> impl IntoView {
    view! { 
      <span class="w-fit inline-flex items-baseline indent-0 hidden-on-startup">
        {children()}
      </span>
    }
}
