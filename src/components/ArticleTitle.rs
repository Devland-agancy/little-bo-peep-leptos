use crate::constants::{MOBILE_BREAKPOINT, TITLE_MARGIN};
use leptos::{ev::resize, *};
use leptos_use::use_event_listener;

#[component]
pub fn ArticleTitle(
    cx: Scope,
    label: &'static str,
    #[prop(default = "")] on_mobile: &'static str,
) -> impl IntoView {
    let (mobile, set_mobile) = create_signal(cx, false);

    create_effect(cx, move |_| {
        if window().inner_width().unwrap().as_f64().unwrap() <= MOBILE_BREAKPOINT as f64 {
            set_mobile(true)
        }
        let _ = use_event_listener(cx, window(), resize, move |_| {
            if window().inner_width().unwrap().as_f64().unwrap() <= MOBILE_BREAKPOINT as f64 {
                set_mobile(true)
            } else {
                set_mobile(false)
            }
        });
    });

    view! { cx,
      <div class="sm:grid gridColsWidth pt-14" >
        <h1 class="sm:col-start-2 text-3xl sm:text-4xl p-4" style=move || format!("margin-top: {};margin-bottom: {}", if mobile() {TITLE_MARGIN.0} else {TITLE_MARGIN.2}, if mobile() {TITLE_MARGIN.1} else {TITLE_MARGIN.3})>
          <span class="sm:hidden">{if on_mobile == "" {label} else {on_mobile}}</span>
          <span class="hidden sm:block">{label}</span>
        </h1>
      </div>
    }
}
