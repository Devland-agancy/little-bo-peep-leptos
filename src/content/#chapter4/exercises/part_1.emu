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
