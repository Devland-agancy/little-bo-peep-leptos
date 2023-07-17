use leptos::*;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="w-full">
            <div class="flex justify-center items-center fixed lg:absolute w-full bg-white z-50 border-b border-t-10 h-14">
                <div class="font-clickerscript text-3xl pt-2 self-end lg:col-start-2 lg:pl-2">
                    <a href="/">"Little Bo Peep"</a>
                </div>
            </div>
        </div>
    }
}
