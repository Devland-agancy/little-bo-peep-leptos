use super::*;
use leptos::*;

use crate::componants::Article::*;
use crate::componants::ArticleTitle::*;
use crate::componants::Columns::*;
use crate::componants::Link::*;
use crate::componants::Paragraph::*;
use crate::constants::MENU_ITEMS;

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
            {MENU_ITEMS
              .into_iter()
              .map(|(title, on_mobile, url)| view! { cx, <MenuItem label=title on_mobile=on_mobile href=url /> })
              .collect_view(cx)}
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
    let set_route = use_context::<WriteSignal<&str>>(cx).unwrap();

    view! { cx,
      <a href={["/article/", href].concat()} class="block" on:click=move |_| set_route(href)>
        <span class="sm:hidden">{if on_mobile == "" {label} else {on_mobile}}</span>
        <span class="hidden sm:block">{label}</span>
      </a>
    }
}
