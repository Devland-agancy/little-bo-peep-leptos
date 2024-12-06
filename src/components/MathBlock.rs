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
    children: Children,
    #[prop(default = "")] id: &'static str,
    #[prop(default = Height::Small)] _height: Height,
    #[prop(default = 0)] margin_right: i16,
    #[prop(default = 0)] margin_left: i16,
    #[prop(default = "svg")] child_tag: &'static str,
) -> impl IntoView {
    let node_ref = create_node_ref::<Div>();
    let (is_wide, set_is_wide) = create_signal(false);
    let GlobalState {
        math_rendered,
        on_mobile,
        ..
    } = use_context().unwrap();

    create_effect(move |_| {
        math_rendered.get();
        let node_ref = node_ref.get();
        set_timeout(
            move || {
                if node_ref.is_some() {
                    let math_box = node_ref
                        .unwrap()
                        .get_elements_by_tag_name(child_tag)
                        .item(0);
                    if math_box.is_some() {
                        let math_box_width = math_box.unwrap().client_width() as f64;
                        let window_width = if on_mobile.get() {
                            window().inner_width().unwrap().as_f64().unwrap()
                        } else {
                            DESKTOP_CENTERED_PARAGRAPH_WIDTH
                        };
                        if math_box_width > window_width {
                            set_is_wide.set(true);
                        }
                    }
                }
            },
            Duration::from_secs(4),
        );
    });
    create_effect(move |_| {
        let _ = use_event_listener(window(), resize, move |_| {
            if node_ref.get_untracked().is_some() {
                let math_box = node_ref
                    .get()
                    .unwrap()
                    .get_elements_by_tag_name(child_tag)
                    .item(0);
                if math_box.is_some() {
                    let math_box_width = math_box.unwrap().client_width() as f64;
                    let window_width = if on_mobile.get() {
                        window().inner_width().unwrap().as_f64().unwrap()
                    } else {
                        DESKTOP_CENTERED_PARAGRAPH_WIDTH
                    };
                    if math_box_width > window_width {
                        set_is_wide.set(true);
                    } else {
                        set_is_wide.set(false);
                    }
                }
            }
        });
    });

    view! {
      <div
        node_ref=node_ref
        id=id
        class="mathblock slice hidden-on-startup h-fit"
        class=("wide", is_wide)
        class=("wide_desktop", move || is_wide.get() && !on_mobile.get())
        class=("desktop", move || !on_mobile.get())

        style=format!("margin-right: {}px", margin_right)
        style=move || {
            format!(
                "margin-left: {}px; margin-right: {}px",
                margin_left,
                margin_right,
            )
        }
      >
        {children()}
      </div>
      //<span class="mathblock-span"></span>
    }
}

#[component]
pub fn CustomBlock(children: Children, #[prop(default = "")] style: &'static str) -> impl IntoView {
    view! {
      <div
        class="mathblock block-element"
        style=style
      >
        {children()}
      </div>
    }
}
