use crate::{
    components::Image::Image,
    components::Paragraph::Paragraph,
    components::Section::{Pause, Spacer},
    constants::SECTION_DIVIDER_ACTIVATION_HEIGHT,
    global_state::GlobalState,
};
use leptos::{ev::resize, *};
use leptos_use::use_event_listener;

#[component]
pub fn SectionDivider(cx: Scope) -> impl IntoView {
    let (hidden, set_hidden) = create_signal(cx, false);

    create_effect(cx, move |_| {
        request_animation_frame(move || {
            if window().inner_height().unwrap().as_f64().unwrap()
                >= SECTION_DIVIDER_ACTIVATION_HEIGHT as f64
            {
                set_hidden(false)
            } else {
                set_hidden(true)
            };

            let _ = use_event_listener(cx, window(), resize, move |_| {
                if window().inner_height().unwrap().as_f64().unwrap()
                    >= SECTION_DIVIDER_ACTIVATION_HEIGHT as f64
                {
                    set_hidden(false)
                } else {
                    set_hidden(true)
                }
            });
        });
    });
    let GlobalState {
        show_section_divider,
        ..
    } = use_context::<GlobalState>(cx).unwrap();

    view! { cx,
      <Show
        fallback=move |_| view! { cx, <Pause>""</Pause> }
        when=move || !hidden() && show_section_divider()
      >
      <Paragraph>
        <Image container_classes="section-divider" src="/images/section_divider.svg" width="100%" >
          ""
          <Spacer/>
        </Image>
      </Paragraph>
      </Show>
    }
}
