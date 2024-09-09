use leptos::html::Div;
use leptos::*;
use web_sys::Node;

use super::element_contains_child::element_contains_child;

pub fn re_attach_img(node_ref: &NodeRef<Div>) {
    if let Some(node) = node_ref() {
        let parent = node.parent_element().unwrap();

        let parent_is_math = parent
            .first_element_child()
            .unwrap()
            .class_list()
            .contains(&"mathblock");

        let parent_is_image = parent
            .first_element_child()
            .unwrap()
            .class_list()
            .contains(&"displayed-image");

        if parent_is_math {
            let math_element = element_contains_child(&parent, "MathJax_SVG");

            if let Some(math_element) = math_element {
                let img_from_dom =
                    element_contains_child(&parent.parent_element().unwrap(), "side-img");

                if let Some(img) = img_from_dom {
                    let _ = math_element.append_with_node_1(&Node::from(img));
                }
            }
        } else {
            let image_container = parent.first_element_child().unwrap().first_element_child();

            if let Some(image_container) = image_container {
                let img_from_dom =
                    element_contains_child(&parent.parent_element().unwrap(), "side-img");

                if let Some(img) = img_from_dom {
                    let _ = image_container.append_with_node_1(&Node::from(img));
                }
            }
        }
    }
}

pub fn choose_default_anchor(node_ref: &NodeRef<Div>, set_anchor_x: WriteSignal<&str>) {
    if let Some(node) = node_ref() {
        let parent = node.parent_element();
        if parent.is_none() {
            return;
        }

        let parent = node.parent_element().unwrap();

        let parent_is_math = parent
            .first_element_child()
            .unwrap()
            .class_list()
            .contains(&"mathblock");

        let parent_is_image = parent
            .first_element_child()
            .unwrap()
            .class_list()
            .contains(&"displayed-image");

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
        } else if parent_is_image {
            let image_container = parent.first_element_child().unwrap().first_element_child();

            if let Some(image_container) = image_container {
                let image_width = image_container.client_width();
                let screen_width = parent.first_element_child().unwrap().client_width();

                set_anchor_x(if image_width > screen_width {
                    "image_edge"
                } else {
                    "paragraph_edge"
                })
            }
        }
    }
}
