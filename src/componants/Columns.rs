use leptos::*;

#[component]
pub fn Columns(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="relative text-xl sm:leading-relaxed -translate-x-[1500px] sm:translate-x-0 grid grid-cols-[1500px_100%_1500px] sm:grid sm:grid-cols-[1fr_30.5rem_1fr]">
            {children(cx)}
        </div>
    }
}