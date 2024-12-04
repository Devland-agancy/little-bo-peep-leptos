use leptos::*;
use crate::global_state::GlobalState;
use crate::constants::CENTERED_PARAGRAPH_X_MARGIN;

#[component]
pub fn CenterDisplay(
    #[prop(default = "")] classes: &'static str,
    children: Children
) -> impl IntoView {
    let GlobalState { show_areas, .. } = use_context::<GlobalState>().unwrap();

    view! {
      <span
        style=format!("margin-inline: {CENTERED_PARAGRAPH_X_MARGIN}")
        class=move || {
            format!(
                "text-center block {} {}",
                classes,
                if show_areas.get() { "bg-[#ebe3a0b0]" } else { "" }
            )
        }
      >
        {children()}
      </span>
    }
}

#[component]
pub fn CentralItalicDisplay(
    #[prop(default = "")] classes: &'static str,
    children: Children
) -> impl IntoView {
    let GlobalState { show_areas, .. } = use_context::<GlobalState>().unwrap();

    view! {
      <span
        style=format!("margin-inline: {CENTERED_PARAGRAPH_X_MARGIN}")
        class=move || {
            format!(
                "font-baskerville-italic text-center block {} {}",
                classes,
                if show_areas.get() { "bg-[#ebe3a0b0]" } else { "" }
            )
        }
      >
        {children()}
      </span>
    }
}

#[component]
pub fn NoBreak(
    children: Children
) -> impl IntoView {
    view! {
      <span
        class="nobreak"
      >
        {children()}
      </span>
    }
}