use std::time::Duration;

use crate::{
    global_state::GlobalState,
    page::state::PageState,
    utils::attach_img_to_math::{attach_img_to_math, choose_default_anchor},
};
use js_sys::Array;
use leptos::{
    html::{Div, Img},
    *,
};
use web_sys::Node;

#[component]
pub fn ImageLeft(
    cx: Scope,
    src: &'static str,
    #[prop(default = false)] hidden_in_mobile: bool,
    #[prop(default = "center")] img_position: &'static str,
    #[prop(default = "center")] pivot_position: &'static str,
    #[prop(default = "")] anchor_x: &'static str,

    #[prop(default = "0px")] offset_y: &'static str,
    #[prop(default = "0px")] offset_x: &'static str,

    #[prop(default = -1.0)] line: f32,

    #[prop(default = "-1.5rem")] squiggle_x: &'static str,
    #[prop(default = "30%")] squiggle_y: &'static str,

    #[prop(default = "")] children_x: &'static str,
    #[prop(default = "")] children_y: &'static str,
    #[prop(default = 0)] width: i32,
    #[prop(default = false)] clickable_on_desktop: bool,
    #[prop(default = "")] padding: &'static str,

    children: Children,
) -> impl IntoView {
    let set_page_state =
        use_context::<WriteSignal<PageState>>(cx).expect("set_page_state context to exist");
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let show_left = move || page_state() == PageState::ShowLeft;
    let image_ref = create_node_ref::<Img>(cx);
    let node_ref = create_node_ref::<Div>(cx);
    let GlobalState {
        show_areas,
        margin_scroll_value,
        on_mobile,
        ..
    } = use_context::<GlobalState>(cx).unwrap();
    let (image_width, set_image_width) = create_signal(cx, 0_f64);
    let line_height = move || if on_mobile.get() { 28.0 } else { 32.5 };
    let (anchor_x_signal, set_anchor_x_signal) = create_signal(cx, anchor_x);

    create_effect(cx, move |_| {
        if let Some(img) = image_ref() {
            set_image_width(img.offset_width() as f64)
        };

        set_timeout(
            move || {
                // choose max width betweem formula and screen as default value for anchor_x
                if anchor_x == "" {
                    choose_default_anchor(&node_ref, set_anchor_x_signal);
                }
                if anchor_x_signal() == "formula_edge" {
                    attach_img_to_math(&node_ref);
                }
            },
            Duration::from_secs(3),
        );
    });

    view! { cx,
      <div
        node_ref=node_ref
        style=move || {
            let mut line_str = "".to_string();

            if line == -1.0 {
                line_str = match pivot_position {
                    "bottom" => "100%".to_string(),
                    "top" => "0%".to_string(),
                    _ => "50%".to_string(),
                };
            } else {
                line_str = (line * line_height()).to_string() + "px";
            }

            let left_pos = if anchor_x_signal() == "formula_edge" {
              "0"
            } else { "-1rem" };

            format!(
                "top: {}, left: {}",
                line_str,
                left_pos
            )
        }

        class="absolute -translate-x-1/2 w-1 h-1"
      >
        /* show pivot */
        <div
        class="w-1 h-1 relative z-20"
        class=("bg-red-500", move || show_areas())
        ></div>
        <button
          on:click=move |e| {
              e.stop_propagation();
              set_page_state
                  .update(|value| {
                      *value = match value {
                          PageState::ShowArticle => PageState::ShowLeft,
                          _ => PageState::ShowArticle,
                      };
                  });
              margin_scroll_value.set(image_width())
          }

          style=move || {
              format!(
                  "right: calc(-100% + {}); top: calc(50% + {}); transform: translateY(calc(-50% + {})); padding: {}",
                  offset_x,
                  offset_y,
                  match img_position {
                      "bottom" => "-50%",
                      "top" => "50%",
                      _ => "0%",
                  },
                  padding,
              )
          }

          class="flex shrink-0 transition-opacity duration-300 lg:transition-none lg:opacity-100 z-10 absolute"
          class=("pointer-events-none", show_left)
          class=("outline-[20px]", move || show_areas())
          class=("outline-[#3f9aff7d]", move || show_areas())
          class=("outline", move || show_areas())
          class=("w-max", move || width == 0)
        >
          <div style=move || {
              format!(" top: {}; left: {}", children_y, children_x)
          }>{children(cx)}</div>
          <img node_ref=image_ref src=src style=move || format!("min-width: {}px", width)/>

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
         class="absolute right-0 w-1 h-1"
         class=("bg-blue-900", move || show_areas())
         >
        </div>
        /*  */

          <Show fallback=|_| () when=move || hidden_in_mobile>
            <div
              class="block sm:hidden absolute"
              class=("outline-[20px]", move || show_areas())
              class=("outline-[#3f9aff7d]", move || show_areas())
              class=("outline", move || show_areas())

              style=move || {
                  format!(
                      "right: calc({} - 40px); top: calc({} - 40px); padding: {}",
                      squiggle_x,
                      squiggle_y,
                      "2.6rem",
                  )
              }
            >
              <img src="/images/squiggle.png" class="h-11"/>
            </div>
          </Show>
        </button>
      </div>
    }
}
