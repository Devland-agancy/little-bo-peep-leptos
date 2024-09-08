/*
|> Image
    src images/svg_ch4_polaroids.svg

|> Exercises

    |> Exercise

        What does...

        $$
        A_1(t_0 + h) \,\times\, \dots \,\times\, A_{i-1}(t_0 + h) \,\times\, {A_i(t_0 + h) - A_i(t_0)\over h} \,\times\, A_{i + 1}(t_0) \,\times\, \cdots  \,\times\, A_n(t_0)
        $$

        |> ImageRight
            src images/svg_ch4_big_product_parchment_cloud.svg
            edge formula_edge
        
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

            |> div
                align center

                |> InlineImage
                    class ch4_pacman1_inline_number_pellet
                    src images/svg_ch4_pacman1_pellet1_inline.svg
                    style margin-right:1em
                $-6.75\te{c}\hlfbk/\hlfbk\te{s}\,\,\times\,\,0.5\te{s}\,\,=\,\,-3.375\,\te{cells}$
                
            |> ImageRight
                src images/svg_ch4_pacman1_cells_per_second_times_seconds_cloud.svg

            |> div
                align center

                |> InlineImage
                    class ch4_pacman1_inline_number_pellet
                    src images/svg_ch4_pacman1_pellet2_inline.svg
                    style margin-right:1em
                $-6.75\te{c}\hlfbk/\hlfbk\te{s}\,\,\times\,\,0.4\te{s}\,\,=\,\,-2.7\,\te{cells}$

            |> div
                align center

                |> InlineImage
                    class ch4_pacman1_inline_number_pellet
                    src images/svg_ch4_pacman1_pellet3_inline.svg
                    style margin-right:1em
                $6.75\te{c}\hlfbk/\hlfbk\te{s}\,\,\times\,\,0.9\te{s}\,\,=\,\,6.075\,\te{cells}$

            |> div
                align center

                |> InlineImage
                    class ch4_pacman1_inline_number_pellet
                    src images/svg_ch4_pacman1_pellet4_inline.svg
                    style margin-right:1em
                $6.75\te{c}\hlfbk/\hlfbk\te{s}\,\,\times\,\,0.4\te{s}\,\,=\,\,2.7\,\te{cells}$

            |> div
                align center

                |> InlineImage
                    class ch4_pacman1_inline_number_pellet
                    src images/svg_ch4_pacman1_pellet5_inline.svg
                    style margin-right:1em
                $-6.75\te{c}\hlfbk/\hlfbk\te{s}\,\,\times\,\,0.4\te{s}\,\,=\,\,-2.7\,\te{cells}$

            |> div
                align center

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

            |> div
                align center

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

            |> div
                align center

                |> InlineImage
                    class ch4_inline_blackwhite_particles
                    src images/svg_ch4_circle_at_2_4_p1white_inline.svg
                $_x$

            for the function that gives 
            the $x$-coordinate of the purple-white particle
            as a function of time (in more detail,

            |> div
                align center

                |> InlineImage
                    class ch4_inline_blackwhite_particles
                    src images/svg_ch4_circle_at_2_4_p1white_inline.svg
                $_x : \rr \ra \rr$

            to emphasize that WE ARE TALKING ABOUT A FUNCTION, e.g.,

            |> div
                align center

                |> InlineImage
                    class ch4_inline_blackwhite_particles
                    src images/svg_ch4_circle_at_2_4_p1white_inline.svg
                $_x(2)$

            is the $x$-coordinate of 
            the purple-white particle at
            $t = 2$, etc), and write

            |> div
                align center

                |> InlineImage
                    class ch4_inline_blackwhite_particles
                    src images/svg_ch4_circle_at_2_4_p1white_inline.svg
                $_y$

            for the function that gives
            the $y$-coordinate of the purple-white 
            particle as a function of time, and so on 
            for all the other particles.

            For example,

            |> div
                align center

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

            |> div
                align center

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

            |> div
                align center
                
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

            |> div
                align center

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

            |> div
                align center

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

            |> div
                align center

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

            |> div
                align center

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

            |> div
                align center

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
*/