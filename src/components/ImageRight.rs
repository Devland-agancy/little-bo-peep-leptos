use crate::{
    global_state::GlobalState, page::state::PageState,
    utils::cast_element_to_html_element::cast_element_to_html_element,
};
use ev::Event;
use leptos::{
    html::{Div, Img},
    *,
};
use std::{borrow::Borrow, cell::RefCell, time::Duration};
use wasm_bindgen::{closure::Closure, JsCast};

#[component]
pub fn ImageRight(
    src: &'static str,
    #[prop(default = true)] use_squiggle_on_mobile: bool,
    #[prop(default = true)] _attached: bool,
    #[prop(default = "center")] img_position: &'static str,
    #[prop(default = "center")] y: &'static str,
    #[prop(default = "")] _edge: &'static str,
    #[prop(optional)] line: f32,

    #[prop(default = "0px")] offset_y: &'static str,
    #[prop(default = "0px")] offset_x: &'static str,

    #[prop(default = "30%")] squiggle_y: &'static str,

    #[prop(default = "")] children_x: &'static str,
    #[prop(default = "")] children_y: &'static str,
    #[prop(default = false)] clickable_on_desktop: bool,
    #[prop(default = "")] padding: &'static str,
    #[prop(default = "")] width: &'static str,
    #[prop(default = false)] popup: bool,
    #[prop(optional)] children: Option<Children>,

) -> impl IntoView {
    let GlobalState {
        show_areas,
        on_mobile,
        tab,
        solutions_state,
        show_squiggles,
        ..
    } = use_context::<GlobalState>().unwrap();

    let solution_open = create_memo(move |_| {
        if solutions_state.get().len() > tab.get() {
            solutions_state.get()[tab.get()]
        } else {
            false
        }
    });

    let (solution_fully_opened, set_solution_fully_opened) =
        create_signal(solution_open.get_untracked());
    create_effect(move |_| {
        if solution_open.get() {
            set_timeout(
                move || {
                    let _ = set_solution_fully_opened.try_set(true);
                },
                Duration::from_millis(1000),
            )
        } else {
            set_solution_fully_opened.set_untracked(false);
            set_timeout(
                // sometimes the above line executes before 1 second of the above block is passed so we make sure is stays false
                move || {
                    let _ = set_solution_fully_opened.try_set(false);
                },
                Duration::from_millis(1000),
            )
        }
    });

    let line_height = move || if on_mobile.get() { 28.0 } else { 32.5 };
    let container_ref = create_node_ref::<Div>();
    let (scale, set_scale) = create_signal("1".to_string());
    let (transform_origin, set_transform_origin) = create_signal("unset".to_string());

    let (attached_to_image, set_attached_to_image) = create_signal(false);

    create_effect(move |_| {
        let container_ref = Box::new(container_ref.get());
        let cb = Closure::wrap(Box::new(move |_: Event| {
            if let Some(container_ref) = container_ref.borrow() {
                let prev_sibling = container_ref.previous_element_sibling().unwrap();
                let scale_value_from_prev_sibling = cast_element_to_html_element(prev_sibling)
                    .unwrap()
                    .dataset()
                    .get("scale_side_images");

                if scale_value_from_prev_sibling.is_some() {
                    let _ = set_scale.try_set(scale_value_from_prev_sibling.clone().unwrap());
                    let _  = set_transform_origin.try_set(format!("calc({} * 100%) top 0", scale_value_from_prev_sibling.unwrap()));
                    let _ = set_attached_to_image.try_set(true);
                }
            }
        }) as Box<dyn FnMut(_)>);

        let _ = document()
            .add_event_listener_with_callback("image_scale", &cb.as_ref().unchecked_ref());
        cb.forget();
    });

    view! {
      <div
        node_ref=container_ref
        style=move || {
            let line_str: String;
            let left_pos = "calc(100% - 0.5rem)".to_string();
            if line > 0.0 {
                line_str = ((line - 0.5) * line_height()).to_string() + "px";
            } else if line < 0.0 {
                line_str = format!("calc(100% + {}px)", (0.5 + line) * line_height())
            } else {
                line_str = match y {
                    "bottom" => "100%".to_string(),
                    "top" => "0%".to_string(),
                    "center" => "50%".to_string(),
                    _ => y.to_string(),
                };
            }
            format!("top: {}; left: {}", line_str, left_pos)
        }

        class="side-img absolute -translate-x-1/2 !w-1 !p-0  h-1 "
      >
        <div class="w-1 h-1 relative z-20" class=("bg-red-500", move || show_areas.get())></div>

        <div
          style=move || {
              format!(
                  "left: {}; top: calc(50% + {}); transform: translateY(calc(-50% + {} + {})); padding: {}; scale: {}; transform-origin: {}",
                  offset_x,
                  if offset_y.contains("%") { "0px" } else { offset_y },
                  match img_position {
                      "bottom" => "-50%",
                      "top" => "50%",
                      _ => "0%",
                  },
                  if offset_y.contains("%") { offset_y } else { "0px" },
                  padding,
                  scale.get(),
                  transform_origin.get()
              )
          }

          class="flex shrink-0 transition-opacity duration-300 lg:transition-none lg:opacity-100  z-10 absolute w-max"
          class=("lg:pointer-events-none", move || !clickable_on_desktop)
          class=("outline-[20px]", move || show_areas.get())
          class=("outline-[#3f9aff7d]", move || show_areas.get())
          class=("outline", move || show_areas.get())
        >

          <div class="absolute z-10" style=move || format!("top: {children_y}; left: {children_x}")>
            {children.map(|c| c())}
          </div>
          <img
            src=src
            class=("max-w-max", move || width == "")
            style=move || {
                format!(
                    "transform: {};
                    transform-origin: 100% 51% 0px;
                    transition: {}s;
                    width: {width};",
                    if !popup || (popup && solution_fully_opened.get())  { "scale(1)" } else { "scale(1, 0)" },
                    if !popup || (popup && solution_fully_opened.get())  { "0.5" } else { "0" },
                )
            }
          />

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
            class=("bg-blue-900", move || show_areas.get())
          ></div>
        </div>

        <Show fallback=|| () when=move || use_squiggle_on_mobile && show_squiggles.get() && !attached_to_image.get()>
          <div
            class="squiggle block sm:hidden absolute"
            class=("outline-[20px]", move || show_areas.get())
            class=("outline-[#3f9aff7d]", move || show_areas.get())
            class=("outline", move || show_areas.get())

            style=move || {
                format!(
                    "left: 50%; top: {squiggle_y}; transform: translate(-50%, -50%); padding: 2.6rem",
                )
            }
          >
            <img src="/images/squiggle.png" class="h-11 min-w-[45px]"/>
          </div>
        </Show>
      </div>
    }
}
