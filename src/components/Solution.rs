use crate::{
    components::Section::Spacer,
    constants::{GREEN_DIV_HEIGHT, TEXT_X_PADDING},
    global_state::GlobalState,
};
use leptos::{
    ev::{click, resize},
    html::Div,
    *,
};
use leptos_router::{use_navigate, NavigateOptions, State};
use leptos_use::use_event_listener;
use std::{cell::RefCell, cmp::min};
use std::{sync::Arc, time::Duration};
use web_sys::{MouseEvent, ScrollBehavior, ScrollIntoViewOptions};

#[component]
pub fn Solution(solution_number: usize, children: Children) -> impl IntoView {
    let GlobalState {
        solutions_state,
        solution_transition_duration,
        ..
    } = use_context::<GlobalState>().unwrap();

    let transition_duration = move || {
        if solution_transition_duration.get().len() > solution_number {
            solution_transition_duration.get()[solution_number]
        } else {
            1000
        }
    };

    let solution_open = create_memo(move |_| {
        if solutions_state.get().len() > solution_number {
            solutions_state.get()[solution_number]
        } else {
            false
        }
    });

    let (content_height, set_content_height) = create_signal(0);

    let node_ref = create_node_ref::<Div>();
    let button = create_node_ref::<Div>();

    create_effect(move |_| {
        let node_ref = RefCell::new(node_ref.get());
        let node_ref_clone = node_ref.clone();

        if node_ref.take().is_some() {
            if solution_open.get() {
                if node_ref.take().unwrap().offset_height() == 0 {
                    set_timeout(
                        move || {
                            set_content_height.set(node_ref.take().unwrap().offset_height());
                        },
                        Duration::from_secs(1),
                    )
                } else {
                    set_content_height.set(node_ref.take().unwrap().offset_height());
                }
            } else {
                set_content_height.set(0)
            }
        }

        set_timeout(
            move || {
                if node_ref_clone.take().is_some() {
                    // tre get untracked is used in case of route change before timeout ends
                    if let Some(solution_open) = solution_open.try_get_untracked() {
                        if solution_open {
                            set_content_height.set(node_ref_clone.take().unwrap().offset_height());
                        } else {
                            set_content_height.set(0)
                        }
                    }
                }
            },
            Duration::from_secs(3),
        )
    });

    create_effect(move |_| {
        let node_ref = node_ref.get();

        let _ = use_event_listener(window(), resize, move |_| {
            GlobalState::update_solutions_state(
                solution_transition_duration,
                solution_number,
                min(1000, node_ref.as_ref().unwrap().offset_height()),
            );

            if node_ref.is_some() {
                if solution_open.get_untracked() {
                    set_content_height.set(node_ref.as_ref().unwrap().offset_height());
                } else {
                    set_content_height.set(0)
                }
            }
        });
    });

    create_effect(move |_| {
        let node_ref = node_ref.get();
        set_timeout(
            move || {
                if let Some(node_ref) = node_ref {
                    GlobalState::update_solutions_state(
                        solution_transition_duration,
                        solution_number,
                        min(1000, node_ref.offset_height()),
                    );
                }
            },
            Duration::from_secs(3),
        );
    });

    let (transition, set_transition) = create_signal(false);

    let navigate = use_navigate();

    let (solution_fully_opened, set_solution_fully_opened) =
        create_signal(solution_open.get_untracked());
    let (handle, set_handle) = create_signal(None);
    create_effect(move |_| {
        if solution_open.get() {
            let timeout_handle = set_timeout_with_handle(
                move || set_solution_fully_opened.set(true),
                Duration::from_millis(transition_duration() as u64),
            );
            set_handle.set(Some(timeout_handle))
        } else {
            if let Some(handle) = handle.get() {
                handle.unwrap().clear();
            };
            set_solution_fully_opened.set(false);
            set_timeout(
                // sometimes the above line executes before 1 second of the above block is passed so we make sure is stays false
                move || {
                    let _ = set_solution_fully_opened.try_set(false);
                },
                Duration::from_millis(transition_duration() as u64),
            )
        }
    });

    view! {
      <div
        node_ref=button
        class="relative col-start-2"
        style=format!("padding-inline: {}", TEXT_X_PADDING)
      >
        <SolutionSVG
          solution_number=solution_number
          on_click=move |_| {
              let element_pos = window().inner_height().unwrap().as_f64().unwrap()
                  - button.get().unwrap().get_bounding_client_rect().bottom();
              let should_scroll_to_button_first = element_pos
                  > GREEN_DIV_HEIGHT as f64 + 40_f64 + 56_f64;
              if solution_open.get() && should_scroll_to_button_first {
                  let options = ScrollIntoViewOptions::new();
                  options.set_behavior(ScrollBehavior::Smooth);
                  document()
                      .get_element_by_id("exo")
                      .unwrap()
                      .scroll_into_view_with_scroll_into_view_options(&options);
              }
              set_transition.set(true);
              set_timeout(move || set_transition.set(false), Duration::from_millis(1100));
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
                          + &format!("&opened={}", !solution_open.get())
                  } else {
                      new_url = window().location().pathname().unwrap() + &search_params
                          + &format!("&opened={}", !solution_open.get())
                  }
                  let _ = navigate(&new_url, options);
              }
              GlobalState::update_solutions_state(
                  solutions_state,
                  solution_number,
                  !solution_open.get(),
              );
          }
        />

      </div>
      <div
        class="solution col-start-2 transition-[height]  relative"
        class=("pointer-events-none", move || !solution_open.get())
        class=("animated-height-full", move || solution_open.get())
        class=("overflow-y-clip", move || !solution_fully_opened.get())

        style=move || {
            format!(
                "height: {}px; transition-duration: {}ms",
                content_height.get(),
                transition_duration(),
            )
        }
      >

        <div
          node_ref=node_ref
          class="transition-all"
          class=("-translate-y-full", move || !solution_open.get())
          style=move || { format!("transition-duration: {}ms", transition_duration()) }

          class=("transition-all", move || transition.get())
        >
          {children()}
        </div>

      </div>
    }
}

#[component]
pub fn SolutionSVG<F>(solution_number: usize, on_click: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let GlobalState {
        solutions_state, ..
    } = use_context::<GlobalState>().unwrap();
    let solution_open = create_memo(move |_| {
        if solutions_state.get().len() > solution_number {
            solutions_state.get()[solution_number]
        } else {
            false
        }
    });
    let button = create_node_ref::<Div>();

    let on_click_arc: Arc<F> = Arc::new(on_click);
    create_effect(move |_| {
        //let on_click_clone = on_click.clone();
        let cloned_on_click = Arc::clone(&on_click_arc);
        let _ = use_event_listener(button, click, move |e| {
            cloned_on_click(e);
        });
    });

    view! {
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
              class=("active_solution_button_rect", move || !solution_open.get())
              class=("inactive_solution_button_rect", move || solution_open.get())

              width="109"
              height="36"
            ></rect>

            <path
              id="solution_button_lip"
              class="solution_button_transition"
              class=("active_solution_button_lip", move || !solution_open.get())
              class=("inactive_solution_button_lip", move || solution_open.get())
              d="M 0 10 v -10 h 109 v 10 M 0 26 v 10 h 109 v -10"
            ></path>

            <g
              id="solution_button_finger_pair"
              class="solution_button_transition"
              class=("active_solution_button_hands", move || !solution_open.get())
              class=("inactive_solution_button_hands", move || solution_open.get())
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
