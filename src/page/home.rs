use leptos::*;

use crate::components::ArticleTitle::*;
use crate::components::Columns::*;
use crate::components::Paragraph::*;
use crate::constants::MENU_ITEMS;
use crate::global_state::GlobalState;
use render_chapters::render_chapters_list;

#[component]
pub fn View(cx: Scope) -> impl IntoView {
    view! { cx,
      <ArticleTitle label="Chapters"/>
      <ArticleBody/>
    }
}

#[component]
fn ArticleBody(cx: Scope) -> impl IntoView {
    view! { cx,
      <Columns>
        <Paragraph>
          <ul class="leading-9 lg:leading-10 text-2xl lg:text-3xl">
            {render_chapters_list!(.)}
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
