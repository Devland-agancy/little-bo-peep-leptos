use crate::components::Article::*;
use crate::components::ArticleTitle::*;
use crate::components::Columns::*;
use crate::components::ImageLeft::*;
use crate::components::ImageRight::*;
use crate::components::Math::*;
use crate::components::MathBlock::*;
use crate::components::Paragraph::*;
use crate::components::Span::*;
use crate::components::Table::*;
use elm_to_view::elm;

use leptos::*;

#[component]
pub fn View(cx: Scope) -> impl IntoView {
    view! { cx,
        <ArticleTitle label="Chapter 2: Slopes"/>
        <ArticleBody/>
    }
}

#[component]
fn ArticleBody(cx: Scope) -> impl IntoView {
    elm! {cx,
      "file:/content/ch_3.emu"
    }
}
