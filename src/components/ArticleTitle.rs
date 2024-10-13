use crate::constants::{
    CHAPTER_TITLE_BOTTOM_MARGIN_DESKTOP, CHAPTER_TITLE_BOTTOM_MARGIN_MOBILE,
    CHAPTER_TITLE_TOP_MARGIN_DESKTOP, CHAPTER_TITLE_TOP_MARGIN_MOBILE, MOBILE_SCREEN_MAX_WIDTH,
};
use leptos::{ev::resize, *};
use leptos_use::use_event_listener;

#[component]
pub fn ArticleTitle(
    label: &'static str,
    #[prop(default = "")] on_mobile: &'static str,
    #[prop(default = "")] class: &'static str,
) -> impl IntoView {
    let (mobile, set_mobile) = create_signal(false);

    create_effect(move |_| {
        if window().inner_width().unwrap().as_f64().unwrap() <= MOBILE_SCREEN_MAX_WIDTH as f64 {
            set_mobile.set(true)
        }
        let _ = use_event_listener(window(), resize, move |_| {
            if window().inner_width().unwrap().as_f64().unwrap() <= MOBILE_SCREEN_MAX_WIDTH as f64 {
                set_mobile.set(true)
            } else {
                set_mobile.set(false)
            }
        });
    });

    view! {
      <div class="sm:grid gridColsWidth pt-14">
        <h1
          class=format!("sm:col-start-2 text-3xl sm:text-4xl p-4 {class}")
          style=move || {
              format!(
                  "margin-top: {};margin-bottom: {}",
                  if mobile.get() {
                      CHAPTER_TITLE_TOP_MARGIN_MOBILE
                  } else {
                      CHAPTER_TITLE_TOP_MARGIN_DESKTOP
                  },
                  if mobile.get() {
                      CHAPTER_TITLE_BOTTOM_MARGIN_MOBILE
                  } else {
                      CHAPTER_TITLE_BOTTOM_MARGIN_DESKTOP
                  },
              )
          }
        >

          <span class="sm:hidden">{if on_mobile == "" { label } else { on_mobile }}</span>
          <span class="hidden sm:block">{label}</span>
        </h1>
      </div>
    }
}
