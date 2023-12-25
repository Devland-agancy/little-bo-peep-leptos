use crate::page::state::PageState;
use leptos::{ev::resize, *};
use leptos_use::use_event_listener;

#[component]
pub fn Image(
    cx: Scope,
    src: &'static str,
    #[prop(default = "")] id: &'static str,
    #[prop(default = "")] container_classes: &'static str,
    #[prop(default = "")] image_classes: &'static str,
    #[prop(default = "")] height: &'static str,
    #[prop(default = "")] width: &'static str,
    #[prop(default = "")] cloud_image: &'static str,
) -> impl IntoView {
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();

    view! { cx,
      <div
        class=move || {
            format!(
                "my-[15px] relative col-start-2 scrollbar-hidden md:overflow-x-visible {}",
                container_classes,
            )
        }

        class=("overflow-x-scroll", move || page_state() == PageState::ShowArticle)
      >
        <img
          id=id
          src=src
          style= move || format!("height: {}; width: {};", height, width)
          class=move || {
              format!(
                  "max-w-none m-auto {}",
                  image_classes,
              )
          }
        />

      </div>
    }
}
