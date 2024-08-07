use leptos::*;

use crate::components::ArticleTitle::*;
use crate::components::Columns::*;
use crate::components::Image::*;
use crate::components::Paragraph::*;
use crate::components::Section::*;

use crate::global_state::GlobalState;
use render_chapters::{render_articles_list, render_content_for_article};

#[component]
pub fn View(cx: Scope) -> impl IntoView {
    view! { cx,
      <ArticleTitle label="Chapters"/>
      <ArticleBody/>
    }
}

#[component]
fn ArticleBody(cx: Scope) -> impl IntoView {
    let GlobalState {
        btc_alignment_on_left,
        ..
    } = use_context(cx).unwrap();

    view! { cx,
      <Columns>
        <Paragraph>
          <ul class="leading-9 lg:leading-10 text-2xl lg:text-3xl">
            {render_articles_list!("chapters")}
          </ul>
          {render_content_for_article!("bootcamps" , r#"
            <Spacer />
            <Image src="/images/seperator.svg" width="375px">""</Image>
            <Spacer />
            <h1 class="sm:col-start-2 text-3xl sm:text-4xl mb-5"
                class=("text-right", move || !btc_alignment_on_left())
            >
                "Bootcamps"
            </h1>
          "#)}
          <ul class="leading-9 lg:leading-10 text-2xl lg:text-3xl"
              class=("text-right", move || !btc_alignment_on_left())
          >
            {render_articles_list!("bootcamps")}
          </ul>

        </Paragraph>
      </Columns>
    }
}

#[component]
fn MenuItem(
    cx: Scope,
    href: &'static str,
    label: &'static str,
    #[prop(optional)] on_mobile: &'static str,
) -> impl IntoView {
    let GlobalState { route, .. } = use_context(cx).unwrap();

    view! { cx,
      <a href=["/article/", href].concat() class="block" on:click=move |_| route.set(href)>
        <span class="sm:hidden">{if on_mobile == "" { label } else { on_mobile }}</span>
        <span class="hidden sm:block">{label}</span>
      </a>
    }
}
