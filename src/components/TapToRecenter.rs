use crate::global_state::GlobalState;
use leptos::*;

#[component]
pub fn TapToRecenter(cx: Scope) -> impl IntoView {
    let GlobalState {
        margin_mode,
        on_mobile,
        ..
    } = use_context::<GlobalState>(cx).unwrap();

    view! { cx,
      <Show when=move || on_mobile() fallback=move |_| () >
          <div
            class="fixed left-0 bg-[#ffb380ff] p-[.2rem] w-full transition-all duration-500"
            class=("bottom-0", move || margin_mode())
            class=("bottom-[-2em]", move || !margin_mode())
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
