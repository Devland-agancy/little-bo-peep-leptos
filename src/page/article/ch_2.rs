use crate::componants::Article::*;
use crate::componants::ArticleTitle::*;
use crate::componants::Columns::*;
use crate::componants::Image::*;
use crate::componants::ImageLeft::*;
use crate::componants::ImageRight::*;

use crate::componants::Math::*;
use crate::componants::MathBlock::*;

use crate::componants::Paragraph::*;
use crate::componants::Span::*;

use elm_to_view::elm_to_view;
use leptos::*;

#[component]
pub fn View(cx: Scope) -> impl IntoView {
    view! { cx,
      <Article>
        <ArticleTitle on_mobile="Chapter 2: Slopes" label="Chapter 2: The Slope Formula" />
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
            
            To explain now, the slope of a line is...

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
            containerClasses="pl-[2px]"

        |> Paragraph   

            On the other hand, the line below has slope $-0.5$, because it goes up 
            by %minus% $0.5$ units with each unit to the right:
    
        |> Image
            src="/images/slope_0.5_example.svg"
            containerClasses="pl-[19.5px]"

        |> Paragraph   

            (Etc.)

        |> Paragraph   
            margin_top = 15

            *Measuring Slope.*
            The slope of a line is also the ratio of vertical change
            to horizontal change between any two distinct points $A$, $B$
            on the line:

        |> Image
            src="/images/31.svg"

        |> Paragraph   

            $$ \te{slope} = {\te{vertical change from \$A\$ to \$B\$} \over \te{horizontal change from \$A\$ to \$B\$}} $$
            Indeed, dividing the vertical change by the horizontal change
            gives the per-horizontal-unit vertical change.

        |> Paragraph   

            More precisely, if
            $$ A = (x_1, y_1) $$
            and
            $$ B = (x_2, y_2) $$
            then
            $$ x_2 - x_1 $$
            and
            $$ y_2 - y_1 $$
            are the horizontal %&% the vertical change, respectively,
            from $A$ to $B$, so

            |> MathBlock

                $$\te{slope} = {y_2 - y_1 \over x_2 - x_1}$$

                |> ImageRight
                    src="/images/32.svg"
                    width=600
                    hidden_in_mobile=true
                    squiggle_top="44%"
                    squiggle_left="-0.5rem"

        |> Paragraph   

            more succinctly. We call this the %slope formula%.

        |> Paragraph   
            margin_top = 15

            *Example 1.*
            A line that passes through the points $$A = (-2, 5)$$ and $$B = (4, 1)$$
            has slope
            $$ \frac{1 - 5}{4 - (-2)} = \frac{-4}{6} = - \frac{2}{3}. $$

        |> Paragraph   
            align = Align::Center

            \* \* \* \*

        |> Paragraph   
            margin_top = 15

            (The main thing to understand about Example 1 is that
            $$ 1 - 5 $$
            is the vertical change from $A$ to $B$, while
            $$ 4 - (-2) $$
            is the horizontal change from $A$ to $B$.)

        |> Paragraph   
            margin_top = 15

            *Sign Combinations.*
            Technically, quantities such as
            $$ x_2 - x_1$$
            and
            $$ y_2 - y_1 $$

        |> Paragraph   

            are not distances but %differences% (or, equivalently, %changes%).
            A distance, by definition, is a nonnegative number, while a
            difference carries no such restriction.
        |> Paragraph   
            indent = Indent::Line

            In particular, since
            $$ x_2 - x_1 $$
            can be positive or negative, while
            $$ y_2 - y_1 $$

        |> Paragraph   

            can be positive or negative or zero (more on zero below),
            the following sign
            combinations arise (lines of slope zero not included):
        
      "#
    }
}
