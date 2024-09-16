|> Exercise

    Sketch the derivative of a function with the
    following graph (what _looks_ like a sharp corner
    _is_ a sharp corner):

    |> Image
        src images/svg_ch4_slope_one_half_see_saw.svg
    
    |> Solution

        That would be:

        |> Image
            src images/svg_ch4_slope_one_half_see_saw_derivative.svg

        (The derivative is $1/2$ when the slope is $1/2$,
        is $-1/2$ when the slope is $-1/2$, and is undefined at
        the corners.)

|> Exercise

    Would the derivative of
    
    $$
    y = {1\over x}
    $$
    
    be a very large negative number, or a very
    large positive number, near $x = 0$? Or would
    it depend on which side of 0 you are?

    |> Solution

        The graph of $y = {1 \over x}$ looks like so:

        |> Image
            src images/svg_ch4_one_over_x.svg
    
        As one can see, the slope is very negative
        near $x = 0$, on either side. So the answer is:
        “very large negative”.

|> Exercise

    Sketch the
    
    __second__
    
    derivative of the graph in Exercise 1.
    
    |> Solution

        The second derivative is zero wherever the
        first derivative is flat, and is undefined
        wherever the first derivative is undefined;
        this gives the second derivative the following 
        pockmarked appearance:

        |> Image
            src images/svg_ch4_slope_one_half_see_saw_second_derivative.svg

        |> Pause

        _Note 1._
        Taking even further derivatives produces
        the same graph back, over and over again.

        |> Pause

        _Note 2._
        “first derivative” is a synonym of “derivative”. 


|> Exercise

    If we pretend that the graph of Exercise 1
    depicts the 
    |> del
    
        distance that a car has traveled
        as a function of time,
    &ensp;position of a car as a function of 
    time, with hours (hr) on the $x$-axis 
    and kilometers (km) on the $y$-axis, what
    do the units become on the axes of the first
    and second derivatives?

    |> Solution

        The units on the $y$ axis become kilometers,
        kilometers per hour, and kilometers per
        hours squared (or “kilometers per hour, per
        hour”), including the first graph. (Each time
        another derivative is taken, divide the
        units of the $y$ axis by the units of the 
        $x$ axis.) These are the position, velocity,
        and acceleration of the car as a function
        of time:

        |> Image
            src images/svg_ch4_slope_one_half_see_saw_with_units.svg

        |> Image
            src images/svg_ch4_slope_one_half_see_saw_with_units_derivative.svg

        |> Image
            src images/svg_ch4_slope_one_half_see_saw_with_units_second_derivative.svg

|> Exercise

    Is the following equation correct, incorrect,
    or nonsensical?
    
    $$
    (x \ra x + 1) \,+\, (u \ra 2u + 1) \,=\, (t \ra 3t + 2)
    $$

    |> Solution

        The equation is true!
        Syntatically,

        $$
        (x \ra x + 1) \,+\, (u \ra 2u + 1)
        $$

        is a
        
        __sum of functions__
    
        because $x \ra x + 1$ and $u \ra 2u + 1$ are
        both functions. Now by definition, the sum
        
        $$
        f + g
        $$
        
        of functions $f$ and $g$ is the function
        
        $$
        x \ra f(x) + g(x)
        $$
        
        that maps a number to the sum of the individual
        values of the functions. So—for example—
        
        $$
        \begin{align}
        & \,\,\,((x \ra x + 1) \,+\, (u \ra 2u + 1))(5) \\
        =& \,\,\,(x \ra x + 1)(5) + (u \ra 2u + 1)(5) \up{1.5} \\
        =& \,\,\,(5 + 1) + (2\cdot 5 + 1) \up{1.5} \\
        =& \,\,\,3\cdot 5 + 2 = 17 \up{1.5}
        \end{align}
        $$
        
        and—with a general input $t$—
        
        $$
        \begin{align}
        & \,\,\,((x \ra x + 1) \,+\, (u \ra 2u + 1))(t) \\
        =& \,\,\,(x \ra x + 1)(t) + (u \ra 2u + 1)(t) \up{1.5} \\
        =& \,\,\,(t + 1) + (2t + 1) \up{1.5} \\
        =& \,\,\,3t + 2 \up{1.5}
        \end{align}
        $$
        
        which implies that, indeed,
        
        $$
        (x \ra x + 1) \,+\, (u \ra 2u + 1)
        $$
        
        is the function that maps each real number $t$ to $3t + 2$,
        i.e., is equal to the function $t \ra 3t + 2$. (!!)

        |> Pause

        _Note 1._
        One can also do the main computation with $x$
        in place of $t$:

        $$
        \begin{align}
        & \,\,\,((x \ra x + 1) \,+\, (u \ra 2u + 1))(x) \\
        =& \,\,\,(x \ra x + 1)(x) + (u \ra 2u + 1)(x) \up{1.5} \\
        =& \,\,\,(x + 1) + (2x + 1) \up{1.5} \\
        =& \,\,\,3x + 2 \up{1.5}
        \end{align}
        $$

        Here we have two different $x$'s: the $x$ that
        denotes the input, and the $x$ that is used as
        a placeholder to describe how the first function
        acts.


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


|> Exercise

    Where is the rate of change of the function
    below, on the part shown, greatest? And
    where is the

    __rate of change of the rate of change__

    greatest?

    |> Image
        src images/svg_ch4_narrow_and_less_narrow_bends.svg

    |> Solution

        The rate of change is the slope, which
        is greatest along the right-hand portion
        of the curve:

        |> Image
            src images/svg_ch4_narrow_and_less_narrow_bends_steepest.svg

        On the other hand,
        
        __the rate of change of the rate of change__
        
        [a.k.a., second derivative] is the rate
        of change  of the slope, and that will
        be greatest at the first bend of the curve,
        where the slope is changing at the fastest 
        rate:

        |> Image
            src images/svg_ch4_narrow_and_less_narrow_bends_curviest.svg

        (Well, believe us or not, but we're right!)

|> Exercise

    In the following graph, which curve might 
    be a derivative of which other curve?

    |> Image
        src images/svg_ch4_cosine_curves.svg

    |> Solution

        As it happens—and by the exact method
        that we used to generate these curves—the
        blue is the derivative of the red:

        |> Image
            src images/svg_ch4_cosine_curves_blue_red_only.svg

        Likewise, the derivative of the blue
        is the yellow, the derivative of the yellow
        is the green, and the derivative of the 
        green is the red, at which point it starts
        all over again! (For example, the 
        _fifth_ derivative of the red curve
        is the blue, because the _fourth_
        derivative of the red curve is itself.)

        |> Pause

        _Note 1._
        After all, the slope of these curves
        keeps oscillating between two fixed 
        values—the “most slanted up” and the 
        “most slanted down”—so their derivatives
        were always going to have an oscillatory
        pattern, as well.

        |> Pause

        _Note 2._
        Because “most slanted up” occurs when
        the curve has not yet crested, but when
        the derivative _is already_ in the
        process of cresting (that's why it's
        “most slanted up”), the derivative is
        ahead of the original curve by half a
        bump, not the other way around:

        |> Image
            src images/svg_ch4_cosine_curves_cresting.svg

        |> Pause

        _Note 3._
        When we examine the velocity of a
        particle moving in the plane, we examine
        the velocities of its shadow on the
        $x$- and $y$-axes:

        |> Image
            src images/svg_ch4_cosine_curves_two_dimensional_vel.svg

        The velocities of the two shadows
        encode the overall “two-dimensional”
        velocity of the particle. (No need for 
        quotes, really: the velocity _is_ 
        two-dimensional.) 

        Here's another point of view: just like

        __position__
        is encoded by a pair of numbers—sometimes
        known as the _position vector_ by
        the way, where “vector” is a term of art
        for “pair of numbers”—so the
        
        __velocity__
        
        is encoded by a pair of numbers—equally
        known as the _velocity vector_—which
        is no coincidence, because the first
        coordinate of the
        
        __velocity vector__
        
        is the derivative of the first coordinate
        of the
        
        __position vector__
        
        and likewise for the second coordinate—two
        coordinates, two rates of change!

        Geometrically, if we use the $x$- and
        $y$-components $v_x$ and $v_y$ of the 
        velocity to draw an arrow emanating 
        from a point on the curve, this arrow
        is tangent to the curve, and the

        |> ImageRight
            src images/svg_ch4_cosine_curves_tangent_velocity_cloud.svg

        __length__
        
        of the arrow is the
        
        __speed__
        
        of the particle at that moment in time.
        More precisely, if you let the particle
        drift at the exact same $x$- and 
        $y$-velocities $v_x$ and $v_y$ that you
        measured at the root of the arrow for
        one unit of time, the particle would 
        cover exactly the length of the arrow
        in that one unit of time, no more no less, 
        because the particle would cover $v_x$
        units in $x$ and $v_y$ units in $y$.
        And speed being
        
        __distance per unit time__
        
        the length of the arrow is, therefore,
        the speed!

        Now consider not one but four particles,
        going around a unit circle in clockwise
        fashion, 90° apart in phase, at unit 
        speed (“unit speed” = speed 1, “unit 
        circle” = radius 1) (ps: We center the
        circle at the origin):

        |> Image
            src images/svg_ch4_cosine_curves_unit_circle.svg

        The

        __position vectors__
        
        of the particles are as follows:

        |> Image
            src images/svg_ch4_cosine_curves_unit_circle_position_vectors.svg

        (You can't really see it so well, but
        each arrow originates at $(0, 0)$.) While
        the

        __velocity vectors__

        are as follows:

        |> Image
            src images/svg_ch4_cosine_curves_unit_circle_velocity_vectors.svg

        (Like the position vectors, the 
        velocity vectors keep changing instant by 
        instant—this is the subtlety of calculus!)
        The velocity vectors have length $1$ because
        the speed is $1$, _&_ are brushed
        in the direction of travel.

        (Nb: When we draw a vector as an arrow
        we mean that the first coordinate of the
        vector is equal to the horizontal 
        displacement from the tail of the arrow
        to the head of the arrow, and likewise
        that the second coordinate is equal to the
        vertical displacement from the tail of the
        arrow to the head of the arrow.)

        |> ImageRight
            src images/svg_ch4_cosine_curves_vector_illustration_cloud.svg

        Due to the 90° rotations and uniform 
        lengths of $1$, one particle's velocity
        vector is another particle's position
        vector; as one example, the red particle's
        velocity vector is the blue particle's 
        position vector:

        |> Image
            src images/svg_ch4_cosine_curves_unit_circle_equality.svg

        From the $x$-coordinates, for example,

        __the velocity in $x$ of the red particle
        is the position in $x$ of the blue particle__

        at any given moment in time. This also
        means:

        __the rate of change of the $x$-coordinate
        of the red particle is the $x$-coordinate
        of the blue particle__

        ...because “velocity in $x$” is the same
        as “rate of change of the $x$-coordinate”.

        Concretely, if you graph the
        $x$-coordinates of the red and blue 
        particles on the same graph, the rate of
        change of the red particle's $x$-coordinate
        will equal the value of the blue particle's
        $x$-coordinate. These are the reds and blue
        curves from the problem statement, if we
        start the red particle at position

        $$
        (1, 0)
        $$

        at time $t = 0$:

        |> Image
            src images/svg_ch4_cosine_curves_verified.svg

        If we add the $x$-coordinates of the green
        and yellow particles, we find the graph
        from the problem statement!

        |> Pause

        _Note 4._
        If needed, here is an illustration of 
        one $360^\circ$ rotation of the particles
        of Note 3,  with each curve being an 
        $x$-coordinate:

        |> Image
            src images/svg_ch4_cosine_curves_rolling_wheel.svg

        (If the above just looks like a 
        confusing mess then don't sweat it—it's
        not that important.)

        |> Pause

        _Note 5._
        To reiterate, take a look at this
        figure again:

        |> Image
            src images/svg_ch4_cosine_curves_cresting.svg

        The derivative is

        __ahead__

        of the red particle, so that 
        $x$-coordinates you see _now_ on the
        blue particle will be seen _a little later_
        on the red particle! (In particular, if you
        wanted to generate the same curves with a
        counterclockwise rotation, you could do that,
        but you would have to reverse the order of
        the particles around the circle to keep the
        blue particle ahead of the red particle,
        the yellow particle ahead of the blue 
        particle, etc.)


|> Exercise

    How can we generate the following set of
    curves by rotating points around a circle,
    and tracking their $x$-coordinates? (This
    graph is an exact $2$&#x200b;$\times$ [“two
    x”] vertical dilation of the graph in Exercise
    12.) Should we use a circle of radius $2$,
    or make the points go twice as fast? Or both?
    Or something else yet?

    |> Image
        src images/svg_ch4_scaled_cosine_curves.svg

    |> Solution

        The values oscillate between $+2$ and
        $-2$, so we need a circle of radius $2$
        to generate these curves. Also the
        values go through one cycle in the same 
        amount of time as the particles of
        Exercise 12, but the circle has twice the 
        circumference (having twice the radius),
        so the particles are going twice as fast!
        (I.e.: speed 2, since the particles of
        Exercise 12 have unit speed.)

        |> Pause

        _Note 1._
        In this and in the previous exercise the
        units of time and distance are “anonymous”: 
        distance could be meters, kilometers, or
        anything, and time could be seconds, hours,
        etc—it doesn't matter. However, one should
        be aware that what amounts to
        
        __unit speed__
        
        under one set of units is no longer “unit
        speed” under a different set of units—this
        is not a “physical” property of the 
        particles, but, rather, a “mathematical”
        property that holds only for one specific
        “tweaking” of the units.


|> Exercise

    Exercise 12 exhibits a function $f$—in fact,
    four different functions $f$—such that

    $$
    f' \ne f
    $$

    and

    $$
    f'' \ne f
    $$

    and

    $$
    f''' \ne f
    $$

    but

    $$
    f^{(4)} = f
    $$

    surprise, surprise! Can you do the same with
    “$5$” instead of “$4$”? I.e., find a function
    $f$ such that

    $$
    f^{(n)} \ne f
    $$

    for $n = 1, 2, 3, 4$ but

    $$
    f^{(5)} = f
    $$

    ...?

    |> Solution

        We can naïvely try to imitiate how the
        curves of Exercise 16 are generated by
        placing five equally spaced particles
        around the unit circle (“the” unit circle
        is the one centered at $(0, 0)$, by
        convention), instead of 4:

        |> Image
            src images/svg_ch4_5_euler_position_vectors.svg

        The idea would be that the

        __velocity vector__

        of the red particle is the

        __position vector__

        of the blue particle, 
        likewise for the blue and yellow particles,
        and so on. (Position vectors shown above.)
        For example, at the instant above, the
        velocity vectors would be as follows:

        |> Image
            src images/svg_ch4_5_euler_velocity_vectors.svg

        The velocity vectors are

        -|NOT|-

        tangent to the unit circle, and so the
        particles will leave the circle!
        (But that's OK.) In one-tenth a unit
        of time, for example, the particles would
        travel approximately one-tenth their
        velocity vectors, that would bring them
        to approximately these new positions:

        |> Image
            src images/svg_ch4_5_euler_after_1_10th.svg

        In the next one-tenth unit of time we 
        can apply a similar approximation again,
        advancing the particles by ${1\over 10}$th
        of [the current approximation to] their
        velocity vectors. Skipping the construction
        lines:

        |> Image
            src images/svg_ch4_5_euler_after_2_10th.svg

        Applying the same process for $8$ more
        steps:

        |> Image
            src images/svg_ch4_5_euler_after_10_10th.svg

        To be clear, in the above figure, the
        position of the red particle at, say, the
        fifth step...

        |> Image
            src images/svg_ch4_5_euler_after_5_10th.svg

        ...is obtained by starting from the red
        particle's position at the fourth step...

        |> Image
            src images/svg_ch4_5_euler_after_4_10th.svg

        ...and adding one-tenth of the approximation
        that we have to the red particle's velocity
        vector at that moment, that approximation
        being namely the blue particle's position 
        vector at the fourth step ($t = {4\over 10}$)...

        |> Image
            src images/svg_ch4_5_euler_after_4_10th_b.svg

        ...and we do the same for each particle,
        to advance to the next step.

        If we stop $10$ times as often,
        advancing the clock by ${1\over 100}$th of
        a unit of time at each step, the same figure
        becomes just a blur (still going from
        $t = 0$ to $t = 1$):

        |> Image
            src images/svg_ch4_5_euler_after_100_100th_blur.svg

        To visualize such a fine-grained
        approximation we need to revert to drawing
        the particles as points. In the following
        figure the colored paths are points that
        come from a “${1\over 100}$th” approximation,
        while the orange dots are the old positions
        obtained from a “${1\over 10}$th”
        approximation:

        |> Image
            src images/svg_ch4_5_euler_after_100_100th_points.svg

        Zooming in a bit (or else we still can't
        see anything):

        |> Image
            src images/svg_ch4_5_euler_after_100_100th_zoomed.svg

        In any case, even the “${1\over 100}$th”
        approximation is just an approximation,
        but the point is that such approximations
        do converge to a set of “true” particle
        paths, as pictured in Fig$.$ 1, that can
        be computed by some wizards; as time can
        be played forward or backward, these paths
        form doubly-infinite spirals—in to infinity,
        out to infinity.

        |> ImageLeft
            offset_x 1.5em
            src images/svg_ch4_5_euler_spiral_figure.svg
            children_x 50%
            children_y 2em

            |> span
                font-size 1.4em

                Fig. 1

        In any case [take two] the point is that
        whether or not you are one of the wizards,
        you can

        __guess__

        the existence of these five paths—sort 
        of “feel” that they exist! (This is a 
        moral consolation prize, at least.)

        We can also convert the paths into a 
        function

        $$
        f
        $$

        that satisfies the problem requirements.

        For example let $f$ be the function that,
        given a time $t$, outputs the $x$-coordinate
        of the red particle at $t$; then, to spell
        it all out, since

        __the rate of change of the 
        $x$-coordinate of the red particle is the 
        $x$-coordinate of the blue particle__

        $f'$ is the $x$-coordinate of the blue 
        particle; and since 

        __the rate of change of the 
        $x$-coordinate of the blue particle is the
        $x$-coordinate of the yellow particle__

        $f''$ is the $x$-coordinate of the
        yellow particle; and since

        __the rate of change of the
        $x$-coordinate of the yellow particle is the
        $x$-coordinate of the green particle__

        $f'''$ is the $x$-coordinate of the
        green particle; and since

        __the rate of change of the 
        $x$-coordinate of the green particle
        is the $x$-coordinate of the purple
        particle__

        $f''''$ is the $x$-coordinate of the
        purple particle; and since

        __the rate of change of the 
        $x$-coordinate of the purple particle is the 
        $x$-coordinate of the red particle__

        $f''''' = f^{(5)}$ equals $f$.

        |> Pause
        _Note 1._
        If you graph the $x$-coordinates of the
        5 particles over time, each in their 
        color, you get a graph like so, in which
        blue is the derivative of red, yellow is
        the derivative of blue, etc; the function 
        $f$ can be taken to be any one of these
        curves:

        |> Image
            src images/svg_ch4_5_euler_final_graph.svg

        |> Pause
        _Note 2._
        There is nothing special about 
        $x$-coordinates vis-à-vis $y$-coordinates.
        You can also define $f(t)$ to be, e.g.,
        the $y$-coordinate of the red particle
        at time $t$.

        |> Pause
        _Note 3._
        It is worth noting that, in fact, the 
        $x$- and $y$-coordinates live separate
        lives. The rate of change of each 
        $x$-coordinate is some other $x$-coordinate, 
        and the rate of change of each $y$-coordinate
        is some other $y$-coordinate.

        |> Pause
        _Note 4._
        Adding to this observation, we don't
        _need_ to start the particles in
        a symmetric configuration. Symmetry only
        helps to picture how the positions of the
        particles will evolve without making any
        computations. We also don't _need_
        to work in two dimensions. We can place
        the particles in a one-dimensional world,
        e.g., ...

        |> Image
            src images/svg_ch4_5_euler_one_dimensional.svg

        ...(the initial positions really don't
        matter much, as long as you don't give 
        all the particles the _same_ initial
        position, or else you won't have $f \ne f'$
        etc) and stipulate the same rules, namely
        that the 
        
        __velocity__
        
        (now $1$-dimensional) of the red particle
        be the
        
        __position__
        
        (now $1$-dimensional) of the blue particle
        and so on—you can “release” the particles
        from their initial configuration and
        simulate (or compute exactly, if you have
        the know-how) their motion by the same
        methods as above. The five position
        functions obtained are each a solution
        $f$ to the problem. (But this solution will
        typically look more chaotic than the curves
        from Note 1.) 

        |> Pause
        _Note 5._
        In fact, our symmetric two-dimensional
        solution is an instance in which you can say
        that

        __the whole is simpler than the parts__

        |> ImageLeft
            src images/svg_ch4_5_euler_hearts_left.svg

        |> ImageRight
            src images/svg_ch4_5_euler_hearts_right.svg

        in that you would never spot the symmetry
        at play, or have a chance of eyeballing
        the long-term evolution of the system, if
        you were shown just the $x$-coordinates,
        or just the $y$-coordinates, on their own!


|> Exercise

    If we seek a function $f : \rr \ra \rr$
    such that

    $$
    f^{(17)} = f
    $$

    and such that $f \ne 0$ (or: $f \ne (x \ra 0)$,
    pedantically) and such that $f$ grows relatively
    slowly in either the positive or negative direction
    of the number line, insofar as such things are 
    concerned, what would our options be? 

    |> Solution

        Take $17$ particles equally spaced out along
        the unit circle, such as these (shown here 
        with position vectors):

        |> Image
            src images/svg_ch4_17_position_vectors.svg

        Set the velocity of particle
        $$
        {\Large 1}
        $$
        equal to the position of particle
        $$
        {\Large 5}
        $$
        and keep going by this pattern, making the
        velocity of each particle equal to the
        position of the particle that is $4$ later;
        in the configuration above, the velocity
        vectors end up looking like so, for example:

        |> Image
            src images/svg_ch4_17_velocity_vector_1.svg

        |> ImageRight
            src images/svg_ch4_17_velocity_vector_1_cloud.svg

        Maintaining this relationship at all 
        points in time, and given that the velocity 
        vectors point very slightly outward from
        the unit circle, and because all the
        symmetry and all the angles are maintained
        as we play time forward or backward, the
        particles spiral gently outward/inward from
        the circle for time forward/backward,
        respectively. Taking $f(t)$ to be the $x$- 
        or $y$-coordinate of any one of the particles
        (e.g., particle $1$) at time $t$ gives an
        oscillating function whose $17$th derivative
        is itself (because the rate of change of
        the $x$-coordinate of particle $1$ is the
        $x$-coordinate of particle $5$, etc, until
        we make it all the way back to particle $1$),
        and that grows comparatively slowly over
        time. ~The End~

        |> Pause
        _Note 1._
        In case you're curious, the actual spiral paths
        of the particles look like so:

        |> Image
            src images/svg_ch4_17_paths.svg

        ...and if you take the $x$-coordinates of
        the particles over time, with time $t = 0$
        corresponding to the original configuration
        depicted where particle 1 is at $(1, 0)$, you
        find 
        |> del

            paths
        &ensp;functions that look like so:

        |> Image
            src images/svg_ch4_17_actual_functions.svg

        For example, the derivative of curve 
        |> InlineImage
            src images/svg_ch4_17_particle1_inline.svg
        , highlighted below in red, is curve 
        |> InlineImage
            src images/svg_ch4_17_particle5_inline.svg
        , highlighted in blue:

        |> Image
            src images/svg_ch4_17_actual_functions_with_highlights.svg

        ...and taking sixteen more derivatives 
        starting from curve 
        |> InlineImage
            src images/svg_ch4_17_particle5_inline.svg
            style margin-right:0.8em
        we would go through curves 
        |> InlineImage
            src images/svg_ch4_17_particle9_inline.svg
        , 
        |> InlineImage
            src images/svg_ch4_17_particle13_inline.svg
        , 
        |> InlineImage
            src images/svg_ch4_17_particle17_inline.svg
        , 
        |> InlineImage
            src images/svg_ch4_17_particle4_inline.svg
        , ..., 
        |> InlineImage
            src images/svg_ch4_17_particle14_inline.svg
        before finally coming back to curve
        |> InlineImage
            src images/svg_ch4_17_particle1_inline.svg
            style margin-right:0.1em
        !

        |> Pause
        _Note 2._
        It can be interesting to examine what
        goes wrong if we attempt to make the 
        velocity vectors even more tangent to the
        unit circle. For example, if we start the
        particles so that particle 5 is at
        $90^\circ$ exactly from particle 1, 
        particle 9 is at $90^\circ$ exactly from
        particle 5, and so on, until we reach
        particle 14, the last particle in this
        order; then we have the following starting
        configuration:

        |> Image
            src images/svg_ch4_17_other_start.svg

        To parse the above figure, understand
        that:

        |> ul
            style margin:20px 20px 0px 50px

            |> li
            
                the red arrows indicate which particle
                takes its velocity from the position
                of which other particle; for example,
                particle 1 has velocity equal to the
                position of particle 5
            
            |> li

                particles that occupy the same starting
                position on the unit circle appear
                stacked together, as a representation
                device; for example, particle 2 has the
                same starting position as particle 5

        (Note that the red arrows have to form
        a cycle of length 17 in order for us to
        later extract a function $f$ such that 

        $$
        f^{(17)} = f
        $$

        but this is the case: the red arrows only
        “close the loop” after going through all
        17 particles!)

        In this starting configuration, all
        velocity vectors are exactly tangent to
        the unit circle

        -|EXCEPT|-
        
        for particle 14, whose velocity vector, 
        being the position of particle 1, is straight
        out from the circle! So, as we “start time”,
        particle 14 will push out from the circle,
        that will in turn affect particle 10, and
        so on, until all particles end up being
        “peeled off” from the circle, in due time;
        if you are so curious, the particle
        trajectories end up like so (shown only for
        $t \geq 0$):

        |> Image
            src images/svg_ch4_17_other_start_paths.svg

        The particles shoot of to infinity in short
        order—the solution is much worse—for fun we
        have also highlighted two particle trajectories
        in this figure:

        |> ul
            style margin:1em 1em 0em 2.5em

            |> li

                in 
                |> span
                    style border:1px solid blue

                    blue
                , particle 1, the last particle
                to be (noticeably*) “peeled off” from
                the circle (*all particles are 
                instantaneously peeled off from the 
                circle to _some_ degree, as one 
                particle's slight deviation affects
                the next, that affects the next, etc)

            |> li

                in 
                |> span
                    style border:1px solid red
                    
                    red
                , particle 14, the _first_ particle
                to leave the circle—but because its
                velocity vector is given by particle 1,
                which itself starts by going around in
                a circle, it, too, starts out by going
                around in a circle!

        (The point is: if your velocity vector 
        is tracing a circle centered at $(0, 0)$—at 
        a uniform rate—then you, too, are going
        around in a circle—it's just that your
        circle could be centered anywhere, not
        necessarily at $(0, 0)$!)


|> Exercise

    Add elements to the following drawing...

    |> Image
        src images/svg_ch4_newton_quotient_unfinished.svg

    ...such that it becomes a “complete”
    illustration of this here algebraic 
    expression...

    $$
    {f(x+h) - f(x) \over h}
    $$

    ...and reveal the “geometric meaning” of
    the expression, if any.

    |> Solution

        This version pictures all the elements
        that appear in the fraction:

        |> Image
            src images/svg_ch4_newton_quotient_finished.svg

        The point is: the fraction

        $$
        {f(x + h) - f(x)\over h}
        $$

        is seen to have the form _rise over run_, 
        and is more precisely equal to the slope
        of the pale brown line going through the
        point 

        $$
        (x, f(x))
        $$
        |> ImageLeft
            src images/svg_ch4_new_quatient_x_fx_cloud.svg

        at one end, and

        $$
        (x + h, f(x + h))
        $$

        |> ImageRight
            offset_y -0.5em
            src images/svg_ch4_new_quatient_x_fxplush_cloud.svg

        at the other end. (This is also the case
        if $h$ is negative, by the way.)

        |> Pause
        _Note 1._ 
        A fraction of this form is called a _Newton
        quotient_.
    
        |> Pause
        _Note 2._
        The pale brown line is sometimes known as
        the _secant_ [through $(x, f(x))$, $(x+h, f(x+h))$].
        “Secant” is a general term for “line passing
        through two specified points on another
        curve”.
    
        |> Pause
        _Note 3._
        If we let $h$ drop to $0$, and if $f$ is
        differentiable at $x$, the 
        Newton-quotient-a.k.a.-slope-of-the-secant
        approaches
        
        $$
        f'(x)
        $$ 
        
        because the secant approaches the tangent,
        in that case, and the slope of the secant
        is also, perforce, approaching the slope
        of the tangent, which is $f'(x)$. (But you
        cannot directly set $h = 0$, because
        
        $$
        {f(x+0)-f(x)\over 0} = {0 \over 0}
        $$
        
        is undefined.)


|> Exercise

    In this exercise we consider two points
    in time $t_0$ and $t_0 + \Delta{}t$ (here
    “$\Delta{}t$”, read “delta $t$”, is a
    standard notation for a small amount of
    time):

    |> Image
        src images/svg_ch4_RATS_timeline.svg

    We also consider quantities $A$ and $B$
    that are changing with time; $A$ and $B$
    have some value at $t_0$, and, say, grow
    to be larger at $t_0 + \Delta{}t$:

    |> Image
        src images/svg_ch4_RATS_timeline_with_A_B.svg

    More specifically, we are interested in
    the change in the value of the product 
    
    $$
    \Large AB
    $$
    
    over said course of time.
    
    To introduce an unsolicited metaphor,
    imagine $A$ and $B$ as

    |> Image
        src images/svg_ch4_RATS_rats.svg

    that are crossing a hallway surveyed by
    a cat. One side of the hallway is time
    $t_0$, the other side of the hallway is
    time $t_0 + \Delta{}t$. So great is their
    terror that $A$ and $B$ have decided to 
    scurry across the hallway one at a time.
    First $A$ will go, then $B$. In so, we 
    can separate the following moments of 
    interest (“moments” that exist inside the
    metaphorical timeline of the story, not
    on the $t$-number line, to be clear):

    |> ImageLeft
        offset_x 0.5em
        offset_y -0.5em
        src images/svg_ch4_RATS_corridor_cloud.svg

    |> ol
        style margin:0 1.5em 1em 2em

        |> li
            style margin-top:0.5em
            
            when $A$ and $B$ are both still at $t_0$
        
        |> li
            style margin-top:0.5em

            when $A$ has made it to $t_0 + \Delta{}t$,
            and $B$ is still at $t_0$

        |> li
            style margin-top:0.5em
            
            |> del                
            
                when $B$'s tail is sticking out of the
                cat's mouth, and
            &ensp;when $A$ and $B$ have both made it
            to $t_0 + \Delta{}t$

    Correspondingly, the product

    $$
    \Large AB
    $$

    changes in two increments: first as $A$
    makes it to the other side of the hallway
    (and $A$ grows bigger); then as $B$ joins
    him/her (and $B$ grows bigger). In an
    equation:

    |> Image
        src images/svg_ch4_RATS_equation.svg

    If we divide the above equation by $\Delta{}t$
    and let $\Delta{}t$ drop to $0$, what does
    each term become?

    |> Solution

        Dividing by $\Delta{}t$:

        |> Image
            src images/svg_ch4_RATS_equation_over_Dt.svg

        As $\Delta{}t$ approaches $0$, the term
        on the left-hand side approaches
        
        $$
        (AB)'(t_0)
        $$
        
        where we view $A$ and $B$ as functions
        of time with, therefore, the product $AB$ 
        also becoming a function of time. (By 
        definition, $AB$ is the function
        
        $$
        t \rightarrow A(t)B(t)
        $$
        
        where $A(t)$ is the value of $A$ at time 
        $t$, $B(t)$ is the value of $B$ at time
        $t$.) Indeed, a ratio of the form
        
        $$
        {f(t_0 + \Delta{}t) - f(t_0)\over \Delta{}t}
        $$
        
        is a Newton quotient (cf. Exercise 16),
        that approaches 
        
        $$
        f'(t_0)
        $$
        
        as $\Delta{}t$ approaches $0$, assuming
        $f$ is differentiable at $t_0$ (cf.
        Exercise 16 Note 3), and

        |> Image
            src images/svg_ch4_RATS_over_Dt_first_term.svg

        has the form

        $$
        {f(t_0 + \Delta{}t) - f(t_0)\over \Delta{}t}
        $$

        for $f = AB$.

        |> ImageLeft
            src images/svg_ch4_RATS_for_f_equals_AB_cloud.svg

        The first term on the right-hand side, for
        its part, approaches

        |> ImageRight
            src images/svg_ch4_RATS_first_term_cloud.svg

        as $\Delta{}t$ approaches $0$. Indeed,
        when you write it out, that term becomes
        the algebraic expression

        $$
        {A(t_0 + \Delta{}t)B(t_0) - A(t_0)B(t_0) \over \Delta{}t}
        $$

        where every term on top contains a
        “$B(t_0)$”, that can therefore be factored
        out, giving us the equivalent expression

        $$
        B(t_0)\cdot{A(t_0 + \Delta{}t) - A(t_0) \over \Delta{}t}
        $$

        that, you will notice, has the form 

        $$
        B(t_0)\cdot{f(t_0 + \Delta{}t) - f(t_0) \over \Delta{}t}
        $$

        for $f = A$, and thus approaches

        $$
        B(t_0) \cdot A'(t_0)
        $$

        as $\Delta t$ approaches $0$, by the property
        of the Newton quotient.

        Lastly the most interesting term is the
        second term on the right-hand side!
        Symmetrically to the first term on the
        right-hand side, the second term approaches 

        |> ImageRight
            src images/svg_ch4_RATS_second_term_cloud.svg
        
        $$
        A(t_0)B'(t_0)
        $$
        
        as $\Delta{}t$ approaches $0$, but the
        reasons are slightly different! (Slightly.)
        Indeed, this term, written out, is
        
        $$
        {A(t_0 + \Delta{}t)B(t_0+\Delta{}t) - A(t_0+ \Delta{}t)B(t_0) \over \Delta{}t}
        $$
        
        which is equal to
        
        $$
        A(t_0 + \Delta{}t)\cdot{B(t_0+\Delta{}t) - B(t_0) \over \Delta{}t}
        $$
        
        by factoring out the common term $A(t_0 + \Delta{}t)$;
        and
        
        $$
        {B(t_0+\Delta{}t) - B(t_0) \over \Delta{}t}
        $$
        
        approaches
        
        $$
        B'(t_0)
        $$
        
        as $\Delta{}t$ approaches $0$, like before
        (when we had $AB$ or $A$ instead of $B$)
        whereas
        
        $$
        A(t_0 + \Delta{}t)
        $$
        
        —which is a bit different from before—approaches
        
        $$
        A(t_0)
        $$
        
        as $\Delta{}t$ approaches $0$—so that makes up
        $A(t_0)B'(t_0)$. (The

        __differentiability__

        of $A$ and $B$ at $t_0$—that we are tacitly
        assuming—implies

        __continuity__

        as well, which implies that $A(t_0 + \Delta{}t)$
        approaches $A(t_0)$ as $\Delta t$ approaches $0$.)

        Summarizing, the three terms separately
        approach
        
        $$
        (AB)'(t_0)
        $$
        $$
        B(t_0)A'(t_0)
        $$
        $$
        A(t_0)B'(t_0)
        $$
        
        as $\Delta{}t$ approaches $0$ and, in fact, 
        because the equation holds no matter how
        close we make each term to its respective limit
        above, one can conclude that
        
        $$
        (AB)'(t_0) = B(t_0)A'(t_0) + A(t_0)B'(t_0)
        $$
        
        for functions $A$, $B$ differentiable at
        a point $t_0$. 

        Nb: This result is known as the _product rule_.

        |> Pause
        _Note 1._
        Keeping things alphabetical everywhere, the 
        same equation is more often written

        $$
        (AB)'(t_0) = A'(t_0)B(t_0) + A(t_0)B'(t_0)
        $$

        with “$A'(t_0)B(t_0)$” in the middle. (But
        which is the same, of course, as $B(t_0)A'(t_0)$.)




|> Exercise

    The identity
    
    $$
    (f + g)' = f' + g'
    $$
    
    happens to be true for differentiable
    functions $f$, $g$. What English-language
    aphorism can summarize it? (This identity
    is known as the _sum rule_, by the way.)

    |> Solution

        One can say

        __the derivative of the sum is the sum of the
        derivatives__

        or

        __the rate of change of the sum is the sum of
        the rates of change__

        or (we made this one up)

        __the rate of change of the aggregate is the sum
        of the rates of change of the components__

        (etc).


|> Exercise

    If we rewrite the “product rule” of Exercise
    17 in the same terse style as the “sum rule”
    of Exercise 19, what do we obtain?

    |> Solution

        The form of...

        $$
        (fg)'(t_0) = f'(t_0)g(t_0) + f(t_0)g'(t_0)
        $$

        ...that follows the style of...

        $$
        (f + g)' = f' + g'
        $$

        ...is...

        $$
        (fg)' = f'g + fg'
        $$

        ...this. (Valid for differentiable functions
        $f$, $g:$ $\rr \ra \rr$.)
    
        |> Pause        
        _Note 1._
        Whereas
        
        $$
        (fg)'(t_0) = f'(t_0)g(t_0) + f(t_0)g'(t_0)
        $$
        
        is an equality between real numbers,
        
        $$
        (fg)' = f'g + fg'
        $$
        
        is an equality between functions. So there is 
        a more-than-skin-deep difference between the
        two forms. Also note that each form has its
        own “qualitatively distinct” qualifying conditions.
        
        (To wit,
        
        $$
        (fg)'(t_0) = f'(t_0)g(t_0) + f(t_0)g'(t_0)
        $$
        
        holds “for $t_0$ at which $f$ and $g$ are
        differentiable”, while
        
        $$
        (fg)' = f'g + fg'
        $$
        
        holds “for differentiable functions $f$, $g$”.)


|> Exercise

    If the identities
    
    $$
    (f + g)' = f' + g'
    $$
    
    and
    
    $$
    (fg)' = f'g + fg'
    $$
    
    for differentiable $f$, $g$ are deemed 
    “differentiation formulas”, then what is a
    third “differentiation formula” _already 
    encountered_ (in possibly disguised form) prior 
    to this point?

    |> Solution

        That would be the fact that
        
        $$
        (cf)' = cf'
        $$
        
        for all differentiable functions $f : \rr \ra \rr$,
        for all $c \in \rr$, mentioned in Exercise 10
        for $c = 2$.

        |> Pause
        _Note 1._
        You can also write 
        
        $$
        (cf)' = c \cdot f'
        $$
        
        if it helps clarify the difference between the
        left- and right-hand sides. (The difference being
        namely “($c$ times $f$) prime” on the left vs. “c
        times ($f$ prime)” on the right.)


|> Exercise

    The solution to the previous exercise
    erroneously assumes that the product of a
    constant and a function has been defined. It
    has not! Keeping in mind that the _sum_
    of two functions $f$, $g: \rr \ra \rr$
    
    $$
    f + g
    $$
    
    is defined by the equation
    
    $$
    f + g = (x \ra f(x) + g(x))
    $$
    
    while their composition is defined by
    
    $$
    f \circ g = (x \ra f(g(x)))
    $$
    
    and so on, what is the similar, most logical
    definition for 
    
    $$
    cf
    $$
    
    where $c \in \rr$ and $f : \rr \ra \rr$?

    |> Solution

        The “logical” definition is:
        
        $$
        cf = (x \ra cf(x))
        $$
        
        where the product “$cf(x)$” is an ordinary
        multiplication between two real numbers,
        because $c$ is a real number and $f(x)$ is
        a real number! (In this way, the product of
        a function by a real number “bootstraps”
        off of the ordinary product of real numbers—this
        is already similar to what happens for the
        definition... 
        
        $$
        fg = (x \ra f(x)g(x))
        $$
        
        of the product of two functions from $\rr$ to $\rr$,
        or with the case of function addition, that
        relies on real number addition.) BUT. There
        is a MORE CLEVER way of doing the definition.
        Which is to define

        $$
        cf = (x \ra c)f
        $$

        where the right-hand-side is _one function
        times another_, i.e., _a product of functions_,
        which is something that has ITSELF ALREADY BEEN
        DEFINED. (!) (To wit, the definition of
        function multiplication is that

        $$
        fg = (x \ra f(x)g(x))
        $$

        of course.) (Wait we just mentioned that
        already.) Mathematicians LOVE to bootstrap off
        an intermediate step, instead of going back to
        the beginning, so the second way is clearly the
        superior definition!!


|> Exercise

    The definition
    
    $$
    f + g = (x \ra f(x) + f(x))
    $$
    
    for a sum of functions $f, g : \rr \ra \rr$
    can also be written
    
    $$
    (f + g)(x) = f(x) + g(x)
    $$
    
    in the sense that either of these equations tells
    you how $f + g$ acts on an arbitrary input. (Which
    is what you need to do, to define a function. A
    slight subtlety is that the definition
    
    $$
    f + g = (x \ra f(x) + f(x))
    $$
    
    announces more clearly via its notation that 
    “$f + g$” _is a function_ and not some other
    object, like a number, but this is a minor point.)
    Rewrite the definitions of
    
    $$
    f \circ g
    $$
    $$
    fg
    $$
    $$
    f/g
    $$
    $$
    f - g
    $$
    
    in the style of the second equation. For extra
    credit: use a different symbol each time to denote
    the input.

    |> Solution

        E.g.:

        $$
        (f \circ g)(x) = f(g(x))
        $$
        $$
        (fg)(u) = f(u)g(u)
        $$
        $$
        (f/g)(z) = f(z)/g(z)
        $$
        $$
        (f - g)(t) = f(t) - g(t)
        $$

        (Looking at these definitions we must really admit that
        we prefer the first form, with the arrow, found at the
        end of the chapter—it's more explicit!)
    

|> Exercise

    What does...

    $$
    A_1(t_0 + h) \,\times\, \dots \,\times\, A_{i-1}(t_0 + h) \,\times\, {A_i(t_0 + h) - A_i(t_0)\over h} \,\times\, A_{i + 1}(t_0) \,\times\, \cdots  \,\times\, A_n(t_0)
    $$

    |> ImageRight
        src images/svg_ch4_big_product_parchment_cloud.svg
    
    ...approach as $h$ goes to $0$, if $A_1, \dots, A_n$ 
    $: \rr \ra \rr$ are differentiable at the point $t_0$?

    |> Solution

        We can start with the fraction in the
        middle of the product:
        
        |> Image
            src images/svg_ch4_big_product_fraction_outline.svg
        
        This is seen to be a Newton quotient
        (cf. Exercise 16)
        
        $$
        f(x + h) - f(x)\over h
        $$
        
        with $f = A_i$, $x = t_0$, per which
        (Exercise 16 Note 3), the fraction approaches
        
        $$
        A_i'(t_0)
        $$
        
        as $h$ approaches $0$, given also the assumption 
        that each of the functions $A_1$, ..., $A_n$ 
        (including $A_i$) is differentiable at $t_0$.
        
        Next down in order of interesting-ness we presumably
        have the terms $A_1(t_0 + h)$ through $A_{i-1}(t_0 + h)$
        at the beginning of the product...

        |> Image
            src images/svg_ch4_big_product_prefix_outline.svg

        ...; here the

        __differentiability__

        of $A_1$ at $t_0$ implies the

        __continuity__

        of $A_1$ at $t_0$, which implies that
        
        $$
        A_1(t_0 + h)
        $$
        
        approaches
        
        $$
        A_1(t_0)
        $$
        
        as $h$ approaches $0$. (These various
        technicalities concerning a generic function
        $f : \rr \ra \rr$ are mentioned in the solution
        to Exercise 17.) Similarly for $A_2(t_0 + h)$,
        etc, up to $A_{i-1}(t_0 + h)$, so
        
        $$
        A_1(t_0 + h) \,\times\, \dots \,\times\, A_{i-1}(t_0 + h)
        $$
        
        approaches
        
        $$
        A_1(t_0) \,\times\, \dots \,\times\, A_{i-1}(t_0)
        $$
        
        as $h$ approaches $0$. (If some quantities are
        each approaching a different value, then the 
        product-of-the-quantities will approach the 
        product-of-the-values—something not mentioned
        in the solution to Exercise 17, but that might
        have been.)

        Lastly one has the tail end of the product,
        where $h$ does not even appear:

        |> Image
            src images/svg_ch4_big_product_suffix_outline.svg

        Because $h$ does not appear here, the tail end

        __stays put__

        where it is, irrespective of the value of $h$.
        So that was easy! Altogether, the answer is
        therefore: 

        |> Image
            src images/svg_ch4_big_product_final_answer.svg

        ...with a lone “$A_i'$” in the middle.


|> Exercise

    The function below is also the red
    curve from Exercise 12, known as the
    _cosine function_ (already encountered
    in Chapter 3, Exercise 7). Knowing that
    this function is the $x$-coordinate of a
    point rotating at unit speed around a unit
    circle, find, by inspection of the graph,
    a rational approximation to the circumference
    of a unit circle.

    |> Image
        src images/svg_ch4_cosine_for_eta.svg

    |> Solution

        Because the particle is going at unit speed the
        circumference of the unit circle is equal to the
        amount of time it takes the particle to complete one
        revolution of the circle. That is, for example, the
        length of this yellow interval:
        
        |> Image
            src images/svg_ch4_cosine_for_eta_with_yellow.svg
        
        One revolution around the circle is also
        made up of four quarter-revolutions, where
        each quarter-revolution of the circle is “half a bump”,
        on the graph:

        |> Image
            src images/svg_ch4_cosine_for_eta_with_two_yellows.svg

        Going a bit further, _seven_ of these
        quarter-revolutions appear to take up exactly
        $t = 11$ units of time (!!!!!!!!!!!) (or maybe
        just a _little_ less than $11$ units, if
        you zoom in):
        
        |> Image
            src images/svg_ch4_cosine_for_eta_revealed.svg

        Therefore

        $$
        \Large {11 \over 7}
        $$

        is an approximation to the quarter-circumference of
        the circle, and

        $$
        \Large 4 \cdot {11 \over 7} = {44 \over 7}
        $$

        is an approximation to the circumference of a unit
        circle.
        
        |> Pause
        _Note 1._
        This approximation ends up being about 
        half-a-part-in-a-thousand too large 
        ($0.040249943...\%$ too large) (or just: 
        <span class="nobreak">“$0.00040249943...$</span>
        too large”), which is strikingly good, all things
        considered.

        |> Pause
        _Note 2._
        Numerically, note that
        
        $$
        \Large {44 \over 7} = 6.285714\dots
        $$
        
        is a bit larger than $6$, which agrees with
        what we see here for the length of a full
        revolution...
        
        |> Image
            src images/svg_ch4_cosine_for_eta_tau_verification.svg

        ...whereas

        $$
        \Large {11 \over 7} = 1.571428\dots
        $$

        is about $1.6$, which also appears to agree
        with what we can see on the graph about the
        length of a quarter-revolution:

        |> Image
            src images/svg_ch4_cosine_for_eta_eta_verification.svg

        (So, we have some secondary “visual confirmation”
        of our approximations.)


|> Exercise

    The graphs below are the horizontal and
    vertical velocities...

    |> Image
        src images/svg_ch4_pacman1_hor0.svg

    |> ImageLeft
        offset_y 1em
        src images/svg_ch4_pacman1_x_prime_t_scloud.svg

    |> Image
        src images/svg_ch4_pacman1_ver0.svg

    ...of PACMAN, with unit of
    distance of one “cell”, or “c”—the distance 
    between two food pellets—and units of velocity 
    of “cells per second”, or “c/s”—also,
    $x$-coordinates increase towards the right, 
    and $y$-coordinates increase towards the top:

    |> Image
        height 590px
        src images/svg_ch4_pacman1_maze.svg

    Where is Pacman at $t = 49$s? (Note: Pacman 
    is NOT assumed to be anywhere in particular at 
    $t = 40$s—you have to figure that out from the 
    data!)

    |> Solution

        Let's start by examining Pacman's first six displacements,
        appearing here in blue (positive displacements, going to the right 
        or up) and red (negative displacements, going to the left or down):

        |> Image
            src images/svg_ch4_pacman1_hor1.svg
        |> Image
            src images/svg_ch4_pacman1_ver1.svg

        We can estimate the duration of each
        displacement to the closest 10th of a second
        (mistakes of estimation can be made, we shall recover):

        |> Image
            src images/svg_ch4_pacman1_hor2.svg
        |> Image
            src images/svg_ch4_pacman1_ver2.svg

        We can also estimate the velocity to be

        $$
        \pm 6.75\te{c}\hlfbk/\hlfbk\te{s}
        $$
        when it is nonzero (for displacement
        |> InlineImage
            class ch4_pacman1_inline_number_pellet
            src images/svg_ch4_pacman1_pellet5_inline.svg
        the velocity might seem more like $-6.8\te{c}\hlfbk/\hlfbk\te{s}$ 
        at the least, but we've already made more
        significant errors while eyeballing the durations, so
        nevermind). Using

        $$
        (\te{velocity}) \times (\te{amount of time}) = (\te{displacement})
        $$

        |> ImageRight
            src images/svg_ch4_pacman1_velocity_times_amt_time_cloud.svg

        then gives us the following estimates for the
        |> del

            amount of travel
        &ensp;
        |> del

            during the
        &ensp;six displacements:

        |> Center

            |> InlineImage
                class ch4_pacman1_inline_number_pellet
                src images/svg_ch4_pacman1_pellet1_inline.svg
                style margin-right:1em
            $-6.75\te{c}\hlfbk/\hlfbk\te{s}\,\,\times\,\,0.5\te{s}\,\,=\,\,-3.375\,\te{cells}$
            
        |> ImageRight
            src images/svg_ch4_pacman1_cells_per_second_times_seconds_cloud.svg

        |> Center

            |> InlineImage
                class ch4_pacman1_inline_number_pellet
                src images/svg_ch4_pacman1_pellet2_inline.svg
                style margin-right:1em
            $-6.75\te{c}\hlfbk/\hlfbk\te{s}\,\,\times\,\,0.4\te{s}\,\,=\,\,-2.7\,\te{cells}$

        |> Center

            |> InlineImage
                class ch4_pacman1_inline_number_pellet
                src images/svg_ch4_pacman1_pellet3_inline.svg
                style margin-right:1em
            $6.75\te{c}\hlfbk/\hlfbk\te{s}\,\,\times\,\,0.9\te{s}\,\,=\,\,6.075\,\te{cells}$

        |> Center

            |> InlineImage
                class ch4_pacman1_inline_number_pellet
                src images/svg_ch4_pacman1_pellet4_inline.svg
                style margin-right:1em
            $6.75\te{c}\hlfbk/\hlfbk\te{s}\,\,\times\,\,0.4\te{s}\,\,=\,\,2.7\,\te{cells}$

        |> Center

            |> InlineImage
                class ch4_pacman1_inline_number_pellet
                src images/svg_ch4_pacman1_pellet5_inline.svg
                style margin-right:1em
            $-6.75\te{c}\hlfbk/\hlfbk\te{s}\,\,\times\,\,0.4\te{s}\,\,=\,\,-2.7\,\te{cells}$

        |> Center

            |> InlineImage
                class ch4_pacman1_inline_number_pellet
                src images/svg_ch4_pacman1_pellet6_inline.svg
                style margin-right:1em
            $6.75\te{c}\hlfbk/\hlfbk\te{s}\,\,\times\,\,0.9\te{s}\,\,=\,\,6.075\,\te{cells}$

        Given the horizontal/vertical alternation
        of displacements, this would nominally 
        imply the following set of initial motions:

        |> Image
            src images/svg_ch4_pacman1_initial_displacements_before_rounding.svg

        But these are approximate numbers and the
        true values must be integers, except for
        |> InlineImage
            class ch4_pacman1_inline_number_pellet
            src images/svg_ch4_pacman1_pellet1_inline.svg
        . (Because we don't know where Pacman started
        out. For
        |> InlineImage
            class ch4_pacman1_inline_number_pellet
            src images/svg_ch4_pacman1_pellet6_inline.svg
        &ensp;the next displacement, if you look back at
        the graphs, is horizontal, so yes.) In fact,
        if you look at the maze,

        $$
        3
        $$
        
        cells is the smallest amount that Pacman
        can travel vertically when changing $y$-coordinate,
        between two moments of horizontal motion.
        The next smallest possible amounts are

        $$
        4
        $$
        |> ImageLeft
            width 700px
            src images/svg_ch4_pacman1_disp_4_cloud.svg

        and

        $$
        6
        $$
        |> ImageLeft
            width 1400px
            src images/svg_ch4_pacman1_disp_6_cloud.svg

        and

        $$
        7
        $$
        |> ImageLeft
            width 700px
            src images/svg_ch4_pacman1_disp_7_cloud.svg

        cells, with $5$ not being a possibility.
        In the horizontal direction,
        the smallest amounts are

        $$
        3, 6, \te{ and } 9
        $$

        (and $12$ and ...) which is even more restrictive.
        Now if each of our duration 
        measurements carries an error of no more than

        $$
        \pm{}0.2\te{s}
        $$

        each computed displacement is at most 

        $$
        6.75\te{c}\hlfbk/\hlfbk\te{s}\,\times\,\pm{}0.2\te{s}\,=\,\pm1.35\te{c}
        $$

        from the truth, give or take the small
        difference between $6.75\te{c}\hlfbk/\hlfbk\te{s}$
        and the actual velocity. So

        $$
        -2.7\te{c}
        $$

        must be either

        $$
        -3\te{c}
        $$

        or

        $$
        -4\te{c}
        $$

        these being the only two posssible integer
        vertical displacements within $\pm1.35$c of $-2.7$c. 
        Then, applying similar logic to each
        measurement, the initial motions must be:

        |> Image
            src images/svg_ch4_pacman1_initial_motions.svg

        The maze fits these constraints in only two places
        (note that
        |> InlineImage
            class ch4_pacman1_inline_number_pellet
            src images/svg_ch4_pacman1_pellet2_inline.svg
        &ensp;and 
        |> InlineImage
            class ch4_pacman1_inline_number_pellet
            src images/svg_ch4_pacman1_pellet4_inline.svg
        &ensp;equal $-3\te{c}$ and $3\te{c}$, in each
        case):

        |> Image
            height 590px
            src images/svg_ch4_pacman1_maze_two_possibilities.svg

        Looking into the future, 
        the next three displacements are 
        right/down/right and last
        ~$3.2$s/~$0.4$s/~$1.3$s
        respectively:

        |> Image
            src images/svg_ch4_pacman1_hor3.svg
        |> Image
            src images/svg_ch4_pacman1_ver3.svg

        (Nb: Imagine translating these intervals to the left or right until
        the start of the interval is at an integer value: this is a good
        way to estimate the length.)

        Because displacement
        |> InlineImage
            class ch4_pacman1_inline_number_pellet
            src images/svg_ch4_pacman1_pellet7_inline.svg
        &ensp;is horizontal to the right the only possible
        remaining solution is the right-hand one,
        or else Pacman would collide with the ghost cage,
        with displacement
        |> InlineImage
            class ch4_pacman1_inline_number_pellet
            src images/svg_ch4_pacman1_pellet6_inline.svg
        &ensp;equal to 6c not 7c, or else Pacman would 
        collide with a wall:

        |> Image
            height 590px
            src images/svg_ch4_pacman1_maze_would_collide.svg

        Since

        $$
        6.75\te{c}\hlfbk/\hlfbk\te{s}\,\times\,3.2\te{s}\,=\,21.6\te{c}
        $$

        it seems that displacement
        |> InlineImage
            class ch4_pacman1_inline_number_pellet
            src images/svg_ch4_pacman1_pellet7_inline.svg
        &ensp;brings Pacman all the way around the maze to
        the left edge of the ghost cage, like so...

        |> Image
            height 590px
            src images/svg_ch4_pacman1_maze_bring_around.svg

        ...though it is hard to measure that distance;
        but this is confirmed by the fact that the next
        two displacements are “down by $3$ and to the right”;
        specifically, since

        $$
        -6.75\te{c}\hlfbk/\hlfbk\te{s}\,\times\,0.4\te{s}\,=\,-2.7\te{c}
        $$

        displacement
        |> InlineImage
            class ch4_pacman1_inline_number_pellet
            src images/svg_ch4_pacman1_pellet8_inline.svg
        &ensp;must be $-3$c or $-4$c;
        must actually be $-3$c since displacement
        |> InlineImage
            class ch4_pacman1_inline_number_pellet
            src images/svg_ch4_pacman1_pellet9_inline.svg
        is to the right; so, notwithstanding the exact 
        length of displacement
        |> InlineImage
            class ch4_pacman1_inline_number_pellet
            src images/svg_ch4_pacman1_pellet9_inline.svg
        , there is only one possibility for displacements
        |> InlineImage
            class ch4_pacman1_inline_number_pellet
            src images/svg_ch4_pacman1_pellet1_inline.svg
        through
        |> InlineImage
            class ch4_pacman1_inline_number_pellet
            src images/svg_ch4_pacman1_pellet8_inline.svg
        :

        |> Image
            height 590px
            src images/svg_ch4_pacman1_maze_final_solution.svg

        So at $t = 49$s, between displacements
        |> InlineImage
            class ch4_pacman1_inline_number_pellet
            src images/svg_ch4_pacman1_pellet7_inline.svg
        &ensp;and
        |> InlineImage
            class ch4_pacman1_inline_number_pellet
            src images/svg_ch4_pacman1_pellet8_inline.svg
        , Pacman is immediately to the left of the ghost cage. 


|> Exercise

    Same question, but for the following maze...

    |> Image
        height 551px
        src images/svg_ch4_pacman2_maze.svg

    ...and for the following velocity data, with the 
    horizontal and vertical velocities
    superimposed on one graph (just a cosmetic 
    change—note that green is the vertical velocity)...

    |> Image
        src images/svg_ch4_pacman2_frankenstein.svg

    ...and asking for Pacman's position at $t = 34$s.

    |> Solution

        It seems well-advised to start by heuristically verifying
        that Pacman's speed remains
        $$
        \approx 6.75\te{c}\hlfbk/\hlfbk\te{s}
        $$
        no matter the direction that Pacman is headed, as long as Pacman is
        in motion.

        For example, take the instant $t \approx 23.7$s, when the 
        $x$- and <span class="nobreak">$y$-velocities</span> are 
        both about (?) $4.8\te{c}\hlfbk/\hlfbk\te{s}$:

        |> Image
            src images/svg_ch4_pacman2_frankenstein_at_23_7.svg

        The velocity vector (cf. Exercise 12) is therefore about

        $$
        (4.8, 4.8)
        $$

        in units of $\te{c}\hlfbk/\hlfbk\te{s}$ at $t \approx 23.7$s,
        and the speed, being the length of the velocity
        vector (cf. Exercise 12), is about

        $$
        \sqrt{4.8^2 + 4.8^2} = \sqrt{2} \times 4.8 = 6.788...
        $$

        |> ImageLeft
            src images/svg_ch4_pacman2_4_point_8_cloud.svg

        (Pythagoras!)
        in units of $\te{c}\hlfbk/\hlfbk\te{s}$ as well,
        and

        $$
        6.788... \approx 6.75
        $$

        which supports, in this case, the hypothesis that
        Pacman's speed is $\approx 6.75\te{c}\hlfbk/\hlfbk\te{s}$
        regardless of the direction of travel.


        For more verification, take $t = 26\te{s}$, 
        at which point the velocity vector is roughly

        $$
        (6.5, -2)
        $$

        cells per second:

        |> Image
            src images/svg_ch4_pacman2_frankenstein_at_26.svg

        This gives a speed of

        $$
        \sqrt{6.5^2 + 2^2} = \sqrt{46.25} = 6.800...
        $$

        cells per second, Again close to $6.75\te{c}\hlfbk/\hlfbk\te{s}$. (!)

        For two more verifications take $t = 27\te{s}$
        and $t = 30\te{s}$:

        |> Image
            src images/svg_ch4_pacman2_frankenstein_at_27_and_30.svg

        The speed at $t = 27\te{s}$ is approximately

        $$
        \sqrt{3^2 + 6.2^2} = \sqrt{47.44} = 6.888
        $$

        cells per second, while the speed at $t = 30\te{s}$
        is approximately

        $$
        \sqrt{5.6^2 + 3.6^2} = \sqrt{44.32} = 6.657
        $$

        cells per second. Both close-ish to
        $6.75\te{c}\hlfbk/\hlfbk\te{s}$! For one last
        verification (truly the last, we promise)
        consider $t = 31\te{s}$:

        |> Image
            src images/svg_ch4_pacman2_frankenstein_at_31.svg

        This yields a speed of

        $$
        \sqrt{2.6^2 + 6.2^2} = \sqrt{45.2} = 6.723...
        $$

        cells per second, again close to $6.75\te{c}\hlfbk/\hlfbk\te{s}$! 
        (Closest so far, in fact.)

        We now admit, after this “heuristic verification”, 
        that Pacman goes approximately the same speed
        regardless of direction, namely something in
        the vicinity of $6.75\te{c}\hlfbk/\hlfbk\te{s}$.

        Now consider the time interval from $t = 22.7\te{s}$ to $t = 24.7\te{s}$:

        |> Image
            src images/svg_ch4_pacman2_frankenstein_first_big_purple.svg

        Both the $x$- and $y$-velocities are nonzero
        during this interval, which indicates the presence
        of a curve. The curve starts with vertical motion
        and ends with horizontal motion:

        |> Image
            src images/svg_ch4_pacman2_frankenstein_first_big_purple_annotations.svg

        Thus Pacman starts the curve going up, and ends the curve
        going right. Moreover, it takes Pacman
        
        $$
        24.7\te{s} - 22.7\te{s} = 3\te{s}
        $$
        
        to complete the curve (we know the curve is fully
        completed from the purely horizontal motion at
        either end), from which the curve must be
        approximately
        
        $$
        6.75\te{c}\hlfbk/\hlfbk\te{s} \times 3\te{s} = 20.25\te{c}
        $$
        
        in length! In turn, using the approximation
        
        $$
        \approx {11 \over 7}
        $$
        
        for the quarter-circumference of a unit circle
        (cf$.$ Exercise 24), this would indicate that
        the curve (which is a quarter-circle, as 
        all curves in this maze) has radius
        
        $$
        \approx {20.25\te{c} \over 11/7} = {7 \times 20.25\te{c} \over 11} = {141.75\te{c} \over 11} = 12.886...\te{c}
        $$
        
        where we give up and use a calculator at the
        last step. But the possible radii are $3$, $6$,
        $9$ and $12$. This all but rules out all of the
        maze curves except the one that has radius $12$,
        and that allows a traversal that starts upward
        and ends rightward; we mean the upper left
        curve of the maze:

        |> Image
            height 551px
            src images/svg_ch4_pacman2_upper_left_highlight.svg

        From there, Pacman goes right for a bit, then
        takes another curve 3 seconds long, that starts
        rightward and ends downward:

        |> Image
            src images/svg_ch4_pacman2_frankenstein_second_big_purple.svg

        ...this second curve must, of course, be the
        upper right-hand corner of the maze, that has the
        appropriate length, position, and orientation:

        |> Image
            height 551px
            src images/svg_ch4_pacman2_upper_right_highlight.svg

        What is extremely strange, however, is that
        Pacman immediately follows the end of this curve
        with rightward motion:

        |> Image
            src images/svg_ch4_pacman2_frankenstein_first_warning_purple.svg

        In fact, it is also strange that Pacman _preceded_
        the first curve with rightward motion (when that curve
        starts at the leftmost edge of the maze):

        |> Image
            src images/svg_ch4_pacman2_frankenstein_second_warning_purple.svg

        Looking back over our work, 
        we find that we made a mistake when we wrote

        $$
        24.7\te{s} - 22.7\te{s} = 3\te{s}
        $$

        the corrected version of that being of course

        $$
        24.7\te{s} - 22.7\te{s} = 2\te{s}
        $$

        (the second curve likewise lasted $2\te{s}$,
        not $3\te{s}$) making the length and radius of
        the first curve two-thirds of whatever we previously 
        computed (because $2\te{s}$ is two-thirds of
        $3\te{s}$), i.e.,
        
        $$
        {2 \over 3} \times 12.886...\te{c}
        $$
        
        for the _radius_ of the first (and second)
        curve, which means that the first and second curves
        actually had radii $9$, undoubtedly,
        and that Pacman's initial motion followed the 
        one-inside track (the two rightward motions are 
        easily seen to be ~$3\te{c}$ each):

        |> Image
            height 551px
            src images/svg_ch4_pacman2_corrected_highlights.svg

        Next, after some downward motion we are faced
        with a long, juicy, down-and-then-left curve,
        which must surely be the bottom-right curve of
        radius $12$:

        |> Image
            src images/svg_ch4_pacman2_frankenstein_third_big_purple.svg

        Indeed, the curve lasts ~$2.7$s, and 
        
        $$
        6.75\te{c}\hlfbk/\hlfbk\te{s} \times 2.7\te{s} = 18.225\te{c}
        $$
        
        is approximately the same as 
        
        $$
        {11 \over 7} \cdot 12\te{c} = 18.85...\te{c}
        $$
        
        confirming the radius of $12\te{c}$ and the location
        of the curve.
        Pacman's trajectory so far is then:

        |> Image
            height 551px
            src images/svg_ch4_pacman2_continued_highlights.svg

        Next Pacman seems to reverse course, and briefly
        re-enters the curve (going right and up a tiny bit):

        |> Image
            src images/svg_ch4_pacman2_frankenstein_back_in_purple.svg

        But then changes again, and re-exits the curve
        (going left and down a tiny bit):

        |> Image
            src images/svg_ch4_pacman2_frankenstein_back_out_purple.svg

        Then Pacman goes left-and-then-right-again by
        some small amount:

        |> Image
            src images/svg_ch4_pacman2_frankenstein_back_out_purple.svg

        At this point—and in particular at 
        $t = 34\te{s}$—Pacman is between a moment 
        of purely horizontal motion and purely vertical
        motion; since the left-and-then-right-again 
        motion did obviously not bring Pacman $3$ cells
        over to the left (which is the next place 
        after the curve exit that is connected to both 
        horizontal and vertical paths),
        Pacman must 
        be at the bottom-left exit of the bottom-right
        maze corner, still.

        |> Pause
        _Note 1._
        Feel free to follow Pacman all the way
        to the end of the timeseries. He ends up somewhere
        near...


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


|> Exercise

    Continuing with the fundraising rat as in the 
    previous two exercises, assume that the functions
    $f$ and $g$ have these graphs:

    |> Image
        src images/svg_ch4_f_g_actual_exercise_f.svg

    |> Image
        src images/svg_ch4_f_g_actual_exercise_g.svg

    |> Solution

        At $t = 2$s the rat is running at a velocity of
        
        $$
        {5\over 3\up{1}}[{\te{m/s}}]
        $$
        
        by the slope of this line segment on $y = g(t)$:

        |> Image
            src images/svg_ch4_f_g_actual_exercise_g_with_slope.svg

        Moreover at $t = 2$s the rat 
        has reached
        $2 \cdot (5/3) = 10/3 = 3.\overline{33}$m,
        where the dollars-per-meter earnings
        rate is one-tenth of a dollar per meter, by
        the slope of this segment on the graph $y = f(x)$:

        |> Image
            src images/svg_ch4_f_g_actual_exercise_f_with_slope.svg

        Multiplying the $5/3$ meters-per-second velocity
        by the $1/10$ dollars-per-meter rate gives
        us the dollars-per-second rate at $t = 2$s (our 
        final answer—recall that $(f \circ g)'(2)$ is 
        the dollars-per-second rate at $t = 2$s, by Exercise 35):

        $$
        \left({5\over 3\up{1}}\left[{\te{m} \over \te{s}}\right]\right) \times \left({1 \over 10\up{1}}\left[{\te{\$} \over \te{m}}\right]\right) = {5\over 30\up{1}}\left[{\te{\$} \over \te{s}}\right].
        $$

        |> ImageRight
            src images/svg_ch4_f_g_actual_exercise_cloud.svg

        Or since
        $$
        {5 \over 3} = 1.6666...
        $$
        that would be
        $$
        {0.1666...}[\te{\$/s}]
        $$
        in decimal, i.e., 
        |> span
            style font-family:Clicker Script;font-size:1.3em;

            ~sixteen~ 
        point $666...$ cents per second.

        |> Pause
        _Postscript._
        It turns out that
        
        $$
        {5 \over 30} = {1 \over 6}
        $$
        
        as we weren't really paying attention to the 
        possibility of simplifying the fraction. In
        particular,
        
        $$
        {1 \over 6} = 0.1666...
        $$
        
        as, indeed, $0.1666...$ is half of $0.333...$! (Learn
        something every day.)


|> Exercise

    Conjecture a general formula for 
    $$
    (f \circ g)'(t)
    $$
    for arbitrary (differentiable, say) functions
    $f, g : \rr \ra \rr$. (If it helps, interpret $f$
    and $g$ exactly as in the scenario of the fundraising
    race, cf$.$ Exercises 34-37.)

    |> Solution

        The sought-for formula is
    
        $$
        g'(t)\cdot f'(g(t))
        $$
        
        because—to come back to the example of the 
        fundraising race—one must multiply the meters-per-second
        velocity at time $t$ (that is, $g'(t)$) by the dollars-per-meter 
        earnings rate at position $g(t)$ (that is, $f'(g(t))$) to obtain
        the final dollars-per-second earning rate at time $t$
        (that is, $(f \circ g)'(t)$).

        (For example, the solution to Exercise 36 can actually
        be written
        
        $$
        g'(2) \cdot f'(g(2))
        $$
        
        since, indeed, $g(2) = 3.333...$. [Remember that we ended
        up multiplying $g'(2) = {5\over 3}\te{m/s}$ by 
        $f'(3.333...) = {1\over 10}\te{\$/s}$—the “$3.333...$” is
        $g(2)$.])

        Nb: This result is known as the _chain rule_.

        |> Pause
        _Note 1._ Said _chain rule_
        is more commonly written...
        $$
        (f \circ g)'(x) = f'(g(x))g'(x)
        $$
        ...with “$g'(x)$” last. (And “$x$” instead of “$t$”.)

        |> Pause
        _Note 2._ One can also write the chain rule very succinctly like so... 

        $$
        (f \circ g)' = (f' \circ g)g'
        $$

        ...where the right-hand-side is the product of $g'$ and $f' \circ g$.



|> Exercise

    Sketch the velocity vector of a particle
    going at three times unit speed (“speed $3$” in common
    parlance) clockwise around a circle of radius $2$.
    What path does the velocity vector describe 
    over time? (I.e., if you cut-paste the velocity
    vector back to the origin, so that its “tail” 
    is at $(0, 0)$, what curve does the far end of the
    vector describe?) Over how much time?

    |> Solution

        The velocity vector is an arrow of length $3$
        tangent to a circle of radius $2$, brushed
        clockwise:

        |> Image
            src images/svg_velocity_vector_radius_2_speed_3_sketch_1.svg

        If we bring the tail of the vector back to
        $(0, 0)$ we find an arrow of length $3$
        tracing a circle of radius $3$:

        |> Image
            src images/svg_velocity_vector_radius_2_speed_3_centered.svg

        Lastly, the velocity vector does a full revolution
        of the red circle in the same amount of time that
        the particle does a full revolution of the blue circle,
        which is

        $$
        {2\cdot (\te{circumference of a unit circle}) \over \te{3}}
        $$

        because the circumference of the blue circle
        is twice the circumference of a unit circle,
        and the particle is going at speed $3$.

        |> Pause
        _Note 1._ 
        In such diagrams we recycle the axes
        to plot quantities of several different
        dimensions: positions (in blue, in this
        case) have dimensions of length ([L]) while
        velocities (in red, in this case) have 
        dimensions of length over time ([L/T]).

|> Exercise

    What is the 
    
    __acceleration vector__
    
    (velocity vector of the velocity vector)
    of the particle from Exercise 39?

    |> Solution

        The velocity vector of Exercise 39 travels
        in a circle of radius $3$ in the same amount
        of time that the position vector 
        travels around a circle of radius $2$.
        The speed of the velocity vector is therefore
        $1.5$ times the speed of the position vector,
        or $1.5 \times 3 = 4.5$.

        As the derivative of the velocity vector,
        the acceleration vector is therefore
        a vector of length $4.5$ (= the speed of the 
        velocity vector) brushed clockwise along 
        the path of the velocity vector:

        |> Image
            src images/svg_ch4_what_is_the_acceleration_vector_uncentered.svg

        Or, if we translate the acceleration vector back 
        to the origin and trace out its path over time 
        (either way is fine):

        |> Image
            src images/svg_ch4_what_is_the_acceleration_vector_centered.svg

        |> Pause
        _Note 1._
        You truly have to think of the acceleration
        vector as “the velocity of the velocity 
        vector”—if the velocity vector is changing, the
        acceleration vector is nonzero!

|> Exercise

    Sketch the velocity vector, acceleration vector,
    and jerk vector of a particle going around a circle
    of radius $3$ at speed $2$. (Clockwise, say.)

    |> Solution

        The velocity vector has length $2$, because the
        particle has speed $2$. So the velocity vector
        looks like so, while attached to the particle path
        (top) or brought back to the origin (bottom):

        |> Image
            src images/svg_ch4_velocity_vector_radius_3_speed_2_uncentered.svg

        |> Image
            src images/svg_ch4_velocity_vector_radius_3_speed_2_centered.svg

        Moreover (!) the speed of the velocity vector is 
        $2/3$ the speed of the particle, because the 
        velocity vector goes around a circle of $2/3$ the 
        radius in the same amount of time. So the velocity 
        vector has speed
        
        $$
        \Large 2 \cdot {2\over 3} = {4\over 3}
        $$
        
        from which the acceleration vector—that can be 
        described as “the velocity vector of the velocity
        vector”—has length ${4\over 3}$ (the speed of the
        velocity vector), and looks like so (in either representation):

        |> Image
            src images/svg_ch4_velocity_vector_radius_3_speed_2_acceleration_uncentered.svg

        |> Image
            src images/svg_ch4_velocity_vector_radius_3_speed_2_acceleration_centered.svg

        Lastly the acceleration vector has speed
        
        $$
        \Large {4\over 3}\cdot {2\over 3} = {8 \over 9}
        $$
        
        by virtue of circling a circle of radius $2/3$ 
        that of the velocity vector, that has speed $4/3$, 
        in the same amount of time. Since the jerk is the
        derivative of the acceleration, this becomes the
        length of the jerk vector, that is exactly opposite
        to the velocity vector, being twice $90^\circ$ away:

        |> Image
            src images/svg_ch4_velocity_vector_radius_3_speed_2_jerk_uncentered.svg

        |> Image
            src images/svg_ch4_velocity_vector_radius_3_speed_2_jerk_centered.svg

        |> StarDivider

        |> Pause
        _Note 1._
        If the particle's original path is centered at
        $(0, 0)$ then that path constitutes a fourth
        circle obeying the same pattern
        of $2/3$-ratios between the successive radii:

        |> Image
            src images/svg_ch4_velocity_vector_radius_3_speed_2_with_position.svg


|> Exercise

    Sketch the velocity vector, acceleration vector,
    and jerk vector of a particle going around a circle
    of radius $r$ at speed $v$.
    (You can assume say $v/r \approx 1.2$ for the sake
    of your sketch.) Give algebraic expressions for the 
    lengths of the various vectors.

    |> Solution

        While the particle goes around a circle of 
        radius $r$, the velocity vector goes around a
        circle of radius $v$. (Indeed
        $v$,
        being the speed, is the length of the velocity
        vector, and the length of the velocity vector is the radius 
        of the circle traced by the velocity vector.)
        So the circle traced by the velocity vector is
        
        $$
        \Large {v \over r}
        $$
        
        times as large as the circle traced by the position
        vector.
        Therefore, the 
        velocity vector goes 
        
        $$
        \Large {v \over r}
        $$
        
        times as fast as the position vector! 
        (The two vectors trace
        their respective circles in the same amount of time, so the 
        only difference in speed is caused by differences in the 
        radii—and this is the ratio of the radii.) 
        Therefore, the
        velocity vector has speed
        
        $$
        \Large v \cdot {v \over r} = {v^2 \over r}
        $$
        
        ...as obtained by multiplying the speed of the
        position vector ($v$) by the ratio of the speeds ($v/r$).
        This is also the length of the acceleration vector.
        (Speed of velocity vector = length of acceleration vector.)

        Next, the ratio
        
        $$
        \Large {\te{speed}\over \te{radius}}
        $$
        
        is the same for the velocity vector as it is for the
        position vector, because both “speed” and “radius” are
        scaled up by a factor
        
        $$
        \Large {v \over r}
        $$
        
        compared to the position vector. So
        
        $$
        \Large {\te{speed}\over \te{radius}} = {v \over r}
        $$
        
        for the velocity vector as well as for the position
        vector. But we can also write this ratio as
        
        $$
        \Large {\te{length of acceleration vector}\over \te{radius}}
        $$
        
        since the speed of the velocity vector is the length
        of the acceleration vector, or as
        
        $$
        \Large {\te{length of acceleration vector}\over \te{radius of velocity vector circle}}
        $$
        
        to be more exact, or as
        
        $$
        \Large {\te{radius of acceleration vector circle}\over \te{radius of velocity vector circle}}
        $$
        
        in yet another way! Therefore, the circle traced by
        the acceleration vector is
        
        $$
        {\Large {v \over r}}
        $$
        
        times as large as the circle traced by the velocity 
        vector, and the same pattern starts all over again!

        (In other words,
        each time we take a derivative we 
        find that the vector whose derivative we are taking
        has speed
        
        $$
        {\Large {v \over r}}
        $$
        
        times the speed of the previous vector whose 
        derivative we took, resulting in a circle
        
        $$
        {\Large {v \over r}}
        $$
        
        times as large as the current circle, resulting in
        a future speed $v/r$ times as large for the next 
        derivative, etc, etc.)

        Concretely, the length of the jerk
        vector will be
        
        $$
        {\Large {v^2 \over r} \cdot {v \over r} = {v^3 \over r^2}}
        $$
        
        because the length of the acceleration is $v^2/r$,
        and the length of the derivative
        of the jerk would be 
        
        $$
        {\Large {v^3 \over r^2} \cdot {v \over r} = {v^4 \over r^3}}
        $$
        
        because the length of the jerk is $v^3/r^2$, etc. 
        (Not that we needed to go beyond the jerk.)

        Coming back to a sketch of all this, if

        $$
        {\Large {v \over r} \approx 1.2}
        $$

        the sketch will involve concentric circles
        of successive ratio $\approx 1.2$ with the
        successive vectors being off by $90^\circ$.
        The position circle might not be centered at
        $(0, 0)$, so we didn't include it in this sketch
        (this sketch presumes clockwise motion, but
        it's unimportant):

        |> Image
            src images/svg_ch4_angular_velocity_1_point_2_uncentered.svg

        But if the position circle is centered at $(0, 0)$,
        it becomes the first circle in the sequence:

        |> Image
            src images/svg_ch4_angular_velocity_1_point_2_centered.svg

        |> StarDivider

        |> Pause
        _Note 1._
        The ratio 
        
        $$
        \Large {v \over r}
        $$

        is known as the

        __angular velocity__
        
        of the particle. You can think of the angular velocity
        as
        
        $$
        \Large {\te{speed}\over \te{radius}}
        $$
        
        directly per the expression above, or as 
        
        $$
        {\Large {\te{distance per unit time}\over \te{radius}}}
        $$
        
        since that is just the definition of “speed”, but which also
        means that you can think of the angular velocity as
        
        $$
        \Large {\te{number of radii per unit time}}
        $$
        
        or, say, just as
        
        $$
        \Large {\te{radii per unit time}}
        $$
        
        in other words. (The “number or radii” covered by an arc
        is also known as the _radian measure_ of the arc—an
        alternate measure of angle—so this can also be phrased
        _radians per unit time_, in that sense.) What is
        noteworthy is that the angular velocity of the position
        vector is the same as the angular velocity of the
        velocity vector, of the acceleration vector, etc, and
        it also constitutes the ratio between the successive
        lengths of all these vectors!

        |> Pause
        _Note 2._
        A common notation for the angular velocity of a 
        particle is
        $$
        \Large \omega
        $$
        which means that the velocity vector, acceleration
        vector, and jerk vector have lengths
        have lengths
        $$
        \Large \omega{}^1r
        $$
        $$
        \Large \omega{}^2r
        $$
        $$
        \Large \omega{}^3r
        $$
        respectively, where $r$ is the radius of the circle,
        as the angular velocity is the ratio of the lengths
        of the successive vectors, as noted.
        (PS: As the length of the velocity vector is also
        known as the speed, $\omega^1r = \omega{}r$ is also
        the speed, by another name.) (PPS: We couldn't resist
        writing “$\omega^1r$” instead of “$\omega{}r$”, to keep
        things extra symmetric _&_ typographically aligned.)


|> Exercise

    Four particles are moving at speed $3$
    around a circle of radius $3$ centered at $(0, 0)$,
    spaced out by $90^\circ$:

    |> Image
        src images/svg_ch4_four_particles_radius_3.svg

    Sketch the position vector, velocity vector,
    acceleration vector, and jerk vector of each 
    particle. What is the angular velocity of each 
    particle?

    |> Solution

        Starting with the second part of the
        question, because the particles are going around 
        a circle of radius $3$ at speed $3$ the
        angular velocity (cf. Exercise 37) is
        
        $$
        \Large {3\over 3} = 1
        $$
        
        which means that the ratio of the lengths of
        all the vectors will be $1$, i.e., all vectors
        (velocity, acceleration, jerk) will have the 
        same length as the radius, which is $3$.

        Keeping in mind that the jerk is 
        $90^\circ$ ahead of the acceleration is $90^\circ$
        ahead of the velocity, etc, in the direction of
        rotation, the sixteen vectors—four for each
        particle—are therefore as follows:

        |> Image
            src images/svg_ch4_four_particles_radius_3_four_minis.svg

        (In particular, the purple particle's position
        ends up being the velocity of the red particle,
        and many other identities of the sort.)

|> Exercise

    What are the dimensions of angular velocity?
    (For example, the dimensions of velocity are
    “length over time”, ([L/T]).)

    |> Solution

        _Solution 1._
        Angular velocity is
        
        __speed over radius__
        
        which has dimensions
        
        $$
        {\Large {\te{L/T} \over \te{L}}}
        $$
        
        because speed has dimensions of length over time,
        L/T, while the radius has dimensions of length, L;
        this simplifies...

        |> Image
            src images/svg_ch4_angular_velocity_dimensions_computation_with_simplification.svg

        ...down to dimensions of “one over time”.

        |> Pause
        _Solution 2._
        Angular velocity is
        
        __radians per unit time__
        
        or
        
        __number of radii per unit time__

        (if you prefer), which is a “one over time” quantity,
        because radians are dimensionless. 
        
        Indeed, “radian” is short for “number of radii that fit
        inside the arc length”, which is one length divided
        by another length, which is, therefore, dimensionless. 


|> Exercise

    What is the angular velocity of an object
    going at a speed of $10'000$ kilometers per hour around a
    circle of radius $3$ meters?

    |> Solution

        The angular velocity, being speed over radius, is
        
        $$
        {10'000\,\te{km}/\te{hr} \over 3\te{m}} = {10^7\te{m}\over 3\te{m}}\cdot {1\over \te{hr}} = {3.\overline{3}}\cdot 10^6\,\te{hr}^{-1}
        $$
        
        or some $3$ and a third million radii per hour! (That's
        $925.\overline{925}$ radii per second, to bring it down
        to more “human” dimensions.) (You can also say
        $925.\overline{925}$ _radians_ per second.)


|> Exercise

    Imagine a single particle in a one-dimensional
    world, whose velocity equals its position; at $t = 0$,
    the particle is sitting at $x = 1$:

    |> Image
        src images/svg_ch4_one_particle_at_1.svg

    If we play time backward, will the particle 
    ever reach $x = 0$? 

    |> Solution

        Going back in time,
        examine how long it would take
        the particle to cross each of the intervals defined by
        the following geometric progression* (*see Note 1):

        |> Image
            src images/svg_ch4_one_particle_interval_subdivision.svg

        The interval from $0.5$ to $1$
        takes _at least_

        $$
        {0.5 \over 1} = 0.5
        $$

        |> ImageLeft
            src images/svg_ch4_one_particle_speed_equals_time_cloud.svg

        time to cross, because the maximum speed of the particle
        inside of that interval is $1$.
        Similarly, the interval from 
        
        $$
        x = 0.25
        $$
        
        to
        
        $$
        x = 0.5
        $$
        
        takes _at least_
        
        $$
        {0.25 \over 0.5} = 0.5
        $$
        
        time to cross, because the maximum speed of the particle
        inside of that interval is $0.5$!
        And, again, the interval from 
        
        $$
        x = 0.125
        $$
        
        to
        
        $$
        x = 0.25
        $$
        
        takes at least
        
        $$
        {0.125 \over 0.25} = 0.5
        $$
        
        time to cross, because the maximum speed of the particle
        inside of that interval is $0.25$. Etc—each interval 
        takes _at least_
        
        $$
        0.5
        $$
        
        units of time to cross, because the length of each
        interval is half of the maximum speed within the interval!
        But there are infinitely many intervals, and, therefore,
        it takes
        
        __at least__
        
        infinitely much time to make it to $x = 0$, where the
        “infinitely” comes from adding infinitely many $0.5$'s
        together! (In other words, the particle never makes it to
        $x = 0$, no matter how far back in time you look.)

        A so-called

        __geometric progression__
        
        is a sequence of numbers in which each number
        is a fixed multiple of the previous number.
        For example,
        
        $$
        100,\, 300,\, 900,\, 2700
        $$
        
        is a (finite) geometric progression, because each number
        is the previous number multiplied by $3$,
        and
        
        $$
        1,\, 0.5,\, 0.25,\, 0.125,\, 0.0625,\, \ldots
        $$
        
        is an (infinite) geometric progression, because each number
        is the previous multiplied by $0.5$.


|> Exercise

    Take a system of two particles on the real
    line; at time $t = 0$, the first particle (yellow)
    is at $x = -1$, while the second one (blue)
    is at $x = 1$:

    |> Image
        src images/svg_ch4_three_one_dimensional_particles.svg

    If the velocity of the yellow particle is set to track
    the position of the blue particle and vice-versa, 
    give a qualitative sketch of the
    position-as-a-function-of-time (time on the $x$
    axis, position on the $y$ axis) of the two 
    particles. If we add also the graph of the position
    of the red particle from Exercise 41 to this set of
    graphs, what symmetries
    exist altogether between the three graphs?

    |> Solution

        For $t > 0$ the yellow and blue particles
        approach $0$ in a kind of “radioactive decay”
        pattern; for $t < 0$,
        they spin
        off to $-\infty$ and $\infty$ respectively at an
        accelerating rate:

        |> Image
            src images/svg_ch4_three_one_dimensional_particles_blue_yellow.svg

        If we add the graph of the red particle to the
        mix, it is simply the mirror image of the
        blue particle's position through the $y$ axis
        ($y$ axis that is ironically labeled “$x$”):
    
        |> Image
            src images/svg_ch4_three_one_dimensional_particles_blue_yellow_red.svg

        Indeed, for the red graph,

        __the slope equals the $y$-value__

        (velocity = position), while

        __the slope equals minus the $y$-value__

        for the blue graph (velocity = position
        of yellow = minus own position). 
        (We forgot to mention that the blue and
        yellow graphs are mirror images of one another
        through the horizontal axis—this is one of the 
        “symmetries” that the problem statement asks about, though.)
        As taking a mirror image through the $y$ axis
        negates slopes without affecting $y$-coordinates,
        while both the blue and red graphs have the same
        value at $t = 0$, this explains why the mirror
        image of one graph fits the constraints of the other
        and vice-versa:

        |> Image
            src images/svg_ch4_three_one_dimensional_particles_red_and_blue_symmetry.svg
    
        Also note that all graphs have slopes of
        $
        \pm 1
        $
        at $t = 0$, as we tried to reflect in the sketches, 
        because each corresponding particle position 
        is either $1$ or $-1$.

|> Exercise

    Four particles are placed 
    at intervals of $90^\circ$
    around a circle of radius $1$ 
    centered at $(2, 4)$ in the plane:

    |> Image
        src images/svg_ch4_circle_at_2_4.svg

    The velocity of each particle is set to 
    the position of the next particle clockwise
    around the circle, with this relationship
    maintained at all points in time.
    If the configuration above shows time 
    $t = 0$, how will the positions of the
    particles evolve? Discuss both positive and 
    negative values of $t$.

    |> Solution

        At $t = 0$
        the particles have position vectors that
        are up and the right,
        so the velocity vectors will be
        up and to the right,
        so the particles will move more up and
        to the right, and the velocity vectors
        will become more “up and to the right”, 
        and so on—broadly speaking, for $t > 0$
        there will ensue
        a kind of 
        four-particle explosion
        that goes up and to the right,
        off to $(+\infty, +\infty)$.

        |> ImageRight
            src images/svg_ch4_circle_at_2_4_with_some_vectors_cloud.svg

        For $t < 0$ motion will be
        down and to the left, at least initially—it
        is hard to forecast off the top of one's
        head (unless you have a sudden flash of insight)
        what will happen for larger negative
        values of $t$.

        HOWEVER.

        It is possible to say much more.

        To go deeper, we introduce eight new
        particles, comprising the original colors but
        in white and black flavors:

        |> Center

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1white_inline.svg
            , 
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2white_inline.svg
            , 
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3white_inline.svg
            , 
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4white_inline.svg
            , 
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1black_inline.svg
            , 
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2black_inline.svg
            , 
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3black_inline.svg
            , 
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4black_inline.svg

        At $t = 0$ the white particles are just a
        translate of the original particles, such
        that the circle on which they lie is 
        centered at $(0, 0)$:

        |> Image
            src images/svg_ch4_circle_at_2_4_white_version.svg
        
        The black particles, for their part, are
        piled on top of one another at $(x, y) = (2, 4)$ at $t = 0$, 
        that we draw as four quarter-pies
        of different colors, like a UNO card:

        |> Image
            src images/svg_ch4_circle_at_2_4_black_version.svg

        Within each group we set the velocity of
        the purple particle to the position of the
        yellow particle, 
        the velocity of the yellow particle to the
        position of the green particle, and so on,
        like for the original set of particles.

        In this case the white particles will
        rotate at unit speed around their circle of
        radius $1$ centered at the origin, just like
        the particles discussed in the solution to
        Exercise 12, that obey a similar set of
        constraints
        (albeit with a different set of colors). 

        The black particles, for their part, 
        behave as a single fused-together particle
        whose velocity is equal to its position,
        and will
        see their motion confined to an infinite 
        half-line through
        $(0, 0)$ and $(2, 4)$, as their velocity—being
        equal to their position—stays parallel to the
        line between them and the origin, meaning
        they are “stuck” to that line.

        Also note that the

        __speed__

        of the black particles, being equal to the

        __length of the velocity vector__

        of said particles, is equal to the

        __length of the position vector__

        |> ImageRight
            src images/svg_ch4_circle_at_2_4_position_equals_velocity_cloud.svg

        of said particles, is equal to the

        __distance to the origin__

        of said particles,
        since the length of the position vector is
        the distance to the origin.

        This means that if we introduce gradations
        to the afore-mentioned half-line through $(0, 0)$
        and $(2, 4)$...

        |> Image
            src images/svg_ch4_circle_at_2_4_UNO_with_gradated_line.svg
        
        ...indicating the distance to the origin,
        the black particles behave like a 
        one-dimensional system comprising a single
        particle on a half-line (or entire line,
        it doesn't hurt) whose velocity is equal to
        its position on this line:

        |> Image
            src images/svg_ch4_circle_at_2_4_UNO_flat_half_line.svg
        
        The behavior of such a particle is
        identical to the behavior of the red
        particle from Exercise 41, 
        except that the current
        “UNO particle” has a slight head-start over the
        red particle from Exercise 41, being
        at position $x = 2\sqrt{5}$ 
        instead of at position $x = 1$ at $t = 0$.
        (!!)

        |> ImageLeft
            src images/svg_ch4_circle_at_2_4_sqrt_20_cloud.svg
        
        This describes an “understandable”
        behavior of the black and white particles.

        Next we write

        |> Center

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1white_inline.svg
            $_x$

        for the function that gives 
        the $x$-coordinate of the purple-white particle
        as a function of time (in more detail,

        |> Center

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1white_inline.svg
            $_x : \rr \ra \rr$

        to emphasize that WE ARE TALKING ABOUT A FUNCTION, e.g.,

        |> Center

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1white_inline.svg
            $_x(2)$

        is the $x$-coordinate of 
        the purple-white particle at
        $t = 2$, etc), and write

        |> Center

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1white_inline.svg
            $_y$

        for the function that gives
        the $y$-coordinate of the purple-white 
        particle as a function of time, and so on 
        for all the other particles.

        For example,

        |> Center

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1white_inline.svg
            $_x\,\!\!\!' = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2white_inline.svg
            $_x$

        because the rate of change of the 
        $x$-coordinate of the purple-white particle 
        is the value of the $x$-coordinate
        of the yellow-white particle; we also have

        |> Center

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1white_inline.svg
            $_x\,\!\!\!' + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1black_inline.svg
            $_x\,\!\!\!' = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2white_inline.svg
            $_x + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2black_inline.svg
            $_x$
        
        by adding two such equations together; this can also be
        written

        |> Center
            
            $($
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1white_inline.svg
            $_x + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1black_inline.svg
            $_x)' = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2white_inline.svg
            $_x + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2black_inline.svg
            $_x$
        
        by the sum rule; but this gives us an idea!; we can
        try to _define_ the original particles 
        |> InlineImage
            class ch4_inline_blackwhite_particles
            src images/svg_ch4_circle_at_2_4_p1_inline.svg
        , 
        |> InlineImage
            class ch4_inline_blackwhite_particles
            src images/svg_ch4_circle_at_2_4_p2_inline.svg
        , 
        |> InlineImage
            class ch4_inline_blackwhite_particles
            src images/svg_ch4_circle_at_2_4_p3_inline.svg
        , 
        |> InlineImage
            class ch4_inline_blackwhite_particles
            src images/svg_ch4_circle_at_2_4_p4_inline.svg
        &ensp;by setting...

        |> Center

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1_inline.svg
            $_x = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1white_inline.svg
            $_x + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1black_inline.svg
            $_x$
            
            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1_inline.svg
            $_y = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1white_inline.svg
            $_y + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1black_inline.svg
            $_y$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2_inline.svg
            $_x = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2white_inline.svg
            $_x + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2black_inline.svg
            $_x$
            
            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2_inline.svg
            $_y = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2white_inline.svg
            $_y + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2black_inline.svg
            $_y$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3_inline.svg
            $_x = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3white_inline.svg
            $_x + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3black_inline.svg
            $_x$
            
            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3_inline.svg
            $_y = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3white_inline.svg
            $_y + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3black_inline.svg
            $_y$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4_inline.svg
            $_x = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4white_inline.svg
            $_x + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4black_inline.svg
            $_x$
            
            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4_inline.svg
            $_y = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4white_inline.svg
            $_y + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4black_inline.svg
            $_y$

        ...and see if these definitions satisfy the constraints
        of the problem! (We momentarily have two different
        purple particles: the one from the problem statement, 
        and the one that we have just defined; but that's ok,
        as long as we are aware of this small semantic
        transgression, it is not such a big deal, 
        and we shall soon prove that these two
        particles are one and the same.)
        For starters...

        |> Center

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1_inline.svg
            $_x\,\!\!\!' = ($
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1white_inline.svg
            $_x + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1black_inline.svg
            $_x\!\rt{0.05})' = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2white_inline.svg
            $_x + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2black_inline.svg
            $_x =$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2_inline.svg
            $_x$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1_inline.svg
            $_y\,\!\!\!' = ($
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1white_inline.svg
            $_y + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1black_inline.svg
            $_y\!\rt{0.05})' = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2white_inline.svg
            $_y + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2black_inline.svg
            $_y =$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2_inline.svg
            $_y$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2_inline.svg
            $_x\,\!\!\!' = ($
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2white_inline.svg
            $_x + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2black_inline.svg
            $_x\!\rt{0.05})' = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3white_inline.svg
            $_x + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3black_inline.svg
            $_x =$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3_inline.svg
            $_x$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2_inline.svg
            $_y\,\!\!\!' = ($
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2white_inline.svg
            $_y + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2black_inline.svg
            $_y\!\rt{0.05})' = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3white_inline.svg
            $_y + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3black_inline.svg
            $_y =$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3_inline.svg
            $_y$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3_inline.svg
            $_x\,\!\!\!' = ($
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3white_inline.svg
            $_x + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3black_inline.svg
            $_x\!\rt{0.05})' = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4white_inline.svg
            $_x + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4black_inline.svg
            $_x =$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4_inline.svg
            $_x$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3_inline.svg
            $_y\,\!\!\!' = ($
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3white_inline.svg
            $_y + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3black_inline.svg
            $_y\!\rt{0.05})' = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4white_inline.svg
            $_y + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4black_inline.svg
            $_y =$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4_inline.svg
            $_y$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4_inline.svg
            $_x\,\!\!\!' = ($
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4white_inline.svg
            $_x + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4black_inline.svg
            $_x\!\rt{0.05})' = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1white_inline.svg
            $_x + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1black_inline.svg
            $_x =$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1_inline.svg
            $_x$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4_inline.svg
            $_y\,\!\!\!' = ($
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4white_inline.svg
            $_y + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4black_inline.svg
            $_y\!\rt{0.05})' = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1white_inline.svg
            $_y + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1black_inline.svg
            $_y =$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1_inline.svg
            $_y$

        ...or...

        |> Center

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1_inline.svg
            $_x\,\!\!\!' = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2_inline.svg
            $_x$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1_inline.svg
            $_y\,\!\!\!' = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2_inline.svg
            $_y$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2_inline.svg
            $_x\,\!\!\!' = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3_inline.svg
            $_x$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2_inline.svg
            $_y\,\!\!\!' = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3_inline.svg
            $_y$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3_inline.svg
            $_x\,\!\!\!' = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4_inline.svg
            $_x$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3_inline.svg
            $_y\,\!\!\!' = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4_inline.svg
            $_y$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4_inline.svg
            $_x\,\!\!\!' = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1_inline.svg
            $_x$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4_inline.svg
            $_y\,\!\!\!' = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1_inline.svg
            $_y$

        ...cutting out the middle computation, so the constraints
        relating particle velocities to particle positions are
        satisfied (e.g., the velocity vector of the purple particle
        is the position vector of the yellow particle); 
        for seconders, evaluating these definitions at $t = 0$ gives...

        |> Center

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1_inline.svg
            $_x(0) = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1white_inline.svg
            $_x(0) + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1black_inline.svg
            $_x(0) = 2\,\,+$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1white_inline.svg
            $_x(0)$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1_inline.svg
            $_y(0) = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1white_inline.svg
            $_y(0) + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1black_inline.svg
            $_y(0) = 2\,\,+$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1white_inline.svg
            $_y(0)$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2_inline.svg
            $_x(0) = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2white_inline.svg
            $_x(0) + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2black_inline.svg
            $_x(0) = 2\,\,+$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2white_inline.svg
            $_x(0)$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2_inline.svg
            $_y(0) = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2white_inline.svg
            $_y(0) + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2black_inline.svg
            $_y(0) = 2\,\,+$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2white_inline.svg
            $_y(0)$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3_inline.svg
            $_x(0) = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3white_inline.svg
            $_x(0) + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3black_inline.svg
            $_x(0) = 2\,\,+$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3white_inline.svg
            $_x(0)$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3_inline.svg
            $_y(0) = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3white_inline.svg
            $_y(0) + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3black_inline.svg
            $_y(0) = 2\,\,+$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3white_inline.svg
            $_y(0)$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4_inline.svg
            $_x(0) = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4white_inline.svg
            $_x(0) + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4black_inline.svg
            $_x(0) = 2\,\,+$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4white_inline.svg
            $_x(0)$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4_inline.svg
            $_y(0) = $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4white_inline.svg
            $_y(0) + $
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4black_inline.svg
            $_y(0) = 2\,\,+$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4white_inline.svg
            $_y(0)$

        ...or...

        |> Center

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1_inline.svg
            $_x(0) = 2\,\,+$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1white_inline.svg
            $_x(0)$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1_inline.svg
            $_y(0) = 2\,\,+$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p1white_inline.svg
            $_y(0)$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2_inline.svg
            $_x(0) = 2\,\,+$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2white_inline.svg
            $_x(0)$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2_inline.svg
            $_y(0) = 2\,\,+$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p2white_inline.svg
            $_y(0)$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3_inline.svg
            $_x(0) = 2\,\,+$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3white_inline.svg
            $_x(0)$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3_inline.svg
            $_y(0) = 2\,\,+$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p3white_inline.svg
            $_y(0)$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4_inline.svg
            $_x(0) = 2\,\,+$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4white_inline.svg
            $_x(0)$

            |> br

            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4_inline.svg
            $_y(0) = 2\,\,+$
            |> InlineImage
                class ch4_inline_blackwhite_particles
                src images/svg_ch4_circle_at_2_4_p4white_inline.svg
            $_y(0)$

        ...cutting out the middle computation, which is to
        say that the positions at time $t = 0$ of our 
        newly-defined particles 
        |> InlineImage
            class ch4_inline_blackwhite_particles
            src images/svg_ch4_circle_at_2_4_p1_inline.svg
        , 
        |> InlineImage
            class ch4_inline_blackwhite_particles
            src images/svg_ch4_circle_at_2_4_p2_inline.svg
        , 
        |> InlineImage
            class ch4_inline_blackwhite_particles
            src images/svg_ch4_circle_at_2_4_p3_inline.svg
        &ensp;and
        |> InlineImage
            class ch4_inline_blackwhite_particles
            src images/svg_ch4_circle_at_2_4_p4_inline.svg
        &ensp;are the translate of the white particle
        positions at $t = 0$ back up and to the right
        by the vector $(2, 4)$, which brings those
        positions back to the original positions of 
        |> InlineImage
            class ch4_inline_blackwhite_particles
            src images/svg_ch4_circle_at_2_4_p1_inline.svg
        , 
        |> InlineImage
            class ch4_inline_blackwhite_particles
            src images/svg_ch4_circle_at_2_4_p2_inline.svg
        , 
        |> InlineImage
            class ch4_inline_blackwhite_particles
            src images/svg_ch4_circle_at_2_4_p3_inline.svg
        &ensp;and 
        |> InlineImage
            class ch4_inline_blackwhite_particles
            src images/svg_ch4_circle_at_2_4_p4_inline.svg
        &ensp;as they appear in the problem statement! I.e.,
        our newly-defined
        particles 
        |> InlineImage
            class ch4_inline_blackwhite_particles
            src images/svg_ch4_circle_at_2_4_p1_inline.svg
        , 
        |> InlineImage
            class ch4_inline_blackwhite_particles
            src images/svg_ch4_circle_at_2_4_p2_inline.svg
        , 
        |> InlineImage
            class ch4_inline_blackwhite_particles
            src images/svg_ch4_circle_at_2_4_p3_inline.svg
        &ensp;and 
        |> InlineImage
            class ch4_inline_blackwhite_particles
            src images/svg_ch4_circle_at_2_4_p4_inline.svg
        &ensp;are in the desired place at $t = 0$!

        In other words, the proposed definitions of 
        |> InlineImage
            class ch4_inline_blackwhite_particles
            src images/svg_ch4_circle_at_2_4_p1_inline.svg
        , 
        |> InlineImage
            class ch4_inline_blackwhite_particles
            src images/svg_ch4_circle_at_2_4_p2_inline.svg
        , 
        |> InlineImage
            class ch4_inline_blackwhite_particles
            src images/svg_ch4_circle_at_2_4_p3_inline.svg
        &ensp;and 
        |> InlineImage
            class ch4_inline_blackwhite_particles
            src images/svg_ch4_circle_at_2_4_p4_inline.svg
        &ensp;“work” in the sense of satisfying all the conditions
        of the problem statement, and are, indeed, the
        solution we seek.

        Qualitatively, this implies that the particles
        can be understood as four particles rotating at 
        unit speed around a circle of radius $1$ (the 
        white particles) where the center of circle (the 
        UNO particle) is moving at exponential rate along
        a half-line. In particular, the particles remain
        at constant distance from one another for all $t$,
        whether that seems intuitive or not.

        Concretely, 
        the particle trajectories
        end up like so, locally around $t = 0$:
        
        |> Image
            src images/svg_ch4_circle_at_2_4_solution_curves.svg
        
        The above plot goes from $t = -5$ to 
        $t \approx 1$—winding further back in time would produce
        near-perfect counterclockwise circular motion,
        as the black particles rush up to $(0, 0)$
        and come to a near-halt rather fast,
        leaving only the residual motion of the white
        particles!

        _Note 1._
        When we said that, for $t > 0$, there ensues
        
        -|“a [kind of] four-particle explosion”|-
        
        in the first paragraph of the solution, the word 
        “explosion” might be misleading, implying increased
        distances between the particles over time. This is
        not the case! (But we didn't know any better, back
        then.)

        |> Pause
        _Note 2._
        As you might already have caught on, but is
        maybe worth emphasizing,

        __speed__

        is not the same thing as

        __velocity__

        because, specifically, speed is

        __distance per unit time__

        —a nonnegative number—whereas velocity is

        __displacement per unit time__

        —a vector-valued quantity, or $\pm$-valued quantity,
        in 1 dimension!


|> Exercise

    Find a nonzero function $f$ and a nonzero
    constant $a \in \rr$ such that
    
    $$
    f'(x) = f(x + a)
    $$
    
    for all $x$. 

    |> Solution

        Recall the curves from Exercise 12:

        |> Image
            src images/svg_ch4_cosine_recall.svg

        The blue curve is the derivative of the red curve
        but is also the horizontal translate of the red curve by
        $a$ units to the left, where

        $$
        a
        $$

        |> ImageLeft
            src images/svg_ch4_cosine_recall_cloud.svg

        is the distance between adjacent bumps. Thus if

        $$
        f
        $$

        is the function that generates the red curve, then

        $$
        f'(x) = f(x + a)
        $$

        using the fact that

        $$
        y = f(x + a)
        $$

        is the horizontal translate of $y = f(x)$ by $a$ 
        units to the left, in general for any function
        $f$ and constant $a \in \rr$, as discussed 
        in Exercise 14 of Chapter 3. (Well, this shows
        one solution, at least.)


|> Exercise

    Express the...

    |> ul
        style margin:1em 0 0 2.5em

        |> li

            associativity of function composition
        |> li

            associativity of function kmultiplication
        |> li

            associativity of function addition</li>
        |> li

            commutativity of function multiplication</li>
        |> li

            commutativity of function addition</li>

    ...as well as the..

    |> ul
        style margin:1em 0 0 2.5em

        |> li
            
            associativity of real number multiplication
        |> li
            
            associativity of real number addition
        |> li
            
            commutativity of real number multiplication
        |> li
            
            commutativity of real number addition

    ...in the form of self-contained, formal statements.

    |> Solution

        For the functions:

        |> ul
            style margin:1em 0 0 2.5em

            |> li

                _the associativity of function multiplication is
                the fact that $(f \circ g) \circ h = f \circ (g \circ h)$
                for all functions $f$, $g$, $h$ such that 
                $h : D \ra C$, $g : C \ra B$, $f : B \ra A$ 
                [for arbitrary sets $A$, $B$, $C$, $D$]_

            |> li
            
                _the associativity of function multiplication is
                the fact that $f(gh) = (fg)h$ for all $f, g, h : \rr \ra \rr$_

            |> li
            
                _the associativity of function addition is
                the fact that $f + (g + h) = (f + g) + h$ for all $f, g, h : \rr \ra \rr$_

            |> li
            
                _the commutativity of function multiplication is
                the fact that $fg = gf$ for all $f, g : \rr \ra \rr$_

            |> li
            
                _the commutativity of function addition is
                the fact that $f + g = g + f$ for all $f, g : \rr \ra \rr$_

        |> ImageLeft
            src images/svg_ch4_express_the_f_g_h_A_B_C_D_cloud.svg

        For the real numbers:

        |> ul
            style margin:1em 0 0 2.5em

            |> li
            
                _the associativity of [real number] multiplication is
                the fact that $a(bc) = (ab)c$ for all $a, b, c \in \rr$_

            |> li
            
                _the associativity of [real number] addition is
                the fact that $a + (b + c) = (a + b) + c$ for all $a, b, c \in \rr$_

            |> li
            
                _the commutativity of [real number] multiplication is
                the fact that $ab = ba$ for all $a, b \in \rr$_

            |> li
            
                _the commutativity of [real number] addition is
                the fact that $a + b = b + a$ for all $a, b \in \rr$_

        |> Pause
        _Note 1._
        We never took the time to prove the associativity
        of function addition, but it is easy to prove! 
        (For other proofs see Exercise 32, Exercise 33, as
        well as Exercise 9, Chapter 3.)