use leptos::*;

use crate::components::ArticleTitle::*;
use crate::components::Home::*;

#[component]
pub fn View() -> impl IntoView {
    view! { 
      <ArticleTitle label=""/>
      <Body />
    }
}
