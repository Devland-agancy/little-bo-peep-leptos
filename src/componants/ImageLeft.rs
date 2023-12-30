use crate::page::state::PageState;
use leptos::{
    html::{Button, Img},
    *,
};

#[component]
pub fn ImageLeft(
    cx: Scope,
    src: &'static str,
    #[prop(default = false)] hidden_in_mobile: bool,
    #[prop(default = "center")] position: &'static str,
    #[prop(default = "")] pos_y: &'static str,
    #[prop(default = "0px")] pos_x: &'static str,

    #[prop(default = "-1.5rem")] squiggle_right: &'static str,
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
    let show_left = move || page_state() == PageState::ShowLeft;
    let image_ref = create_node_ref::<Img>(cx);
    let show_areas = use_context::<ReadSignal<bool>>(cx).unwrap();

    view! { cx,
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
        }

        style=move || {
            format!(
                "transform: translateX(calc(0px - 100%)); left: {}; top: {}; padding: {}",
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

        class="flex shrink-0 transition-opacity duration-300 lg:transition-none lg:opacity-100 z-10 absolute"
        class=("pointer-events-none", show_left)
        class=("outline-[20px]", move || show_areas())
        class=("outline-blue-300", move || show_areas())
            class=("outline", move || show_areas())

      >
        <div style=move || format!(" top: {}; left: {}", children_y, children_x)>{children(cx)}</div>
        <img node_ref=image_ref src=src style=move || format!("min-width: {}px", width)/>

        <Show fallback=|_| () when=move || hidden_in_mobile>
          <div
            class="block sm:hidden absolute"
            class=("outline-[20px]", move || show_areas())
            class=("outline-blue-300", move || show_areas())
            class=("outline", move || show_areas())

            style=move || format!("right: calc({} - 40px); top: calc({} - 40px); padding: {}", squiggle_right, squiggle_top, "2.6rem")
          >
            <img src="/images/squiggle.png" class="h-11"/>
          </div>
        </Show>
      </button>
    }
}
