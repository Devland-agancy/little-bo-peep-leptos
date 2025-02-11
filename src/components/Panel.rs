use std::rc::Rc;

use crate::components::Checkbox::Checkbox;
use crate::components::Link::CustomLink;

use crate::{constants::HAMBURGER_MENU_HEIGHT, global_state::GlobalState};
use leptos::*;
use leptos_router::{use_location, use_navigate, NavigateOptions, State};
use render_chapters::{render_articles_list, render_based_on_env, render_content_for_article};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum MenuState {
    Open,
    OpenPressed,
    Closed,
    ClosedPressed,
}

#[component]
pub fn Panel() -> impl IntoView {
    let menu_state = use_context::<ReadSignal<MenuState>>().unwrap();
    let menu_closed = move || {
        menu_state.get() == MenuState::Closed || menu_state.get() == MenuState::ClosedPressed
    };
    let GlobalState {
        show_areas,
        show_section_divider,
        btc_alignment_on_left,
        show_squiggles,
        ..
    } = use_context::<GlobalState>().unwrap();

    let toggle_scroll = move |overflow: &str| {
        let body = document().body().unwrap();
        if menu_closed() {
            body.style().set_property("overflow", overflow).unwrap();
        } else {
            body.style().set_property("overflow", overflow).unwrap();
        }
    };

    view! {
      <div
        id="sidebar"
        class="w-full z-50 fixed translate-x-0 translate-y-0 right-0 top-14 flex self-start font-baskerville text-xl leading-3 sm:leading-5 select-none transition ease-linear  duration-300"
        class=("duration-500", menu_closed) // disappear
        class=("duration-200", move || !menu_closed()) //appear

        style=move || format!("transform: translateX({})", if menu_closed() { "100%" } else { "0" })
      >

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

          class="select-none overscroll-none absolute right-0 w-[16rem] sm:w-[22rem] z-40 bg-stone-100 overflow-scroll translate-y-0 sm:translate-y-[-1px]"
        >
          <div class="select-none scrollbar-hidden sm:h-full pt-[0.6em] px-[1em] overflow-y-hidden [&>ul]:mb-[8px] [&>ul]:p-0">

            {render_content_for_article!(
                "chapters", r#"
              <Title label="Chapters"/>
          "#
            )} <ul>{render_articles_list!("chapters")}</ul>
            {render_content_for_article!(
                "bootcamps", r#"
              <Title label="Bootcamps"/>
            "#
            )}
            <ul class=(
                "text-right",
                move || !btc_alignment_on_left.get(),
            )>{render_articles_list!("bootcamps")}</ul> <Title label="Options"/>
            <Option signal=show_areas label="Areas"/>
            {render_based_on_env!(
                r##"
                /* show_section_divider */
                  <Option 
                    signal=show_section_divider
                    label="Section Dividers"
                  />
                "##,
                ""
            )}
            {render_based_on_env!(
                r##"
                /* show_squiggles */
                  <Option 
                    signal=show_squiggles
                    label="Squiggles"
                  />
                "##,
                ""
            )}

          </div>
        </div>
      </div>
    }
}

#[component]
pub fn Title(label: &'static str) -> impl IntoView {
    let GlobalState { on_mobile, .. } = use_context().unwrap();

    view! {
      <h1 class="text-3xl font-baskerville-italic mt-[5px] mb-[7px] flex justify-between items-center">
        <img src=move|| format!("/images/title_line{}.svg", if on_mobile.get() {"_panel"} else {""} ) class="w-[3rem] sm:w-24"/>
        {label}
        <img src=move|| format!("/images/title_line{}.svg", if on_mobile.get() {"_panel"} else {""} ) class="rotate-180 w-[3rem] sm:w-24"/>
      </h1>
    }
}

#[component]
pub fn MenuItem(
    href: &'static str,
    article_type: &'static str,
    label: &'static str,
    #[prop(optional)] on_mobile: &'static str,
) -> impl IntoView {
    view! {
    <CustomLink
        base_href="/article/"
        href=href
        class="flex items-baseline justify-between leading-9 sm:leading-8 text-2xl"
      >
        <span class="block">{article_type}</span>
        <span class="dots"></span>

        <span class="sm:hidden">{if on_mobile == "" { label } else { on_mobile }}</span>
        <span class="hidden sm:block">{label}</span>
      </CustomLink>
    }
}

#[component]
pub fn Option(signal: RwSignal<bool>, label: &'static str) -> impl IntoView {
    view! {
      <div class="flex justify-between items-center text-2xl pb-1.5 sm:pb-2">
        <p>{label}</p>
        <Checkbox value=signal/>
      </div>
    }
}
