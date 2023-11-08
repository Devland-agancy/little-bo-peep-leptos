use leptos::{html::Div, html::Svg, *};
use web_sys::Node;

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
                set_solution_open(false)
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
                set_solution_open(false)
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
        class=("disabled", move || selected_tab() == 0)
        on:click=move |_| {
            if selected_tab() != _vec().len() - 1 {
                set_selected_tab(selected_tab() + 1);
                set_solution_open(false);
            } else {
                let location_split = document().location().unwrap().to_string().split("/");
                let curr_chapter_str = location_split
                    .get(location_split.length() - 1)
                    .as_string()
                    .unwrap();
                let curr_chapter_vec: Vec<&str> = curr_chapter_str.split("_").collect();
                let curr_chapter = curr_chapter_vec[curr_chapter_vec.len() - 1]
                    .parse::<u16>()
                    .unwrap();
                let _ = navigate(&format!("/article/ch_{}", curr_chapter + 1), Default::default());
                set_solution_open(false);
            }
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
                <path
                  d="M17 32V17"
                  stroke="black"
                  stroke-width="1.5"
                  stroke-miterlimit="2"
                  stroke-linecap="round"
                ></path>
                <path
                  d="M17 16C16.4477 16 16 16.4477 16 17C16 17.5523 16.4477 18 17 18V16ZM33 17L23 11.2265V22.7735L33 17ZM17 18H24V16H17V18Z"
                  fill="black"
                ></path>
              }
          }

          when=move || selected_tab() == _vec().len() - 1
        >
          <path
            d="M27.7673 20.6572L17.7673 14.8837V26.4307L27.7673 20.6572ZM9.76733 21.6572H18.7673V19.6572H9.76733V21.6572Z"
            fill="black"
          ></path>
          <path
            d="M33.7673 20.6572L23.7673 14.8837V26.4307L33.7673 20.6572ZM7.76733 21.6572H24.7673V19.6572H7.76733V21.6572Z"
            fill="black"
          ></path>
        </Show>

      </svg>
    }
}

#[component]
pub fn tabs(cx: Scope, labels: Vec<&'static str>, children: ChildrenFn) -> impl IntoView {
    let (selected_tab, set_selected_tab) = create_signal(cx, 0);
    let solution_open = use_context::<ReadSignal<bool>>(cx).unwrap();

    view! { cx,
      <div class="text-xl flex items-center justify-center gap-2 col-start-2 hidden-on-startup mb-8">
        <LabelsView vec=labels.clone() selected_tab=selected_tab set_selected_tab=set_selected_tab/>
      </div>
      <For
        each=move || children(cx).nodes.into_iter().enumerate()

        key=|label| label.0
        view=move |cx, label| {
            view! { cx,
              <div
                class="col-start-2 relative transition-opacity duration-500"
                class=("opacity-0", move || selected_tab() != label.0)
                class=("h-0", move || selected_tab() != label.0)
                class=("w-0", move || selected_tab() != label.0)
                class=("transition-none", move || selected_tab() != label.0)
              >
                {label.1}
              </div>
            }
        }
      />

      <Show fallback=|_| () when=move || solution_open()>
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
