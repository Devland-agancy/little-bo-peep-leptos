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
    #[prop(default = "")] y_anchor: &'static str,
    #[prop(default = "")] margin_left: &'static str,
    #[prop(default = "")] margin_right: &'static str,
    children: Children,
) -> impl IntoView {
    view! { cx,
      <div class="bg-cover inline-block relative" style=move || format!("width: {}; height: {}; background-image: url({}); top: {}; margin-left: {}; margin-right: {};", width, height, src, y_anchor, margin_left, margin_right)>

      </div>
    }
}
