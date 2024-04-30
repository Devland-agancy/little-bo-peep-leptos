use crate::constants::CENTERED_PARAGRAPH_X_MARGIN;
use crate::{components::Paragraph::Align, global_state::GlobalState};
use leptos::*;

#[component]
pub fn Span(
    cx: Scope,
    #[prop(default = "")] classes: &'static str,
    #[prop(default = false)] bold: bool,
    #[prop(default = false)] italic: bool,
    #[prop(default = Align::None)] align: Align,
    children: Children,
) -> impl IntoView {
    let GlobalState { show_areas, .. } = use_context::<GlobalState>(cx).unwrap();
    let centered = align == Align::Center;

    view! { cx,
        <span
            style=format!("margin-inline: {}", if centered { CENTERED_PARAGRAPH_X_MARGIN } else {"0"})
            class=move || {
                format!(
                    "{} {} {} {} {}",
                    classes,
                    if italic { "font-baskerville-italic" } else { "" },
                    if bold { "font-baskerville-bold" } else { "" },
                    if centered { "text-center block" } else { "" },
                    if centered && show_areas() { "bg-[#ebe3a0b0]" } else { "" }
            )}
        >{children(cx)}</span>
    }
}
