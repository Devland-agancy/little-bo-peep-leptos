use leptos::*;

#[component]
pub fn ImageLink( children: Children) -> impl IntoView {
    view! { 
      <span class="relative cursor-pointer lg:pointer-events-none">
        {children()}
        <img
          src="/images/squiggle.png"
          class="absolute left-[2%] top-[35%] h-[40px] rotate-[91deg] lg:hidden"
        />
      </span>
    }
}
