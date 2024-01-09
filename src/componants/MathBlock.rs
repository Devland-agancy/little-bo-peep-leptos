use crate::{global_state::GlobalState, page::state::PageState};
use leptos::{ev::resize, html::Div, *};
use leptos_use::use_event_listener;

#[derive(PartialEq)]
pub enum Height {
    Fit,
    Small,
}
#[component]
pub fn MathBlock(
    cx: Scope,
    children: Children,
    #[prop(default = "")] id: &'static str,
    #[prop(default = Height::Small)] height: Height,
    #[prop(default = 16)] margin_right: i16,
    #[prop(default = 16)] margin_left: i16,
    #[prop(default = "2rem")] arrow_position_y: &'static str,
    #[prop(default = "-2.5rem")] arrow_position_x: &'static str,
    #[prop(default = false)] arrow_hidden: bool,
    #[prop(default = "svg")] child_tag: &'static str,
    #[prop(default = 0)] show_arrow_at_width: i32,
) -> impl IntoView {
    let node_ref = create_node_ref::<Div>(cx);
    let (is_wide, set_is_wide) = create_signal(cx, false);
    let set_page_state = use_context::<WriteSignal<PageState>>(cx).unwrap();
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let set_right_image_x_pos = use_context::<WriteSignal<f64>>(cx).unwrap();
    let GlobalState { show_areas, .. } = use_context::<GlobalState>(cx).unwrap();

    let (margin_left_active, set_margin_left_active) = create_signal(cx, true);

    create_effect(cx, move |_| {
        if show_arrow_at_width as f64 > window().inner_width().unwrap().as_f64().unwrap() {
            set_is_wide(true);
        } else {
            if node_ref().is_some() {
                let math_box = node_ref()
                    .unwrap()
                    .get_elements_by_tag_name(child_tag)
                    .item(0);
                if math_box.is_some() {
                    let math_box_width = math_box.unwrap().client_width() as f64;
                    let window_width = window().inner_width().unwrap().as_f64().unwrap();
                    if math_box_width + margin_left as f64 - 2_f64 > window_width {
                        request_animation_frame(move || {
                            set_margin_left_active(false);
                            if math_box_width - 2_f64 > window_width {
                                set_is_wide(true);
                                set_margin_left_active(true);
                            }
                        });
                    }
                }
            }
        }
    });
    create_effect(cx, move |_| {
        let _ = use_event_listener(cx, window(), resize, move |_| {
            if show_arrow_at_width as f64 > window().inner_width().unwrap().as_f64().unwrap() {
                set_is_wide(true);
            } else {
                if node_ref().is_some() {
                    let math_box = node_ref()
                        .unwrap()
                        .get_elements_by_tag_name(child_tag)
                        .item(0);
                    if math_box.is_some() {
                        let math_box_width = math_box.unwrap().client_width() as f64;
                        let window_width = window().inner_width().unwrap().as_f64().unwrap();
                        if math_box_width + margin_left as f64 - 2_f64 > window_width {
                            set_margin_left_active(false);
                            if math_box_width - 2_f64 > window_width {
                                set_is_wide(true);
                                set_margin_left_active(true);
                            }
                        } else {
                            set_is_wide(false);
                            set_margin_left_active(true);
                        }
                    }
                }
            }
        });
    });
    view! { cx,
      <div
        node_ref=node_ref
        id=id
        class="mathblock text-xl flex items-center justify-center col-start-2 hidden-on-startup relative"
        class=("h-20", height == Height::Small)
        class=("h-fit", height == Height::Fit)

        style=format!("margin-right: {}px", margin_right)
        style=move || {
            format!(
                "margin-left: {}px; margin-right: {}px",
                if margin_left_active() { margin_left } else { 0 },
                if margin_left_active() { margin_right } else { 0 },
            )
        }
      >

        {children(cx)}
        <div
          on:click=move |e| {
              e.stop_propagation();
              if page_state() == PageState::ShowArticle {
                  set_page_state.update(|value| *value = PageState::ShowRight);
                  set_right_image_x_pos(95_f64);
              }
          }

          class="block cursor-pointer absolute h-fit w-24 py-8"
          class=("hidden", move || !is_wide() | arrow_hidden)
          class=("outline-[20px]", move || show_areas())
          class=("outline-[#3f9aff7d]", move || show_areas())
          class=("outline", move || show_areas())

          style=move || format!("top: {}; right: {}", arrow_position_y, "-2.5rem")
        >
          <img src="/images/cream.svg" class="m-auto h-2.5"/>
        </div>
      </div>
    }
}
