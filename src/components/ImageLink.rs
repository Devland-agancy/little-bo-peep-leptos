use crate::{global_state::GlobalState, page::state::PageState};
use leptos::*;

#[component]
pub fn ImageLink(
    cx: Scope,
    direction: &'static str,
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
