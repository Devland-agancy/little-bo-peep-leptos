use leptos::*;

use crate::components::ArticleTitle::*;
use crate::components::Home::*;

#[component]
pub fn View(cx: Scope) -> impl IntoView {
    view! { cx,
      <ArticleTitle label=""/>
      <Body />
    }
}
