
|> Exercise

    Describe what a function might look like
    if its second derivative has this graph (broadly):

    |> Image
        src images/svg_ch4_one_long_line.svg

    |> Solution

        As the second derivative is the

        __rate of change of the slope__

        places where the second derivative is zero are 
        places where the slope of the function is constant.
        So the function will have a constant slope over
        each of these purple intervals (we're going to
        assume that what looks like $0$ is $0$, and eyeball 
        where that starts and ends, the statement
        does say “broadly” anyway):

        |> Image
            src images/svg_ch4_one_long_line_with_purple.svg

        As

        __constant slope__

        means

        __line segment__

        the function will therefore be a 
        line segment, over each of the purple intervals!

        Between these line segments, however, things
        happen, and there is a change in slope! The
        change in slope is given by the “amount of bump”
        in the second derivative between the intervals.
        As it turns out, the area enclosed by the bump
        gives the total change in slope:

        |> Image
            src images/svg_ch4_one_long_line_positive_area.svg

        (We won't argue this right now, but it's 
        sort-of-intuitive.)
        For bumps lying below the $x$-axis the area
        counts as negative; that negative area is, again,
        the total change in slope from one end of the bump
        to the other:

        |> Image
            src images/svg_ch4_one_long_line_negative_area.svg

        In any case the areas are all the same _in absolute
        value_, meaning that whatever slope is gained as
        we pass over a positive bump, the same amount
        is lost again as we pass over a negative bump!
        Thus, the line segments of the original function
        will alternate between “low” and “high”
        slopes—as we pass over a positive bump we switch
        from a “low slope” line segment to a “high slope”
        line segment, and vice-versa when we pass over a
        negative bump.

        Put 

        $$
        \Large a
        $$

        for the area of a positive bump (per appearances, 
        $a \approx 1$), and
        
        |> ImageRight
            src images/svg_ch4_one_long_line_a_is_about_1_cloud.svg

        $$
        \Large c
        $$

        for the slope of a “low slope” line segment. Then a
        “high slope” line segment has slope 

        $$
        \Large c + a
        $$

        since we add $a$ to the slope each time we go over
        a positive bump. 
        (And the slope goes back down to

        $$
        \Large (c + a) + (-a) = c
        $$

        when we pass over a negative bump, with $-a$ being
        the (negative) area of a negative bump.)

        With these variables in place, here is a generic
        illustration of a graph (in black) whose second
        derivative is the one from the statement (faded in
        the background):

        |> Image
            src images/svg_ch4_one_long_line_solution_sketch_1.svg

        |> ImageLeft
            src images/svg_ch4_one_long_line_c_is_approx_0_point_2_cloud.svg

        In this example $c \approx 0.2$, but $c$ can be any
        value—this is not constrained by the second derivative.
        Moreover any amount of vertical translation can 
        also be introduced to the graph. (Vertical
        translation does not affect the derivative,
        much less the second derivative.)

        For another example, if $c = -a/2$,
        meaning $c \approx -1/2$,
        the graph ends up a perfectly balanced see-saw that 
        stays confined to a bounded range of $y$-values:

        |> Image
            src images/svg_ch4_one_long_line_solution_sketch_2.svg

        |> ImageLeft
            src images/svg_ch4_one_long_line_c_is_approx_minus_a_over_2_cloud.svg

        Moreover, like the previous graph, this solution
        can also be vertically translated by any amount!
        (And same for any solution.)

        For yet another example, here is a graph in which
        $c + a = 0$, $c = -a \approx -1$:

        |> Image
            src images/svg_ch4_one_long_line_solution_sketch_3.svg

        Again, any of these graphs are

        __equally valid__

        solutions, and, for the last time,

        __any amount of vertical translation can be introduced__

        (you can move the graphs up and down). So in
        other words we have a “two-parameter family of
        solutions”: one parameter of the solution—free
        to choose—is $c$—while another parameter—independently
        free to choose—is the amount of vertical
        translation.

        To claim a truly good “theoretical” understanding
        of the solution, however, we should also determine
        this rise here, if we can, as a function of $c$
        and $a$, i.e., the amount of rise between the end
        of one line segment and the start of the next:

        |> Image
            src images/svg_ch4_one_long_line_what_are_the_rises_1.svg

        In fact, is not entirely clear that there aren't
        possibly _two different_ values of this rise,
        for the two different kinds of “connector curves”
        that exist (the concave ones and the convex ones):

        |> Image
            src images/svg_ch4_one_long_line_what_are_the_rises_2.svg

        (It will turn out that 
        the rises are all the same
        but we're just pointing out.)
        Focusing on the case
        of a convex connector curve, note 
        that the rise is lower bounded by
        $$
        \Large 1.6c
        $$
        where $1.6 = 0.8 + 0.8$ is the length (run) of
        the connector curve, because $c$ is the _lowest 
        slope_ found anywhere inside the connector
        curve:

        |> Image
            src images/svg_ch4_one_long_line_connector_curve_lower_bound.svg

        Symmetrically,

        $$
        \Large 1.6(c + a)
        $$

        is an upper bound on the rise, because $c + a$ is the
        _greatest slope_ anywhere inside the
        connector curve:

        |> Image
            src images/svg_ch4_one_long_line_connector_curve_upper_bound.svg

        To go any further we must add the first
        derivative to this sketch—the first derivative
        has value
        $$
        \Large c
        $$
        where the function has slope $c$, has value
        $$
        \Large c + a
        $$
        where the function has slope $c + a$, and
        climbs up/down along an S-shaped curve outside
        of those intervals, adhering
        to a slope that is given by the value of the
        second derivative: 

        |> Image
            src images/svg_ch4_one_long_line_what_are_the_rises_3.svg

        The afore-mentioned lower bound of 

        $$
        \Large 1.6c
        $$

        coincides with the area of a rectangle that _lies
        below_ the graph of the derivative:

        |> Image
            src images/svg_ch4_one_long_line_connector_curve_lower_bound_area.svg

        Whereas the afore-mentioned upper bound of 
    
        $$
        \Large 1.6(a + c)
        $$

        coincides with the area of a rectangle that
        _lies above_ the graph of the derivative:

        |> Image
            src images/svg_ch4_one_long_line_connector_curve_upper_bound_area.svg

        In other words, the rise of the convex connector
        curve is lower and upper bounded by these
        two areas. It will be helpful to write this
        as a pictorial inequality:

        |> Image
            width 800px
            src images/svg_ch4_one_long_line_inequality_1.svg

        But we can tighten the inequality by dividing the
        areas halfway (we'll let you think about this 
        one—if you don't get it, don't worry, because we'll
        revisit the same topic in detail at some point):

        |> Image
            width 800px
            src images/svg_ch4_one_long_line_inequality_2.svg

        Or even:

        |> Image
            width 800px
            src images/svg_ch4_one_long_line_inequality_3.svg

        If we take this logic to its bitter conclusion, we
        find the _e_quality:

        |> Image
            width 800px
            src images/svg_ch4_one_long_line_equality.svg

        And because the S-curve is centrally symmetric
        (the slopes at equal distance from the center are
        the same because those slopes can be read off the
        second derivative, and the second derivative bump
        is left-right symmetric) we can compute the area
        that the curve encloses exactly, by a geometric
        surgery:

        |> Image
            src images/svg_ch4_one_long_line_area_surgery.svg

        Long story short, the area enclosed, which is
        also the rise of the connector curve, is
        
        $$
        \Large 1.6\cdot (c + {a\over 2})
        $$
        
        ...that can be read as “run times average slope”
        (because
        
        $$
        \Large 1.6
        $$
        
        is the run while the slope (first derivative)
        spends equal amounts of time, in equal measure,
        above and below the value

        $$
        \Large c + {a \over 2}
        $$

        that is, indeed, the average of $c$ and $c + a$).
        For concave connector curves the
        S-curve of the derivative is...

        |> Image
            src images/svg_ch4_one_long_line_concave_connector.svg

        ...flipped around from before, going from
        high to low, but the area enclosed by the S-curve
        is the same. This area is also the rise of the
        connector curve. Hence, long story short—for the
        second time—all connector curves have rise
        
        $$
        \Large 1.6\cdot (c + {a\over 2})
        $$
        
        and we can annotate our sketch of the
        “generic” solution with this additional piece of
        information, if we want. (Well...

        |> Image
            src images/svg_ch4_one_long_line_with_rises_shown.svg

        ...there, no one can accuse us of not doing the
        homework ourselves.)


|> Exercise

    Apply the definition
    
    $$
    fg = (u \ra f(u)g(u))
    $$
    
    of function multiplication
    in order to show that

    $$
    (fg)h = f(gh)
    $$

    for all functions $f, g, h : \rr \ra \rr$,
    or, namely, to show that

    __($f$ times $g$) times $h$__

    equals

    __$f$ times ($g$ times $h$)__

    for all functions $f, g, h : \rr \ra \rr$.

    |> Solution

        It is necessary and sufficient to show that
        
        $$
        ((fg)h)(u)
        $$
        
        is the same as
        
        $$
        (f(gh))(u)
        $$
        
        for an arbitrary input $u \in \rr$, in order to
        show that 
        
        $$
        (fg)h
        $$
        
        and
        
        $$
        f(gh)
        $$
        
        are the same function. (Function equality is
        based on input-output behavior: two functions are
        equal if and only if every input is mapped to the
        same output under either function. See Note 6,
        Exercise 9, Chapter 3.)

        Starting up,
        
        $$
        ((fg)h)(u) = (fg)(u) \cdot h(u)
        $$
        
        by the definition of function multiplication,
        and
        
        $$
        (f(gh))(u) = f(u) \cdot (gh)(u)
        $$
        
        likewise. Moreover,
        
        $$
        (fg)(u) = f(u) \cdot g(u)
        $$
        
        and
        
        $$
        (gh)(u) = g(u) \cdot h(u)
        $$
        
        by the same definition again. Therefore,
        
        $$
        ((fg)h)(u) = (f(u) \cdot g(u)) \cdot h(u)
        $$
        
        on the one hand, and
        
        $$
        (f(gh))(u) = f(u) \cdot (g(u) \cdot h(u))
        $$
        
        on the other hand. But
        
        $$
        (f(u) \cdot g(u)) \cdot h(u) = f(u) \cdot (g(u) \cdot h(u))
        $$
        
        by the associativity of ordinary real number
        multiplication. (Not function multiplication:
        _real number multiplication_.) So
        
        $$
        ((fg)h)(u)
        $$
        
        equals
        
        $$
        (f(gh))(u)
        $$
        
        for arbitrary $u$, which completes the proof.

        |> Pause
        _Note 1._
        In words, we have just established the
        
        __associativity of function multiplication__
        
        while we had previously established the

        __associativity of function composition__

        (if you recall that one) in Exercise 9 of Chapter 
        3.

        |> Pause
        _Note 2._
        By this result, we can write
        
        $$
        fgh
        $$
        
        without any parentheses at all: 
        it doesn't matter whether we think of this 
        product as $(fg)h$ or $f(gh)$, the result 
        is the same.

|> Exercise

    Prove that
    
    $$
    f + g = g + f
    $$
    
    and that
    
    $$
    fg = gf
    $$
    
    for all $f, g : \rr \ra \rr$, using the fact that 
    
    $$
    a + b = b + a
    $$
    
    and that
    
    $$
    ab = ba
    $$
    
    for all $a, b \in \rr$. 
    (Prove something for functions by using the
    corresponding fact for numbers, namely.)

    |> Solution

        Given an arbitrary $u \in \rr$ we have
        
        $$
        (f + g)(u) = f(u) + g(u)
        $$
        
        and
        
        $$
        (g + f)(u) = g(u) + f(u)
        $$
        
        by the definition of function addition. But
        
        $$
        f(u) + g(u) = g(u) + f(u)
        $$
        
        by the commutativity of real number addition
        [$f(u)$ and $g(u)$ are both real numbers—the

        __commutativity__

        of real number addition is the fact that

        $$
        a + b = b + a
        $$

        for all real numbers $a$, $b$, mentioned in the
        statement—so we can use this here]; thus

        $$
        (f + g)(u) = (g + f)(u)
        $$

        for all $u \in \rr$, which implies

        $$
        f + g = g + f
        $$

        by definition of function equality. 

        For the second half we have, similarly,

        $$
        \begin{align}
        (fg)(u) &= f(u) \cdot g(u) \\
        &= g(u) \cdot f(u) \up{1.5}\\
        &= (gf)(u) \up{1.5}
        \end{align}
        $$

        for arbitrary $u\in \rr$, where the first and
        last equality are by the definition of a
        product of functions and where
        the middle equality is by commutativity of real
        number multiplication. [That would be the fact that

        $$
        ab = ba
        $$

        for all $a, b \in \rr$, as mentioned in the statement.] Hence

        $$
        fg
        $$

        and

        $$
        gf
        $$

        agree on an arbitrary input, hence $fg = gf$ by
        definition of function equality.



|> Exercise

    A rat is running a fundraising race. The function
    
    $$
    \Large f : \rr \ra \rr
    $$
    
    gives the amount raised as a function of position;
    specifically, ${f(x)}$ is the total number of 
    $\te{\$}$'s earned by virtue of running 
    $x$ meters from the start of the race; 
    a second function
    
    $$
    \Large g : \rr \ra \rr
    $$
    
    gives the position of the rat as a function of time;
    specifically, ${g(t)}$ is the position from the start,
    in meters, reached by the rat at $t$ seconds after the
    start of the race.

    In this case, what does $f \circ g$ compute?

    |> Solution

        It computes
        the amount earned by the rat as a function of time. In 
        more detail, $(f \circ g)(t)$
        is the number of $\te{\$}$'s earned by the rat at $t$
        seconds after the start of the race.

        |> Pause
        _Note 1._
        In even more detail,
        $$
        g(t)
        $$
        is the position in meters of the rat $t$
        seconds after start, by definition 
        of $g$, at which position the rat has
        earned
        $$
        f(g(t))
        $$
        $\te{\$}$'s in total, by definition of $f$.
        And
        $$
        f(g(t))
        $$
        is
        $$
        (f \circ g)(t)
        $$
        by definition of “$\circ$”.

        |> Pause
        _Note 2._
        If it helps, 
        here is a pictorialization of the 
        “units transformation pipeline” that 
        occurs inside $f \circ g$: 

        |> Image
            src images/svg_ch4_f_g_composition_units.svg

        |> Pause
        _Note 3._
        To emphasize, $f(x)$ is the

        __~ total ~__

        amount earned when position $x$ is reached.
        In real life $f$'s
        graph might therefore look something like this, 
        while inventing some numbers:

        |> Image
            src images/svg_ch4_f_g_first_example_of_f.svg

        In the above the rat earns $\te{\$}$3 for the 
        first 50m, after which the dollar-per-meter 
        rate is reduced.
        Or 
        $f$
        could look like this,
        with discrete “unlocks”: 

        |> Image
            src images/svg_ch4_f_g_second_example_of_f.svg

        In our mathematicians' imaginations, however, $f$
        might also look like this, nice and differentiable,
        with periods of negative slope (corresponding to
        parts of the course that momentarily lose you money):

        |> Image
            src images/svg_ch4_f_g_third_example_of_f.svg

        (Etc.)

|> Exercise

    What does $(f \circ g)'$ compute,
    keeping the same setup as in Exercise 34?

    |> Solution

        It computes the dollars-per-second earnings rate 
        as a function of time. In full detail,

        $$
        (f \circ g)'(t)
        $$

        is the dollars-per-second rate which the rat is
        fundraising at $t$ seconds after the start of the race.

        _Note 1._
        You don't need to know anything about “$f$”
        or “$g$” to answer this question. You only need to
        know what “$f \circ g$” is. 
