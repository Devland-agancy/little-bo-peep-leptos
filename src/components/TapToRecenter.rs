use crate::global_state::GlobalState;
use leptos::*;

#[component]
pub fn TapToRecenter() -> impl IntoView {
    let GlobalState {
        margin_mode,
        on_mobile,
        ..
    } = use_context::<GlobalState>().unwrap();

    view! {
      <Show when=move || on_mobile.get() fallback=move || () >
          <div
            class="fixed left-0 bg-[#ffb380ff] p-[.2rem] w-full transition-all duration-500"
            class=("bottom-0", move || margin_mode.get())
            class=("bottom-[-2em]", move || !margin_mode.get())
          >
          <p
          class="w-fit m-auto font-BowlbyOne text-[14px]"
          >
            "TAP TO RECENTER"
          </p>
        </div>
      </Show>
    }
}
