use leptos::*;

#[derive(PartialEq)]
pub enum Indent {
    None,
    Line,
    Block,
    Custom(&'static str)
}

#[derive(PartialEq)]
pub enum Align {
    None,
    Center,
    Right,
}

#[component]
pub fn Paragraph(
    cx: Scope,
    children: Children,
    #[prop(default = Indent::None)] indent: Indent,
    #[prop(default = Align::None)] align: Align,
    #[prop(default = 0)] margin_top: i16,
    #[prop(optional)] id: &'static str,
    #[prop(optional)] classes: &'static str,


) -> impl IntoView {
    view! {cx,
        <span
            id=id
            class=format!("col-start-2 px-4 block {}", classes)
            class=("indent-10", indent == Indent::Line)
            class=("pl-10", indent == Indent::Block)
            class=("text-center", align == Align::Center)
            class=("text-right", align == Align::Right)
            class=("text-left", align == Align::None)
            style=format!("margin-top: {}px; text-indent: {}", margin_top, match indent {
                Indent::Custom(s) => s,
                _ => ""
            })

        >
            {children(cx)}
        </span>
    }
}