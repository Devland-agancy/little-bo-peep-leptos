use crate::componants::Paragraph::Align;
use leptos::*;

#[component]
pub fn Span(
    cx: Scope,
    #[prop(default = "")] _class: &'static str,
    #[prop(default = false)] bold: bool,
    #[prop(default = false)] italic: bool,
    #[prop(default = Align::None)] align: Align,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <span class=move || {
            format!(
                "{} {} {} {}",
                _class,
                if italic { "font-baskerville-italic" } else { "" },
                if bold { "font-baskerville-bold" } else { "" },
                if align == Align::Center { "text-center mt-4 block" } else { "" }
            )}


        >{children(cx)}</span>
    }
}
