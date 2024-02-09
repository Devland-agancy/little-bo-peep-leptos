use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

pub fn cast_element_to_html_element(element: Element) -> Option<HtmlElement> {
    element.dyn_into::<HtmlElement>().ok()
}
