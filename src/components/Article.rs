use std::time::Duration;

use crate::global_state::GlobalState;
use ev::click;
use leptos::{
    ev::{scrollend, touchend},
    html::Div,
    *,
};
use leptos_use::{use_document, use_event_listener};
use wasm_bindgen::JsCast;
use web_sys::{ScrollBehavior, ScrollToOptions};

#[component]
pub fn Article(children: Children) -> impl IntoView {
    // can_click is for disabling click on page transition
    let GlobalState { margin_mode, .. } = use_context::<GlobalState>().unwrap();

    create_effect(move |_| {
        let options = ScrollToOptions::new();
        options.set_left(1500.0);
        options.set_behavior(ScrollBehavior::Instant);
        window().scroll_with_scroll_to_options(&options);

        let scroll_back = move || {
            let options = ScrollToOptions::new();

            if !margin_mode.get()
                && window().scroll_x().unwrap() > 1300.0
                && window().scroll_x().unwrap() < 1700.0
            {
                options.set_left(1500.0);
                options.set_behavior(ScrollBehavior::Smooth);
                window().scroll_with_scroll_to_options(&options);
                margin_mode.set(false);
                return ();
            } else {
                margin_mode.set(true);
            }
        };

        let _ = use_event_listener(document(), scrollend, move |_| scroll_back());
        let _ = use_event_listener(document(), touchend, move |_| scroll_back());
    });

    let _ = use_event_listener(use_document(), click, move |ev| {
        if let Some(target) = ev.target() {
            let sidebar = document().get_element_by_id("sidebar").unwrap();
            let menu_btn = document().get_element_by_id("menu-button").unwrap();
            if let Some(element) = target.dyn_ref::<web_sys::Element>() {
                if sidebar.contains(Some(element)) {}
                if !sidebar.contains(Some(element)) && !menu_btn.contains(Some(element)) {
                    let options = ScrollToOptions::new();
                    options.set_behavior(ScrollBehavior::Smooth);
                    options.set_left(1500.0);
                    window().scroll_with_scroll_to_options(&options);
                    set_timeout(
                        move || {
                            margin_mode.set(false);
                        },
                        Duration::from_millis(100),
                    );
                }
            }
        }
    });

    // for right_images we autoscroll to their position
    view! {
      <div class="">
        <div
          class="relative flex justify-center align-center w-full min-h-screen left-[1500px]"
          id="Article"
        >
          <div class="w-full transition duration-300 sm:overflow-visible sm:translate-x-0">
            // for left image we transle based on image width

            <div class="font-baskerville w-full">
              {children()}
            </div>
          </div>
          <ColumnButtonLeft/>
          <ColumnButtonRight/>

        </div>
      </div>
      <MathJaxTypeset/>
    }
}

#[component]
pub fn MathJaxTypeset() -> impl IntoView {
    view! {
      <script>
        window.MathJax.typesetPromise().then(() => {
            document.querySelectorAll(".hidden-on-startup").forEach((elem) => {
              elem.classList.remove("hidden-on-startup");
              elem.classList.add("animate-appear");
            });
        });
      </script>
    }
}

#[component]
pub fn ColumnButtonRight() -> impl IntoView {
    view! {
      <div
        style="width: 1500px;"
        class="z-40 transition duration-300 absolute grid grid-cols-4 justify-end items-center w-full h-full translate-x-3/4 lg:translate-x-[85%] opacity-100 pointer-events-none"
      ></div>
    }
}

#[component]
pub fn ColumnButtonLeft() -> impl IntoView {
    view! {
      <div
        style="width: 1500px;"
        class="z-40 transition duration-300 lg:hidden absolute grid grid-cols-4 justify-end items-center w-full h-full -translate-x-3/4 lg:translate-x-[85%] pointer-events-none"
      ></div>
    }
}
