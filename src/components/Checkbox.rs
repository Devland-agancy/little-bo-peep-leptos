use leptos::*;

#[component]
pub fn Checkbox(cx: Scope, value: RwSignal<bool>) -> impl IntoView {
    view! {
        cx,
        <div
          on:click=move |_|{
            value.set(!value.get())
          }
          class="w-4 h-4  border border-black rounded flex items-center justify-center cursor-pointer"
          class=("bg-[#c1ebff]", move || value.get())
          class=("hover:bg-[#9ac1d3]", move || value.get())
          class=("bg-white", move || !value.get())
          class=("hover:bg-[#f3f3f3]", move || !value.get())
         >
          <svg width="14" height="9" viewBox="0 0 13 13" fill="none" xmlns="http://www.w3.org/2000/svg">
            <rect width="2.09" height="7.33987" rx="1.045" transform="matrix(0.460058 -0.887889 0.625737 0.780034 0 6.85571)" fill="white"/>
            <rect width="2.09" height="13.38" rx="1.045" transform="matrix(0.529272 0.848452 -0.560655 0.828049 11.5157 0)" fill="white"/>
          </svg>
        </div>
    }
}
