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

#[derive(Debug, Clone, PartialEq)]
pub enum MenuState {
    Open,
    Closed,
}

#[component]
fn ChapterMenu(cx: Scope) -> impl IntoView {
    let menu_state = use_context::<ReadSignal<MenuState>>(cx).unwrap();

    let menu_closed = move || menu_state() == MenuState::Closed;

    view! {cx,
        <Show
            when=menu_closed
            fallback=|cx| view! { cx, <MenuOpen/> }
        >
            <MenuClosed/>
        </Show>
    }
}

#[component]
fn MenuClosed(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="absolute right-0">
            <MenuButton/>
        </div>
    }
}

struct MenuItem {
    title: &'static str,
    url: &'static str,
}

const MENU_ITEMS: &'static [(&'static str, &'static str)] = &[
    ("Chapter 1: A Few Refreshers", "ch_1"),
    ("Chapter 2: Slopes", "ch_2"),
];

#[component]
fn MenuOpen(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="absolute right-0">
            <MenuButton/>
        </div>
        <div class="w-full z-50 fixed right-0 top-14 flex self-start font-baskerville text-xl leading-tight select-none">
            <div class="absolute right-0 max-w-full w-80 h-screen z-40 px-4 py-2 bg-stone-100">
                <h2 class="font-baskerville-italic text-2xl pb-2">"Chapters"</h2>
                <MenuItems />
            </div>
        </div>
    }
}

#[component]
fn MenuItems(cx: Scope) -> impl IntoView {
    view! {cx,
        <ul>
        {MENU_ITEMS
            .into_iter()
            .map(|(title, url)| view! {cx, <MenuItem href=url>{title}</MenuItem>})
            .collect_view(cx)
        }
        </ul>
    }
}

#[component]
fn MenuItem(cx: Scope, href: &'static str, children: Children) -> impl IntoView {
    view! {cx,
        <li class="-indent-4 px-4 pb-2">
        <a href=["/article/", href].concat() class="text-stone-900 hover:text-sky-800">
        {children(cx)}
        </a>
        </li>
    }
}

#[component]
fn MenuButton(cx: Scope) -> impl IntoView {
    let menu_state = use_context::<ReadSignal<MenuState>>(cx).unwrap();

    let menu_open = move || menu_state() == MenuState::Open;

    view! {cx,
        <Show
            when=menu_open
            fallback=|cx| view! {cx, <MenuButtonClosed/>}
        >
            <MenuButtonOpen/>
        </Show>
    }
}

#[component]
fn MenuButtonClosed(cx: Scope) -> impl IntoView {
    let set_menu_state = use_context::<WriteSignal<MenuState>>(cx).unwrap();
    view! {cx,
        <button
            on:click=move |_| set_menu_state.update(|value| *value = match value {
                MenuState::Closed => MenuState::Open,
                MenuState::Open => MenuState::Closed
            })
            class="flex items-center justify-center h-8 w-9 m-3 rounded bg-transparent fill-[rgb(30,30,30)] hover:fill-stone-600 active:bg-stone-900 active:fill-stone-100"
        >
            <HamburgerIcon/>
        </button>
    }
}

#[component]
fn MenuButtonOpen(cx: Scope) -> impl IntoView {
    let set_menu_state = use_context::<WriteSignal<MenuState>>(cx).unwrap();
    view! {cx,
        <button
            on:click=move |_| set_menu_state.update(|value| *value = match value {
                MenuState::Closed => MenuState::Open,
                MenuState::Open => MenuState::Closed
            })
            class="flex items-center justify-center h-8 w-9 m-3 rounded bg-stone-900 fill-stone-100 hover:bg-stone-700 hover:fill-stone-50 active:bg-transparent active:fill-[rgb(30,30,30)]"
        >
            <HamburgerIcon/>
        </button>
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
