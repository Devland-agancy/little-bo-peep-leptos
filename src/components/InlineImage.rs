use crate::{
    constants::MOBILE_BREAKPOINT, global_state::GlobalState, page::state::PageState,
    utils::cast_element_to_html_element::cast_element_to_html_element,
};
use leptos::{ev::resize, html::Div, *};
use leptos_use::use_event_listener;

#[component]
pub fn InlineImage(
    cx: Scope,
    src: &'static str,
    width: &'static str,
    height: &'static str,
    children: Children,
) -> impl IntoView {
    view! { cx,
      <div style=move || format!("width: {}; height: {}; background-image: url({})", width, height, src)>

      </div>
    }
}
