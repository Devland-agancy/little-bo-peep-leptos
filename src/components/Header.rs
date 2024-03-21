use crate::components::Checkbox::Checkbox;
use crate::page::state::PageState;
use crate::{
    constants::{MENU_ITEMS, MOBILE_BREAKPOINT},
    global_state::GlobalState,
};
use leptos::{
    ev::{resize, scroll},
    *,
};
use leptos_use::use_event_listener;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
      <div class="select-none w-full ">
        <div class="select-none flex justify-center items-center fixed bg-white z-40 h-14 w-full left-0">
          <Title/>
        </div>
      </div>
      <div class="h-0 border-b left-[-1500px] w-[4400px] top-[55px] fixed z-50">
      </div>
    }
}

#[component]
fn Title(cx: Scope) -> impl IntoView {
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let GlobalState { route, .. } = use_context(cx).unwrap();

    view! { cx,
      <div
        class="select-none w-full pl-4 grid gridColsWidth h-full border-r-0"
        id="Header"
      >
        <div class="font-clickerscript text-3xl pt-2 self-end sm:col-start-2 sm:pl-2 pb-2">
          <a on:click=move |_| route.set("/") href="/">"Little Bo Peep"</a>
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
pub fn ChapterMenu(cx: Scope) -> impl IntoView {
    view! { cx, <MenuOpen/> }
}

#[component]
fn MenuOpen(cx: Scope) -> impl IntoView {
    let menu_state = use_context::<ReadSignal<MenuState>>(cx).unwrap();
    let menu_closed =
        move || menu_state() == MenuState::Closed || menu_state() == MenuState::ClosedPressed;
    let GlobalState {
        show_areas,
        show_section_divider,
        ..
    } = use_context::<GlobalState>(cx).unwrap();
    view! { cx,
      <div
        class="w-full z-50 fixed translate-x-0 translate-y-0 right-0 top-14 flex self-start font-baskerville text-xl leading-3 sm:leading-5 select-none transition ease-linear  duration-300"
        style=move || format!("transform: translateX({})", if menu_closed() { "100%" } else { "0" })
      >
        <div class="select-none touch-none overscroll-none absolute right-0 w-2/3 max-w-xs z-40 bg-stone-100 overflow-scroll h-[calc(100vh_-_56px)] translate-y-0 sm:translate-y-[-1px]">
          <div class="select-none scrollbar-hidden min-h-[calc(100vh_-_55px)] sm:h-full px-4 py-3 overflow-y-hidden">
            <h2 class="font-baskerville-italic text-2xl pb-1.5 sm:pb-2">"Chapters"</h2>
            <MenuItems/>
            <h2 class="font-baskerville-italic text-2xl mt-4 pb-1.5 sm:pb-2 text-right">"Options"</h2>
            <div class="flex items-center justify-end gap-2 text-lg sm:text-xl pb-1.5 sm:pb-2">
              <p>"Show Areas"</p>
              <Checkbox value=show_areas />
            </div>
            <div class="flex items-center justify-end gap-2 text-lg sm:text-xl">
              <p>"Show Section Dividers"</p>
              <Checkbox value=show_section_divider />
            </div>
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
            .map(|(title, on_mobile, url)| view! { cx, <MenuItem label=title on_mobile=on_mobile href=url /> })
            .collect_view(cx)}

      </ul>
    }
}

#[component]
fn MenuItem(
    cx: Scope,
    href: &'static str,
    label: &'static str,
    #[prop(optional)] on_mobile: &'static str,
) -> impl IntoView {
    let GlobalState { route, .. } = use_context(cx).unwrap();

    view! { cx,
      <li class="-indent-6 px-6 pb-1.5 sm:pb-2">
        <a
          on:click=move |_|{
            route.set(href);
          }
          href=["/article/", href].concat()
          class="text-stone-900 hover:text-sky-800 text-lg sm:text-xl"
        >
        <span class="sm:hidden">{if on_mobile == "" {label} else {on_mobile}}</span>
        <span class="hidden sm:block">{label}</span>
        </a>
      </li>
    }
}

#[component]
pub fn MenuButton(cx: Scope) -> impl IntoView {
    let set_menu_state = use_context::<WriteSignal<MenuState>>(cx).unwrap();
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();

    let menu_state = use_context::<ReadSignal<MenuState>>(cx).unwrap();
    let menu_closed =
        move || menu_state() == MenuState::Closed || menu_state() == MenuState::ClosedPressed;
    let GlobalState {
        show_areas,
        burger_background,
        on_mobile,
        ..
    } = use_context::<GlobalState>(cx).unwrap();

    let (button_opacity, set_button_opacity) = create_signal::<f64>(cx, 1_f64);
    let (window_scroll, set_window_scroll) = create_signal::<f64>(cx, 0_f64);
    let (scrolled_header, set_scrolled_header) = create_signal::<bool>(cx, true);

    create_effect(cx, move |_| {
        set_button_opacity(1_f64 - window().scroll_y().unwrap() / 800_f64);

        let _ = use_event_listener(cx, window(), scroll, move |_| {
            set_button_opacity(1_f64 - window().scroll_y().unwrap() / 800_f64);
            set_window_scroll(window().scroll_y().unwrap())
        });
    });

    create_effect(cx, move |_| {
        set_scrolled_header(window().scroll_y().unwrap() >= 56_f64);

        window_event_listener(scroll, move |_| {
            set_scrolled_header(window().scroll_y().unwrap() >= 56_f64);
        })
    });

    view! { cx,
      <div
        class="h-14 w-14 fixed right-0 border-l sm:border-l-0 border-b z-50"
        class=("hover:border-b-0", move ||  menu_closed() && !on_mobile.get() && window_scroll() > 0_f64 )
        class=("sm:border-b-0", move || !menu_closed() || window_scroll() > 56_f64)
        style=move || {
          format!(
              "opacity: {}",
              if menu_closed() && !on_mobile.get() { button_opacity() } else { 1_f64 }
            )
        }

      >
        <button
          class="select-none flex items-center justify-center h-8 w-8 m-3 fill-[rgb(30,30,30)] hover:fill-stone-600"

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
        >
          <HamburgerIcon/>
        </button>
      </div>

      //vertical line between burger and rest of header
      <div
        class="hidden sm:block h-14 w-[1px] absolute right-14 top-0 border-r transition-opacity z-50 opacity-0"
        class=("opacity-100", move || page_state() == PageState::ShowArticle)

        ></div>
        <div

        class="w-14 fixed right-0 z-40 h-14 "
        class=("h-[10rem]", move || burger_background.get())
        class=("h-[5.25rem]", move || !scrolled_header() && !burger_background.get())

        style=move || {
        format!(
            " background-color: {}; ",
            if scrolled_header() && burger_background.get() { "transparent" } else if show_areas() { "#fff000" } else { "#fff" },
          )
      } >
    </div>
    }
}

#[component]
fn HamburgerIcon(cx: Scope) -> impl IntoView {
    let menu_state = use_context::<ReadSignal<MenuState>>(cx).unwrap();
    let menu_closed =
        move || menu_state() == MenuState::Closed || menu_state() == MenuState::ClosedPressed;
    view! { cx,
      <svg
       width="30px" height="30px" version="1.1" viewBox="0 0 30 30">
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
