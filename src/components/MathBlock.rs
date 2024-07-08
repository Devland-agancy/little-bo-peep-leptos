use crate::{global_state::GlobalState, page::state::PageState};
use leptos::{ev::resize, html::Div, *};
use leptos_use::use_event_listener;

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
) -> impl IntoView {
    view! { cx,
      <div
        id=id
        class="mathblock block-element text-xl col-start-2 hidden-on-startup relative h-fit"

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
      <span></span>
    }
}
