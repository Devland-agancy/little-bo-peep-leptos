use crate::componants::Article::*;
use crate::componants::ArticleTitle::*;
use crate::componants::Columns::*;
use crate::componants::ImageLeft::*;
use crate::componants::ImageRight::*;
use crate::componants::Math::*;
use crate::componants::MathBlock::*;
use crate::componants::Paragraph::*;
use crate::componants::Span::*;

use leptos::*;
//use elm_to_view::elm_to_view;

#[component]
pub fn View(cx: Scope) -> impl IntoView {
    view! { cx,
      <Article>
        <ArticleTitle>"Chapter 2: Slopes"</ArticleTitle>
        <Columns>
          <ArticleBody/>
        </Columns>
      </Article>
    }
}

#[component]
fn ArticleBody(cx: Scope) -> impl IntoView {
    view! {cx, "s"}
    //elm_to_view! {
    //  cx,
    //  "/home/chaker/code/lp/little-bo-peep-leptos/public/chapters/chapter2.emu"
    //}
}
