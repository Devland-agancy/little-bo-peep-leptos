use leptos::*;

#[component]
pub fn Columns( children: Children) -> impl IntoView {
    view! { 
      <div class="leading-[28px] sm:leading-[32.5px] -mt-4">
        {children()}
      </div>
    }
}
