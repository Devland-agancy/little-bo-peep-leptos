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