use js_sys::Array;
use leptos::{html::Span, *};

use crate::global_state::GlobalState;

#[component]
pub fn Indent(children: Children) -> impl IntoView {
    let node_ref = create_node_ref::<Span>();
    let GlobalState { route, .. } = use_context::<GlobalState>().unwrap();

    create_effect(move |_| {
        route.get();
        if let Some(node) = node_ref.get() {
            if let Some(parent) = node.parent_element() {
                // parent is div with class .pr
                // if it has a sibling we add indent-fix to it else we add indent-fix to the sibling of grandparent div with class .slice
                let mut selected_parent = parent.clone();
                if parent.previous_element_sibling().is_none() {
                    if let Some(grand_parent) = parent.parent_element() {
                        selected_parent = grand_parent.clone();
                    }
                }
                if let Some(prev_sibling) = selected_parent.previous_element_sibling() {
                    if prev_sibling.class_list().contains("spacer") {
                        let indent_fix = Array::of1(&"indent-fix".into());
                        let _ = prev_sibling.class_list().add(&indent_fix);
                    }
                }
            }
        }
    });

    view! {
      <span node_ref=node_ref class="indent-10 block">
        {children()}
      </span>
    }
}
