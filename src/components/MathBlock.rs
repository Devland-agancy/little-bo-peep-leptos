use std::time::Duration;

use ev::resize;
use html::Div;
use leptos::*;
use leptos_use::use_event_listener;

use crate::constants::DESKTOP_CENTERED_PARAGRAPH_WIDTH;
use crate::global_state::GlobalState;

#[derive(PartialEq)]
pub enum Height {
    Fit,
    Small,
}
#[component]
pub fn MathBlock(
    cx: Scope,
    children: Children,
    #[prop(default = "")] id: &'static str,
    #[prop(default = Height::Small)] _height: Height,
    #[prop(default = 0)] margin_right: i16,
    #[prop(default = 0)] margin_left: i16,
    #[prop(default = "svg")] child_tag: &'static str,
) -> impl IntoView {
    let node_ref = create_node_ref::<Div>(cx);
    let (is_wide, set_is_wide) = create_signal(cx, false);
    let GlobalState {
        math_rendered,
        on_mobile,
        ..
    } = use_context(cx).unwrap();

    create_effect(cx, move |_| {
        math_rendered();
        set_timeout(
            move || {
                if node_ref().is_some() {
                    let math_box = node_ref()
                        .unwrap()
                        .get_elements_by_tag_name(child_tag)
                        .item(0);
                    if math_box.is_some() {
                        let math_box_width = math_box.unwrap().client_width() as f64;
                        let window_width = if on_mobile() {
                            window().inner_width().unwrap().as_f64().unwrap()
                        } else {
                            DESKTOP_CENTERED_PARAGRAPH_WIDTH
                        };
                        if math_box_width > window_width {
                            set_is_wide(true);
                        }
                    }
                }
            },
            Duration::from_secs(4),
        );
    });
    create_effect(cx, move |_| {
        let _ = use_event_listener(cx, window(), resize, move |_| {
            if node_ref().is_some() {
                let math_box = node_ref()
                    .unwrap()
                    .get_elements_by_tag_name(child_tag)
                    .item(0);
                if math_box.is_some() {
                    let math_box_width = math_box.unwrap().client_width() as f64;
                    let window_width = if on_mobile() {
                        window().inner_width().unwrap().as_f64().unwrap()
                    } else {
                        DESKTOP_CENTERED_PARAGRAPH_WIDTH
                    };
                    if math_box_width > window_width {
                        set_is_wide(true);
                    } else {
                        set_is_wide(false);
                    }
                }
            }
        });
    });

    view! { cx,
      <div
        node_ref=node_ref
        id=id
        class="mathblock block-element text-xl col-start-2 hidden-on-startup relative h-fit"
        class=("wide", is_wide)
        class=("wide_desktop", move || is_wide() && !on_mobile())

        style=format!("margin-right: {}px", margin_right)
        style=move || {
            format!(
                "margin-left: {}px; margin-right: {}px",
                margin_left,
                margin_right,
            )
        }
      >

        {children(cx)}
      </div>
      <span class="mathblock-span"></span>
    }
}

#[component]
pub fn CustomBlock(
    cx: Scope,
    children: Children,
    #[prop(default = "")] style: &'static str,
) -> impl IntoView {
    let node_ref = create_node_ref::<Div>(cx);
    let (is_wide, set_is_wide) = create_signal(cx, false);
    let GlobalState {
        math_rendered,
        on_mobile,
        ..
    } = use_context(cx).unwrap();

    view! { cx,
      <div
        class="mathblock block-element"
        style=style
      >
        {children(cx)}
      </div>
    }
}
