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