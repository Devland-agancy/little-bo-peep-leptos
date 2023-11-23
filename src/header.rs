use leptos::{
    ev::{resize, scroll},
    *,
};
use leptos_use::use_event_listener;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
      <div class="select-none w-full">
        <div class="select-none flex justify-center items-center fixed sm:absolute bg-white z-50 border-b border-t-10 h-14 border-r-0 sm:border-r w-full sm:w-[calc(100vw-3.5rem)]">
          <Title/>
          <MenuButton/>
          <ChapterMenu/>
        </div>
      </div>
    }
}

#[component]
fn Title(cx: Scope) -> impl IntoView {
    view! { cx,
      <div
        class="select-none w-full pl-4 sm:pl-[calc(1rem+3.5rem)] sm:grid sm:grid-cols-[1fr_30.5rem_1fr]"
        id="Header"
      >
        <div class="font-clickerscript text-3xl pt-2 self-end sm:col-start-2 sm:pl-2">
          <a href="/">"Little Bo Peep"</a>
        </div>
      </div>
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum MenuState {
    Open,
    OpenPressed,
    Closed,
    ClosedPressed,
}

#[component]
fn ChapterMenu(cx: Scope) -> impl IntoView {
    view! { cx, <MenuOpen/> }
}

const MENU_ITEMS: &'static [(&'static str, &'static str)] = &[
    ("Chapter 1: A Few Refreshers", "ch_1"),
    ("Chapter 2: Slopes", "ch_2"),
];

#[component]
fn MenuOpen(cx: Scope) -> impl IntoView {
    let menu_state = use_context::<ReadSignal<MenuState>>(cx).unwrap();
    let menu_closed =
        move || menu_state() == MenuState::Closed || menu_state() == MenuState::ClosedPressed;

    view! { cx,
      <div
        class="w-full z-50 fixed translate-x-0 translate-y-0 right-0 top-14 flex self-start font-baskerville text-xl leading-3 sm:leading-5 select-none transition ease-linear  duration-300"
        style=move || format!("transform: translateX({})", if menu_closed() { "100%" } else { "0" })
      >
        <div class="select-none touch-none overscroll-none absolute right-0 w-2/3 max-w-xs z-40 bg-stone-100 overflow-scroll h-[calc(100vh_-_56px)] translate-y-0 sm:translate-y-[-1px]">
          <div class="select-none scrollbar-hidden min-h-[calc(100vh_-_55px)] sm:h-full px-4 py-3 overflow-y-hidden">
            <h2 class="font-baskerville-italic text-2xl pb-2">"Chapters"</h2>
            <MenuItems/>
          </div>
        </div>
      </div>
    }
}

#[component]
fn MenuItems(cx: Scope) -> impl IntoView {
    view! { cx,
      <ul>
        {MENU_ITEMS
            .into_iter()
            .map(|(title, url)| view! { cx, <MenuItem href=url>{title}</MenuItem> })
            .collect_view(cx)}

      </ul>
    }
}

#[component]
fn MenuItem(cx: Scope, href: &'static str, children: Children) -> impl IntoView {
    view! { cx,
      <li class="-indent-6 px-6 pb-1.5 sm:pb-2">
        <a
          href=["/article/", href].concat()
          class="text-stone-900 hover:text-sky-800 text-lg sm:text-xl"
        >
          {children(cx)}
        </a>
      </li>
    }
}

#[component]
fn MenuButton(cx: Scope) -> impl IntoView {
    let set_menu_state = use_context::<WriteSignal<MenuState>>(cx).unwrap();
    let menu_state = use_context::<ReadSignal<MenuState>>(cx).unwrap();
    let menu_closed =
        move || menu_state() == MenuState::Closed || menu_state() == MenuState::ClosedPressed;

    let (button_opacity, set_button_opacity) = create_signal::<f64>(cx, 1_f64);
    let (screen_is_lg, set_screen_is_lg) = create_signal::<bool>(cx, true);
    let (window_scroll, set_window_scroll) = create_signal::<f64>(cx, 0_f64);

    create_effect(cx, move |_| {
        set_button_opacity(1_f64 - window().scroll_y().unwrap() / 800_f64);

        let _ = use_event_listener(cx, window(), scroll, move |_| {
            set_button_opacity(1_f64 - window().scroll_y().unwrap() / 800_f64);
            set_window_scroll(window().scroll_y().unwrap())
        });
    });

    create_effect(cx, move |_| {
        set_screen_is_lg(window().inner_width().unwrap().as_f64().unwrap() >= 640_f64);
        window_event_listener(resize, move |_| {
            set_screen_is_lg(window().inner_width().unwrap().as_f64().unwrap() >= 640_f64);
        })
    });

    view! { cx,
      <div
        style=move || {
            format!(
                "opacity: {}",
                if menu_closed() && screen_is_lg() { button_opacity() } else { 1_f64 },
            )
        }

        class="h-14 w-14 fixed right-0 border-l sm:border-l-0  sm:border-b sm:mt-[1px]"
        class=("hover:border-b-0", move ||  menu_closed() && screen_is_lg() && window_scroll() > 0_f64 )
        class=("sm:border-b-0", move || !menu_closed() || window_scroll() > 56_f64)
      >
        <button
          on:mouseover=move |_| set_button_opacity(1_f64)
          on:pointerdown=move |_| {
              set_menu_state
                  .update(|value| {
                      *value = match value {
                          MenuState::Closed => MenuState::OpenPressed,
                          MenuState::ClosedPressed => MenuState::OpenPressed,
                          MenuState::Open => MenuState::ClosedPressed,
                          MenuState::OpenPressed => MenuState::ClosedPressed,
                      };
                  })
          }

          on:pointerleave=move |_| {
              set_menu_state
                  .update(|value| {
                      *value = match value {
                          MenuState::ClosedPressed => MenuState::Open,
                          MenuState::OpenPressed => MenuState::Closed,
                          _ => *value,
                      }
                  });
              set_button_opacity(1_f64 - window().scroll_y().unwrap() / 800_f64)
          }

          on:pointerup=move |_| {
              set_menu_state
                  .update(|value| {
                      *value = match value {
                          MenuState::Closed => MenuState::Closed,
                          MenuState::ClosedPressed => MenuState::Closed,
                          MenuState::Open => MenuState::Open,
                          MenuState::OpenPressed => MenuState::Open,
                      };
                  })
          }

          class="select-none flex items-center justify-center h-8 w-8 m-3 bg-transparent fill-[rgb(30,30,30)] hover:fill-stone-600"
        >

          <HamburgerIcon/>
        </button>
      </div>
    }
}

#[component]
fn HamburgerIcon(cx: Scope) -> impl IntoView {
    let menu_state = use_context::<ReadSignal<MenuState>>(cx).unwrap();
    let menu_closed =
        move || menu_state() == MenuState::Closed || menu_state() == MenuState::ClosedPressed;

    view! { cx,
      <svg width="30px" height="30px" version="1.1" viewBox="0 0 30 30">
        <g>
          <rect
            x="5"
            y="6"
            width="20"
            height="3"
            rx="1.5"
            ry="1.5"
            class=move || {
                format!("menu-icon-svg {}", if menu_closed() { "" } else { "close-icon-svg-1" })
            }
          >
          </rect>
          <rect
            x="5"
            y="13.5"
            width="20"
            height="3"
            rx="1.5"
            ry="1.5"
            class="menu-icon-svg"
            style=move || format!("opacity: {}", if menu_closed() { "100" } else { "0" })
          ></rect>
          <rect
            x="5"
            y="21"
            width="20"
            height="3"
            rx="1.5"
            ry="1.5"
            class=move || {
                format!("menu-icon-svg {}", if menu_closed() { "" } else { "close-icon-svg-2" })
            }
          >
          </rect>
        </g>
      </svg>
    }
}
