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