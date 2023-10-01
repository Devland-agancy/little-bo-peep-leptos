use leptos::{html::{Img} ,*};
use crate::page::state::PageState;

#[component]
pub fn ImageRight(
  cx: Scope,
  translate: &'static str,
  src: &'static str,
  #[prop(default = false)] absolute: bool,
  #[prop(optional)] top: i32,
  #[prop(optional)] right: i32,

  #[prop(default = "")] children_inset: &'static str,
  children: Children

) -> impl IntoView {
    let set_page_state =
        use_context::<WriteSignal<PageState>>(cx).expect("set_page_state context to exist");
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let show_right = move || page_state() == PageState::ShowRight;

    let set_right_image_x_pos = use_context::<WriteSignal<f64>>(cx).unwrap();
    let image_ref = create_node_ref::<Img>(cx);

   /*  let styleStr = move || format!("transform: translate{}; left: {}px; right: {}px; top: {}px", translate, left , right , top ) */
    view! {cx,
        <div class="col-start-3 h-0 flex items-center justify-start">
            <button
                on:click=move |e| {
                    e.stop_propagation();
                    set_page_state.update(|value| *value = match value {
                        PageState::ShowArticle => PageState::ShowRight,
                        _ => PageState::ShowArticle
                    });
                    set_right_image_x_pos.update(|val| *val = f64::from(image_ref().unwrap().get_bounding_client_rect().left() - 50_f64))
                }
                style=move || format!("transform: translate{}; right: {}px; top: {}px", translate, right, top )
                class="flex shrink-0 transition-opacity duration-300 lg:transition-none lg:opacity-100 lg:pointer-events-none z-10"
                class=("pointer-events-none", show_right)
                class=("absolute", move || absolute)
            >
                <div
                    class="absolute"
                    style=move || format!("inset: {}", children_inset)
                >
                    {children(cx)}
                </div>
                <img src=src node_ref=image_ref />
            </button>
        </div>
    }
}