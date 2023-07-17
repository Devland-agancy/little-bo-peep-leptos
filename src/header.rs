use leptos::*;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="w-full">
            <div class="flex justify-center items-center fixed lg:absolute w-full bg-white z-50 border-b border-t-10 h-14">
                <Title />
                <ChapterMenu />
            </div>
        </div>
    }
}

#[component]
fn Title(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="w-full md:w-192 lg:w-full pl-4 lg:grid lg:grid-cols-[1fr_32.5rem_1fr]" id="Header">
            <div class="font-clickerscript text-3xl pt-2 self-end lg:col-start-2 lg:pl-2">
                <a href="/">"Little Bo Peep"</a>
            </div>
        </div>
    }
}

#[component]
fn ChapterMenu(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="absolute right-0">
            <button class="flex items-center justify-center h-8 w-9 m-3 fill-stone-500 hover:fill-stone-600 bg-white transition">
                <HamburgerIcon />
            </button>
        </div>
    }
}

#[component]
fn HamburgerIcon(cx: Scope) -> impl IntoView {
    view! {cx,
        <svg height="25px" width="30px" viewBox="0 0 450 512">
            <path d="M0 96C0 78.3 14.3 64 32 64H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H32C14.3 128 0 113.7 0 96zM0 256c0-17.7 14.3-32 32-32H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H32c-17.7 0-32-14.3-32-32zM448 416c0 17.7-14.3 32-32 32H32c-17.7 0-32-14.3-32-32s14.3-32 32-32H416c17.7 0 32 14.3 32 32z">
            </path>
        </svg>
    }
}
