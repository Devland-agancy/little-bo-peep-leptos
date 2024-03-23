use js_sys::Array;
use leptos::{html::Span, *};

#[component]
pub fn Indent(cx: Scope, children: Children) -> impl IntoView {
    let node_ref = create_node_ref::<Span>(cx);

    create_effect(cx, move |_| {
        if let Some(node) = node_ref() {
            let class = Array::of1(&"-mt-4".into());
            let _ = node.parent_element().unwrap().class_list().add(&class);
        }
    });

    view! { cx,
      <span
        node_ref=node_ref
        class="indent indent-10 inline-table"
      >
        {children(cx)}
      </span>
    }
}
