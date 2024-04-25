use leptos::log;
use wasm_bindgen::JsCast;
use web_sys::Element;

pub fn element_contains_child(element: &Element, child_class: &str) -> Option<Element> {
    // Check if the current element has the target class
    if element.class_list().contains(child_class) {
        return Some(element.clone());
    }

    // Recursively check the children of the current element
    let children = element.children();
    for i in 0..children.length() {
        if let Some(child) = children.item(i) {
            if let Some(child_element) = child.dyn_into::<Element>().ok() {
                if let Some(found_element) = element_contains_child(&child_element, child_class) {
                    return Some(found_element);
                }
            }
        }
    }

    // If none of the children contain the target class, return false
    None
}
