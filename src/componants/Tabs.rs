use std::time::Duration;

use crate::utils::get_chapter::get_chapter;
use leptos::*;
use leptos_router::use_location;
use serde::{Deserialize, Serialize};
use serde_json;
use wasm_bindgen::JsCast;
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
                set_selected_tab(selected_tab() - 1);
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
                set_selected_tab(selected_tab() + 1);
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
    let (_vec, set_vec) = create_signal(cx, vec);
    let set_solution_open = use_context::<WriteSignal<bool>>(cx).unwrap();
    let navigate = leptos_router::use_navigate(cx);

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
    let (chapter, _) = create_signal(cx, get_chapter(location));

    create_effect(cx, move |_| {
        let mut stored_selected_tab = None;
        match window().local_storage() {
            Ok(Some(storage)) => {
                let stored_selected_tab_key = format!("{}_exercice", chapter());

                stored_selected_tab = Some(storage.get_item(&stored_selected_tab_key));
            }
            _ => {}
        }
        if let Some(sst) = stored_selected_tab {
            match sst {
                Ok(Some(value)) => {
                    set_selected_tab(value.parse::<usize>().unwrap());
                }
                _ => {}
            }
        }
    });

    create_effect(cx, move |_| {
        let mut stored_solution_opened = None;

        match window().local_storage() {
            Ok(Some(storage)) => {
                let stored_solution_opened_key =
                    format!("{}_exo_{}_opened", chapter(), selected_tab());
                stored_solution_opened = Some(storage.get_item(&stored_solution_opened_key))
            }
            _ => {}
        }

        if let Some(sso) = stored_solution_opened {
            match sso {
                Ok(Some(value)) => {
                    log!(
                        "vvvvvvvv{}, {}",
                        value,
                        format!("{}_exo_{}_opened", chapter(), selected_tab())
                    );
                    set_solution_open(value == "true");
                }
                _ => {}
            }
        }
    });

    create_effect(cx, move |_| {
        solution_open();
        selected_tab();
        set_timeout(
            move || match window().local_storage() {
                Ok(Some(storage)) => {
                    let exo = Exercice {
                        ex_number: selected_tab().to_string(),
                        ex_chapter: chapter(),
                        ex_opened: solution_open().to_string(),
                    };
                    let selected_exo = format!("{}_exercice", exo.ex_chapter);
                    let ex_opened = format!("{}_exo_{}_opened", exo.ex_chapter, exo.ex_number);

                    let _ = storage.set_item(&selected_exo, &exo.ex_number);
                    let _ = storage.set_item(&ex_opened, &exo.ex_opened);
                }
                _ => (),
            },
            Duration::from_millis(100),
        );
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
