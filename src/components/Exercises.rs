use crate::components::Image::Image;
use crate::{
    constants::GREEN_DIV_HEIGHT, global_state::GlobalState, utils::get_chapter::get_chapter,
};
use leptos::*;
use leptos_router::{use_location, use_navigate, NavigateOptions, State};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use web_sys::{ScrollBehavior, ScrollIntoViewOptions};

#[derive(Serialize, Deserialize)]
struct Exercice {
    ex_number: String,
    ex_chapter: String,
    ex_opened: String,
}

const NAVIGATE_OPTIONS: NavigateOptions = NavigateOptions {
    resolve: true,
    replace: false,
    scroll: false,
    state: State(None),
};

#[component]
fn LabelsView(
    vec: Vec<&'static str>,
    selected_tab: ReadSignal<usize>,
    set_selected_tab: WriteSignal<usize>,
) -> impl IntoView {
    let (_vec, _) = create_signal(vec);
    let navigate = use_navigate();
    let navigate_ = use_navigate();
    let location = use_location();
    let (chapter, _) = create_signal(get_chapter(location));

    view! {
      <svg
        width="43"
        height="43"
        viewBox="0 0 43 43"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        class="tab cursor-pointer overflow-visible"
        class=("disabled", move || selected_tab.get() == 0)
        on:click=move |_| {
            if selected_tab.get() != 0 {
                let new_tab = selected_tab.get() - 1;
                set_selected_tab.set(new_tab);
                let mut stored_opened_value = false;
                match window().local_storage() {
                    Ok(Some(storage)) => {
                        let stored_solution_opened_key = format!(
                            "{}_exo_{}_opened",
                            chapter.get(),
                            new_tab,
                        );
                        stored_opened_value = storage
                            .get_item(&stored_solution_opened_key)
                            .unwrap_or(Some("false".to_string()))
                            .unwrap_or("false".to_string()) == "true";
                    }
                    _ => {}
                }
                let _ = navigate(
                    &format!(
                        "{}?tab={}&opened={}",
                        window().location().pathname().unwrap(),
                        new_tab,
                        stored_opened_value,
                    ),
                    NAVIGATE_OPTIONS,
                );
            }
        }
      >

        <path
          class="overflow-visible"
          d="M35.4941 1H6.65545C3.53203 1 1 3.53203 1 6.65545V35.4941C1 38.6175 3.53203 41.1495 6.65545 41.1495H35.4941C38.6175 41.1495 41.1495 38.6175 41.1495 35.4941V6.65545C41.1495 3.53203 38.6175 1 35.4941 1Z"
          fill=move || format!("{}", if selected_tab.get() != 0 { "#EEFFAA" } else { "#EBEBEB" })
          fill-opacity=move || format!("{}", if selected_tab.get() != 0 { "0.4" } else { "1" })
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
        class=("disabled", move || selected_tab.get() == _vec.get().len() - 1)
        on:click=move |_| {
            if selected_tab.get() != _vec.get().len() - 1 {
                let new_tab = selected_tab.get() + 1;
                set_selected_tab.set(new_tab);
                let mut stored_opened_value = false;
                match window().local_storage() {
                    Ok(Some(storage)) => {
                        let stored_solution_opened_key = format!(
                            "{}_exo_{}_opened",
                            chapter.get(),
                            new_tab,
                        );
                        stored_opened_value = storage
                            .get_item(&stored_solution_opened_key)
                            .unwrap_or(Some("false".to_string()))
                            .unwrap_or("false".to_string()) == "true";
                    }
                    _ => {}
                }
                let _ = navigate_(
                    &format!(
                        "{}?tab={}&opened={}",
                        window().location().pathname().unwrap(),
                        new_tab,
                        stored_opened_value,
                    ),
                    NAVIGATE_OPTIONS,
                );
            }
        }
      >

        <path
          class="overflow-visible"
          d="M35.4941 1H6.65545C3.53203 1 1 3.53203 1 6.65545V35.4941C1 38.6175 3.53203 41.1495 6.65545 41.1495H35.4941C38.6175 41.1495 41.1495 38.6175 41.1495 35.4941V6.65545C41.1495 3.53203 38.6175 1 35.4941 1Z"
          fill=move || {
              format!("{}", if selected_tab.get() != _vec.get().len() - 1 { "#EEFFAA" } else { "#EBEBEB" })
          }

          fill-opacity=move || {
              format!("{}", if selected_tab.get() != _vec.get().len() - 1 { "0.4" } else { "1" })
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
fn EndLabelsView(vec: Vec<&'static str>, selected_tab: usize) -> impl IntoView {
    let (_vec, _) = create_signal(vec);

    view! {
      <svg
        width="43"
        height="43"
        viewBox="0 0 43 43"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        class="tab cursor-pointer overflow-visible z-10"
        on:click=move |_| {
            let options = ScrollIntoViewOptions::new();
            options.set_behavior(ScrollBehavior::Smooth);
            document()
                .get_element_by_id("exo")
                .unwrap()
                .scroll_into_view_with_scroll_into_view_options(&options);
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
          fallback=move || {
              view! {
                <path
                  d="M20 32C20 32.5523 20.4477 33 21 33C21.5523 33 22 32.5523 22 32H20ZM21 11L15.2265 21H26.7735L21 11ZM22 32L22 20H20L20 32H22Z"
                  fill="black"
                ></path>
              }
          }

          when=move || selected_tab == _vec.get().len() - 1
        >
          <path
            d="M20 32C20 32.5523 20.4477 33 21 33C21.5523 33 22 32.5523 22 32H20ZM21 11L15.2265 21H26.7735L21 11ZM22 32L22 20H20L20 32H22Z"
            fill="black"
          ></path>
        </Show>

      </svg>
    }
}
#[derive(Clone)]
pub struct SelectedTab {
    pub tab: ReadSignal<usize>,
}

#[component]
pub fn Exercises(labels: Vec<&'static str>, children: ChildrenFn) -> impl IntoView {
    let (selected_tab, set_selected_tab) = create_signal(0);
    let (_labels, _) = create_signal(labels.clone());
    let GlobalState {
        labels: global_labels,
        tab,
        solutions_state,
        solution_transition_duration,
        ..
    } = use_context::<GlobalState>().unwrap();

    let solution_open = create_memo(move |_| {
        if solutions_state.get().len() > selected_tab.get() {
            solutions_state.get()[selected_tab.get()]
        } else {
            false
        }
    });

    global_labels.set(labels.clone());
    tab.set(selected_tab.get_untracked());

    let location = use_location();
    let url_params = location.clone().query;
    let (chapter, _) = create_signal(get_chapter(location));

    create_effect(move |_| {
        GlobalState::init_solutions_state(solutions_state, global_labels.get().len(), false);
        GlobalState::init_solutions_state(
            solution_transition_duration,
            global_labels.get().len(),
            1000,
        );
    });

    create_effect(move |_| {
        let mut stored_selected_tab = None;
        match window().local_storage() {
            Ok(Some(storage)) => {
                let stored_selected_tab_key = format!("{}_exercice", chapter.get());

                stored_selected_tab = Some(storage.get_item(&stored_selected_tab_key));
            }
            _ => {}
        }
        if let Some(sst) = stored_selected_tab {
            match sst {
                Ok(Some(value)) => {
                    set_selected_tab.set(value.parse::<usize>().unwrap());
                }
                _ => {}
            }
        }
    });

    create_effect(move |_| {
        selected_tab.get();
        tab.set(selected_tab.get());
        set_timeout(
            move || {
                let mut stored_solution_opened = None;
                match window().local_storage() {
                    Ok(Some(storage)) => {
                        let stored_solution_opened_key =
                            format!("{}_exo_{}_opened", chapter.get(), selected_tab.get());
                        stored_solution_opened = Some(storage.get_item(&stored_solution_opened_key))
                    }
                    _ => {}
                }

                if let Some(sso) = stored_solution_opened {
                    match sso {
                        Ok(Some(value)) => {
                            GlobalState::update_solutions_state(
                                solutions_state,
                                selected_tab.get(),
                                value == "true",
                            );
                        }
                        _ => {
                            GlobalState::update_solutions_state(
                                solutions_state,
                                selected_tab.get(),
                                false,
                            );
                        }
                    }
                } else {
                    GlobalState::update_solutions_state(solutions_state, selected_tab.get(), false);
                }
            },
            Duration::from_millis(50),
        );
    });

    create_effect(move |_| {
        solution_open.get();
        selected_tab.get();
        set_timeout(
            move || match window().local_storage() {
                Ok(Some(storage)) => {
                    let exo = Exercice {
                        ex_number: selected_tab.get().to_string(),
                        ex_chapter: chapter.get(),
                        ex_opened: solution_open.get().to_string(),
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

    create_effect(move |_| {
        // this shoould run only of first website load
        set_timeout(
            move || {
                let _url_params = url_params.get();
                let stored_selected_tab = _url_params.get("tab");

                if let Some(sst) = stored_selected_tab {
                    if let Ok(tab) = sst.parse::<usize>() {
                        set_selected_tab.set(tab);
                    }
                } else {
                    // here we handle case where values are stored in localstorage and search params are not defined , so we define them and change the url .

                    let mut _solution_opened = None;
                    let mut _selected_tab = None;

                    match window().local_storage() {
                        Ok(Some(storage)) => {
                            let _solution_opened_key =
                                format!("{}_exo_{}_opened", chapter.get(), selected_tab.get());
                            _solution_opened = Some(storage.get_item(&_solution_opened_key));

                            let _selected_tab_key = format!("{}_exercice", chapter.get());
                            _selected_tab = Some(storage.get_item(&_selected_tab_key));
                        }
                        _ => {}
                    }

                    let navigate = use_navigate();

                    let _ = navigate(
                        &format!(
                            "{}?tab={}&opened={}",
                            window().location().pathname().unwrap(),
                            _selected_tab
                                .unwrap_or(Ok(None))
                                .unwrap_or(None)
                                .unwrap_or("0".to_string()),
                            _solution_opened
                                .unwrap_or(Ok(None))
                                .unwrap_or(None)
                                .unwrap_or("false".to_string()),
                        ),
                        NAVIGATE_OPTIONS,
                    );
                }
            },
            Duration::from_millis(50),
        );
    });

    create_effect(move |_| {
        // this shoould run only of first website load
        set_timeout(
            move || {
                let _url_params = url_params.get();
                let stored_solution_opened = _url_params.get("opened");
                if let Some(sso) = stored_solution_opened {
                    GlobalState::update_solutions_state(
                        solutions_state,
                        selected_tab.get(),
                        sso == "true",
                    );
                }
            },
            Duration::from_millis(50),
        );
    });

    view! {
      <Image
        id="exo"
        src="/images/seperator.png"
        height="50px"
        container_classes="flex items-center mt-[15px] mb-[40px]"
      >
        ""
      </Image>
      <div class="text-xl flex items-center justify-center gap-2 col-start-2 hidden-on-startup mt-[2px]">
        <LabelsView vec=labels.clone() selected_tab=selected_tab set_selected_tab=set_selected_tab/>
      </div>
      <div class="col-start-2 h-[31px]"></div>
      <For
        each=move || {
            children()
                .nodes
                .into_iter()
                .filter(move |node| {
                    if let Some(text) = node.as_text() {
                        return text.content != r#""#.into_view().as_text().unwrap().content;
                    }
                    return true;
                })
                .enumerate()
        }

        key=|label| label.0
        children=move |label| {
            view! {
              <div
                class="slice text-xl grid grid-cols-[1500px_100%_1500px] sm:grid gridColsWidth relative -translate-x-[1500px] sm:translate-x-0 transition-opacity duration-500 "
                class=("opacity-0", move || selected_tab.get() != label.0)
                class=("h-0", move || selected_tab.get() != label.0)
                class=("transition-none", move || selected_tab.get() != label.0)
                class=("overflow-hidden", move || selected_tab.get() != label.0)
              >
                {label.1}
              </div>
            }
        }
      />
    }
}

#[component]
pub fn Exercise(children: ChildrenFn) -> impl IntoView {
    let GlobalState {
        labels,
        tab,
        solutions_state,
        show_areas,
        solution_transition_duration,
        ..
    } = use_context::<GlobalState>().unwrap();

    let solution_open = create_memo(move |_| {
        if solutions_state.get().len() > tab.get() {
            solutions_state.get()[tab.get()]
        } else {
            false
        }
    });

    let (solution_fully_opened, set_solution_fully_opened) =
        create_signal(solution_open.get_untracked());

    let transition_duration = create_memo(move |_| {
        if solution_transition_duration.get().len() > tab.get() {
            solution_transition_duration.get()[tab.get()]
        } else {
            1000
        }
    });

    create_effect(move |_| {
        if solution_open.get() {
            set_timeout(
                move || {
                    let _ = set_solution_fully_opened.try_set(true);
                },
                Duration::from_millis(transition_duration.get() as u64),
            )
        } else {
            set_solution_fully_opened.set(false);
            set_timeout(
                // sometimes the above line executes before 1 second of the above block is passed so we make sure is stays false
                move || {
                    let _ = set_solution_fully_opened.try_set(false);
                },
                Duration::from_millis(transition_duration.get() as u64),
            )
        }
    });

    let (bot_div, set_bot_div) = create_signal(true);
    create_effect(move |_| {
        if solution_open.get() {
            set_timeout(
                move || {
                    let _ = set_bot_div.try_set(false);
                },
                Duration::from_millis(transition_duration.get() as u64),
            )
        } else {
            set_timeout(
                move || {
                    let _ = set_bot_div.try_set(true);
                },
                Duration::from_millis(transition_duration.get() as u64),
            )
        }
    });

    view! {
      {children()}
      <div class="col-start-2 h-[31px]"></div>
      <div
        class="text-xl flex items-center justify-center gap-2 col-start-2 transition-opacity"
        style=move || format!("transition-duration: {}ms", if solution_open.get() { 1000 } else { 100 })

        class=("opacity-0", move || !(solution_open.get() && solution_fully_opened.get()))
        class=("delay-[2s]", move || bot_div.get())
      >
        <EndLabelsView vec=labels.get_untracked() selected_tab=tab.get_untracked()/>
      </div>

      <div
        class="transition-all col-start-2"
        style=move || {
            format!(
                "height: {}px; background-color: {}; transition-duration: {}ms",
                if !solution_open.get() || bot_div.get() { GREEN_DIV_HEIGHT } else { 0 },
                if show_areas.get() { "#00440050" } else { "" },
                if solution_open.get() { 1000 } else { 0 },
            )
        }
      >
      </div>
    }
}

#[component]
pub fn ExerciseQuestion(children: ChildrenFn) -> impl IntoView {
    view! {
      <div class="animate-appear-slow col-start-2 block relative flex flex-col">{children()}</div>
    }
}
