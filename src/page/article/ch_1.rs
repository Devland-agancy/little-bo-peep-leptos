use crate::componants::MathBlock::*;
use crate::componants::Math::*;
use crate::componants::Paragraph::*;
use crate::componants::Image::*;
use crate::componants::ImageLeft::*;
use crate::componants::ImageRight::*;
use crate::componants::Article::*;
use crate::componants::ArticleTitle::*;
use crate::componants::Span::*;
use crate::componants::Solution::*;
use crate::componants::Grid::*;
use crate::componants::List::*;
use crate::componants::Columns::*;
use crate::componants::Tabs::*;
use elm_to_view::elm_to_view;
use leptos::*;

#[component]
pub fn View(cx: Scope) -> impl IntoView {
    view! {cx,
        <Article>
            <ArticleTitle>"Chapter 1: A Few Refreshers"</ArticleTitle>
            <ArticleBody />
        </Article>
    }
}

#[component]
fn ArticleBody(cx: Scope) -> impl IntoView {
    let html_code = elm_to_view!{cx, 
        r#"
    |> Article 
        author = Matthew Griffith
        title = How I Learned /elm-markup/
        description =
            How I learned to use elm-markup.

    "Lorem ipsum dolor sit amet,"

    |> Image
        src = http://placekitten.com/g/200/300
        description = What a cute kitten.
    "#
    };
     html_code 
}
