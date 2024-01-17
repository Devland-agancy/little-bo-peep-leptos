use leptos::{
    ev::{resize, scroll},
    html::Span,
    *,
};
use leptos_use::use_event_listener;

use crate::global_state::GlobalState;

#[derive(PartialEq)]
pub enum Indent {
    None,
    Line,
    Block,
    Custom(&'static str),
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
    let GlobalState {
        burger_background,
        show_areas,
        ..
    } = use_context::<GlobalState>(cx).unwrap();
    let node_ref = create_node_ref::<Span>(cx);

    create_effect(cx, move |_| {
        let toggle = move || {
            if let Some(span) = node_ref() {
                burger_background.set(
                    window().inner_width().unwrap().as_f64().unwrap()
                        - span.get_bounding_client_rect().right()
                        > 55_f64 - 16_f64,
                )
            }
        };
        toggle();
        let _ = use_event_listener(cx, window(), scroll, move |_| toggle());
        let _ = use_event_listener(cx, window(), resize, move |_| toggle());
    });

    view! { cx,

      <span
        id=id
        node_ref=node_ref
        class=format!("col-start-2 px-4 block relative {}", classes)
        class=("indent-10", indent == Indent::Line)
        class=("pl-10", indent == Indent::Block)
        class=("text-center", align == Align::Center)
        class=("text-right", align == Align::Right)
        class=("text-left", align == Align::None)
        class=("test-bg", move || show_areas())
        style=format!(
            "margin-top: {}px; text-indent: {}",
            margin_top,
            match indent {
                Indent::Custom(s) => s,
                _ => "",
            },
        )
      >

        {children(cx)}
      </span>
    }
}
