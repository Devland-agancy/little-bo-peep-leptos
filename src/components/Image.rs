use crate::{
    global_state::GlobalState, page::state::PageState,
    utils::cast_element_to_html_element::cast_element_to_html_element,
};
use leptos::{ev::resize, html::Div, *};
use leptos_use::use_event_listener;

#[component]
pub fn Image(
    src: &'static str,
    #[prop(default = "")] id: &'static str,
    #[prop(default = "")] container_classes: &'static str,
    #[prop(default = "")] _image_classes: &'static str,
    #[prop(default = "")] height: &'static str,
    #[prop(default = "")] width: &'static str,
    #[prop(default = 0_f64)] padding_left: f64,
    #[prop(default = 0_f64)] padding_right: f64,
    #[prop(default = false)] cloud_image: bool,
    children: Children,
) -> impl IntoView {
    let image_ref = create_node_ref::<Div>();
    let (is_wider_than_screen, set_is_wider_than_screen) = create_signal(false);
    let (is_wider_than_text, set_is_wider_than_text) = create_signal(false);
    let (show_padding, set_show_padding) = create_signal(true);
    let GlobalState {
        show_areas,
        on_mobile,
        margin_mode,
        ..
    } = use_context::<GlobalState>().unwrap();

    create_effect(move |_| {
        if image_ref.get().is_some() {
            let image = image_ref
                .get()
                .unwrap()
                .get_elements_by_tag_name("img")
                .item(0);
            if let Some(img) = image {
                let image_width = cast_element_to_html_element(img).unwrap().offset_width() as f64;
                let window_width = window().inner_width().unwrap().as_f64().unwrap();
                let text_col_width = image_ref
                    .get()
                    .unwrap()
                    .parent_element()
                    .unwrap()
                    .client_width() as f64;

                if image_width + 10_f64 > window_width {
                    set_is_wider_than_screen.set(true);
                }
                if image_width > text_col_width {
                    set_is_wider_than_text.set(true);
                }

                if image_width + padding_left + padding_right > window_width {
                    set_show_padding.set(false)
                }
            }
        }
    });
    create_effect(move |_| {
        let _ = use_event_listener(window(), resize, move |_| {
            if image_ref.get().is_some() {
                let image = image_ref
                    .get()
                    .unwrap()
                    .get_elements_by_tag_name("img")
                    .item(0);
                if let Some(img) = image {
                    let image_width =
                        cast_element_to_html_element(img).unwrap().offset_width() as f64;
                    let window_width = window().inner_width().unwrap().as_f64().unwrap();
                    let text_col_width = image_ref
                        .get()
                        .unwrap()
                        .parent_element()
                        .unwrap()
                        .client_width() as f64;
                    if image_width + 10_f64 > window_width {
                        set_is_wider_than_screen.set(true);
                    } else {
                        set_is_wider_than_screen.set(false);
                    }
                    if image_width > text_col_width {
                        set_is_wider_than_text.set(true);
                    } else {
                        set_is_wider_than_text.set(false);
                    }
                    if image_width + padding_left + padding_right > window_width {
                        set_show_padding.set(false)
                    } else {
                        set_show_padding.set(true)
                    }
                }
            }
        });
    });

    let (scaled_down, set_scaled_down) = create_signal(on_mobile.get());

    let calc_scale = move || {
        if let Some(image) = image_ref.get() {
            let image_width = image.offset_width() as f64;
            let screen_width = window().inner_width().unwrap().as_f64().unwrap();
            if screen_width < image_width && scaled_down.get() {
                screen_width / (image_width + 32.0)
            } else {
                1.0
            }
        } else {
            1.0
        }
    };

    let calc_margin = move || {
        if let Some(image) = image_ref.get() {
            let image_width = image.offset_width() as f64;
            let screen_width = window().inner_width().unwrap().as_f64().unwrap();
            if screen_width < image_width && scaled_down.get() {
                -((1.0 - (screen_width / (image_width + 32.0))) / 2.0) * 100.0
            } else {
                0.0
            }
        } else {
            0.0
        }
    };
    view! {
      // on desktop . both divs should be max-width
      // on mobile . if opened is true . image should be max-width else it should fit to screen
      <div
        node_ref=image_ref
        style=move || {
            format!(
                "padding-left: {}px; padding-right: {}px; margin-block: {}%",
                if show_padding.get() { padding_left } else { 0_f64 },
                if show_padding.get() { padding_right } else { 0_f64 },
                calc_margin()
            )
        }
        class=move || {
            format!(
                "displayed-image relative col-start-2 scrollbar-hidden sm:overflow-x-visible m-auto transition-all w-max {}",
                container_classes

            )
        }
        class=("fill-available", move || !is_wider_than_text.get())
      >
        <div
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
            class="w-max"
        >
            <img
                on:click=move |_| {
                    set_scaled_down.set(!scaled_down.get())
                }

                id=id
                src=src
                style=move || format!("height: {height}; scale: {}; ", calc_scale())
                class="m-auto transition-image-scale"


                class=("outline-[20px]", move || show_areas.get() && cloud_image && is_wider_than_screen.get())
                class=("outline-[#3f9aff7d]", move || show_areas.get() && cloud_image && is_wider_than_screen.get())
                class=("outline", move || show_areas.get() && cloud_image && is_wider_than_screen.get())
            />
        </div>
        {children()}
      </div>
    }
}
