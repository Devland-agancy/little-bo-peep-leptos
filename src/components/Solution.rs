use crate::{
    components::Section::Spacer,
    constants::{GREEN_DIV_HEIGHT, TEXT_LEFT_PADDING, TEXT_RIGHT_PADDING},
    global_state::GlobalState,
};
use leptos::{
    ev::{click, resize},
    html::Div,
    *,
};
use leptos_router::{use_navigate, NavigateOptions, State};
use leptos_use::use_event_listener;
use std::cmp::min;
use std::{sync::Arc, time::Duration};
use web_sys::{MouseEvent, ScrollBehavior, ScrollIntoViewOptions};

#[component]
pub fn Solution(cx: Scope, solution_number: usize, children: Children) -> impl IntoView {
    let GlobalState {
        solutions_state,
        solution_transition_duration,
        ..
    } = use_context::<GlobalState>(cx).unwrap();

    let transition_duration = move || {
        if solution_transition_duration.get().len() > 0 {
            solution_transition_duration.get()[solution_number]
        } else {
            1000
        }
    };

    let solution_open = move || {
        if solutions_state.get().len() > 0 {
            solutions_state.get()[solution_number]
        } else {
            false
        }
    };
    let (content_height, set_content_height) = create_signal(cx, 0);

    let node_ref = create_node_ref::<Div>(cx);
    let button = create_node_ref::<Div>(cx);

    create_effect(cx, move |_| {
        if node_ref().is_some() {
            if solution_open() {
                if node_ref().unwrap().offset_height() == 0 {
                    set_timeout(
                        move || {
                            set_content_height(node_ref().unwrap().offset_height());
                        },
                        Duration::from_secs(1),
                    )
                } else {
                    set_content_height(node_ref().unwrap().offset_height());
                }
            } else {
                set_content_height(0)
            }
        }

        set_timeout(
            move || {
                if node_ref().is_some() {
                    if solution_open() {
                        set_content_height(node_ref().unwrap().offset_height());
                    } else {
                        set_content_height(0)
                    }
                }
            },
            Duration::from_secs(3),
        )
    });

    create_effect(cx, move |_| {
        let _ = use_event_listener(cx, window(), resize, move |_| {
            GlobalState::update_solutions_state(
                solution_transition_duration,
                solution_number,
                min(1000, node_ref().unwrap().offset_height()),
            );

            if node_ref().is_some() {
                if solution_open() {
                    set_content_height(node_ref().unwrap().offset_height());
                } else {
                    set_content_height(0)
                }
            }
        });
    });

    create_effect(cx, move |_| {
        set_timeout(
            move || {
                GlobalState::update_solutions_state(
                    solution_transition_duration,
                    solution_number,
                    min(1000, node_ref().unwrap().offset_height()),
                );
            },
            Duration::from_secs(3),
        );
    });

    let (transition, set_transition) = create_signal(cx, false);

    let navigate = use_navigate(cx);

    let (solution_fully_opened, set_solution_fully_opened) = create_signal(cx, solution_open());
    let (handle, set_handle) = create_signal(cx, None);
    create_effect(cx, move |_| {
        if solution_open() {
            let timeout_handle = set_timeout_with_handle(
                move || set_solution_fully_opened(true),
                Duration::from_millis(transition_duration() as u64),
            );
            set_handle(Some(timeout_handle))
        } else {
            if let Some(handle) = handle() {
                handle.unwrap().clear();
            };
            set_solution_fully_opened(false);
            set_timeout(
                // sometimes the above line executes before 1 second of the above block is passed so we make sure is stays false
                move || set_solution_fully_opened(false),
                Duration::from_millis(transition_duration() as u64),
            )
        }
    });

    view! { cx,
      <div
        node_ref=button
        class="relative col-start-2"
        style=format!("padding-left: {}; padding-right: {}", TEXT_LEFT_PADDING, TEXT_RIGHT_PADDING)
      >
        <SolutionSVG
          solution_number=solution_number
          on_click=move |_| {
              let element_pos = window().inner_height().unwrap().as_f64().unwrap()
                  - button().unwrap().get_bounding_client_rect().bottom();
              let should_scroll_to_button_first = element_pos
                  > GREEN_DIV_HEIGHT as f64 + 40_f64 + 56_f64;
              if solution_open() && should_scroll_to_button_first {
                  let mut options = ScrollIntoViewOptions::new();
                  options.behavior(ScrollBehavior::Smooth);
                  document()
                      .get_element_by_id("exo")
                      .unwrap()
                      .scroll_into_view_with_scroll_into_view_options(&options);
              }
              set_transition(true);
              set_timeout(move || set_transition(false), Duration::from_millis(1100));
              let options = NavigateOptions {
                  resolve: true,
                  replace: false,
                  scroll: false,
                  state: State(None),
              };
              if let Ok(search_params) = window().location().search() {
                  let new_url: String;
                  if search_params.contains("&opened=true") {
                      new_url = window().location().pathname().unwrap()
                          + &search_params.replace("&opened=true", "&opened=false")
                  } else if search_params.contains("&opened=false") {
                      new_url = window().location().pathname().unwrap()
                          + &search_params.replace("&opened=false", "&opened=true")
                  } else if &search_params == "" {
                      new_url = window().location().pathname().unwrap() + "?tab=0"
                          + &format!("&opened={}", !solution_open())
                  } else {
                      new_url = window().location().pathname().unwrap() + &search_params
                          + &format!("&opened={}", !solution_open())
                  }
                  let _ = navigate(&new_url, options);
              }
              GlobalState::update_solutions_state(
                  solutions_state,
                  solution_number,
                  !solution_open(),
              );
          }
        />

      </div>
      <div
        class="solution col-start-2 transition-[height]  relative"
        class=("pointer-events-none", move || !solution_open())
        class=("animated-height-full", move || solution_open())
        class=("overflow-y-clip", move || !solution_fully_opened())

        style=move || {
            format!(
                "height: {}px; transition-duration: {}ms",
                content_height(),
                transition_duration(),
            )
        }
      >

        <div
          node_ref=node_ref
          class="transition-all"
          class=("-translate-y-full", move || !solution_open())
          style=move || { format!("transition-duration: {}ms", transition_duration()) }

          class=("transition-all", move || transition())
        >
          {children(cx)}
        </div>

      </div>
    }
}

#[component]
pub fn SolutionSVG<F>(cx: Scope, solution_number: usize, on_click: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let GlobalState {
        solutions_state, ..
    } = use_context::<GlobalState>(cx).unwrap();
    let solution_open = move || {
        if solutions_state.get().len() > 0 {
            solutions_state.get()[solution_number]
        } else {
            false
        }
    };
    let button = create_node_ref(cx);

    let on_click_arc: Arc<F> = Arc::new(on_click);
    create_effect(cx, move |_| {
        //let on_click_clone = on_click.clone();
        let cloned_on_click = Arc::clone(&on_click_arc);
        let _ = use_event_listener(cx, button, click, move |e| {
            cloned_on_click(e);
        });
    });

    view! { cx,
      <Spacer/>
      <div id="solution-button" node_ref=button class="column cursor-pointer">
        <svg class="mx-auto h-[37px] overflow-visible">
          <g class="solution_button_svg">
            <rect
              id="solution_button_focus_rect"
              class="focus_alpha_fill"
              x="-7"
              y="-7"
              width="123"
              height="50"
            ></rect>

            <rect
              id="solution_button_focus_rect"
              class="solution_button_transition"
              class=("active_solution_button_rect", move || !solution_open())
              class=("inactive_solution_button_rect", move || solution_open())

              width="109"
              height="36"
            ></rect>

            <path
              id="solution_button_lip"
              class="solution_button_transition"
              class=("active_solution_button_lip", move || !solution_open())
              class=("inactive_solution_button_lip", move || solution_open())
              d="M 0 10 v -10 h 109 v 10 M 0 26 v 10 h 109 v -10"
            ></path>

            <g
              id="solution_button_finger_pair"
              class="solution_button_transition"
              class=("active_solution_button_hands", move || !solution_open())
              class=("inactive_solution_button_hands", move || solution_open())
            >
              <g transform="translate(101.5, 18)">
                <use_ href="#finger_pointing_left"></use_>
              </g>
              <g transform="scale(-1, 1) translate(-8, 20)">
                <use_ href="#finger_pointing_left"></use_>
              </g>
              <use_ href="#solution_button_text"></use_>
            </g>
          </g>
        </svg>
      </div>
      <Spacer/>
      <Spacer/>
    }
}
