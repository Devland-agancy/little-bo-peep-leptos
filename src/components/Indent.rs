use js_sys::Array;
use leptos::{html::Span, *};

use crate::global_state::GlobalState;

#[component]
pub fn Indent(cx: Scope, children: Children) -> impl IntoView {
    let node_ref = create_node_ref::<Span>(cx);
    let GlobalState { route, .. } = use_context::<GlobalState>(cx).unwrap();

    create_effect(cx, move |_| {
        route();
        if let Some(node) = node_ref() {
            if let Some(parent) = node.parent_element() {
                if let Some(parent) = parent.parent_element() {
                    if let Some(prev_sibling) = parent.previous_element_sibling() {
                        if prev_sibling.class_list().contains("spacer") {
                            let indent_fix = Array::of1(&"indent-fix".into());
                            let _ = prev_sibling.class_list().add(&indent_fix);
                        }
                    }
                }
            }
        }
    });

    view! { cx,
      <span node_ref=node_ref class="indent-10 block">
        {children(cx)}
      </span>
    }
}
