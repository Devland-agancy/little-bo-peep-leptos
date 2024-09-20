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
