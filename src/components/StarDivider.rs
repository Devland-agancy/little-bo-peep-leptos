use crate::components::VerticalChunk::Align;
use crate::components::Span::*;
use leptos::*;

#[component]
pub fn StarDivider() -> impl IntoView {
    return view! { <Span align=Align::Center>"* * * *"</Span> };
}
