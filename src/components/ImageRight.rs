use std::time::Duration;

use crate::{
    global_state::GlobalState,
    page::state::PageState,
    utils::{
        attach_img_to_math::{attach_img_to_math, choose_default_anchor},
        element_contains_child::element_contains_child,
    },
};
use leptos::{
    html::{Div, Img},
    *,
};
use web_sys::Node;

#[component]
pub fn ImageRight(
    cx: Scope,
    src: &'static str,
    #[prop(default = true)] use_squiggle_on_mobile: bool,
    #[prop(default = "center")] img_position: &'static str,
    #[prop(default = "center")] y: &'static str,
    #[prop(default = "")] edge: &'static str,
    #[prop(default = -1.0)] line: f32,

    #[prop(default = "0px")] offset_y: &'static str,
    #[prop(default = "0px")] offset_x: &'static str,

    #[prop(default = "0px")] squiggle_x: &'static str,
    #[prop(default = "30%")] squiggle_y: &'static str,

    #[prop(default = "")] children_x: &'static str,
    #[prop(default = "")] children_y: &'static str,
    #[prop(default = false)] clickable_on_desktop: bool,
    #[prop(default = "")] padding: &'static str,

    children: Children,
) -> impl IntoView {
    let set_page_state =
        use_context::<WriteSignal<PageState>>(cx).expect("set_page_state context to exist");
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let show_right = move || page_state() == PageState::ShowRight;
    let GlobalState {
        show_areas,
        margin_scroll_value,
        on_mobile,
        ..
    } = use_context::<GlobalState>(cx).unwrap();
    let node_ref = create_node_ref::<Div>(cx);
    let image_ref = create_node_ref::<Img>(cx);
    let (image_width, set_image_width) = create_signal(cx, 0_f64);
    let line_height = move || if on_mobile.get() { 28.0 } else { 32.5 };
    let (edge_signal, set_edge_signal) = create_signal(cx, edge);

    create_effect(cx, move |_| {
        request_animation_frame(move || {
            if let Some(img) = image_ref() {
                set_image_width(img.offset_width() as f64)
            }

            set_timeout(
                move || {
                    // choose max width betweem formula and screen as default value for edge
                    if edge == "" {
                        choose_default_anchor(&node_ref, set_edge_signal);
                    }
                    if edge_signal() == "formula_edge" {
                        attach_img_to_math(&node_ref);
                    }
                },
                Duration::from_secs(3),
            );
        });
    });

    view! { cx,
      <div
          node_ref=node_ref
          style=move || {
            let mut line_str = "".to_string();
            let mut left_pos = "calc(100% - 0.5rem)".to_string();

            if line == -1.0 {
              line_str = match y {
                  "bottom" => "100%".to_string(),
                  "top" => "0%".to_string(),
                  _ => "50%".to_string(),
              };
            } else {
                line_str = (line * line_height()).to_string() + "px";
            }

            if edge_signal() == "formula_edge" {
              left_pos = "100%".to_string();
            }
            format!(
                "top: {}; left: {}",
                line_str,
                left_pos
            )
        }
        class="side-img absolute -translate-x-1/2 w-1 h-1 "
      >
        <div
          class="w-1 h-1 relative z-20"
          class=("bg-red-500", move || show_areas())
        ></div>

        <button
          on:click=move |e| {
              e.stop_propagation();
              if page_state() == PageState::ShowArticle {
                  set_page_state(PageState::ShowRight);
                  margin_scroll_value.set(image_width())
              }
          }

          style=move || {
              format!(
                  "left: {}; top: calc(50% + {}); transform: translateY(calc(-50% + {} + {})); padding: {}",
                  offset_x,
                  if offset_y.contains("%") {"0px"} else {offset_y} ,
                  match img_position {
                      "bottom" => "-50%",
                      "top" => "50%",
                      _ => "0%",
                  },
                  if offset_y.contains("%") {offset_y} else {"0px"} ,
                  padding
              )
          }

          class="flex shrink-0 transition-opacity duration-300 lg:transition-none lg:opacity-100  z-10 absolute w-max"
          class=("pointer-events-none", show_right)
          class=("lg:pointer-events-none", move || !clickable_on_desktop)
          class=("outline-[20px]", move || show_areas())
          class=("outline-[#3f9aff7d]", move || show_areas())
          class=("outline", move || show_areas())
        >

          <div class="absolute" style=move || format!("top: {}; left: {}", children_y, children_x)>
            {children(cx)}
          </div>
          <img node_ref=image_ref src=src class="max-w-max"/>

          /* test  */
          <div
              style=move || {
                format!(
                    "top: calc(50% - {}); transform: translateY(-50%)",
                    match img_position {
                        "bottom" => "-50%",
                        "top" => "50%",
                        _ => "0%",
                    },
                )
            }
            class="absolute left-0 w-1 h-1"
            class=("bg-blue-900", move || show_areas())
            >
          </div>
        </button>
        /*  */
        <Show fallback=|_| () when=move || use_squiggle_on_mobile>
          <div
            class="block sm:hidden absolute"
            class=("outline-[20px]", move || show_areas())
            class=("outline-[#3f9aff7d]", move || show_areas())
            class=("outline", move || show_areas())

            style=move || {
                format!(
                    "left: 50%; top: {}; transform: translate(-50%, -50%); padding: {}",
                    squiggle_y,
                    "2.6rem",
                )
            }
          >
            <img src="/images/squiggle.png" class="h-11 min-w-[45px]"/>
          </div>
        </Show>
      </div>
    }
}
