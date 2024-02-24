use crate::{global_state::GlobalState, page::state::PageState};
use leptos::{html::Img, *};

pub enum Direction {
    Left,
    Right,
}

#[component]
pub fn ImageLink(
    cx: Scope,
    direction: Direction,
    scroll_by: f64,
    children: Children,
) -> impl IntoView {
    let set_page_state =
        use_context::<WriteSignal<PageState>>(cx).expect("set_page_state context to exist");
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let GlobalState {
        margin_scroll_value,
        ..
    } = use_context::<GlobalState>(cx).unwrap();

    view! { cx,
      <span
        on:click=move |e| {
          e.stop_propagation();
          return;
          if page_state() == PageState::ShowArticle {
            match direction {
              Direction::Left => {
                set_page_state(PageState::ShowLeft);
                margin_scroll_value
                    .set(scroll_by);
              },
              Direction::Right => {
                set_page_state(PageState::ShowRight);
                margin_scroll_value
                    .set(200.0);
              }
            }
          }
        }
        class="relative cursor-pointer lg:pointer-events-none"
      >
        {children(cx)}
          <img
            src="/images/squiggle.png"
            class="absolute left-[2%] top-[35%] h-[40px] rotate-[91deg] lg:hidden"
          />
      </span>
    }
}
