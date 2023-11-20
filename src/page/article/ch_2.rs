use crate::componants::Article::*;
use crate::componants::ArticleTitle::*;
use crate::componants::Columns::*;
use crate::componants::Image::*;
use crate::componants::ImageLeft::*;
use crate::componants::ImageRight::*;

use crate::componants::Math::*;
use crate::componants::Paragraph::*;
use crate::componants::Span::*;

use elm_to_view::elm_to_view;
use leptos::*;

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
    elm_to_view! {
      cx,
      r#"
      |> Paragraph 

          *The Definition.*
          The %slope% of a line is a mathematical measure of how
          “steep” a line is.
          Here are a few examples (for an explanation of the values,
          see below):

      |> Image
          src="/images/chapter_2_1.svg"
          height=890

      |> Paragraph    
          
          The slope of a line is...

      |> Paragraph   
          margin_top = 15

          the number of units the line goes up with each unit to the right
          
      |> Paragraph
          margin_top = 15

          ...assuming that numbers on the $y$-axis increase going up and that 
          numbers on the $x$-axis increase going right, as is usually the case.
          One can also describe slope as...
      "#
    }
}
