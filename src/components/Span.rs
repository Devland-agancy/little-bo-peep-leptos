use crate::constants::CENTERED_PARAGRAPH_X_MARGIN;
use crate::{components::VerticalChunk::Align, global_state::GlobalState};
use leptos::*;

#[component]
pub fn Span(
    #[prop(default = "")] classes: &'static str,
    #[prop(default = false)] bold: bool,
    #[prop(default = false)] italic: bool,
    #[prop(default = Align::None)] align: Align,
    children: Children,
) -> impl IntoView {
    let GlobalState { show_areas, .. } = use_context::<GlobalState>().unwrap();
    let centered = align == Align::Center;

    view! {
      <span
        style=format!("margin-inline: {}", if centered { CENTERED_PARAGRAPH_X_MARGIN } else { "0" })
        class=move || {
            format!(
                "{} {} {} {} {}",
                classes,
                if italic { "font-baskerville-italic" } else { "" },
                if bold { "font-baskerville-bold" } else { "" },
                if centered { "block-element text-center block my-4" } else { "" },
                if centered && show_areas.get() { "bg-[#ebe3a0b0]" } else { "" },
            )
        }
      >

        {children()}
      </span>
    }
}

#[component]
pub fn DisplayCentered(
    #[prop(default = "")] classes: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
      <Span classes=classes align=Align::Center>
        {children()}
      </Span>
    }
}

