use std::rc::Rc;

use crate::global_state::GlobalState;
use leptos::*;
use leptos_router::{use_navigate, NavigateOptions, State};
use std::time::Duration;

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
        class=move || format!("cursor-pointer {}",class)
        on:click=move |_| {
          let navigate = Rc::clone(&navigate);
          if margin_mode() {
            margin_mode.set(false);
            let _ = set_timeout(move || {
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
            }, Duration::from_millis(500));
          }
          else {
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
          }
        }
    >
        {children(cx)}
      </a>
    }
}
