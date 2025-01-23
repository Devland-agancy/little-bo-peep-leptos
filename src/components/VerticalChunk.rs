use crate::components::Section::Spacer;
use crate::constants::TEXT_X_PADDING;
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
pub fn VerticalChunk(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] indent: bool,
    #[prop(optional)] id: &'static str,
    #[prop(optional)] classes: &'static str,
) -> impl IntoView {
    let GlobalState {
        burger_background,
        show_areas,
        ..
    } = use_context::<GlobalState>().unwrap();
    let node_ref = create_node_ref::<Span>();

    create_effect(move |_| {
        let toggle = move || {
            if let Some(span) = node_ref.get() {
                burger_background.set(
                    window().inner_width().unwrap().as_f64().unwrap()
                        - span.get_bounding_client_rect().right()
                        > 55_f64 - 16_f64,
                )
            }
        };
        toggle();
        let _ = use_event_listener(window(), scroll, move |_| toggle());
        let _ = use_event_listener(window(), resize, move |_| toggle());
    });

    view! {
    <div class=format!("slice {classes}")>
      <span
        id=id
        node_ref=node_ref
        class=format!("block")
        class=("indent-10", indent)
        class=("pl-10", indent)
        class=("test-bg", move || show_areas.get())
      >
        {children.map(|c| c())}
      </span>
    </div>
    }
}

#[component]
pub fn InnerParagraph(
    children: Children,
    #[prop(default = Indent::None)] indent: Indent,
    #[prop(default = Align::None)] align: Align,
    #[prop(optional)] id: &'static str,
    #[prop(optional)] classes: &'static str,
    #[prop(default = false)] no_padding: bool,
) -> impl IntoView {
    let GlobalState { show_areas, .. } = use_context::<GlobalState>().unwrap();

    view! {

      <span
        id=id
        class=format!("pr col-start-2 block relative {}", classes)
        class=("indent-10", indent == Indent::Line)
        class=("pl-10", indent == Indent::Block)
        class=("text-center", align == Align::Center)
        class=("my-2", align == Align::Center)
        class=("text-right", align == Align::Right)
        class=("text-left", align == Align::None)
        class=("test-bg", move || show_areas.get())
        style=format!(
            "text-indent: {}; padding-left: {}; padding-right: {}",
            match indent {
                Indent::Custom(s) => s,
                _ => "",
            },
            if !no_padding { TEXT_X_PADDING } else { "0" },
            if !no_padding { TEXT_X_PADDING } else { "0" },
        )
      >

        {children()}
      </span>
    <Spacer/>
    }
}
