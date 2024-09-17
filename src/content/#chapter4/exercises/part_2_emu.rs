|> Exercise

    Complete the missing units for each strip
    below, based on those units that are given:

    |> Image
        src images/svg_ch4_missing_units.svg

    |> Solution

        The pattern to respect is that, each time 
        you take a derivative, the units on the $x$
        axis stay the same, while the units on the
        $y$ axis become divided by those on the $x$ 
        axis. This gives the unique solutions:

        |> Image
            src images/svg_ch4_missing_units_solution.svg

        |> Pause
        _Note 1._
        A unit of “$1$” is a
        
        __dimensionless__
        
        unit. Dimensionless units arise when 
        quantities are divided by like quantities.
        Think of dimensionless quantities as “pure 
        fractions” or “pure ratios”. (Percentages
        are dimensionless—in fact the term
        
        __percentage__
        
        is synonymous with

        __dimensionless ratio__
    
        though if you spoke to people about 
        “dimensionless ratios” they would look at 
        you funny. Also percentages are a system of
        notation, whereby the symbol “%” means 
        “divide the preceding number by 100, in 
        order to discover the numerical value of 
        the ratio I'm talking about”.) (To drive
        it home: In Chinese, the written expressions 
        “$23\%$” and “$23/100$” are 
        indistinguishable when read out loud; they
        are both  read “$23$ over $100$”; that is
        the simple _&_ correct way!)

|> Exercise

    Among the functions below, which is the
    zeroth, first, and second derivative? 
    (I.e., which is $f$, $f'$, and $f''$, 
    assuming that relationship exists.)

    |> Image
        src images/svg_ch4_position_by_time_find_the_order.svg

    |> Solution

        The graphs are already in the right order 
        (hehe): if “$f$” is the original function 
        then $f$ is on the left, $f'$ is in the 
        middle, and $f''$ is on the right:

        |> Image
            src images/svg_ch4_position_by_time_find_the_order_answer.svg

        For example, the graph on the left 
        has a slope that starts at $\sim\!-1$ and
        ends at $\sim\!1$, while those are the
        values at which the graph in the middle
        starts and ends (and not coincidentally,
        since the graph in the middle is the
        derivative of the graph on the left!):

        |> Image
            src images/svg_ch4_position_by_time_find_the_order_first_pair.svg

        Moreover the middle graph has slope
        close to $0$ at either end, and some
        slope near $1.5$ or $2$ towards the middle,
        matching the _values_ of the graph
        on the right:

        |> Image
            src images/svg_ch4_position_by_time_find_the_order_second_pair.svg

        (Taking one more derivative would produce a
        zigzag, by the way.)

        |> ImageRight
            src images/svg_ch4_position_by_time_find_the_order_cloud.svg


|> Exercise

    Given these graphs...

    |> Image
        src images/svg_ch4_sketch_the_middle_a.svg

    ...what can you say about $g'(x)$?
    (Produce the best sketch of $g'(x)$ that
    you can, taking into account all the
    information above.) (Don't get us wrong:
    You don't _need_ the second derivative
    to sketch the first derivative, but if
    you're a human and not a machine, it can
    help!)

    |> Solution

        To start with, the slope of $g$ seems to be
        about $-1.5$, $0$ and (a bit greater than) $2$
        at $x = -2$, $x = 0$ and $x = 2$ respectively:

        |> Image
            src images/svg_ch4_sketch_the_middle_a_sol1.svg

        This already gives us three points from which
        to interpolate a basic approximation to the graph 
        $y = g'(x)$:

        |> Image
            src images/svg_ch4_sketch_the_middle_a_sol2.svg

        But the graph of $g''(x)$ indicates more,
        namely that $g'(x)$ has a slope that rises 
        from $\approx 0.2$ near the left edge of 
        the graph up to $1.3$ at $x = 0.5$, before 
        falling again to $0.6$ past $x = 2$:

        |> Image
            src images/svg_ch4_sketch_the_middle_a_sol3.svg

        As a second step, we thus “bend into shape”
        our previous sketch to produce these slopes...

        |> Image
            src images/svg_ch4_sketch_the_middle_a_sol4.svg

        ...achieving our final answer. 

        |> Pause

        _Note 1._ For reference, the actual 
        derivative looks like so:

        |> Image
            src images/svg_ch4_sketch_the_middle_a_sol5.svg


|> Exercise

    Given these graphs...

    |> Image
        src images/svg_ch4_sketch_the_middle_b.svg

    ...sketch $y = h'(x)$, analogously to
    Exercise 10.

    |> Solution
        
        Firstly, the graph of $h(x)$ seems to have
        slope $0$ around $x = 0.6$...

        |> Image
            src images/svg_ch4_sketch_the_middle_b_sol1.svg

        ...which gives us one data point on the
        curve $y = h'(x)$ to start with...

        |> Image
            src images/svg_ch4_sketch_the_middle_b_sol2.svg

        ...moreover, by the graph of $h''(x)$,
        the slope of $h'(x)$ is near $-1/3$ on an
        interval that is approximately (say)
        $[-0.85,0.7]$....

        |> Image
            src images/svg_ch4_sketch_the_middle_b_sol3.svg

        ...so, as a second step, we can extend
        the graph of $h'(x)$ by a segment of slope
        $-1/3$ on this interval:

        |> Image
            src images/svg_ch4_sketch_the_middle_b_sol4.svg

        |> ImageLeft
            src images/svg_ch4_sketch_the_middle_b_cloud.svg

        (To achieve a passable slope of $-1/3$ we
        modeled ourselves on a nearby grid segment.) 
        Next, $h(x)$ has slope $\approx 1.2$ at
        $x = -2$, and slope $\approx -0.9$ (?) at
        $x = 2$:

        |> Image
            src images/svg_ch4_sketch_the_middle_b_sol5.svg

        This gives us two more points on the graph
        $y = h'(x)$:

        |> Image
            src images/svg_ch4_sketch_the_middle_b_sol6.svg

        Then, because the second derivative has
        value $\approx -1/3$ for $x \leq -1.6$
        (about) and for $x \geq 1.5$ (about)...

        |> Image
            src images/svg_ch4_sketch_the_middle_b_sol7.svg

        ...we extend these two new data points by
        segments of slope $-1/3$...

        |> Image
            src images/svg_ch4_sketch_the_middle_b_sol8.svg

        ...on the relevant intervals. (I.e., for 
        $x \leq -1.6$ and for $x \geq 1.5$.) The 
        last step is to join the existing segments 
        by some kind of “connector curves” of 
        yet-to-be-determined shape:

        |> Image
            src images/svg_ch4_sketch_the_middle_b_sol9.svg

        Since $h''(x)$ shows that the two 
        connectors have slopes of about $-1/3$
        at their edges and slopes of about $-1.4$
        and $-1.6$ (respectively) near their
        middles...

        |> Image
            src images/svg_ch4_sketch_the_middle_b_sol10.svg

        ...our final answer, given by the
        following sketch, is obtained by 
        “bending into shape” the connector curves...

        |> Image
            src images/svg_ch4_sketch_the_middle_b_sol11.svg

        ...to give them a slope of $-1/3$ at
        their endpoints, and slopes of $-1.4$, 
        $-1.6$, respectively, in their middles.

        |> Pause

        _Note 1._
        Here is the actual graph of $h'$:

        |> Image
            src images/svg_ch4_sketch_the_middle_b_sol12.svg


|> Exercise

    If you scale the graph of a function $f$
    vertically by a factor $2$—i.e., multiply
    each output by $2$—is the derivative 
    also scaled by $2$?

    |> Solution

        Yes, this is the case. For a joke way
        of seeing it, here is a graph of a 
        putative function $f$, before and after
        scaling:

        |> Image
            src images/svg_ch4_scaling_figure.svg

        The second graph truly is the first
        graph vertically scaled by a factor $2$,
        because the scale on the $y$ axis has
        been doubled. This means that the ratio

        $$
        {\te{rise}\over \te{run}}
        $$

        has doubled in the second graph, because
        “rise” has doubled (each $y$-coordinate
        is twice as large!), whereas “run” stays
        the same. (So the slope of the tangent has
        doubled, so the derivative is doubled.)