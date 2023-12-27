use crate::constants::SHOW_CLICKABLE_ITEMS_BORDERS;
use crate::{
    page::state::PageState, utils::cast_element_to_html_element::cast_element_to_html_element,
};
use leptos::{ev::resize, html::Div, *};
use leptos_use::use_event_listener;

#[component]
pub fn Image(
    cx: Scope,
    src: &'static str,
    #[prop(default = "")] id: &'static str,
    #[prop(default = "")] container_classes: &'static str,
    #[prop(default = "")] image_classes: &'static str,
    #[prop(default = "")] height: &'static str,
    #[prop(default = "")] width: &'static str,
    #[prop(default = false)] cloud_image: bool,
) -> impl IntoView {
    let image_ref = create_node_ref::<Div>(cx);
    let (is_wide, set_is_wide) = create_signal(cx, false);
    let set_page_state = use_context::<WriteSignal<PageState>>(cx).unwrap();
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let set_right_image_x_pos = use_context::<WriteSignal<f64>>(cx).unwrap();

    create_effect(cx, move |_| {
        if image_ref().is_some() {
            let image = image_ref().unwrap().get_elements_by_tag_name("img").item(0);
            if let Some(img) = image {
                let image_width = cast_element_to_html_element(img).unwrap().offset_width() as f64;
                let window_width = window().inner_width().unwrap().as_f64().unwrap();
                if image_width - 10_f64 > window_width {
                    set_is_wide(true);
                }
            }
        }
    });
    create_effect(cx, move |_| {
        let _ = use_event_listener(cx, window(), resize, move |_| {
            if image_ref().is_some() {
                let image = image_ref().unwrap().get_elements_by_tag_name("img").item(0);
                if let Some(img) = image {
                    let image_width =
                        cast_element_to_html_element(img).unwrap().offset_width() as f64;
                    let window_width = window().inner_width().unwrap().as_f64().unwrap();
                    if image_width - 10_f64 > window_width {
                        set_is_wide(true);
                    } else {
                        set_is_wide(false);
                    }
                }
            }
        });
    });
    view! { cx,
      <div
        node_ref=image_ref
        class=move || {
            format!(
                "my-[15px] relative col-start-2 scrollbar-hidden md:overflow-x-visible {}",
                container_classes,
            )
        }

        class=("overflow-x-scroll", move || page_state() == PageState::ShowArticle && !cloud_image)
      >
        <img
          on:click=move |e| {
            if cloud_image && is_wide() && page_state() == PageState::ShowArticle {
              e.stop_propagation();
              set_page_state
                  .update(|value| {
                      *value = PageState::ShowRight;
                  });
              set_right_image_x_pos
                  .update(|val| {
                      *val = 100_f64;
                  })
            }
          }
          id=id
          src=src
          style= move || format!("height: {}; width: {}; {}", height, width, if cloud_image && is_wide() {
            "position:relative; left: 50%; transform: translateX(-50%)"
          } else { "" })
          class=move || {
              format!(
                  "max-w-none m-auto {}",
                  image_classes,
              )
          }
          class=("outline-[20px]", move || SHOW_CLICKABLE_ITEMS_BORDERS && cloud_image && is_wide())
          class=("outline-blue-300", move || SHOW_CLICKABLE_ITEMS_BORDERS && cloud_image && is_wide())
          class=("outline", move || SHOW_CLICKABLE_ITEMS_BORDERS && cloud_image && is_wide())
        />

      </div>
    }
}
