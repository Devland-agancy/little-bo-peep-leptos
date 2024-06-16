use crate::components::Section::Spacer;
use crate::constants::{TEXT_LEFT_PADDING, TEXT_RIGHT_PADDING};
use crate::global_state::GlobalState;
use leptos::{
    ev::{resize, scroll},
    html::Span,
    *,
};
use leptos_use::use_event_listener;

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
    #[prop(optional)] id: &'static str,
    #[prop(optional)] classes: &'static str,
    #[prop(default = false)] no_padding: bool,
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
        class=format!("pr col-start-2 block relative {}", classes)
        class=("indent-10", indent == Indent::Line)
        class=("pl-10", indent == Indent::Block)
        class=("text-center", align == Align::Center)
        class=("my-2", align == Align::Center)
        class=("text-right", align == Align::Right)
        class=("text-left", align == Align::None)
        class=("test-bg", move || show_areas())
        style=format!(
            "text-indent: {}; padding-left: {}; padding-right: {}",
            match indent {
                Indent::Custom(s) => s,
                _ => "",
            },
            if !no_padding { TEXT_LEFT_PADDING } else { "0" },
            if !no_padding { TEXT_RIGHT_PADDING } else { "0" },
        )
      >

        {children(cx)}
      </span>
      <Spacer/>
    }
}

#[component]
pub fn InnerParagraph(
    cx: Scope,
    children: Children,
    #[prop(default = Indent::None)] indent: Indent,
    #[prop(default = Align::None)] align: Align,
    #[prop(optional)] id: &'static str,
    #[prop(optional)] classes: &'static str,
    #[prop(default = false)] no_padding: bool,
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
      <Spacer />
      <span
        id=id
        node_ref=node_ref
        class=format!("pr inner-pr col-start-2 block relative {}", classes)
        class=("indent-10", indent == Indent::Line)
        class=("pl-10", indent == Indent::Block)
        class=("text-center", align == Align::Center)
        class=("my-2", align == Align::Center)
        class=("text-right", align == Align::Right)
        class=("text-left", align == Align::None)
        class=("test-bg", move || show_areas())
        style=format!(
            "text-indent: {}; padding-left: {}; padding-right: {}",
            match indent {
                Indent::Custom(s) => s,
                _ => "",
            },
            if !no_padding { TEXT_LEFT_PADDING } else { "0" },
            if !no_padding { TEXT_RIGHT_PADDING } else { "0" },
        )
      >
        {children(cx)}
      </span>
      <Spacer />
    }
}
