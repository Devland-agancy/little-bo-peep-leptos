use leptos::*;
use crate::page::state::PageState;

#[component]
pub fn Image(cx: Scope, src: &'static str, height: i32, #[prop(default = -1)] mobile_height: i32, 
#[prop(default = "")] containerClasses: &'static str , #[prop(default = "")] imageClasses: &'static str) -> impl IntoView {
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let (screen_mobile, set_screen_mobile) = create_signal(cx, height);

    create_effect(cx, move |_|{
        if window().inner_width().unwrap().as_f64().unwrap() <= 640_f64 && mobile_height > -1 {
            set_screen_mobile(mobile_height)
        }
    });
    view! { cx,
      <div
        class=move || {
            format!(
                "px-4 my-10 relative col-start-2 scrollbar-hidden md:overflow-x-visible {}",
                containerClasses,
            )
        }

        style=move || format!("height: {}px", screen_mobile() + 10)
        class=("overflow-x-scroll", move || page_state() == PageState::ShowArticle)
        class=("translate-x-full", move || page_state() == PageState::ShowRight)
        class=("-translate-x-full", move || page_state() == PageState::ShowLeft)
      >

        <img
          src=src
          style=move || format!("height: {}px", screen_mobile())
          class=move || {
              format!(
                  "max-w-none md:absolute md:-translate-x-1/2 md:left-1/2 m-auto {}",
                  imageClasses,
              )
          }
        />

      </div>
    }
}