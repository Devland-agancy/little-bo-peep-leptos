use std::rc::Rc;

use crate::global_state::GlobalState;
use leptos::*;
use leptos_router::{use_navigate, NavigateOptions, State};
use std::time::Duration;
use web_sys::{ScrollBehavior, ScrollToOptions};

#[component]
pub fn CustomLink(
    cx: Scope,
    href: &'static str,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] base_href: &'static str,

    children: Children,
) -> impl IntoView {
    let navigate = Rc::new(use_navigate(cx));
    let GlobalState {
        route, margin_mode, ..
    } = use_context(cx).unwrap();

    view! { cx,
      <a
        class=move || format!("link cursor-pointer {}",class)
        on:click=move |_| {
            let _ = navigate(
              [base_href, href].concat().as_str(),
              NavigateOptions {
                  resolve: true,
                  replace: false,
                  scroll: false,
                  state: State(None),
              },
          );
          route.set(href);
          margin_mode.set(false);
        }
    >
        {children(cx)}
      </a>
    }
}
