use crate::components::Columns::*;
use crate::components::Paragraph::*;
use crate::components::Section::*;
use leptos::*;

use crate::global_state::GlobalState;
use render_chapters::{render_articles_list, render_content_for_article};

#[component]
pub fn Body(cx: Scope) -> impl IntoView {
    let GlobalState {
        btc_alignment_on_left,
        ..
    } = use_context(cx).unwrap();

    view! { cx,
      <Columns>
        <Paragraph>

          <h1 class="leading-[1] text-[2.5rem] font-merriweather mb-[2.6rem] gap-4 flex justify-center items-center font-lora">
            //<img src="/images/table_of_contents.svg" class="w-[40px] sm:w-[60px]"/>
            <img src="/images/table_of_content.svg" class="max-w-[83%]"/>
            //<img src="/images/table_of_contents.svg" class="flip-y w-[40px] sm:w-[60px]"/>
          </h1>

          {render_content_for_article!(
              "chapters", r#"
            <Title label="Chapters"/>
        "#
          )}
          <ul class="leading-9 lg:leading-10 text-2xl lg:text-3xl">
            {render_articles_list!("chapters")}
          </ul> <Spacer/>
          {render_content_for_article!(
              "bootcamps", r#"
            <Title label="Bootcamps"/>
          "#
          )}
          <ul
            class="leading-9 lg:leading-10 text-2xl lg:text-3xl"
            class=("text-right", move || !btc_alignment_on_left())
          >
            {render_articles_list!("bootcamps")}
          </ul>
        </Paragraph>
      </Columns>
    }
}

#[component]
pub fn Title(cx: Scope, label: &'static str) -> impl IntoView {
    view! { cx,
      <h1 class="text-[2.1rem] font-baskerville-italic mb-5 flex justify-between items-center">
        <img src="/images/title_line.svg" class="w-[30%] sm:w-36"/>
        {label}
        <img src="/images/title_line.svg" class="rotate-180 w-[30%] sm:w-36"/>
      </h1>
    }
}

#[component]
pub fn MenuItem(
    cx: Scope,
    href: &'static str,
    article_type: &'static str,
    label: &'static str,
    #[prop(optional)] on_mobile: &'static str,
) -> impl IntoView {
    let GlobalState { route, .. } = use_context(cx).unwrap();

    view! { cx,
      <a
        href=["/article/", href].concat()
        class="flex items-baseline justify-between"
        on:click=move |_| route.set(href)
      >
        <span class="block">{article_type}</span>
        <span class="dots"></span>
        <span class="sm:hidden">{if on_mobile == "" { label } else { on_mobile }}</span>
        <span class="hidden sm:block">{label}</span>
      </a>
    }
}
