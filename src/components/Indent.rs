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
            let class = Array::of1(&"indent-fix".into());
            if let Some(parent) = node.parent_element() {
                let _ = parent.class_list().add(&class);
            }
        }
    });

    view! { cx,
      <span
        node_ref=node_ref
        class="indent-10 block"
      >
        {children(cx)}
      </span>
    }
}
