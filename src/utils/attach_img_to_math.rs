use leptos::html::Div;
use leptos::*;
use web_sys::Node;

use super::element_contains_child::element_contains_child;

pub fn attach_img_to_math(node_ref: &NodeRef<Div>) {
    if let Some(node) = node_ref() {
        let parent = node.parent_element().unwrap();

        let parent_is_math = parent
            .first_element_child()
            .unwrap()
            .class_list()
            .contains(&"mathblock");

        if parent_is_math {
            let math_element = element_contains_child(&parent, "MathJax_SVG");

            if let Some(math_element) = math_element {
                let img_from_dom =
                    element_contains_child(&parent.parent_element().unwrap(), "side-img");

                if let Some(img) = img_from_dom {
                    let _ = math_element.append_with_node_1(&Node::from(img));
                }
            }
        }
    }
}

pub fn choose_default_anchor(node_ref: &NodeRef<Div>, set_anchor_x: WriteSignal<&str>) {
    if let Some(node) = node_ref() {
        let parent = node.parent_element().unwrap();

        let parent_is_math = parent
            .first_element_child()
            .unwrap()
            .class_list()
            .contains(&"mathblock");

        if parent_is_math {
            let math_element = element_contains_child(&parent, "MathJax_SVG");

            if let Some(math_element) = math_element {
                let math_width = math_element.client_width();
                let screen_width = parent.first_element_child().unwrap().client_width();

                set_anchor_x(if math_width > screen_width {
                    "formula_edge"
                } else {
                    "paragraph_edge"
                })
            }
        }
    }
}
