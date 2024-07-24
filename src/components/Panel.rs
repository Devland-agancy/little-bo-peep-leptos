use crate::components::Checkbox::Checkbox;
use crate::{constants::HAMBURGER_MENU_HEIGHT, global_state::GlobalState};
use leptos::*;
use render_chapters::{render_articles_list, render_based_on_env, render_content_for_article};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum MenuState {
    Open,
    OpenPressed,
    Closed,
    ClosedPressed,
}

#[component]
pub fn Panel(cx: Scope) -> impl IntoView {
    let menu_state = use_context::<ReadSignal<MenuState>>(cx).unwrap();
    let menu_closed =
        move || menu_state() == MenuState::Closed || menu_state() == MenuState::ClosedPressed;
    let GlobalState {
        show_areas,
        show_section_divider,
        btc_alignment_on_left,
        ..
    } = use_context::<GlobalState>(cx).unwrap();
    let btc_panel_alignment_on_left = create_rw_signal(cx, true);

    let toggle_scroll = move |overflow: &str| {
        let body = document().body().unwrap();
        if menu_closed() {
            body.style().set_property("overflow", overflow).unwrap();
        } else {
            body.style().set_property("overflow", overflow).unwrap();
        }
    };

    let (test_contains, set_tc) = create_signal(cx, "");

    view! { cx,
      <div
        id="sidebar"
        class="w-full z-50 fixed translate-x-0 translate-y-0 right-0 top-14 flex self-start font-baskerville text-xl leading-3 sm:leading-5 select-none transition ease-linear  duration-300"
        style=move || format!("transform: translateX({})", if menu_closed() { "100%" } else { "0" })
      >

        <h1 class="text-6xl">{move || test_contains()}</h1>

        <div
          on:mouseenter=move |_| toggle_scroll("hidden")
          on:mouseleave=move |_| toggle_scroll("auto")
          on:touchstart=move |_| toggle_scroll("hidden")
          on:touchend=move |_| toggle_scroll("auto")

          style=format!(
              "min-height: calc(100vh - {}px); height: calc(100vh - {}px)",
              HAMBURGER_MENU_HEIGHT - 1.0,
              HAMBURGER_MENU_HEIGHT,
          )

          class="select-none overscroll-none absolute right-0 w-2/3 max-w-xs z-40 bg-stone-100 overflow-scroll translate-y-0 sm:translate-y-[-1px]"
        >
            <div class="select-none scrollbar-hidden sm:h-full px-4 py-3 overflow-y-hidden">
              <h2 class="font-baskerville-italic text-2xl pb-1.5 sm:pb-2">"Chapters"</h2>
              <ul>{render_articles_list!("chapters")}</ul>
              {render_content_for_article!(
                  "bootcamps",
                  r#"
            <h2 class="font-baskerville-italic text-2xl mt-4 pb-1.5 sm:pb-2"
            class=("text-right", move || !btc_panel_alignment_on_left())
            >"Bootcamps"</h2>
            "#
              )}

              <ul class=(
                  "text-right",
                  move || !btc_panel_alignment_on_left(),
              )>{render_articles_list!("bootcamps")}</ul>
              <h2
                class="font-baskerville-italic text-2xl mt-4 pb-1.5 sm:pb-2"
                class=("text-right", move || btc_panel_alignment_on_left())
              >
                "Options"
              </h2>

              <div
                class="flex justify-end items-center gap-2 text-lg sm:text-xl pb-1.5 sm:pb-2"
                class=("flex-row-reverse", move || !btc_panel_alignment_on_left())
              >
                <p>"Show Areas"</p>
                <Checkbox value=show_areas/>
              </div>

              {render_based_on_env!(
                  r##"
                /* show_section_divider */
                <div class="flex justify-end items-center gap-2 text-lg sm:text-xl pb-1.5 sm:pb-2"
                      class=("flex-row-reverse", move || !btc_panel_alignment_on_left())
                >
                  <p>"Show Section Dividers"</p>
                  <Checkbox value=show_section_divider/>
                </div>
                /* Alignment */
                {render_content_for_article!("bootcamps", r#"
                    <div class="flex justify-end items-center gap-2 text-lg sm:text-xl pb-1.5 sm:pb-2"
                          class=("flex-row-reverse", move || !btc_panel_alignment_on_left())
                    >
                      <p>"C.P. Bootcamps Left"</p>
                      <Checkbox value=btc_panel_alignment_on_left />
                    </div>
                    <div class="flex justify-end items-center gap-2 text-lg sm:text-xl pb-1.5 sm:pb-2"
                          class=("flex-row-reverse", move || !btc_panel_alignment_on_left())
                    >
                      <p>"T.o.C. Bootcamps Left"</p>
                      <Checkbox value=btc_alignment_on_left />
                    </div>"#
                  )}
              "##,
                  ""
              )}

            </div>
        </div>
      </div>
    }
}

#[component]
fn MenuItem(
    cx: Scope,
    href: &'static str,
    label: &'static str,
    article_type: &'static str,

    #[prop(optional)] on_mobile: &'static str,
) -> impl IntoView {
    let GlobalState { route, .. } = use_context(cx).unwrap();

    view! { cx,
      <li class="pb-1.5 sm:pb-2">
        <a
          on:click=move |_| {
              route.set(href);
          }

          href=["/article/", href].concat()
          class="text-stone-900 hover:text-sky-800 text-lg sm:text-xl"
        >
          <span class="sm:hidden">{if on_mobile == "" { label } else { on_mobile }}</span>
          <span class="hidden sm:block">{label}</span>
        </a>
      </li>
    }
}
