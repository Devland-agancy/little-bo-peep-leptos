/*
|> Image
    src images/svg_ch4_polaroids.svg

|> Section    

    *Definitions.*
    The _derivative_ of a function
    
    $$
    f : \rr \ra \rr
    $$
    
    is a (new) function
    
    $$
    f' : \rr \ra \rr
    $$
    
    that gives the slope of $f$ at each point. 
    In other words
    
    $$
    f'(a)
    $$
    
    is the slope of the graph $y = f(x)$ at
    $x = a$. And—surprise!—each pair of graphs
    above is a pair of the form $y = f(x)$ [$=$
    “before”], $y = f'(x)$ [$=$ “after”]. (Meaning,
    the “after” graph records the slope of the 
    “before” graph.) E.g.:

    |> Image
        src images/svg_ch4_explanation1.svg

    Note that $f'\!$ (read “$f$ prime”) remains 
    undefined where $y = f(x)$ has a sharp “corner” 
    and no well-defined slope. By opposition, if 
    there is a well-defined tangent line to 
    $y = f(x)$ at $x = a$ the slope of this
    tangent line supplies the value of $f'(a)$:

    |> Image
        src images/svg_ch4_explanation2.svg

    |> ImageRight
        offset_y 10%
        src images/svg_ch4_explanation2_cloud.svg

    In fact,
    we can
    succinctly describe the derivative by...

    $$
    f'(a) = \te{[slope of tangent line to $y = f(x)$ at $x = a$]}
    $$

    ...with the understanding that $f'(a)$ is 
    undefined if a tangent line does not exist 
    or if the tangent is vertical. But for one 
    last asterisk—and speaking of the existence,
    or not, of the tangent—note that the endpoint
    of a curve does not count as having a tangent,
    and therefore leaves a missing value for the 
    derivative:

    |> Image
        src images/svg_ch4_explanation_one_sided_tangent.svg

    (In other words, “half-tangents” do not actually 
    count as tangents.) 

|> Section

    *Vocabulary.*
    A function $f : \rr \ra \rr$ is

    __differentiable__

    if $\dom\,f' = \dom\,f$. Also, if $a, b \in \rr$, 
    $a < b$, $f$ is
    
    __differentiable on $[a,b]$__
    
    if $[a,b] \subseteq \dom \,f'$. Lastly, $f$ is
    
    __differentiable at $a$__
    
    if $a \in \dom\,f'$.

|> Section

    *Sketching a Derivative.*
    Say that you would like to sketch the derivative
    of the “before” function from the last “before”/“after” pair above
    (the one with the closed endpoints):

    |> Image
        src images/svg_ch4_curve_to_sketch.svg

    One method is simply to eyeball the slope at 
    a few points along the curve, plot these 
    values and interpolate:
    
    |> Image
        src images/svg_ch4_first_derivative_sketch.svg

    |> ImageRight
        src images/svg_ch4_polaroid_cloud.svg
    
    ...voilà!
    
    An alternate approach is to start by
    determining intervals on which the derivative
    is positive and negative, and then to 
    interpolate via the largest (respectively, 
    smallest) value of the derivative in each 
    interval:

    |> Image
        src images/svg_ch4_second_derivative_sketch_pt1.svg

    |> Image
        src images/svg_ch4_second_derivative_sketch_pt2.svg

    The result (at bottom right)
    is a charming “robosketch” of the true
    derivative! (Well, charming in our opinion,
    at least.)


|> Section

    *Derivative of a constant function.*
    A constant function is a function of the form 
    $$
    x \ra B
    $$
    for some $B \in \rr$ independent of $x$.
    The graph of the constant function is the line 
    $$
    y = B
    $$
    of slope $0$. So

    $$
    (x \ra B)' = (x \ra 0)
    $$

    |> ImageLeft
        src images/svg_ch4_constant_derivative_cloud.svg

    because at each $x$-value you find a slope of
    $0$, when you look up (down?) at the graph.

    If we refer to
    $$
    x \ra 0
    $$
    as the

    __zero function__

    we can summarize the situation by saying that

    __~ the derivative of a constant function
    is the zero function ~__
    
    or, more shortly,
    
    __~ the derivative of a constant is zero ~__
 
    (the way people usually state it).

|> Section

    *Derivative of an affine function.*
    An affine function is a function of the form
    $$
    x \ra Ax + B
    $$
    for constants $A$, $B \in \rr$.
    The graph of $x \ra Ax + B$ is a line of 
    slope $A$, so

    $$
    (x \ra Ax + B)' = (x \ra A)
    $$

    |> ImageLeft
        src images/svg_ch4_affine_derivative_cloud.svg

    because the slope of a line of slope $A$ is
    $A$, no matter where you put yourself on the
    line. In particular, $B$ plays no role in the
    derivative! ($\,$Just like in the case of a
    constant function, the derivative leaves no
    trace of $B$'s value—and for the same reason
    that $B$ effects a vertical translation, which
    does not change the slope of anything.)
    
    In words:
    
    __~ the derivative of the affine function 
    $y = ax + b$ is the constant function $y = a$ ~__

    Or, flexing our linguistic prowess a tad more:

    __~ the derivative of an affine function is 
    the coefficient of its linear term ~__
    
    (The “linear term” of $y = ax + b$ is $ax$, 
    of coefficient $a$.)

    |> Example

        One has
        $$
        (x \ra 3x + 1)' = (x \ra 3)
        $$
        as per
        $$
        (x \ra Ax + B)' = (x \ra A)
        $$
        with $A = 3$, $B = 1$.

    |> Example

        One has
        $$
        (x \ra 12 - x)' = (x \ra -1)
        $$
        as per
        $$
        (x \ra Ax + B)' = (x \ra A)
        $$
        with $A = -1$, $B = 12$.

|> Section

    *Units of the Derivative.*
    If units are present, we have
    $$
    \te{$y$ axis units for $f'$} \,= {\te{$y$ axis units for $\f$} \over \te{$x$ axis units for $\f$}}
    $$
    because a value output by $\f'$ is the 
    _slope_ of a tangent line attached to the 
    graph $y = f(x)$, and
    $$
    \te{$x$ axis units for $f'$}\, = \rt{0.02}\,\te{$x$ axis units for $f$}
    $$
    because an input for $\f'$ is, originally, 
    an input for $\f$.

    For example, if the “before” graph has units 
    of...

    |> ul

        |> li

            seconds on the $x$ axis, meters on the 
            $y$ axis

    |> ImageRight
        src images/svg_ch4_units1_cloud.svg
    
    ...then the “after” graph will have units of...

    |> ul
    
        |> li

            seconds on the $x$ axis, meters per
            second on the $y$ axis

    ...while if the “before” graph has units of...
    
    |> ul
    
        |> li

            years on the $x$ axis, dollars on the
            $y$ axis
    
    |> ImageLeft
        src images/svg_ch4_units2_cloud.svg

    ...then the “after” graph will have units of...

    |> ul
        |> li
        
            years on the $x$ axis, dollars per
            year on the $y$ axis

    ...and so on.

    Units might additionally prompt us to refer
    to $f'$ as <br>the

    __rate of change__
    
    of $f$, or, depending, as the

    __instantaneous__

    rate of change of $f$. The latter bit of
    emphasis has to do with the fact that, in a
    general graph, the slope of the tangent
    keeps changing from point to point.


|> Section

    *The second derivative.*
    The _second derivative_ of $f$ is the derivative
    of the derivative of $f$. It is written “$f''$”:

    $$
    \,\,\,f'' = (f')'.
    $$

    Likewise, we have, e.g.,

    $$
    \begin{align}
    \up{0.95}f''' &= (f'')'\\
    \up{1.25}f'''' &= (f''')'\\
    \up{1.25}f''''\psa' &= (f'''')'\\
    \end{align}
    $$

    these being, namely, the _third_, _fourth_
    and _fifth_ derivatives of $f$. One can also 
    write
    
    $$
    f^{(n)}
    $$
    
    for the $n$-th derivative of $f$, so that, for
    example,

    $$
    f^{(7)}
    $$

    means the same as

    $$
    f'''''''
    $$

    but with the advantage that you don't have to
    squint and start re-counting the apostrophes 
    several times over.

    |> Example

        We have

        $$
        (x \ra 3x + 1)'' = (x \ra 0)
        $$

        |> ImageRight
            src images/svg_ch4_3x_plus_one_and_second_cloud.svg

    because, firstly,

    $$
    (x \ra 3x + 1)' = (x \ra 3)
    $$

    and, secondly,

    $$
    (x \ra 3)' = (x \ra 0)
    $$

    so that, from start to finish,

    $$
    (x \ra 3x + 1)'' =  ((x \ra 3x + 1)')' = (x \ra 3)' = (x \ra 0)
    $$

    where we unpeel the onion starting from the
    inside. (Physically difficult.)

    |> Example

        More generally,

        $$
        \,\,\,(x \ra ax + b)'' = (x \ra 0)
        $$

        for all $a, b \in \rr$, by a similar computation;
        a.k.a.:

        __~ the second derivative of an affine function is zero ~__
            
        |> ImageLeft
            src images/svg_ch4_adding_statements_cloud.svg

|> Section

    *Geometric interpretation of the second derivative.*
    The sign of the second derivative—whether
    it is positive or negative—indicates whether
    a graph is “bending upwards” or “bending 
    downwards”. Upward-bending graphs have a positive
    second derivative, whereas downward-bending graphs
    have a negative second derivative:

    |> Image
        src images/svg_ch4_bendiness.svg

    Reason like this: the second derivative is 
    |> del

        “the rate of change of the rate of change”.
    Sorry: “the rate of change of the slope”. 
    (Same difference.) Ergo, if the second derivative
    is positive, the slope is increasing; if the
    second derivative is negative, the slope is 
    decreasing. Moreover, an
        
    __increasing__

    slope gives curves a “bending upwards” shape, while a

    __decreasing__
    
    slope gives curves a “bending downward” shape!
    
    To emphasize, if the second derivative is some

    -|LARGE POSITIVE NUMBER|-

    then the slope is increasing at that rate, 
    which could result in a sharp bend upwards
    in the graph (unless you are near vertical
    already—you can't see the difference between
    slope $100$ and slope $1000$ very well, at most
    scales—nor between $-1000$ and $-100$, for that
    matter).

    Likewise, if the second derivative is some
    
    -|LARGE NEGATIVE NUMBER|-

    then the slope is decreasing at [the absolute 
    value of] that rate, which could result in 
    a sharp bend downwards in the graph (unless
    you are near vertical already, once again,
    because verticality can disguise the presence
    of a significant change in slope, once again).


|> Section

    *Vocabulary #1.*
    Curves with increasing (technically: 
    _nondecreasing_) slope are called _convex_, 
    while curves with decreasing (technically: 
    _nonincreasing_) slope are called _concave_.
    Viz:
    
    |> Image
        src images/svg_ch4_convex_concave.svg

    *Vocabulary #2.*
    An _inflection point_ is a point at the
    interface between convex and concave sections
    of a graph:

    |> Image
        src images/svg_ch4_inflection_point.svg

    |> Example

        The fact that
        
        $$
        (x \ra 3x + 1)'' = (x \ra 0)
        $$
        
        indicates that the graph
        
        $$
        y = 3x + 1
        $$
        
        is neither “bending upwards” nor “bending 
        downwards”—$0$ is neither positive, nor negative.


*/