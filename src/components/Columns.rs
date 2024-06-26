use leptos::*;

#[component]
pub fn Columns(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
      <div class="colmuns relative text-xl leading-[28px] sm:leading-[32.5px] -translate-x-[1500px] sm:translate-x-0 grid grid-cols-[1500px_100%_1500px] sm:grid gridColsWidth -mt-4">
        {children(cx)}
      </div>
    }
}
