use crate::page::state::PageState;
use leptos::{ev::click, html::Div, *};
use leptos_use::use_event_listener;
use std::time::Duration;
use web_sys::{ScrollBehavior, ScrollToOptions};

#[component]
pub fn Article(cx: Scope, children: Children) -> impl IntoView {
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let set_page_state = use_context::<WriteSignal<PageState>>(cx).unwrap();
    let show_right = move || page_state() == PageState::ShowRight;
    let show_left = move || page_state() == PageState::ShowLeft;
    let show_article = move || page_state() == PageState::ShowArticle;
    let right_image_x_pos = use_context::<ReadSignal<f64>>(cx).unwrap();
    let article_node: NodeRef<Div> = create_node_ref::<Div>(cx);

    // can_click is for disabling click on page transition
    let (can_click, set_can_click) = create_signal(cx, true);

    create_effect(cx, move |_| {
        let mut options = ScrollToOptions::new();
        options.behavior(ScrollBehavior::Smooth);
        if show_right() {
            set_can_click(false);
            options.left(right_image_x_pos());
            window().scroll_with_scroll_to_options(&options);
            set_timeout(
                move || {
                    let _ = article_node().unwrap().style("left", "1500px");
                    options.left(right_image_x_pos() + 1500_f64);
                    options.behavior(ScrollBehavior::Instant);
                    window().scroll_with_scroll_to_options(&options);
                    set_can_click(true);
                },
                Duration::from_millis(800),
            );
        } else if show_left() {
            if article_node().is_some() {
                let _ = article_node().unwrap().style("left", "1500px");
            }
            options.left(1500_f64);
            options.behavior(ScrollBehavior::Instant);
            window().scroll_with_scroll_to_options(&options);

            set_timeout(
                move || {
                    options.left(1000_f64);
                    options.behavior(ScrollBehavior::Smooth);
                    window().scroll_with_scroll_to_options(&options);
                },
                Duration::from_millis(100),
            );
        };
    });

    create_effect(cx, move |_| {
        let _ = use_event_listener(cx, document(), click, move |_| {
            if (show_right() || show_left()) && can_click() {
                let mut options = ScrollToOptions::new();
                options.behavior(ScrollBehavior::Smooth);
                options.left(1500_f64);
                window().scroll_with_scroll_to_options(&options);
                set_can_click(false);
                set_timeout(
                    move || {
                        let _ = article_node().unwrap().style("left", "0");
                        options.behavior(ScrollBehavior::Instant);
                        options.left(0_f64);
                        window().scroll_with_scroll_to_options(&options);
                        set_timeout(
                            move || {
                                set_page_state.update(|value| *value = PageState::ShowArticle);
                                set_can_click(true);
                            },
                            Duration::from_millis(700),
                        )
                    },
                    Duration::from_millis(800),
                );
            }
        });
    });
    // for right_images we autoscroll to their position
    view! { cx,
      <div class="pt-14 xl:pt-20 overscroll-none ">
        <div
          node_ref=article_node
          class="absolute flex justify-center align-center w-full pb-14 min-h-screen"
          class=("overflow-hidden", show_article)
          id="Article"
        >
          <div class="w-full transition duration-300 sm:overflow-visible sm:translate-x-0">
            // for left image we transle based on image width

            <div class="font-baskerville w-full">{children(cx)}</div>
          </div>
          <ColumnButton/>
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
pub fn ColumnButton(cx: Scope) -> impl IntoView {
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();

    let show_right = move || page_state() == PageState::ShowRight;
    let show_left = move || page_state() == PageState::ShowLeft;
    let show_article = move || page_state() == PageState::ShowArticle;

    view! { cx,
      <div

        class="z-40 transition duration-300 lg:hidden absolute grid grid-cols-4 justify-end items-center w-full h-full lg:translate-0"
        style="-webkit-tap-highlight-color: transparent;"
        class=("opacity-0", show_article)
        class=("pointer-events-none", show_article)
        class=("opacity-100", show_right)
        class=("-translate-x-3/4", show_right)
        class=("lg:-translate-x-[85%]", show_right)
        class=("opacity-100", show_left)
        class=("translate-x-3/4", show_left)
        class=("lg:translate-x-[85%]", show_left)
      ></div>
    }
}
