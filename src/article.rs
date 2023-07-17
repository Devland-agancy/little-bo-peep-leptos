use leptos::*;

pub mod ch_1;

#[component]
pub fn Article(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="pt-14 lg:pt-20 hidden-on-startup">
            <div class="absolute flex justify-center align-center w-full overflow-hidden" id="Article">
                <div class="w-full md:w-192 lg:w-full transition duration-300 lg:overflow-visible lg:translate-x-0">
                    <div class="font-baskerville w-full">
                        {children(cx)}
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Columns(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="relative indent-0 text-xl sm:leading-relaxed -translate-x-[1000px] lg:translate-x-0 grid grid-cols-[1000px_100%_1000px] lg:grid lg:grid-cols-[1fr_32.5rem_1fr]">
            {children(cx)}
        </div>
    }
}

#[component]
fn Math(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="inline-block">
            {children(cx)}
        </div>
    }
}

#[component]
fn MathBlock(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="indent-0 text-xl min-h-[4rem] flex items-center justify-center col-start-2">
            {children(cx)}
        </div>
    }
}

#[component]
fn Paragraph(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <span class="col-start-2 px-4">
            {children(cx)}
        </span>
    }
}
