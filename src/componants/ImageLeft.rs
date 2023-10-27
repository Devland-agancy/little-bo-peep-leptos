use leptos::{html::{Img} ,*, ev::resize};
use leptos_use::use_event_listener;
use web_sys::HtmlDivElement;
use crate::page::state::PageState;
use wasm_bindgen::JsCast;

#[component]
pub fn ImageLeft(
    cx: Scope,
    translate: &'static str,
    src: &'static str,
    #[prop(default = false)] hidden_in_mobile: bool,
    #[prop(default = false)] absolute: bool,
    #[prop(optional)] top: i32,
    #[prop(optional)] left: i32,
    #[prop(default = "")] children_inset: &'static str,

    #[prop(default = "")] attached_to: &'static str,

    #[prop(default = "-1.5rem")] squiggle_right: &'static str,
    #[prop(default = "46%")] squiggle_top: &'static str,
    
    children: Children,

) -> impl IntoView {
    let set_page_state =
        use_context::<WriteSignal<PageState>>(cx).expect("set_page_state context to exist");
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let show_left = move || page_state() == PageState::ShowLeft;

    let image_ref = create_node_ref::<Img>(cx);

   let (top_pos, set_top_pos) = create_signal(cx, top);
   create_effect(cx, move |_|{
        if attached_to != "" {
            let el = document().get_element_by_id(attached_to);
            if el.is_some() {
                set_top_pos(el.unwrap().dyn_into::<HtmlDivElement>().unwrap().offset_top())
            }
        }
   });
   create_effect(cx, move |_|{
        let _ = use_event_listener(cx, window(), resize, move |_| {
            if attached_to != "" {
                let el = document().get_element_by_id(attached_to);
                if el.is_some() {
                set_top_pos(el.unwrap().dyn_into::<HtmlDivElement>().unwrap().offset_top())
                }
            }
        });
    });
    view! { cx,
      <div class="col-start-1 h-0 flex items-center justify-end">
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
              format!("transform: translate{}; left: {}px; top: {}px", translate, left, top_pos())
          }
          class="flex shrink-0 transition-opacity duration-300 lg:transition-none lg:opacity-100 lg:pointer-events-none z-10"
          class=("pointer-events-none", show_left)
          class=("absolute", move || absolute)
        >
          <div style=move || format!("inset: {}", children_inset)>{children(cx)}</div>
          <img src=src/>

          <Show fallback=|_| () when=move || hidden_in_mobile>
            <div
              class="block sm:hidden absolute"
              style=move || format!("right: {}; top: {}", squiggle_right, squiggle_top)
            >
              <img src="/images/squiggle.png" class="h-11"/>
            </div>
          </Show>
        </button>
      </div>
    }
}
