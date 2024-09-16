/*
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
*/