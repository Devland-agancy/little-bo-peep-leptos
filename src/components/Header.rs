use crate::components::Panel::MenuState;
use crate::constants::{HAMBURGER_MENU_SCROLLY_END_FADE, HAMBURGER_MENU_SCROLLY_START_FADE};
use crate::{constants::HAMBURGER_MENU_HEIGHT, global_state::GlobalState};
use leptos::{ev::scroll, *};
use leptos_use::use_event_listener;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
      <div class="select-none w-full ">
        <div class="select-none flex justify-center items-center fixed bg-white z-40 h-14 w-full left-0">
          <Title/>
        </div>
      </div>
      <div
        style=format!("top: {}px", HAMBURGER_MENU_HEIGHT - 1.0)
        class="h-0 border-b left-[-1500px] w-[4400px] fixed z-50"
      ></div>
    }
}

#[component]
fn Title(cx: Scope) -> impl IntoView {
    let GlobalState { route, .. } = use_context(cx).unwrap();

    view! { cx,
      <div class="select-none w-full pl-4 grid gridColsWidth h-full border-r-0" id="Header">
        <div class="font-clickerscript text-3xl pt-2 self-end sm:col-start-2 sm:pl-2 pb-2">
          <a on:click=move |_| route.set("/") href="/">
            "Little Bo Peep"
          </a>
        </div>
      </div>
    }
}

#[component]
pub fn MenuButton(cx: Scope) -> impl IntoView {
    let set_menu_state = use_context::<WriteSignal<MenuState>>(cx).unwrap();

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

    let calc_opacity = move || {
        f64::min(
            1.0,
            1.0 - (window().scroll_y().unwrap() - HAMBURGER_MENU_SCROLLY_START_FADE)
                / HAMBURGER_MENU_SCROLLY_END_FADE,
        )
    };

    create_effect(cx, move |_| {
        set_button_opacity(calc_opacity());

        let _ = use_event_listener(cx, window(), scroll, move |_| {
            set_window_scroll(window().scroll_y().unwrap());
            set_button_opacity(calc_opacity());
        });
    });

    create_effect(cx, move |_| {
        set_scrolled_header(window().scroll_y().unwrap() >= HAMBURGER_MENU_HEIGHT);

        window_event_listener(scroll, move |_| {
            set_scrolled_header(window().scroll_y().unwrap() >= HAMBURGER_MENU_HEIGHT);
        })
    });

    view! { cx,
      <div
        id="menu-button"
        class="h-14 w-14 fixed right-0 border-l sm:border-l-0 border-b z-50"
        class=(
            "hover:border-b-0",
            move || menu_closed() && !on_mobile.get() && window_scroll() > 0_f64,
        )

        class=("sm:border-b-0", move || !menu_closed() || window_scroll() > HAMBURGER_MENU_HEIGHT)
        style=move || {
            format!(
                "opacity: {}",
                if menu_closed() && !on_mobile.get() { button_opacity() } else { 1_f64 },
            )
        }
      >

        <button
          class="select-none flex items-center justify-center h-8 w-8 m-3 fill-[rgb(30,30,30)] hover:fill-stone-600"
          class=("outline-[10px]", move || show_areas())
          class=("outline-black", move || show_areas())
          class=("outline", move || show_areas())

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
              set_button_opacity(calc_opacity())
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
      <div
        class="w-14 fixed right-0 z-40 h-14 "
        class=("h-[10rem]", move || burger_background.get())
        class=("h-[5.25rem]", move || !scrolled_header() && !burger_background.get())

        style=move || {
            format!(
                " background-color: {}; ",
                if scrolled_header() && burger_background.get() {
                    "transparent"
                } else if show_areas() {
                    "#fff000"
                } else {
                    "#fff"
                },
            )
        }
      >
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
