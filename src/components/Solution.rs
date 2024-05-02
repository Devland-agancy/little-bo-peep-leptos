use crate::{
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
use std::{sync::Arc, time::Duration};
use web_sys::{MouseEvent, ScrollBehavior, ScrollIntoViewOptions};

#[component]
pub fn Solution(cx: Scope, solution_number: usize, children: Children) -> impl IntoView {
    let GlobalState {
        show_areas,
        solutions_state,
        ..
    } = use_context::<GlobalState>(cx).unwrap();
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
                        move || set_content_height(node_ref().unwrap().offset_height()),
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
                        set_content_height(node_ref().unwrap().offset_height())
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
            if node_ref().is_some() {
                if solution_open() {
                    set_content_height(node_ref().unwrap().offset_height())
                } else {
                    set_content_height(0)
                }
            }
        });
    });

    let (bot_div, set_bot_div) = create_signal(cx, true);
    create_effect(cx, move |_| {
        if solution_open() {
            set_timeout(move || set_bot_div(false), Duration::from_secs(1))
        } else {
            set_timeout(move || set_bot_div(true), Duration::from_secs(1))
        }
    });

    let (transition, set_transition) = create_signal(cx, false);

    let navigate = use_navigate(cx);

    view! { cx,
      <div node_ref=button class="my-5 relative col-start-2"
      style=format!("padding-left: {}; padding-right: {}", TEXT_LEFT_PADDING, TEXT_RIGHT_PADDING)
      >
        <SolutionSVG solution_number=solution_number on_click=move |_| {
            // Get the element's bottom position relative to the document
            let element_pos = window().inner_height().unwrap().as_f64().unwrap() -  button().unwrap().get_bounding_client_rect().bottom();

            let should_scroll_to_button_first = element_pos > GREEN_DIV_HEIGHT as f64 + 40_f64+ 56_f64 ; // empty div beneath + solution button margin bot + padding bottom of page

            if solution_open() && should_scroll_to_button_first {
              let mut options = ScrollIntoViewOptions::new();
              options.behavior(ScrollBehavior::Smooth);
              document().get_element_by_id("exo").unwrap().scroll_into_view_with_scroll_into_view_options(&options);
            }
            set_transition(true);
            set_timeout(move || set_transition(false), Duration::from_millis(1100));
            let options = NavigateOptions {
              resolve: true,
              replace: false,
              scroll: false,
              state: State(None)
            };
            if let Ok(search_params) = window().location().search() {
              let new_url: String;
              if search_params.contains("&opened=true") {
                new_url = window().location().pathname().unwrap() + &search_params.replace("&opened=true", "&opened=false")
              }else if search_params.contains("&opened=false"){
                new_url = window().location().pathname().unwrap() + &search_params.replace("&opened=false", "&opened=true")
              }else if &search_params == ""{
                new_url = window().location().pathname().unwrap() + "?tab=0" + &format!("&opened={}", !solution_open())
              }else{
                new_url = window().location().pathname().unwrap() + &search_params + &format!("&opened={}", !solution_open())

              }
              let _ = navigate(&new_url, options);
            }
            GlobalState::update_solutions_state(solutions_state, solution_number, !solution_open());
        }/>
      </div>
      <div
        class="col-start-2 transition-[height] duration-1000 overflow-y-clip relative"
        class=("pointer-events-none", move || !solution_open())
        class=("animated-height-full", move || solution_open())
        style=move || {
            format!("height: {}px", content_height() + if solution_open() { 40 } else { 0 })
        }
      >

        <div
          node_ref=node_ref
          class="flex flex-col container"
          class=("-translate-y-full", move || !solution_open())
          class=("duration-1000", move || transition())
          class=("transition-all", move || transition())
        >
          {children(cx)}
        </div>

      </div>
       <Show fallback=|_| () when=move || !solution_open() || bot_div()>
        <div style=format!("height: {}px; background-color: {}", GREEN_DIV_HEIGHT, if show_areas() { "#00440050" } else { "" })>
        </div>
      </Show>
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
      <div
        id="solution-button"
        node_ref = button
        class="column solution_button_div cursor-pointer mb-12"
      >
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
                <use_ href="#finger_pointing_left"/>
              </g>
              <g transform="scale(-1, 1) translate(-8, 20)">
                <use_ href="#finger_pointing_left"/>
              </g>
              <use_ href="#solution_button_text"/>
            </g>
          </g>
        </svg>
      </div>
    }
}
