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
use crate::componants::Table::*;
use crate::constants::MENU_ITEMS;

use elm_to_view::elm_to_view;
use leptos::*;

#[component]
pub fn View(cx: Scope) -> impl IntoView {
    view! { cx,
        <ArticleTitle on_mobile=MENU_ITEMS[1].1 label=MENU_ITEMS[1].0 />
      <Columns>
        <ArticleBody/>
      </Columns>
    }
}

#[component]
fn ArticleBody(cx: Scope) -> impl IntoView {
    elm_to_view! {
      cx,
      r#"
|> Paragraph 

    *Slopes.*
    The _slope_ of a line is a mathematical measure of how
    “steep” a line is.
    Here are a few examples (for an explanation of the values,
    see below):

|> Image
    src="/images/chapter_2_1.svg"

|> Paragraph    
    
    To explain now, the slope of a line is...
    __the number of units the line goes up with each unit to the right__
    
|> Paragraph
    margin_top = 15

    ...assuming that numbers on the $y$-axis increase going up and that 
    numbers on the $x$-axis increase going right, as is usually the case.
    One can also describe slope as...
    __the amount of vertical change per unit of horizontal change__

|> Paragraph   
    margin_top = 15
    
    ...more elegantly.

|> Paragraph   
    indent = Indent::Line

    Thus, for example, the line below has slope 1 because it goes up by $1$ unit for each unit to the right:

|> Image
    src="/images/slope_1_example.svg"
    padding_left=2_f64

|> Paragraph   

    On the other hand, the line below has slope $-0.5$, because it goes up 
    by _minus_ $0.5$ units with each unit to the right:

|> Image
    src="/images/slope_0.5_example.svg"
    padding_left=19.5_f64

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
    container_classes="mt-6 mb-2"
    cloud_image=true

|> Paragraph   

    $$ 
    \te{slope} = {\te{vertical change from A to B} \over \te{horizontal change from A to B}}
    $$
    Indeed, dividing the vertical change by the horizontal change
    gives the per-horizontal-unit vertical change.

|> Paragraph 
    indent=Indent::Line

    More precisely, if
    $$ A = (x_1, y_1) $$
    and
    $$ B = (x_2, y_2) $$
    then
    $$ x_2 - x_1 $$
    and
    $$ y_2 - y_1 $$
    are the horizontal _&_ the vertical change, respectively,
    from $A$ to $B$, so

    |> MathBlock

        $$\te{slope} = {y_2 - y_1 \over x_2 - x_1}$$

        |> ImageRight
            src="/images/32.svg"
            width=600
            pos_x="-1rem"
            pos_y="-11rem"
            hidden_in_mobile=true
            squiggle_top="47%"
            squiggle_left="-1.5rem"

|> Paragraph   

    more succinctly. We call this the _slope formula_.

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

    are not distances but _differences_ (or, equivalently, _changes_).
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

|>  Table
    cols=vec![120, 120, 120, 120]
    lines=true
    margin_top = 15

    |> tr

        |> td
        
            $x_2 - x_1$

        |> td
        
            $y_2 - y_1$

        |> td
        
            |> img
                src="/images/33.svg" width="50"
                style="margin-top:7px; margin-bottom:7px;"

        |> td
        
            $${y_2-y_1 \over x_2-x_1}$$
    
    |> tr

        |> td
        
            $+$

        |> td
        
            $+$

        |> td

            |> img
                src="/images/34.svg" width="105"
                style="margin-top:7px; margin-bottom:7px;"
        
        |> td
        
            $${+ \over +} = \,+$$
    
    |> tr

        |> td
        
            $-$

        |> td
        
            $-$

        |> td
        
            |> img
                src="/images/35.svg" width="105"
                style="margin-top:10px; margin-bottom:4px;"

        |> td
        
            $${- \over -} = \,+$$
    
    |> tr

        |> td
        
            $+$

        |> td
        
            $-$

        |> td
        
            |> img
                src="/images/36.svg" width="105"
                style="margin-top:10px; margin-bottom:4px;"
        |> td
        
            $${- \over +} = \,-$$
    
    |> tr

        |> td
        
            $-$

        |> td
        
            $+$

        |> td
        
            |> img
                src="/images/37.svg" width="105"
                style="margin-top:10px; margin-bottom:4px;"

        |> td
        
            $$\frac{+}{-} = \,-$$

|> Paragraph   
    margin_top = 15

    In fact, we _should_ be able to algebraically verify that
    the slope formula gives the same answer if $(x_1, y_1)$ and 
    $(x_2, y_2)$ swap places, or, namely, to show that the fractions

    |> ImageRight
        src="/images/38.svg"
        pos_y="-1.65rem"
        pos_x="0"
        squiggle_left="-2rem"
        squiggle_top="38%"
        hidden_in_mobile=true
        width=390
        

|> Paragraph   
            
    $$ {y_2 - y_1 \over x_2 - x_1}\qquad\,\,\,\,\,\te{and}\,\,\,\,\,\qquad{y_1 - y_2 \over x_1 - x_2} $$
    are somehow equal. But, indeed,

    |> MathBlock
    
        $$ {y_2 - y_1 \over x_2 - x_1} = {-(y_2 - y_1) \over -(x_2 - x_1)} = {y_1 - y_2 \over x_1 - x_2} $$

        |> ImageLeft
            src="/images/39.svg"
            width=400
            pos_y="-10.5rem"
            pos_x="1rem"
            squiggle_right="0rem"
            squiggle_top="62%"
            hidden_in_mobile=true
    
    which verifies this hypothesis. In particular,
    $$ {y_2 - y_1 \over x_2 - x_1}\qquad\,\,\,\,\,\te{and}\,\,\,\,\,\qquad{y_1 - y_2 \over x_1 - x_2} $$
    are, truth be said, equally valid versions of the slope formula!

|> Paragraph   
    margin_top = 15

    *Pathological Cases.*
    If
    $$ x_2 - x_1 = 0 $$
    the slope formula
    “breaks down”
    in the sense that division by 0 is undefined. This occurs, e.g., if we attempt to measure the slope of a vertical line:

    |> Image
        src="/images/40.svg"

|> Paragraph   
    margin_top = 15

    Indeed, vertical lines have _undefined_ slope. Moreover
    the bad case
    $$ x_2 - x_1 = 0 $$
    can also occur another way, namely if the points $(x_1, y_1)$
    and $(x_2, y_2)$ coincide.
    In that case, more precisely, the slope formula evaluates to
    $$ \frac{y_2 - y_1}{x_2 - x_1} = \frac{0}{0} $$
    which could be anything. (Technically, “$0/0$” is undefined.)
    Indeed, infinitely many different lines pass through any
    given point!

|> Paragraph   
    margin_top = 15

    *“Rise over Run”.*
    Some people remember the slope
    formula as “slope equals rise over run”
    (i.e., “$\te{slope} = {\te{rise} \over \te{run}}$
    following such a picture:

    |> Image
        src="/images/44.svg"
        container_classes="pt-4"

|> Paragraph   
    margin_top = 15

    In this context, note that, in physics, a one-dimensional
    displacement is measured as
    $$ \left({\text{coordinate} \atop \te{at arrival}}\right)\,\, - \,\,\left({\te{coordinate} \atop \te{at start}}\right) $$
    in accordance, namely, with the coordinate differences
    
    |> p  

        “$x_2 - x_1\!$”, “$y_2 - y_1\!$”
        that appear in the slope formula.

|> Paragraph  
    indent=Indent::Line

    (In order not to discriminate, maybe we should also include this picture:

    |> Image
        src="/images/47.svg"
        container_classes="pt-4"


|> Paragraph   
    margin_top = 15

    Then “rise” and “run” have their signs flipped,
    but the ratio rise-over-run is the same, as already mentioned.)

|> Paragraph   
    margin_top = 15   

    _An Additional Miscellaneous Notation._
    The slope formula is occasionally written
    $$ \te{slope} = \frac{\Delta y}{\Delta x} $$

|> Paragraph   

    where the foreign-looking symbols $\Delta x$, $\Delta y$ can be thought
    of as shorthands for “$x_2 - x_1\!$”, “$y_2 - y_1\!$” respectively.
    (Or, a little more exactly, as shorthands for the phrases “change in
    $x$”, “change in $y$”.)
        
|> Paragraph   
    margin_top = 15  

    *Solving for “rise” and “run”.*
    Multiplying
    $$
    \te{slope} = \frac{\te{rise}}{\te{run}}
    $$
    on each side by “run” gives
    $$
    \te{slope} \times \te{run} = \te{rise}
    $$
    or “rise equals slope times run”.
    After which, dividing each side by “slope”, we find
    $$
    \te{run} = \frac{\te{rise}}{\te{slope}}
    $$
    or “run equals rise over slope”. These last two
    formulas can also be useful. In any case, just remember...

|> Image
    src="/images/48.svg"

|> Paragraph   
    margin_top = 15  

    ...that the slope formula has three different “permutations”
    altogether, or one per variable.

|> Paragraph   
    margin_top = 15  

    *Slopes and Line Equations.*
    An equation of the form
    $$
    y = ax + b
    $$
    where $a$ and $b$ are constants defines a line in the Cartesian plane.
    E.g.:

|> Image
    src="/images/49.svg"
    width="640px"

|> Paragraph   
    margin_top = 15 

    Note that, assuming said $y = ax + b$, one has
    $$
    y = a\cdot 0 + b = b
    $$
    at $x = 0$, so $b$ is the height of the line at $x = 0$.
    (FYI, this height is called the

|> Paragraph   
    margin_top = 15 
    align=Align::Center

    $y$-_intercept_

|> Paragraph   
    margin_top = 15 

    of the line,
    because
    $x = 0$ is where
    the line crosses the $y$ axis.
    But the point
    $$
    (0, b)
    $$
    is also sometimes called the

|> Paragraph   
    margin_top = 15 
    align=Align::Center

    $y$-_intercept_

|> Paragraph   
    margin_top = 15 

    of the line, so the term “$y$-intercept” might either refer to
    the value $b$ or to the point $(0,b)$, depending.)
    On the other hand, at $\,x = 1$, we have
    $$
    y = a\cdot 1 + b = a + b
    $$
    so $y$ increases by $a$ between $x = 0$ and $x = 1\!$.
    In fact, $y$ increases by $a$ each time $x$ increases
    by 1,
    so, 
    by our own definition of slope—the increase in $y$ per unit increase in $x$—$a$ 
    is the slope of $y = ax + b$.

|> Paragraph   
    margin_top = 15 

    *Example 2.*
    The equation
    $$
    y = 100x - 3
    $$
    describes a line of slope 100. (To hammer it in: Think of what happens when $x$
    increases by $1$.)

|> Paragraph   
    margin_top = 15 
    
    On the other hand, an equation of the form
    $$
    y = ax + b
    $$
    cannot describe a vertical line, because $a$ is the slope, while

    |> p 
    
        a vertical line has no slope, so what would $a$ be equal to?
        Instead, a vertical line is described by an equation of the form

        |> ImageLeft
            src="/images/50.svg"
            width=200
            pos_y="0.5rem"
            pos_x="-5rem"
            squiggle_right="-7rem"
            squiggle_top="48%"
            hidden_in_mobile=true

    $$
    x = c
    $$
    (see Fig. 1) where $c \in \rr$ is a constant,
    similarly to the more familiar equation
    $$
    y = b
    $$

    for a horizontal line, where $b \in \rr$ is a constant.


|> Paragraph   
    indent=Indent::Line

    One should also keep in mind that an equation can define a
    line without having either of the forms “$y = ax + b$”
    or “$x = c$”. For example,
    $$
    x + y = 3
    $$

|> Paragraph   
    margin_top = 15

    is equivalent to
    $$
    y = 3 - x
    $$

|> Paragraph   
    margin_top = 15

    and thus describes a line of $y$-intercept $3$ and slope $-1$.

    |> ImageRight
        src="/images/51.svg"
        width=300
        pos_y="-3.5rem"
        pos_x="-3rem"
        squiggle_left="-5.2rem"
        squiggle_top="31%"
        hidden_in_mobile=true

"#
    }
}
