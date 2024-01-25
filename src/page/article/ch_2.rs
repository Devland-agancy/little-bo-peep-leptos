use crate::components::Article::*;
use crate::components::ArticleTitle::*;
use crate::components::Columns::*;
use crate::components::Image::*;
use crate::components::ImageLeft::*;
use crate::components::ImageRight::*;

use crate::components::Math::*;
use crate::components::MathBlock::*;

use crate::components::ImageLink::*;
use crate::components::Paragraph::*;
use crate::components::SectionDivider::*;
use crate::components::Solution::*;
use crate::components::Span::*;
use crate::components::Table::*;
use crate::components::Tabs::*;

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

|> SectionDivider

|> Paragraph   

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

|> SectionDivider

|> Paragraph   

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

|> SectionDivider

|> Paragraph   

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
    or “run equals rise over slope”. Thus:

|> Image
    src="/images/48.svg"

|> Paragraph   
    
    ...as can sometimes be useful to know.  

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
    cannot describe a vertical line, because $a$ is the slope, while a vertical line has no slope, so what would $a$ be equal to?
    Instead, a vertical line is described by an equation of the form

    |> ImageLeft
        src="/images/50.svg"
        width=200
        pos_y="0.5rem"
        pos_x="-5rem"

    $$
    x = c
    $$
    (see @Fig. 1@) where $c \in \rr$ is a constant,
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

|> Paragraph   
    margin_top = 15

    *Slopes and Units.*
    If the $x$- and $y$-axes have units then a line's slope has units
    $$
    {\te{$y$ axis units} \over \te{$x$ axis units}}
    $$
    as should make sense, given that the slope is
    a change in $y$ divided by a change in $x$.
    
|> Paragraph   
    indent=Indent::Line

    For example, if the units on the $y$ axis are meters (“m”) and
    the units on the $x$ axis are seconds (“s”) then the slope has
    units
    $$
    \frac{\te{$y$ axis units}}{\te{$x$ axis units}} = \frac{\te{m}}{\te{s}}
    $$
    also known as _meters per second_. This is precisely the
    case, for example, in the following graph, that purports to
    plot the height of a balloon, in meters, as a function of time
    elapsed, in seconds:
    
    |> Image
        src="/images/52.svg"
        container_classes="mt-[1.4em] mb-[1.2em]"

|> Paragraph   

    The slope of the graph is

|> Paragraph   
    align=Align::Center
    margin_top = 15
    
    $0.75$ _meters per second_

|> Paragraph   
    margin_top = 15

    because the balloon's height
    increases by three _meters_ over the first four _seconds_ (if you had noticed):
    $$
    \te{slope} \left(\!= \frac{\te{rise}}{\te{run}}\right) = \frac{3\te{m}}{4\te{s}} = 0.75\te{m}/\te{s}
    $$
    In fact, the slope is the balloon's upward
    __velocity__

|> Paragraph   
    margin_top = 15

    since velocity is defined as
    __displacement over time__

|> Paragraph   
    margin_top = 15

    and this is precisely the form of the ratio “rise over
    run” for the current graph. (More generally, we have

|> Paragraph   
    align=Align::Center
    margin_top = 15

    “slope = velocity”

|> Paragraph   
    margin_top = 15

    whenever the $y$ axis has dimensions of length and the $x$ axis has
    dimensions of time&mdash;whether the slope turns out to

    |> p

        be $\te{m}/\te{s}$ or $\te{km}/\te{s}$ or km/hour, etc,
        depends on the exact units involved.)

|> Paragraph   
    indent=Indent::Line
    margin_top = 15

    Terminology-wise, slopes are often known as
    __rates of change__

|> Paragraph   
    margin_top = 15

    in the presence of units. More particularly, in the common case
    when the $x$-axis denotes time, the formula
    $$
    \,\,\te{slope} = \frac{\te{rise}}{\te{run}}\,\,
    $$
    can be rephrased as
    $$
    “\rt{0.1}\te{rate of change} \,\,=\,\, \frac{\!\rt{0.15}\te{amount of change}\!\rt{0.1}}{\te{amount of
    time}}\rt{0.1}”
    $$
    where “amount of change”
    is short for “amount of change on the $y$-axis”.
    By extension, taking all three permutations of the slope
    formula into account gives us...

|> Image
    src="/images/53.svg"

|> Paragraph   

    ...these formulas, commonly useful in “applied”
    problems.

|> Paragraph   
    margin_top = 15

    *Example 3.*
    The increase in height of
    the above balloon over a period of 5 seconds is
    $$
    0.75\te{m}/\te{s} \times 5\te{s} = 3.75\te{m}
    $$
    following the template
    $$
    \begin{array}{c}
    \rt{0.08}
    (\te{rate of change}) \times \te{(amount of time)}\\
    = \te{(amount of change)}\rt{0.05}
    \end{array}
    $$
    found in the second line of the table. (Indeed,
    $0.75$m$/$s is the “rate of change” of
    the balloon's height.)

|> Paragraph   
    margin_top = 15

    *Example 4.*
    The amount of time required
    for the balloon to go up by (say) $4$m is
    
    |> MathBlock

        $$
        \frac{4\te{m}}{0.75\te{m}/\te{s}}\! = 5.3333...\te{s}
        $$

        |> ImageRight
            src="/images/54.svg"
            width=930
            pos_x="5rem"
            pos_y="-5.5rem"
            hidden_in_mobile=true
            squiggle_top="40%"
            squiggle_left="3.5rem"
            children_y="34%"
            children_x="29%"
            clickable_on_desktop=true

            $$
            {4 \over 0.75} = 5.3333...\qquad\qquad\,\,\,
            % {\te{m} \over \te{m}/\te{s}}
            %= \te{m} \times {1 \over \left({\te{m} \over \te{s}}\right)}
            %= \te{m} \times {\te{s} \over \te{m}}
            %= \te{s}
            $$

    following the template
    $$
    \te{“}\rt{0.1}
    \te{amount of time} \,\,=\,\, \frac{\!\rt{0.15}\te{amount of change}\!\rt{0.1}}{\te{rate of change}}
    \rt{0.1}\te{”}
    $$
    found in the third line of the table.

|> Paragraph   
    margin_top = 15

    *Postscript: Units vs Dimensions.*
    Comparing

|> Paragraph   
    margin_top = 15
    align=Align::Center

    “the $x$-axis has dimensions of time”

|> Paragraph   
    margin_top = 15

    with

|> Paragraph   
    margin_top = 15
    align=Align::Center

    “the $x$-axis has units of seconds”

|> Paragraph   
    margin_top = 15

    one could easily be tricked into thinking that a “dimension”

    |> p

        is the same thing as a “unit”.
        In fact, dimensions are broader categories, such as, namely,

    __time__
    __length__
    __mass__

|> Paragraph   
    margin_top = 15

    each of which covers _several different_ units.
    For example, in the “time” dimension, one finds
    individual units of the type
    __years, seconds, minutes, hours, days__

|> Paragraph   
    margin_top = 15

    (etc), while in the “length” dimension one finds
    __meters, kilometers, millimeters, yards, feet__
    
|> Paragraph   
    margin_top = 15

    (etc), and so on.
    (You can imagine some of the units found in the “mass”
    dimension, e.g..) On the other hand, dimensions can be multiplied
    and divided just like units.
    For example,
    __length over time__

|> Paragraph   
    margin_top = 15
    
    is another dimension, commonly known as... velocity!

|> Image
    id="exo"
    src="/images/seperator.png"
    height="50px"
    width="160px"
    container_classes="flex items-center mt-[45px] mb-[40px]"

|> Tabs
    labels=vec!["0", "1", "2", "3"]

    |> TabElement

        |> Paragraph 
            classes="animate-appear-slow"

            *Exercise 1. *
            True or false: Lines of slope $ -1 \over 2 $ are perpendicular to lines of slope $2$.

        |> Solution

            |> Paragraph

                This is true, as illustrated by the following pair of lines:

                |> Image
                    src="/images/55.svg"

                |> ImageRight
                    src="/images/56.svg"
                    width=340
                    pos_x="3rem"
                    pos_y="15rem"

                |> ImageLeft
                    src="/images/57.svg"
                    width=340
                    pos_x="13rem"
                    pos_y="19rem"

            |> Paragraph

                In more detail, the two triangles are related by
                a $90^\circ$ rotation and so, likewise, are the lines defined
                by their hypotenuses!
    
            |> Paragraph
                margin_top=15

                _Note 1._ 
                More generally, a line of slope $p$ is perpendicular to a line
                of slope $-1/p$, for all $p \ne 0$. By a similar drawing, for $p > 0$:
    
                |> Image
                    src="/images/58.svg"

                |> ImageRight
                    src="/images/59.svg"
                    width=340
                    pos_x="3rem"
                    pos_y="20rem"

                |> ImageLeft
                    src="/images/60.svg"
                    width=340
                    pos_x="11rem"
                    pos_y="23rem"

            |> Paragraph

                If you're curious, a drawing for the case $p < 0$...

                |> Image
                    src="/images/61.svg"

            |> Paragraph

                ...is like so, but it doesn't really add anything new. 

    |> TabElement

        |> Paragraph 
            classes="animate-appear-slow"

            *Exercise 2. *
            Find the general equation of a line of slope 
            $p$ passing through a point $(x_0, y_0)$.
            (Hint: Start from the slope formula.) 

        |> Solution

            |> Paragraph

                A point $(x,y) \ne (x_0,y_0)$ is on the line of
                slope $p$ if and only if
                $$
                p = {y - y_0 \over x - x_0}
                $$
                because
                $$
                \,\,{y - y_0 \over x - x_0}
                $$
                is the slope of the line segment from $(x_0,y_0)$ 
                to $(x,y)$, and it is necessary and
                sufficient for this segment to have slope $p$ in order
                for the point $(x,y)$ to be on the line!

                |> Paragraph
                    indent=Indent::Line

                    Unfortunately, the equation

                $$
                p = {y - y_0 \over x - x_0}
                $$
                is not an entirely satisfactory answer, because the
                point $(x,y) = (x_0,y_0)$ itself does not satisfy the
                equation. (We find
                $$
                p = {0 \over 0}
                $$
                if we plug in $x = x_0$, $y = y_0$, which is not a valid
                equality because the right-hand side is an undefined quantity.)

                |> Paragraph
                    indent=Indent::Line

                    Instead, multiplying

                $$
                p = {y - y_0 \over x - x_0}
                $$
                on both sides by $x-x_0$, we find the fraction-less equation
                $$
                p(x-x_0) = y-y_0
                $$
                which is satisfied by the point $(x,y) = (x_0,y_0)$ as well as
                by every other point on the line.
                This can be a final answer, and, pleasingly, has the form

                |> MathBlock

                    \$\$\te{“}\te{slope} \times \te{run} = \te{rise}\te{”}\$\$

                    |> ImageRight
                        src="/images/63.svg"
                        width=400
                        pos_x="-1.3rem"
                        pos_y="-1.6rem"
                        hidden_in_mobile=true
                        squiggle_top="36%"
                        squiggle_left="-2.5rem"

                which can also make it easy to remember!

            |> Paragraph
                margin_top=15

                _Note 1._
                The answer we gave is more often written
                $$
                y - y_0 = p(x - x_0)
                $$
                with the two sides of the equation swapped, or
                $$
                y = p(x - x_0) + y_0
                $$
                with $y$ isolated on the left-hand side. From there one
                can also distribute $p(x-x_0)$, obtaining (after putting
                “$-px_0$” last)
                $$
                y = px + y_0 - px_0
                $$
                which has the form
                $$
                y = ax + b
                $$
                with $a = p$, $b = y_0 - px_0$.

    |> TabElement
    
        |> Paragraph 
            classes="animate-appear-slow"

            *Exercise 3. *
            Plot the vertical velocity of an object a mosquito whose height over time is given by this graph (use the same time interval as the graph):

        |> Image
            src="/images/1.svg"

        |> Solution

            |> Paragraph

                Here is the “official” graph of the (vertical) velocity:

            |> Image
                src="/images/64.svg"

            |> Paragraph
                indent=Indent::Line

                On each interval, the velocity is rate of change
                of the height, i.e., the _slope_ of the height.
                For example, the rate of change of the height is
                $$
                {1\te{m} \over 1\te{s}} = 1\te{m}/\te{s}
                $$
                between $-4$s and $-3$s, where the mosquito goes up by one meter
                during a one second period, so the vertical velocity is
                1m$/$s for that time interval, etc.

            |> Paragraph
                margin_top=15

                _Note 1._
                As explained in Chapter 3, an empty circle of this type

            |> Image
                src="/images/65.svg"
                width="30px"

            |> Paragraph

                indicates a “missing” value. 
                Specifically, in our case, the vertical velocity is _undefined_
                wherever the graph of the height has a sharp corner.
                (Because the slope of the graph is not well-defined at such corners.)

            |> Paragraph
                margin_top=15

                _Note 2._ 
                For the time interval from $2$s to $2.5$s, the slope is
                $$
                {-2\rt{0.05}\te{m} \over 0.5\rt{0.05}\te{s}} = -\rt{0.07}4\rt{0.1}\te{m}/\te{s}
                $$
                and similarly for the time interval 
                from $2.5$s to $3$s the slope is
                $$
                {2\rt{0.05}\te{m} \over 0.5\rt{0.05}\te{s}} = 4\rt{0.1}\te{m}/\te{s}
                $$
                because $2/0.5 = 4$.
                (Think: _how many times_ does $0.5$ go into $2$?)

    |> TabElement

        |> Paragraph 
            classes="animate-appear-slow"

            *Exercise 4. *
            Digressing on the second-to-last equation
            in the solution to Exercise 2, explain why
            $$
            y_0 - px_0
            $$
            is the $y$-intercept of the line of slope $p$ through
            the point $(x_0,y_0)$ by using a drawing and “rise equals
            slope times run”.

        |> Solution
            
            |> Paragraph

                E.g.:

            |> Image
                src="/images/66.svg"
                width="640px"

            |> Paragraph

                The rise from the $y$-intercept to $(x_0, y_0)$ is $px_0$, as found
                by “rise equals slope times run”, implying that
                $$
                y_0 - px_0
                $$
                is the $y$-intercept.
                
                _Note 1._ Our drawing makes some implicit
                assumptions, such as $p > 0$ and $x_0 > 0$.
                But
                $$
                px_0
                $$
                is the rise from the $y$-intercept to $(x_0, y_0)$
                regardless of the sign of $p$ or $x_0$ (because “$x_0$” is
                the run in all cases), making
                $$
                y_0 - px_0
                $$
                the $y$-intercept in all cases.

"#
    }
}
