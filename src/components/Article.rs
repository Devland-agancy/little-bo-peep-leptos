use crate::{global_state::GlobalState, page::state::PageState};
use leptos::{
    ev::{click, scrollend, touchend},
    html::Div,
    *,
};
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
    let article_node: NodeRef<Div> = create_node_ref::<Div>(cx);
    let GlobalState {
        margin_scroll_value,
        ..
    } = use_context::<GlobalState>(cx).unwrap();
    // can_click is for disabling click on page transition
    let (can_click, set_can_click) = create_signal(cx, true);
    let (state_changed_by_scroll, set_state_changed_by_scroll) = create_signal(cx, false);

    create_effect(cx, move |_| {
        let mut options = ScrollToOptions::new();
        options.left(1500.0);
        options.behavior(ScrollBehavior::Instant);
        window().scroll_with_scroll_to_options(&options);

        let scroll_back = move || {
            if show_article() {
                let mut options = ScrollToOptions::new();
                set_can_click(true);

                if !state_changed_by_scroll()
                    && window().scroll_x().unwrap() > 1300.0
                    && window().scroll_x().unwrap() < 1700.0
                {
                    options.left(1500.0);
                    options.behavior(ScrollBehavior::Smooth);
                    window().scroll_with_scroll_to_options(&options);
                    return ();
                } else {
                    set_state_changed_by_scroll(true);
                }
            }
        };

        let _ = use_event_listener(cx, document(), scrollend, move |_| scroll_back());
        let _ = use_event_listener(cx, document(), touchend, move |_| scroll_back());
    });

    create_effect(cx, move |_| {
        let _ = use_event_listener(cx, document(), click, move |_| {
            let mut options = ScrollToOptions::new();
            options.behavior(ScrollBehavior::Smooth);
            options.left(1500.0);
            window().scroll_with_scroll_to_options(&options);
        });
    });
    // for right_images we autoscroll to their position
    view! { cx,
      <div class="">
        <div
          node_ref=article_node
          class="relative flex justify-center align-center w-full pb-14 min-h-screen left-[1500px]"
          class=("", show_article)
          id="Article"
        >
          <div class="w-full transition duration-300 sm:overflow-visible sm:translate-x-0">
            // for left image we transle based on image width

            <div class="font-baskerville w-full">{children(cx)}</div>
          </div>
          <ColumnButtonLeft />
          <ColumnButtonRight />

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
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();

    let show_right = move || page_state() == PageState::ShowRight;
    let show_left = move || page_state() == PageState::ShowLeft;
    let show_article = move || page_state() == PageState::ShowArticle;

    view! { cx,
      <div
        style="width: 1500px;"
        class="z-40 transition duration-300 absolute grid grid-cols-4 justify-end items-center w-full h-full translate-x-3/4 lg:translate-x-[85%]"
        class=("opacity-0", show_article)
        class=("pointer-events-none", show_article)
        class=("opacity-100", show_left)
        class=("-translate-x-3/4", || true)
        class=("lg:-translate-x-[85%]", || true)
        class=("opacity-100", || true)

      ></div>
    }
}

#[component]
pub fn ColumnButtonLeft(cx: Scope) -> impl IntoView {
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();

    let show_right = move || page_state() == PageState::ShowRight;
    let show_left = move || page_state() == PageState::ShowLeft;
    let show_article = move || page_state() == PageState::ShowArticle;

    view! { cx,
      <div
        style="width: 1500px;"
        class="z-40 transition duration-300 lg:hidden absolute grid grid-cols-4 justify-end items-center w-full h-full lg:translate-0"
        class=("opacity-0", show_article)
        class=("pointer-events-none", show_article)
        class=("opacity-100", show_right)
        class=("-translate-x-3/4", || true)
        class=("lg:-translate-x-[85%]", show_right)
        class=("opacity-100", || true)
        class=("translate-x-3/4", show_left)
        class=("lg:translate-x-[85%]", || true)
      ></div>
    }
}
