use std::time::Duration;

use crate::utils::get_chapter::get_chapter;
use leptos::*;
use leptos_router::{use_location, use_navigate, NavigateOptions, State};
use serde::{Deserialize, Serialize};
use web_sys::{ScrollBehavior, ScrollIntoViewOptions};

#[derive(Serialize, Deserialize)]
struct Exercice {
    ex_number: String,
    ex_chapter: String,
    ex_opened: String,
}

#[component]
fn LabelsView(
    cx: Scope,
    vec: Vec<&'static str>,
    selected_tab: ReadSignal<usize>,
    set_selected_tab: WriteSignal<usize>,
) -> impl IntoView {
    let (_vec, set_vec) = create_signal(cx, vec);
    let set_solution_open = use_context::<WriteSignal<bool>>(cx).unwrap();
    let solution_open = use_context::<ReadSignal<bool>>(cx).unwrap();

    let navigate = use_navigate(cx);
    let navigate_ = use_navigate(cx);

    view! { cx,
      <svg
        width="43"
        height="43"
        viewBox="0 0 43 43"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        class="tab cursor-pointer overflow-visible"
        class=("disabled", move || selected_tab() == 0)
        on:click=move |_| {
            if selected_tab() != 0 {
                let options = NavigateOptions {
                    resolve: true,
                    replace: false,
                    scroll: false,
                    state: State(None)
                };
                let _ = navigate(&format!(
                    "{}?tab={}&opened={}",
                    window().location().pathname().unwrap(),
                    selected_tab() - 1,
                    solution_open()
                ), options);
            }
        }
      >

        <path
          class="overflow-visible"
          d="M35.4941 1H6.65545C3.53203 1 1 3.53203 1 6.65545V35.4941C1 38.6175 3.53203 41.1495 6.65545 41.1495H35.4941C38.6175 41.1495 41.1495 38.6175 41.1495 35.4941V6.65545C41.1495 3.53203 38.6175 1 35.4941 1Z"
          fill=move || format!("{}", if selected_tab() != 0 { "#EEFFAA" } else { "#bbbbbb" })
          fill-opacity="0.4"
          stroke="black"
          stroke-width="1.5"
          stroke-miterlimit="2"
        ></path>
        <path d="M8 21L18 26.7735V15.2265L8 21ZM17 22H34V20H17V22Z" fill="black"></path>
      </svg>
      <svg
        width="43"
        height="43"
        viewBox="0 0 43 43"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        class="tab cursor-pointer overflow-visible"
        class=("disabled", move || selected_tab() == _vec().len() - 1)
        on:click=move |_| {
            if selected_tab() != _vec().len() - 1 {
              let options = NavigateOptions {
                resolve: true,
                replace: false,
                scroll: false,
                state: State(None)
              };
              let _ = navigate_(&format!(
                  "{}?tab={}&opened={}",
                  window().location().pathname().unwrap(),
                  selected_tab() + 1,
                  solution_open()
              ), options);
            }
        }
      >

        <path
          class="overflow-visible"
          d="M35.4941 1H6.65545C3.53203 1 1 3.53203 1 6.65545V35.4941C1 38.6175 3.53203 41.1495 6.65545 41.1495H35.4941C38.6175 41.1495 41.1495 38.6175 41.1495 35.4941V6.65545C41.1495 3.53203 38.6175 1 35.4941 1Z"
          fill=move || {
              format!("{}", if selected_tab() != _vec().len() - 1 { "#EEFFAA" } else { "#bbbbbb" })
          }

          fill-opacity="0.4"
          stroke="black"
          stroke-width="1.5"
          stroke-miterlimit="2"
        ></path>
        <path
          d="M8 21L18 26.7735V15.2265L8 21ZM17 22H34V20H17V22Z"
          fill="black"
          style="transform: rotate(180deg) translateY(1px); transform-origin: center"
        ></path>
      </svg>
    }
}

#[component]
fn EndLabelsView(
    cx: Scope,
    vec: Vec<&'static str>,
    selected_tab: ReadSignal<usize>,
    set_selected_tab: WriteSignal<usize>,
) -> impl IntoView {
    let (_vec, _) = create_signal(cx, vec);

    view! { cx,
      <svg
        width="43"
        height="43"
        viewBox="0 0 43 43"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        class="tab cursor-pointer overflow-visible z-10"
        on:click=move |_| {
          let mut options = ScrollIntoViewOptions::new();
          options.behavior(ScrollBehavior::Smooth);
          document().get_element_by_id("exo").unwrap().scroll_into_view_with_scroll_into_view_options(&options);

          // this line closes the solution
          /* document().get_element_by_id("solution-button").unwrap().dyn_into::<web_sys::HtmlElement>()
           .unwrap().click(); */
        }
      >

        <path
          class="overflow-visible"
          d="M35.4941 1H6.65545C3.53203 1 1 3.53203 1 6.65545V35.4941C1 38.6175 3.53203 41.1495 6.65545 41.1495H35.4941C38.6175 41.1495 41.1495 38.6175 41.1495 35.4941V6.65545C41.1495 3.53203 38.6175 1 35.4941 1Z"
          fill="#EEFFAA"
          fill-opacity="0.4"
          stroke="black"
          stroke-width="1.5"
          stroke-miterlimit="2"
        ></path>
        <Show
          fallback=move |_| {
              view! { cx,
                <path d="M20 32C20 32.5523 20.4477 33 21 33C21.5523 33 22 32.5523 22 32H20ZM21 11L15.2265 21H26.7735L21 11ZM22 32L22 20H20L20 32H22Z" fill="black"/>
              }
          }
          when=move || selected_tab() == _vec().len() - 1
        >
        <path d="M20 32C20 32.5523 20.4477 33 21 33C21.5523 33 22 32.5523 22 32H20ZM21 11L15.2265 21H26.7735L21 11ZM22 32L22 20H20L20 32H22Z" fill="black"/>
        </Show>

      </svg>
    }
}

#[component]
pub fn tabs(cx: Scope, labels: Vec<&'static str>, children: ChildrenFn) -> impl IntoView {
    let (selected_tab, set_selected_tab) = create_signal(cx, 0);
    let solution_open = use_context::<ReadSignal<bool>>(cx).unwrap();
    let set_solution_open = use_context::<WriteSignal<bool>>(cx).unwrap();

    let location = use_location(cx);
    let (_chapter, _) = create_signal(cx, get_chapter(location.clone()));
    let url_params = location.clone().query;

    create_effect(cx, move |_| {
        let _url_params = url_params();
        let stored_selected_tab = _url_params.get("tab");
        if let Some(sst) = stored_selected_tab {
            if let Ok(tab) = sst.parse::<usize>() {
                set_selected_tab(tab);
            }
        }
    });

    create_effect(cx, move |_| {
        let _url_params = url_params();

        let stored_solution_opened = _url_params.get("opened");
        if let Some(sso) = stored_solution_opened {
            set_solution_open(sso == "true");
        }
    });

    let (solution_fully_opened, set_solution_fully_opened) = create_signal(cx, solution_open());
    create_effect(cx, move |_| {
        if solution_open() {
            set_timeout(
                move || set_solution_fully_opened(true),
                Duration::from_secs(1),
            )
        } else {
            set_solution_fully_opened(false)
        }
    });

    view! { cx,

      <div class="text-xl flex items-center justify-center gap-2 col-start-2 hidden-on-startup mb-[31px] mt-[2px]">
        <LabelsView vec=labels.clone() selected_tab=selected_tab set_selected_tab=set_selected_tab/>
      </div>
      <For
        each=move || children(cx).nodes.into_iter().enumerate()
        key=|label| label.0
        view=move |cx, label| {
            view! { cx,
              <div
                class="col-start-2 relative transition-opacity duration-500 "
                class=("opacity-0", move || selected_tab() != label.0)
                class=("h-0", move || selected_tab() != label.0)
                class=("transition-none", move || selected_tab() != label.0)
                class=("overflow-hidden", move || selected_tab() != label.0)
              >
                {label.1}
              </div>
            }
        }
      />

      <Show fallback=|_| () when=move || solution_fully_opened() >
        <div class="text-xl flex items-center justify-center gap-2 col-start-2">
          <EndLabelsView
            vec=labels.clone()
            selected_tab=selected_tab
            set_selected_tab=set_selected_tab
          />
        </div>
      </Show>
    }
}

#[component]
pub fn TabElement(cx: Scope, children: ChildrenFn) -> impl IntoView {
    view! { cx, {children(cx)} }
}
