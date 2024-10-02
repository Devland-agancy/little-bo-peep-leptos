|> Section

    *Slopes.*
    The _slope_ of a line is a mathematical measure
    of how “steep” a line is. Here are a few examples
    (for an explanation of the values, see below):

    |> Image
        src images/chapter_2_1.svg

    To explain, the slope of a line is...

    __the number of units the line goes up with each
    unit to the right__

    ...assuming that numbers on the $y$-axis increase
    going up and that numbers on the $x$-axis increase
    going right, as is usually the case. One can also
    describe slope as...

    __the amount of vertical change per unit of
    horizontal change__

    ...more elegant!

    For example, the line below has slope 1, because
    it goes up by $1$ unit for each unit to the right:

    |> Image
        src images/slope_1_example.svg
        container_classes pl-[2px]

    On the other hand, the line below has slope
    $-0.5$, because it goes up by _minus_ $0.5$ units
    with each unit to the right:

    |> Image
        src images/slope_0.5_example.svg
        container_classes pl-[19.5px]

    (Etc.)


|> Section

    *Measuring Slope.*
    The slope of a line is also the ratio of vertical
    change to horizontal change between any two
    distinct points $A$, $B$ on the line:

    |> Image
        src images/31.svg
        container_classes mt-6 mb-2
        cloud_image true

    $$ \te{slope} = {\te{vertical change from A to B} \over \te{horizontal change from A to B}} $$

    Indeed, dividing the vertical change by the
    horizontal change gives the per-horizontal-unit
    vertical change.

    More precisely, if

    $$ A = (x_1, y_1) $$

    and

    $$ B = (x_2, y_2) $$

    then

    $$ x_2 - x_1 $$

    and

    $$ y_2 - y_1 $$

    are the horizontal _&_ the vertical change,
    respectively, from $A$ to $B$, so

    $$
    \te{slope} = {y_2 - y_1 \over x_2 - x_1}
    $$

    |> ImageRight
        src images/32.svg
        offset_y -1rem

    more succinctly. We call this the _slope formula_.

    |> Example

        A line that passes through the points

        $$A = (-2, 5)$$

        and

        $$B = (4, 1)$$

        has slope

        $${1 - 5 \over 4 - (-2)} = {-4 \over 6} = - {2 \over 3}.$$

    |> StarDivider

    (The main thing to understand about Example
    1 is that

    $$ 1 - 5 $$

    is the vertical change from $A$ to $B$, while

    $$ 4 - (-2) $$

    is the horizontal change from $A$ to $B$.)


|> Section

    *Sign Combinations.*
    Technically, quantities such as

    $$ x_2 - x_1 $$

    and

    $$ y_2 - y_1 $$

    are not distances but _differences_ (or,
    equivalently, _changes_). A distance, by
    definition, is a nonnegative number, while a
    difference carries no such restriction.

    In particular, since

    $$ x_2 - x_1 $$

    can be positive or negative, while

    $$ y_2 - y_1 $$

    can be positive or negative or zero (more on
    zero below), the following sign combinations
    arise (lines of slope zero not included):

    |> Table
        cols vec![120, 120, 120, 120]
        lines true

        |> tr

            |> td

                $x_2 - x_1$

            |> td

                $y_2 - y_1$

            |> td

                |> img
                    src images/33.svg
                    width 50
                    style margin-top:7px; margin-bottom:7px;

            |> td

                $$
                {y_2-y_1 \over x_2-x_1}
                $$

        |> tr

            |> td

                $+$

            |> td

                $+$

            |> td

                |> img
                    src images/34.svg
                    width 105
                    style margin-top:7px; margin-bottom:7px;

            |> td

                $$
                {+ \over +} = \,+
                $$

        |> tr

            |> td

                $-$

            |> td

                $-$

            |> td

                |> img
                    src images/35.svg
                    width 105
                    style margin-top:10px; margin-bottom:4px;

            |> td

                $$
                {- \over -} = \,+
                $$

        |> tr

            |> td

                $+$

            |> td

                $-$

            |> td

                |> img
                    src images/36.svg
                    width 105
                    style margin-top:10px; margin-bottom:4px;

            |> td

                $$
                {- \over +} = \,-
                $$

        |> tr

            |> td

                $-$

            |> td

                $+$

            |> td

                |> img
                    src images/37.svg
                    width 105
                    style margin-top:10px; margin-bottom:4px;

            |> td

                $$
                \frac{+}{-} = \,-
                $$

    In fact, we _should_ be able to algebraically
    verify that the slope formula gives the same
    answer if $(x_1, y_1)$ and $(x_2, y_2)$ swap
    places, or, namely, to show that the fractions

    |> ImageRight
        src images/38.svg
        line 3

    $$ {y_2 - y_1 \over x_2 - x_1}\qquad\,\,\,\,\,\te{and}\,\,\,\,\,\qquad{y_1 - y_2 \over x_1 - x_2} $$

    are somehow equal. But, indeed,

    $$ {y_2 - y_1 \over x_2 - x_1} = {-(y_2 - y_1) \over -(x_2 - x_1)} = {y_1 - y_2 \over x_1 - x_2} $$

    |> ImageLeft
        src images/39.svg
        offset_y -3.5rem

    which verifies this hypothesis. In particular,

    $$ {y_2 - y_1 \over x_2 - x_1}\qquad\,\,\,\,\,\te{and}\,\,\,\,\,\qquad{y_1 - y_2 \over x_1 - x_2} $$

    are equally valid incarnations of the slope
    formula.


|> Section

    *Pathological Cases.*
    If

    $$ x_2 - x_1 = 0 $$

    the slope formula “breaks down” in the sense that
    division by 0 is undefined. This occurs, e.g., if
    we attempt to measure the slope of a vertical
    line:

    |> Image
        src images/40.svg

    Indeed, vertical lines have _undefined_ slope.
    Moreover the bad case

    $$ x_2 - x_1 = 0 $$

    can also occur another way, namely if the points
    $(x_1, y_1)$ and $(x_2, y_2)$ coincide. In that
    case, more precisely, the slope formula evaluates
    to

    $$ \frac{y_2 - y_1}{x_2 - x_1} = \frac{0}{0} $$

    which could be anything. (Technically, “$0/0$” is
    undefined.) Indeed, infinitely many different
    lines pass through any given point!


|> Section

    *“Rise over Run”.*
    Some people remember the slope formula as “slope
    equals rise over run” (i.e., “$\te{slope} =$
    |> InlineImage
        src images/rise-run.svg
        width 28px
        space_right true
    ”), following such a picture:

    |> Image
        src images/44.svg
        width 300px
        container_classes pt-4 mb-4

    In this context, note that, in physics, a
    one-dimensional displacement is measured as

    $$
    \left({\te{coordinate} \atop \te{at arrival}}\right)\,\, - \,\,\left({\te{coordinate} \atop \te{at start}}\right)
    $$

    in accordance, namely, with the coordinate
    differences “$x_2 - x_1\!$”, “$y_2 - y_1\!$”
    that appear in the slope formula.

    (In order not to discriminate, maybe we should
    also include this picture:

    |> Image
        src images/47.svg
        container_classes pt-4 mb-4

    Then “rise” and “run” have their signs flipped,
    but the ratio rise-over-run is the same, as
    already mentioned.)

    |> Pause

    _An Additional Miscellaneous Notation._
    The slope formula is occasionally written

    $$
    \te{slope} = \frac{\Delta y}{\Delta x}
    $$

    where the foreign-looking symbols $\Delta x$,
    $\Delta y$ can be thought of as shorthands for
    “$x_2 - x_1$”, “$y_2 - y_1$” respectively. (Or,
    a little more exactly, as shorthands for the
    phrases “change in $x$”, “change in $y$”.)


|> Section

    *Solving for “rise” and “run”.*
    Multiplying
    $$
    \te{slope} = {\te{rise} \over \te{run}}
    $$
    on each side by “run” gives

    $$
    \te{slope} \times \te{run} = \te{rise}
    $$

    or “rise equals slope times run”. After which,
    dividing each side by “slope”, we find

    $$
    \te{run} = {\te{rise} \over \te{slope}}
    $$

    or “run equals rise over slope”. Thus:

    |> Image
        src images/48.svg

    ...as can sometimes be useful to know.

|> Section

    *Slopes and Line Equations.* An equation of the
    form

    $$y = ax + b$$

    where $a$ and $b$ are constants defines a line in
    the Cartesian plane. E.g.:

    |> Image
        src images/49.svg
        width 640px

    Note that, assuming said $y = ax + b$, one has

    $$y = a\cdot 0 + b = b$$

    at $x = 0$, so $b$ is the height of the line at
    $x = 0$. (FYI, this height is called the

    __$y$-intercept__

    of the line, because $x = 0$ is where the line
    crosses the $y$ axis. But the point

    $$(0, b)$$

    is also sometimes called the

    __$y$-intercept__

    of the line, so the term “$y$-intercept” might
    either refer to the value $b$ or to the point
    $(0,b)$, depending.) On the other hand, at
    $x = 1$, we have

    $$y = a\cdot 1 + b = a + b$$

    so $y$ increases by $a$ between $x = 0$ and
    $x = 1$. In fact, $y$ increases by $a$ each time
    $x$ increases by 1, so, by our own definition of
    slope—the increase in $y$ per unit increase in
    $x$—$a$ is the slope of $y = ax + b$.

    |> Example

        The equation

        $$y = 100x - 3$$

        defines a line of slope 100.

    On the other hand, an equation of the form

    $$y = ax + b$$

    cannot describe a vertical line, because $a$ is
    the slope, while a vertical line has no slope, so
    what would $a$ be equal to? Instead, a vertical
    line is described by an equation of the form

    $$x = c$$

    |> ImageLeft
        src images/svg_vertical_line_equation.svg
        offset_y 1rem
        offset_x 4rem
        use_squiggle_on_mobile false

    (see Fig. 1) where $c \in \rr$ is a constant, 
    similarly to the more familiar equation

    $$y = b$$

    for a horizontal line, where $b \in \rr$ is a
    constant.

    One should also keep in mind that an equation can
    define a line without having either of the forms
    “$y = ax + b$” or “$x = c$”. For example,

    $$x + y = 3$$

    is equivalent to

    $$y = 3 - x$$

    and thus describes a line of $y$-intercept $3$
    and slope $-1$.

    |> ImageRight
        src images/svg_3_plus_minus_one_times_x_cloud.svg
        line 1
        offset_x 2rem

|> Section

    *Slopes and Units.*
    If the $x$- and $y$-axes have units then a line's
    slope has units

    $$
    {\te{$y$ axis units} \over \te{$x$ axis units}}
    $$

    as should make sense, given that the slope is a
    change in $y$ divided by a change in $x$.

    For example, if the units on the $y$ axis are
    meters (“m”) and the units on the $x$ axis are
    seconds (“s”) then the slope has units

    $$
    {\te{$y$ axis units} \over \te{$x$ axis units}} = {\te{m} \over \te{s}}
    $$

    also known as _meters per second_. This is
    precisely the case, for example, in the following
    graph, that purports to plot the height of a
    balloon, in meters, as a function of time elapsed,
    in seconds:

    |> Image
        src images/52.svg
        container_classes mt-[1.4em] mb-[1.2em]

    The slope of the graph is $0.75$ _meters per
    second_ because the balloon's height increases by
    three _meters_ over the first four _seconds_ (if
    you had noticed):

    $$
    \te{slope} \left(\!= {\te{rise} \over \te{run}}\right) = {3\te{m} \over 4\te{s}} = 0.75\te{m}/\te{s}
    $$

    In fact, the slope is the balloon's upward

    __velocity__

    since velocity is defined as

    __displacement over time__

    and this is precisely the form of the ratio “rise
    over run” for the current graph. (More generally,
    we have

    _|“slope = velocity”|_

    whenever the $y$ axis has dimensions of length and
    the $x$ axis has dimensions of time—whether the
    slope turns out to be $\te{m}/\te{s}$ or $\te{km}/\te{s}$
    or km/hour, etc, depends on the exact units involved.)

    Terminology-wise, slopes are often known as

    __rates of change__

    in the presence of units. More particularly, in the
    common case when the $x$-axis denotes time, the
    formula

    $$\te{slope} = {\te{rise} \over \te{run}}$$

    can be rephrased as

    $$“\rt{0.1}\te{rate of change} \,\,=\,\,
    {\!\rt{0.15}\te{amount of change}\!\rt{0.1} \over \te{amount of time}}\rt{0.1}”$$

    where “amount of change” is short for “amount of
    change on the $y$-axis”. By extension, taking all
    three permutations of the slope formula into account
    gives us...

    |> Image
        src images/53.svg

    ...these formulas, commonly useful in “applied”
    problems.

    |> Example

        The increase in height of the above balloon
        over a period of 5 seconds is

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
        $0.75$m$/$s is the “rate of change” of the
        balloon's height.)

    |> Example

        The amount of time required
        for the balloon to go up by (say) $4$m is

        $$
        \frac{4\te{m}}{0.75\te{m}/\te{s}}\! = 5.3333...\te{s}
        $$

        |> ImageRight
            src images/54.svg
            offset_x -6rem
            children_y 34.5%
            children_x 29%
            clickable_on_desktop true

            $$
            {4 \over 0.75} = 5.3333...\qquad\qquad\,\,\,
            $$

        following the template

        $$
        \te{“}\rt{0.1}
        \te{amount of time} \,\,=\,\, \frac{\!\rt{0.15}\te{amount of change}\!\rt{0.1}}{\te{rate of change}}
        \rt{0.1}\te{”}
        $$

        found in the third line of the table.

|> Section
    divider false

    *Postscript: Units vs Dimensions.*
    Comparing

    _|“the $x$-axis has dimensions of time”|_

    with

    _|“the $x$-axis has units of seconds”|_

    one could easily be tricked into thinking that a
    “dimension” is the same thing as a “unit”. In
    fact, dimensions are broader categories, such as,
    namely,

    __time__
    __length__
    __mass__

    each of which covers _several different_ units.
    For example, in the “time” dimension, one finds
    individual units of the type

    __years, seconds, minutes, hours, days__

    (etc), while in
    the “length” dimension one finds

    __meters, kilometers, millimeters, yards, feet__

    (etc), and so on. (You can imagine some of the
    units found in the “mass” dimension, e.g..) On
    the other hand, dimensions can be multiplied and
    divided just like units. For example,

    __length over time__

    is another dimension, commonly known as...
    velocity!

|> Exercises

    |> Exercise

        True or false: Lines of slope $-1 \over 2$
        are perpendicular to lines of slope $2$.

        |> Solution

            This is true, as illustrated by the following pair of lines:

            |> Image
                src images/55.svg
                container_classes relative w-fit

                |> ImageRight
                    src images/56.svg
                    _attached false
                    offset_y 5rem
                    offset_x -1.3rem
                    use_squiggle_on_mobile false

                |> ImageLeft
                    src images/57.svg
                    _attached false
                    offset_x -7rem
                    offset_y 0.5rem
                    y bottom
                    img_position bottom
                    use_squiggle_on_mobile false

            In more detail, the two triangles are related
            by a $90^\circ$ rotation and so, likewise,
            are the lines defined by their hypotenuses!

            |> Pause

            _Note 1._
            More generally, a line of slope $p$ is
            perpendicular to a line of slope $-1/p$, for
            all $p \ne 0$. By a similar drawing:

            |> Image
                src images/58.svg
                container_classes relative w-fit

                |> ImageRight
                    src images/59.svg
                    _attached false
                    offset_y 7.2rem
                    offset_x -1.2rem
                    use_squiggle_on_mobile false

                |> ImageLeft
                    src images/60.svg
                    _attached false
                    offset_x -5rem
                    offset_y 0.5rem
                    y bottom
                    img_position bottom
                    use_squiggle_on_mobile false

            (If the axes are oriented the usual way then 
            the above drawing covers all the cases $p > 0$,
            whereas if we flip the two number axes to point
            down/left the above drawing covers all the cases 
            $p < 0$—magic!)

    |> Exercise

        Find the general equation of a line of slope
        $p$ passing through a point $(x_0, y_0)$.
        (Hint: Start from the slope formula.)

        |> Solution

            A point $(x,y) \ne (x_0,y_0)$ is on the
            line of slope $p$ if and only if

            $$ p = {y - y_0 \over x - x_0} $$

            |> ImageLeft
                src images/62.svg
                img_position top
                offset_x -7.2rem
                offset_y -7.1rem
                use_squiggle_on_mobile false

            because

            $$ \,\,{y - y_0 \over x - x_0} $$

            is the slope of the line segment from
            $(x_0,y_0)$ to $(x,y)$, and it is necessary
            and sufficient for this segment to have slope
            $p$ in order for the point $(x,y)$ to be on
            the line!

            Unfortunately, the equation

            $$ p = {y - y_0 \over x - x_0} $$

            is not an entirely satisfactory answer,
            because the point $(x,y) = (x_0,y_0)$ itself
            does not satisfy the equation. (We find

            $$ p = {0 \over 0} $$

            if we plug in $x = x_0$, $y = y_0$, which is
            not a valid equality because the right-hand
            side is an undefined quantity.)

            Instead, multiplying

            $$
            p = {y - y_0 \over x - x_0}
            $$

            on both sides by $x-x_0$, we find the
            fraction-less equation

            $$
            p(x-x_0) = y-y_0
            $$

            which is satisfied by the point $(x,y) = (x_0,y_0)$
            as well as by every other point on the line.
            This can be a final answer, and, pleasingly,
            has the form

            $$
            \te{“}\te{slope} \times \te{run} = \te{rise}\te{”}
            $$

            |> ImageRight
                src images/svg_slope_times_run_equals_rise_cloud.svg
                offset_x 2.5rem
                offset_y -0.5rem

            which can also make it easy to remember!

            |> Pause

            _Note 1._
            The answer we gave is more often written

            $$
            y - y_0 = p(x - x_0)
            $$

            with the two sides of the equation swapped, or

            $$
            y = p(x - x_0) + y_0
            $$

            with $y$ isolated on the left-hand side. From
            there one can also distribute $p(x-x_0)$,
            obtaining (after putting “$-px_0$” last)

            $$
            y = px + y_0 - px_0
            $$

            which has the form

            $$
            y = ax + b
            $$

            with $a = p$, $b = y_0 - px_0$.

    |> Exercise

        Plot the vertical velocity of an object a
        mosquito whose height over time is given by
        this graph (use the same time interval as the
        graph):

        |> Image
            src images/1.svg
            container_classes pt-[22px] pb-[15px]
            padding_left 90

        |> Solution

            Here is the “official” graph of the (vertical)
            velocity:

            |> Image
                src images/64.svg
                container_classes pt-[21px] pb-[20px]
                padding_left 46

            On each interval, the velocity is rate of
            change of the height, i.e., the _slope_ of
            the height. For example, the rate of change
            of the height is

            $$
            {1\te{m} \over 1\te{s}} = 1\te{m}/\te{s}
            $$

            between $-4$s and $-3$s, where the mosquito
            goes up by one meter during a one second
            period, so the vertical velocity is 1m$/$s
            for that time interval, etc.

            |> Pause

            _Note 1._
            As explained in Chapter 3, an empty circle of
            this type

            |> Image
                src images/svg_empty_circle_alone.svg

            indicates a “missing” value. Specifically, in
            our case, the vertical velocity is _undefined_
            wherever the graph of the height has a sharp
            corner. (Because the slope of the graph is not
            well-defined at such corners.)

            |> Pause

            _Note 2._
            For the time interval from $2$s to $2.5$s, the
            slope is

            $$ {-2\rt{0.05}\te{m} \over 0.5\rt{0.05}\te{s}} = -\rt{0.07}4\rt{0.1}\te{m}/\te{s} $$

            and similarly for the time interval from
            $2.5$s to $3$s the slope is

            $$ {2\rt{0.05}\te{m} \over 0.5\rt{0.05}\te{s}} = 4\rt{0.1}\te{m}/\te{s} $$

            because $2/0.5 = 4$. (Think: _how many times_
            does $0.5$ go into $2$?)

    |> Exercise

        Digressing on the second-to-last equation
        in the solution to Exercise 2, explain why

        $$ y_0 - px_0 $$

        is the $y$-intercept of the line of slope $p$
        through the point $(x_0,y_0)$ by using a drawing
        and “rise equals slope times run”.

        |> Solution

            E.g.:

            |> Image
                src images/66.svg

            The rise from the $y$-intercept to $(x_0, y_0)$
            is $px_0$, as found by “rise equals slope times
            run”, implying that

            $$
            y_0 - px_0
            $$

            is the $y$-intercept.

            |> Pause

            _Note 1._ Our drawing makes some implicit
            assumptions, such as $p > 0$ and $x_0 > 0$. But

            $$ px_0 $$

            is the rise from the $y$-intercept to
            $(x_0, y_0)$ regardless of the sign of $p$ or
            $x_0$ (because $x_0$ is the run in all cases),
            making

            $$ y_0 - px_0 $$

            the $y$-intercept in all cases.
