use crate::{
    components::Image::Image,
    components::VerticalChunk::VerticalChunk,
    components::Section::{Pause, Spacer},
    constants::SECTION_DIVIDER_ACTIVATION_HEIGHT,
    global_state::GlobalState,
};
use leptos::{ev::resize, *};
use leptos_use::use_event_listener;

#[component]
pub fn SectionDivider() -> impl IntoView {
    let (hidden, set_hidden) = create_signal(false);

    create_effect(move |_| {
        request_animation_frame(move || {
            if window().inner_height().unwrap().as_f64().unwrap()
                >= SECTION_DIVIDER_ACTIVATION_HEIGHT as f64
            {
                set_hidden.set(false)
            } else {
                set_hidden.set(true)
            };

            let _ = use_event_listener(window(), resize, move |_| {
                if window().inner_height().unwrap().as_f64().unwrap()
                    >= SECTION_DIVIDER_ACTIVATION_HEIGHT as f64
                {
                    set_hidden.set(false)
                } else {
                    set_hidden.set(true)
                }
            });
        });
    });
    let GlobalState {
        show_section_divider,
        ..
    } = use_context::<GlobalState>().unwrap();

    view! {
      <Show
        fallback=move || view! { <Pause>""</Pause> }
        when=move || !hidden.get() && show_section_divider.get()
      >
      <div class="spacer"></div>
      <VerticalChunk>
        <Image container_classes="section-divider" src="/images/section_divider.svg" width="100%" >
          ""
        </Image>
      </VerticalChunk>
      </Show>
    }
}
