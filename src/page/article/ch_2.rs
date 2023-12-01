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
        <ArticleTitle>"Chapter 2: The Slope Formula"</ArticleTitle>
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

            *Slopes.*
            the %slope% of a line is a mathematical measure of how
            “steep” a line is.
            Here are a few examples (for an explanation of the values,
            see below):

        |> Image
            src="/images/chapter_2_1.svg"
            id="test-bg"

        |> Paragraph    
            
            To explain now, The slope of a line is...

        |> Paragraph   
            margin_top = 15
            align = Align::Center

            %the number of units the line goes up with each unit to the right%
            
        |> Paragraph
            margin_top = 15

            ...assuming that numbers on the $y$-axis increase going up and that 
            numbers on the $x$-axis increase going right, as is usually the case.
            One can also describe slope as...

        |> Paragraph   
            margin_top = 15
            align = Align::Center
            
            %the amount of vertical change per unit of horizontal change%

        |> Paragraph   
            margin_top = 15
            
            ...more elegantly.

        |> Paragraph   
            indent = Indent::Line

            Thus, for example, the line below has slope 1 because it goes up by $1$ unit for each unit to the right:

        |> Image
            src="/images/slope_1_example.svg"

        |> Paragraph   

            On the other hand, the line below has slope $-0.5$, because it goes up 
            by %minus% $0.5$ units with each unit to the right:
    
        |> Image
            src="/images/slope_0.5_example.svg"

        |> Paragraph   

            (Etc.)

        |> Paragraph   
            margin_top = 15

            *Measuring Slope.*
            The slope of a line is also the ratio of vertical change
            to horizontal change between any two distinct points $A$, $B$
            on the line:
            
      "#
    }
}
