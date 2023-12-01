use crate::page::state::PageState;
use leptos::{ev::resize, *};
use leptos_use::use_event_listener;

#[component]
pub fn Image(
    cx: Scope,
    src: &'static str,
    height: i32,
    #[prop(default = -1)] width: i32,
    #[prop(default = "")] id: &'static str,
    #[prop(default = -1)] mobile_height: i32,
    #[prop(default = "")] containerClasses: &'static str,
    #[prop(default = "")] imageClasses: &'static str,
) -> impl IntoView {
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let (screen_mobile, set_screen_mobile) = create_signal(cx, height);

    create_effect(cx, move |_| {
        if window().inner_width().unwrap().as_f64().unwrap() <= 640_f64 && mobile_height > -1 {
            set_screen_mobile(mobile_height)
        }
    });

    let (image_is_wide, set_image_is_wide) = create_signal(cx, false);
    create_effect(cx, move |_| {
        set_image_is_wide(window().inner_width().unwrap().as_f64().unwrap() <= width as f64)
    });

    create_effect(cx, move |_| {
        let _ = use_event_listener(cx, window(), resize, move |_| {
            set_image_is_wide(window().inner_width().unwrap().as_f64().unwrap() <= width as f64)
        });
    });

    view! { cx,
      <div
        class=move || {
            format!(
                "my-[15px] relative col-start-2 scrollbar-hidden md:overflow-x-visible {} {}",
                if !image_is_wide() {
                  "px-4"
                } else {
                  ""
                 },
                containerClasses,
            )
        }
        style=move || format!(
          "height: {}{}; ",
          if screen_mobile() > 0 {
             ( screen_mobile() +  10 ).to_string()
          } else { "".to_string() },
          if screen_mobile() > 0 {
            "px"
          } else { "auto" },
          )
        class=("overflow-x-scroll", move || page_state() == PageState::ShowArticle)
      >
        <img
          id=id
          src=src
          style=move || format!(
            "height: {}{}; width: {}{}",
            if screen_mobile() > 0 {
               screen_mobile().to_string()
            } else { "".to_string() },
            if screen_mobile() > 0 {
              "px"
            } else { "auto" },
            if width > 0 {
              width.to_string()
            } else { "auto".to_string() },
            if width > 0 {
              "px"
            } else { "auto" }
            )
          class=move || {
              format!(
                  "max-w-none m-auto {} {}",
                  if !image_is_wide() {
                    "absolute -translate-x-1/2 left-1/2"
                  } else {
                    ""
                   },
                  imageClasses,
              )
          }
        />

      </div>
    }
}
