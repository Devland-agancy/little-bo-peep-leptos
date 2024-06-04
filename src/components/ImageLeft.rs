use crate::{
    global_state::GlobalState,
    page::state::PageState,
    utils::attach_img_to_math::{attach_img_to_math, choose_default_anchor},
};
use leptos::{
    html::{Div, Img},
    *,
};
use std::time::Duration;

#[component]
pub fn ImageLeft(
    cx: Scope,
    src: &'static str,
    #[prop(default = true)] use_squiggle_on_mobile: bool,
    #[prop(default = true)] _attached: bool,
    #[prop(default = "center")] img_position: &'static str, // bot, top, center
    #[prop(default = "center")] y: &'static str,            // bot, top, center of pivot ( red dot )
    #[prop(default = "")] edge: &'static str,               // formula_edge, paragraph_edge
    #[prop(optional)] line: f32, // which paragraph line pivot is attached to

    #[prop(default = "0px")] offset_y: &'static str,
    #[prop(default = "0px")] offset_x: &'static str,

    #[prop(default = "30%")] squiggle_y: &'static str,

    #[prop(default = "")] children_x: &'static str,
    #[prop(default = "")] children_y: &'static str,
    #[prop(default = false)] clickable_on_desktop: bool,
    #[prop(default = "")] padding: &'static str,

    children: Children,
) -> impl IntoView {
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let show_left = move || page_state() == PageState::ShowLeft;
    let image_ref = create_node_ref::<Img>(cx);
    let node_ref = create_node_ref::<Div>(cx);
    let GlobalState {
        show_areas,
        on_mobile,
        ..
    } = use_context::<GlobalState>(cx).unwrap();
    let line_height = move || if on_mobile.get() { 28.0 } else { 32.5 };
    let (edge_signal, set_edge_signal) = create_signal(cx, edge);

    create_effect(cx, move |_| {
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
    log!("line {}", line);

    view! { cx,
      <div
        node_ref=node_ref
        style=move || {
            let line_str: String;
            if line > 0.0 {
                line_str = ((line - 0.5) * line_height()).to_string() + "px";
            } else if line < 0.0 {
                line_str = format!("calc(100% + {}px)", (line - 0.5) * line_height())
            } else {
                line_str = match y {
                    "bottom" => "100%".to_string(),
                    "top" => "0%".to_string(),
                    "center" => "50%".to_string(),
                    _ => y.to_string(),
                };
            }
            let left_pos = if edge_signal() == "formula_edge" { "0" } else { "0.5rem" };
            format!("top: {}; left: {}", line_str, left_pos)
        }

        class="absolute -translate-x-1/2 w-1 h-1"
      >

        <div class="w-1 h-1 relative z-20" class=("bg-red-500", move || show_areas())></div>
        <div
          style=move || {
              format!(
                  "right: calc(-100% + {}); top: calc(50% + {}); transform: translateY(calc(-50% + {} + {})); padding: {}",
                  offset_x,
                  if offset_y.contains("%") { "0px" } else { offset_y },
                  match img_position {
                      "bottom" => "-50%",
                      "top" => "50%",
                      _ => "0%",
                  },
                  if offset_y.contains("%") { offset_y } else { "0px" },
                  padding,
              )
          }

          class="flex shrink-0 transition-opacity duration-300 lg:transition-none lg:opacity-100 z-10 absolute w-max"
          class=("pointer-events-none", show_left)
          class=("lg:pointer-events-none", move || !clickable_on_desktop)
          class=("outline-[20px]", move || show_areas())
          class=("outline-[#3f9aff7d]", move || show_areas())
          class=("outline", move || show_areas())
        >
          <div style=move || {
              format!(" top: {}; left: {}", children_y, children_x)
          }>{children(cx)}</div>
          <img node_ref=image_ref src=src class="max-w-max"/>

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
          ></div>

        </div>
        <Show fallback=|_| () when=move || use_squiggle_on_mobile>
          <div
            class="block sm:hidden absolute"
            class=("outline-[20px]", move || show_areas())
            class=("outline-[#3f9aff7d]", move || show_areas())
            class=("outline", move || show_areas())

            style=move || {
                format!(
                    "right: 50%; top: {}; transform: translate(50%, -50%); padding: {}",
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
