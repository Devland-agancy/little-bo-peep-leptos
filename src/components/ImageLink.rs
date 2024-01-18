use crate::{global_state::GlobalState, page::state::PageState};
use leptos::*;

pub enum Direction {
    Left,
    Right,
}

#[component]
pub fn ImageLink(cx: Scope, direction: Direction, children: Children) -> impl IntoView {
    let set_page_state =
        use_context::<WriteSignal<PageState>>(cx).expect("set_page_state context to exist");
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let set_right_image_x_pos = use_context::<WriteSignal<f64>>(cx).unwrap();

    view! { cx,
      <span
        on:click=move |e| {
          e.stop_propagation();
          if page_state() == PageState::ShowArticle {
            match direction {
              Direction::Left => {
                set_page_state(PageState::ShowLeft);
              },
              Direction::Right => {
                set_page_state(PageState::ShowRight);
                set_right_image_x_pos
                    .update(|val| {
                        *val = 200.0;
                    });
              }
            }
          }
        }
        class="relative cursor-pointer text-red-800"
      >
        {children(cx)}
          <img
            src="/images/squiggle.png"
            class="absolute left-[5%] top-[45%] w-[35px] h-[35px] rotate-90"
          />
      </span>
    }
}
