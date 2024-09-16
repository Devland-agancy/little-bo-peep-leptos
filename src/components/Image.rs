use crate::{
    global_state::GlobalState, page::state::PageState,
    utils::cast_element_to_html_element::cast_element_to_html_element,
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
    #[prop(default = 0_f64)] padding_left: f64,
    #[prop(default = 0_f64)] padding_right: f64,
    #[prop(default = false)] cloud_image: bool,
    children: Children,
) -> impl IntoView {
    let image_ref = create_node_ref::<Div>(cx);
    let (is_wider_than_screen, set_is_wider_than_screen) = create_signal(cx, false);
    let (is_wider_than_text, set_is_wider_than_text) = create_signal(cx, false);
    let (show_padding, set_show_padding) = create_signal(cx, true);

    let set_page_state = use_context::<WriteSignal<PageState>>(cx).unwrap();
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let GlobalState {
        show_areas,
        margin_scroll_value,
        ..
    } = use_context::<GlobalState>(cx).unwrap();

    create_effect(cx, move |_| {
        if image_ref().is_some() {
            let image = image_ref().unwrap().get_elements_by_tag_name("img").item(0);
            if let Some(img) = image {
                let image_width = cast_element_to_html_element(img).unwrap().offset_width() as f64;
                let window_width = window().inner_width().unwrap().as_f64().unwrap();
                let text_col_width = image_ref()
                    .unwrap()
                    .parent_element()
                    .unwrap()
                    .client_width() as f64;

                if image_width + 10_f64 > window_width {
                    set_is_wider_than_screen(true);
                }
                if image_width > text_col_width {
                    set_is_wider_than_text(true);
                }

                if image_width + padding_left + padding_right > window_width {
                    set_show_padding(false)
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
                    let text_col_width = image_ref()
                        .unwrap()
                        .parent_element()
                        .unwrap()
                        .client_width() as f64;
                    if image_width + 10_f64 > window_width {
                        set_is_wider_than_screen(true);
                    } else {
                        set_is_wider_than_screen(false);
                    }
                    if image_width > text_col_width {
                        set_is_wider_than_text(true);
                    } else {
                        set_is_wider_than_text(false);
                    }
                    if image_width + padding_left + padding_right > window_width {
                        set_show_padding(false)
                    } else {
                        set_show_padding(true)
                    }
                }
            }
        });
    });
    view! { cx,
      <div
        node_ref=image_ref
        style=move || {
            format!(
                "padding-left: {}px; padding-right: {}px;",
                if show_padding() { padding_left } else { 0_f64 },
                if show_padding() { padding_right } else { 0_f64 },
            )
        }

        class=move || {
            format!(
                "displayed-image relative col-start-2 scrollbar-hidden sm:overflow-x-visible m-auto {} {}",
                container_classes,
                if is_wider_than_text() {"wide wide_desktop w-max"} else { "fill-available" }
            )
        }
      >
        <div
            class="w-max"
            style=move || {
                format!(
                    "height: {};
                    width: {};
                    position: relative;
                    left: 50%;
                    transform: translateX(-50%);",
                    height,
                    width,
                )
            }
        >
            <img
                on:click=move |e| {
                    if cloud_image && is_wider_than_screen() && page_state() == PageState::ShowArticle {
                        e.stop_propagation();
                        set_page_state
                            .update(|value| {
                                *value = PageState::ShowRight;
                            });
                        margin_scroll_value
                            .update(|val| {
                                *val = 100_f64;
                            })
                    }
                }

                id=id
                src=src

                style=move || format!("height: {height}")
                class=("outline-[20px]", move || show_areas() && cloud_image && is_wider_than_screen())
                class=("outline-[#3f9aff7d]", move || show_areas() && cloud_image && is_wider_than_screen())
                class=("outline", move || show_areas() && cloud_image && is_wider_than_screen())
            />
        </div>
        {children(cx)}
      </div>
    }
}
