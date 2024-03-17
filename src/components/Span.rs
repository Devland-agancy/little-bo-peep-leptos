use crate::components::Paragraph::Align;
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
    view! { cx,
        <span class=move || {
            format!(
                "{} {} {} {}",
                classes,
                if italic { "font-baskerville-italic" } else { "" },
                if bold { "font-baskerville-bold" } else { "" },
                if align == Align::Center { "text-center my-2 block" } else { "" }
            )}


        >{children(cx)}</span>
    }
}
