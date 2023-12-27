use crate::constants::SHOW_CLICKABLE_ITEMS_BORDERS;
use crate::page::state::PageState;
use leptos::{
    html::{Button, Img},
    *,
};

#[component]
pub fn ImageRight(
    cx: Scope,
    src: &'static str,
    #[prop(default = false)] hidden_in_mobile: bool,
    #[prop(default = "center")] position: &'static str,
    #[prop(default = "")] pos_y: &'static str,
    #[prop(default = "0px")] pos_x: &'static str,

    #[prop(default = "-1.5rem")] squiggle_left: &'static str,
    #[prop(default = "30%")] squiggle_top: &'static str,

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
    let show_right = move || page_state() == PageState::ShowRight;

    let set_right_image_x_pos = use_context::<WriteSignal<f64>>(cx).unwrap();

    let image_ref = create_node_ref::<Img>(cx);

    view! { cx,
      <button

        on:click=move |e| {
            e.stop_propagation();
            set_page_state
                .update(|value| {
                    *value = match value {
                        PageState::ShowArticle => PageState::ShowRight,
                        _ => PageState::ShowArticle,
                    };
                });
            if page_state() == PageState::ShowRight {
                set_right_image_x_pos
                    .update(|val| {
                        *val = f64::from(image_ref().unwrap().get_bounding_client_rect().left());
                    })
            }
        }

        style=move || {
            format!(
                "transform: translateX(calc(0px + 100%)); right: {}; top: {}; padding: {}",
                pos_x,
                if pos_y != "" {
                    pos_y
                } else {
                    match position {
                        "bottom" => "50%",
                        "top" => "0",
                        _ => "auto",
                    }
                },
                padding
            )
        }

        class="flex shrink-0 transition-opacity duration-300 lg:transition-none lg:opacity-100  z-10 absolute"
        class=("pointer-events-none", show_right)
        class=("lg:pointer-events-none", move || !clickable_on_desktop)
        class=("outline-[20px]", move || SHOW_CLICKABLE_ITEMS_BORDERS)
        class=("outline-blue-300", move || SHOW_CLICKABLE_ITEMS_BORDERS)
        class=("outline", move || SHOW_CLICKABLE_ITEMS_BORDERS)

      >
        <div class="absolute" style=move || format!("top: {}; left: {}", children_y, children_x)>
          {children(cx)}
        </div>
        <img node_ref=image_ref src=src style=move || format!("min-width: {}px", width)/>

        <Show fallback=|_| () when=move || hidden_in_mobile>
          <div
            class="block sm:hidden absolute"
            class=("outline-[20px]", move || SHOW_CLICKABLE_ITEMS_BORDERS)
            class=("outline-blue-300", move || SHOW_CLICKABLE_ITEMS_BORDERS)
            class=("outline", move || SHOW_CLICKABLE_ITEMS_BORDERS)

            style=move || format!("left: calc({} - 40px); top: calc({} - 40px); padding: {}", squiggle_left, squiggle_top, "2.6rem")
          >
            <img src="/images/squiggle.png" class="h-11"/>
          </div>
        </Show>
      </button>
    }
}
