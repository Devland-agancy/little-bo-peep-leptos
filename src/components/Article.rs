use std::time::Duration;

use crate::global_state::GlobalState;
use ev::click;
use leptos::{
    ev::{scrollend, touchend},
    html::Div,
    *,
};
use leptos_use::use_event_listener;
use wasm_bindgen::JsCast;
use web_sys::{ScrollBehavior, ScrollToOptions};

#[component]
pub fn Article(cx: Scope, children: Children) -> impl IntoView {
    let article_node: NodeRef<Div> = create_node_ref::<Div>(cx);
    // can_click is for disabling click on page transition
    let GlobalState {
        on_mobile,
        margin_mode,
        ..
    } = use_context::<GlobalState>(cx).unwrap();

    create_effect(cx, move |_| {
        let mut options = ScrollToOptions::new();
        options.left(1500.0);
        options.behavior(ScrollBehavior::Instant);
        window().scroll_with_scroll_to_options(&options);

        let scroll_back = move || {
            let mut options = ScrollToOptions::new();

            if !margin_mode()
                && window().scroll_x().unwrap() > 1300.0
                && window().scroll_x().unwrap() < 1700.0
            {
                options.left(1500.0);
                options.behavior(ScrollBehavior::Smooth);
                window().scroll_with_scroll_to_options(&options);
                margin_mode.set(false);
                return ();
            } else {
                margin_mode.set(true);
            }
        };

        let _ = use_event_listener(cx, document(), scrollend, move |_| scroll_back());
        let _ = use_event_listener(cx, document(), touchend, move |_| scroll_back());
    });

    create_effect(cx, move |_| {
        let _ = use_event_listener(cx, document(), click, move |ev| {
            if let Some(target) = ev.target() {
                let sidebar = document().get_element_by_id("sidebar").unwrap();
                let menu_btn = document().get_element_by_id("menu-button").unwrap();
                if let Some(element) = target.dyn_ref::<web_sys::Element>() {
                    if sidebar.contains(Some(element)) {}

                    if !sidebar.contains(Some(element)) && !menu_btn.contains(Some(element)) {
                        let mut options = ScrollToOptions::new();
                        options.behavior(ScrollBehavior::Smooth);
                        options.left(1500.0);
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
    });

    // for right_images we autoscroll to their position
    view! { cx,
      <div class="">

        <div
          node_ref=article_node
          class="relative flex justify-center align-center w-full pb-14 min-h-screen left-[1500px]"
          id="Article"
        >
          <div class="w-full transition duration-300 sm:overflow-visible sm:translate-x-0">
            // for left image we transle based on image width

            <div class="font-baskerville w-full">
              {children(cx)}
            </div>
          </div>
          <ColumnButtonLeft/>
          <ColumnButtonRight/>
          <div
            class="fixed left-0 bg-[#ffb380ff] p-[.2rem] w-full transition-all duration-500"
            class=("bottom-0", move || margin_mode())
            class=("bottom-[-2em]", move || !margin_mode())
            >
            <p
              class="w-fit m-auto font-BowlbyOne text-[14px]"
            >
              <Show when=move || on_mobile() fallback=move |_|
                "CLICK ANYWHERE TO RECENTER".into_view(cx)
              >
                "TAP ANYWHERE TO RECENTER"
              </Show>
            </p>
          </div>
        </div>
      </div>
      <MathJaxTypeset/>
    }
}

#[component]
pub fn MathJaxTypeset(cx: Scope) -> impl IntoView {
    view! { cx,
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
pub fn ColumnButtonRight(cx: Scope) -> impl IntoView {
    view! { cx,
      <div
        style="width: 1500px;"
        class="z-40 transition duration-300 absolute grid grid-cols-4 justify-end items-center w-full h-full translate-x-3/4 lg:translate-x-[85%] opacity-100 pointer-events-none"
      ></div>
    }
}

#[component]
pub fn ColumnButtonLeft(cx: Scope) -> impl IntoView {
    view! { cx,
      <div
        style="width: 1500px;"
        class="z-40 transition duration-300 lg:hidden absolute grid grid-cols-4 justify-end items-center w-full h-full -translate-x-3/4 lg:translate-x-[85%] pointer-events-none"
      ></div>
    }
}
