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