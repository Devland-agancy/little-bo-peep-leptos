use crate::{global_state::GlobalState, page::state::PageState};
use leptos::{html::Img, *};

#[component]
pub fn ImageLeft(
    cx: Scope,
    src: &'static str,
    #[prop(default = false)] hidden_in_mobile: bool,
    #[prop(default = "center")] img_position: &'static str,
    #[prop(default = "center")] pivot_position: &'static str,
    #[prop(default = "0px")] offset_y: &'static str,
    #[prop(default = "0px")] offset_x: &'static str,

    #[prop(default = -1.0)] paragraph_line: f32,

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
    let GlobalState {
        show_areas,
        margin_scroll_value,
        on_mobile,
        ..
    } = use_context::<GlobalState>(cx).unwrap();
    let (image_width, set_image_width) = create_signal(cx, 0_f64);
    let line_height = move || if on_mobile.get() { 28.0 } else { 32.5 };

    create_effect(cx, move |_| {
        if let Some(img) = image_ref() {
            set_image_width(img.offset_width() as f64)
        }
    });

    view! { cx,
      <div
        style=move || {
            let mut line_str = "".to_string();
            if paragraph_line == -1.0 {
                line_str = match pivot_position {
                    "bottom" => "100%".to_string(),
                    "top" => "0%".to_string(),
                    _ => "50%".to_string(),
                };
            } else {
                line_str = (paragraph_line * line_height()).to_string() + "px";
            }
            format!(
                "top: {}",
                line_str
            )
        }

        class="absolute -translate-x-1/2 left-[1rem] w-1 h-1"
        class=("bg-red-400", move || show_areas())

      >
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
         class=("bg-blue-400", move || show_areas())
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
