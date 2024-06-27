/*
|> Section
    
    *Syntax.*
    A

    __function__

    is a “rule” for transforming inputs (usually
    numbers) into outputs (usually numbers as well).
    One can think of a function as a box with an
    “input tube” and an “output tube”:
    
    |> Image
        src images/svg_f_box.svg
        container_classes mb-4

    An input goes in via the input tube, is
    processed according to the function's rule,
    and the result comes out the other side.
    (Metaphorically speaking.)

    In the above picture, the name of the function
    is “$f$”.

    Notation-wise, one writes

    $$
    {f(x)}
    $$

    (which is read “$f$ of $x$”, and that's 
    important) for the result of passing an input
    $x$ to a function $f$. For example, if the rule
    according to which $f$ processes inputs is

    __the output is the square of the input__

    then

    $$
    {f(2) = 4}
    $$

    [“$f$ of $2$ equals $4$”] because $2^2 = 4$, and

    $$
    {f(3) = 9}
    $$

    [“$f$ of $3$ equals $9$”] because $3^2 = 9$, and
    
    $$
    { f(0.1) = 0.01}
    $$

    [...] because $0.1^2 = 0.01$, and so on. Also,

    $$
    {f(x) = x^2}
    $$

    [“$f$ of $x$ equals $x^2$”] more generally, which
    is actually the

    __definition__

    of $f$!! (Stated algebraically.)


|> Section

    *Lambda functions.*
    A

    __lambda function__

    is not a type of function, but a type of
    notation  that enables one to define a function 
    without giving it a name, such as “$f$”. In fact 
    there are two different mainstream notations, in 
    this instance. One notation writes

    $$
    \lambda{x}.x^2
    $$

    to mean “the function that maps $x$ to $x^2$” 
    (and by the way,

    $$
    \lambda{z}.z^2
    $$

    is _the same_ function, because 
    it specifies the same in-out mapping—a thing 
    goes to its square—also by the way, the symbol

    $$
    {\Huge \lambda}
    $$

    is the Greek letter “lambda”, giving its name 
    to the topic) while the other notation writes 

    $$
    x \rightarrow x^2
    $$

    to mean the same thing.

    Note that

    $$
    (x \ra x^2)(0.1)
    $$

    means “the function that maps each number to its
    square, of $0.1$”. So...

    $$
    (x \ra x^2)(0.1) = 0.01
    $$

    ...the same as an equation of the form 
    “$f(\dots) = \dots$.

    For more practice:

    $$
    (\lambda x.x^3)(10) = 1000
    $$
    $$
    (\lambda u.u^5)(10) = 100000
    $$
    $$
    (v \ra v^2)(10) = 100
    $$
    $$
    (z \ra z^3)(10) + (t \ra t^2)(5) = 1025.
    $$

    (Etc.) (Indeed, to emphasize again, the variable 
    denoting the input does not matter: it is just a 
    placeholder, and you obtain the same output, and 
    the same _function_, no matter what symbol 
    you choose.\*) (\*As long as you don't collide 
    with other existing variable names.)


|> Section

    *Definition by cases.*
    Sometimes a function is defined by an expression 
    of the form
    
    $$
    x \ra \begin{cases}
    \ldots & \te{if $\ldots$}\\
    \ldots & \te{if $\ldots$}\\
    \vdots & \vdots\\
    \ldots & \te{$\ldots$}
    \end{cases}
    $$
    
    where the right-hand side is a list of mutually 
    exclusive cases to consider according to the 
    value of $x$. Equivalently,
    
    $$
    g(x) = \begin{cases}
    \ldots & \te{if $\ldots$}\\
    \ldots & \te{if $\ldots$}\\
    \vdots & \vdots\\
    \ldots & \te{$\ldots$}
    \end{cases}
    $$
    
    in the case where the function has a name, such
    as “$g$”.

    |> Example

        If VX-11/78A (don't mind the weird name, chosen
        at random) is the function defined by
        
        $$
        \te{VX-11/78A}(x) = \begin{cases} 
        3.5 & \te{if $x = 0$},\\
        2.5\up{1.1} & \te{if $x = 1$},\\
        \te{undefined}\up{1.1} & \te{if $x \ne 0, 1$}
        \end{cases}
        $$
        
        then
        
        $$
        \te{VX-11/78A}(0) = 3.5,
        $$
        
        and
        
        $$
        \te{VX-11/78A}(1) = 2.5,
        $$
        
        aaand... and these are the only two values of 
        $x$ for which VX-11/78A$(x)$ is defined, as 
        specified.


|> Section

     *On arbitrariness.*
    While a function such as VX-11/78A might seem 
    completely arbitrary, one lesson from the former 
    example is that functions _can_ be 
    completely arbitrary! In fact, there are only 
    two “ground rules” to respect in order for 
    something to qualify as a function: *(i)* to 
    output
        
    __one__
        
    output per (accepted) input, and *(ii)* to return 
    the
    
    __same__
        
    output each time on the same input. (Sometimes,
    functions are said to be
    
    __deterministic__
        
    because of *(ii)*.)


|> Section

    *Graphs.*
    The
    
    __graph__

    of a function is a visualization device. A point
    on the graph corresponds to an input for which
    the function is defined. The $x$-coordinate of 
    the point is the value of the input, while the 
    $y$-coordinate is the value of the corresponding 
    output.

    For example, here is a graph of VX-11/78A:

    |> Image
        src images/svg_vx1178A.svg

    The graph has only two points, because VX-78/11A 
    is defined at only two values. One point is...
        
    |> Image
        src images/svg_vx1178A_@0_with_cors.svg

    ...$(0, 3.5)$, because VX-78/11A maps $0$ to 
    $3.5$, while the other point is...
        
    |> Image
        src images/svg_vx1178A_@1_with_cors.svg

    ...$(1, 2.5)$, because VX-78/11A maps $1$ to $2.5$.

    |> Example

        Here is a graph of $x \ra x^2$ on the interval
        $[-1, 1]$ (meaning: going from $x = -1$ to 
        $x = 1$):

        |> Image
            src images/svg_x_squared_1_worked.svg

        Among all the points on this graph that we 
        could discuss, let us name, say, the point 
        $(0.75, 0.5625)$...

        |> Image
            src images/svg_x_squared_2_worked.svg

        ...which finds itself on the graph, namely, 
        because the square of $0.75$ is 
        $0.5625 = 9/16$.

        |> ImageLeft
            src images/svg_0.5625_cloud.svg
            line -1.0
            offset_y 1.7rem

|> Section

    *Domains.*
    The _domain_ of a function $f$—written

    $$
    \dom\, f
    $$

    —is the set of inputs $x$ for which $f(x)$ is
    defined.

    |> Example

        We have

        $$
        \dom\,\rt{0.1} \te{VX-11/78A} = \left\{ 0\rt{0.1}, 1 \right\}
        $$

        because VX-11/78A$(x)$ is only defined at $x = 0$, 
        $1$.

    |> Example

        If DM-1700 (another weirdly named function) is 
        defined by
        
        $$
        \te{DM-1700}(x) = 
        \begin{cases} 
        0             & \te{if $x \leq 0$ or $x \geq 1$},\\
        1 - x\up{1.1} & \te{if $0 < x < 1$}
        \end{cases}
        $$
        
        then
        
        $$
        \dom\,\rt{0.1} \te{DM-1700} = \rr
        $$
        
        because $\te{DM-1700}(x)$ is defined for all
        $x \in \rr$.

    |> Example

        If $g : \rr \ra \rr$ (we are going to explain 
        this notation imminently) is the function given 
        by
        
        $$
        g(x) = \sqrt{x - 1^{\color{white}*\!\!}}
        $$
        
        then
        
        $$
        \begin{align}
        \dom\, g &\,=\, [1, \infty)
        \end{align}
        $$
        
        because the square root of a number is defined 
        if and only if that number is _nonnegative_ 
        (i.e., we need $x - 1 \geq 0$ in order for $g(x)$ 
        to be defined, i.e., we need $x \geq 1$).

    |> Example 

        If $h : \rr \ra \rr$ is defined by

        $$
        h(x) = \frac{1}{x+1}
        $$

        then

        $$
        \begin{align}
        \dom\,h \,=\, \rr\back\{-1\} =\, (-\infty,-1) \cup (-1,\infty)
        \end{align}
        $$

        because $1/(x+1)$ is well-defined if and only if
        division by 0 is avoided, i.e., if and only if 
        $x \ne -1$.


|> Section

    *“From/To” Notation.*
    The notation

    $$
    f : \rr \ra \rr
    $$

    means that $f$ is a function

    __from $\rr$ to $\rr$__
        
    or, which is to say, that

    $$
    \dom f \subseteq \rr
    $$

    [translation: _the domain of $f$ is a subset of 
    the set of real numbers_] and that

    $$
    \{f(x) : x \in \dom f\} \subseteq \rr
    $$

    [translation: _the set of values output by $f$ 
    is a subset of the set of real numbers_].
       
    Generalizing,

    $$
    f : A \ra B
    $$

    means that

    $$
    \dom f \subseteq A
    $$

    (i.e., that $f$ only accepts values from $A$) and 
    that

    $$
    \{f(x) : x \in \dom f\} \subseteq B
    $$

    (i.e., that $f$ only outputs values from $B$), 
    following the pattern above. 


|> Section

    *The Vertical Line Test.*
    As it turns out, the term “graph” just means 
    “set of points in the plane”. So a 

    __function graph__

    (as described above) is just one particular 
    kind of “graph” among other things that are 
    also called “graphs”, but that are not 
    function graphs.

    The so-called

    __vertical line test__

    observes that a graph [$=$ _set of points 
    in the plane_] is a function graph if and 
    only if every $x$-value (a.k.a., input) 
    corresponds to at most one $y$-value (a.k.a., 
    output). In other words, every vertical line 
    should intersect the graph at most once.

    For example, this particular graph...

    |> Image
        src images/svg_wiggle_graph.svg

    is a function graph (or locally at least, from
    what we can see), because every vertical line 
    intersects the graph at most once, but this 
    graph...

    |> Image
        src images/svg_circle.svg

    |> ImageRight
        src images/svg_crossing_cloud_circle.svg

    ...is not the graph of any function, because 
    some vertical lines intersect the graph more 
    than once.

    (Oops. To backtrack and quickly clarify a small 
    matter, an empty circle at the end of a segment, 
    in the vein of the previous figure...

    |> Image
        src images/svg_empty_circle.svg
        container_classes pt-4 mb-3

    ...means that the point in question is _excluded_ 
    from the graph. A filled circle, by opposition, 
    means that the point is included!)


    |> Example
        
        This _upper semicircle_ of unit radius...

        |> Image
            src images/svg_semicircle.svg

        ...passes the vertical line test, and, hence, 
        defines a function.


    |> Example
        
        This graph defines a function...

        |> Image
            src images/svg_factory_function.svg

        ...because it passes the vertical line test, 
        while this graph does not define a function...

        |> Image
            src images/svg_factory_nonfunction.svg
        
        ...because it does _not_ pass the vertical 
        line test!


|> Section

    *A Famous Discontinuity.*
    As already seen, functions can have
    _discontinuities_: a place where the function 
    experiences a sudden “jump” in value.

    For a famous example of a “naturally” occurring 
    discontinuity (that we feel compelled to 
    mention, for some reason) we need look no 
    further than the function

    $$
    {\Large x \ra 0^x}
    $$
    
    as it so happens that
    
    $$
    {0^x = \begin{cases} 0 & \te{if } x > 0\\
    1 & \te{if } x = 0\\
    \te{undefined} & \te{if }x &lt; 0 \end{cases}}
    $$
    
    which implies a discontinuity in the graph of
    $y = 0^x$ at $x = 0$, as pictured here:

    |> Image
        src images/svg_zero_to_the_x.svg
        container_classes mb-4

    (Pretty cool, no?)


|> Section

    *Distinguishing “$f$” and “$f(x)$”.* 
    The difference between
    
    $$
    {\te{VX-11/78A}}
    $$
    
    and
    
    $$
    {\te{VX-11/78A}(x)}
    $$
    
    is that the former is a
    
    __function__
    
    while the latter is a
    
    __value.__
    
    (Well, provided $x \in \{0, 1\}$, to make it 
    well-defined at all.) Likewise, if $f : \rr \ra \rr$, 
    the difference between 
    
    $$
    f
    $$
    
    and 
    
    $$
    f(x)
    $$
    
    is that the former is a
    
    __function__
    
    while the latter is a
    
    __value.__
    
    Amusingly, though, if we add “$x \ra$” in front 
    of “$f(x)$” then we are back to considering a 
    
    __function__
    
    and which is namely the function whose rule is: 
    apply $f$. In fact,

    $$
    f = (x \ra f(x))
    $$
    
    where the above is _an equality between functions_. 
    (You cannot use this equality to
    
    __define__
    
    $f$ because that would lead to a circular 
    definition. But that doesn't make the equality 
    any less true. And btw, you can go “one layer 
    deeper”:
    
    $$
    f = (x \ra f(x)) = (x \ra (t \ra f(t))(x))
    $$
    
    ...where we use the fact that $f = (t \ra f(t))$ 
    in the second equality. You could keep going, 
    replacing each time “$f$” by a self-referential 
    expression, but the process is not intrinsically 
    useful.)


|> Section

    *Distinguishing “$x^3$” and “$x \ra x^3$”.*
    Technically,

    $$
    x^3
    $$

    is a _value_ (not a function) and the way 
    logicians think of it, philosophically speaking, 
    is like so: at inception, every symbol has 
    some default value attached, absent any other 
    context.
    
    By contrast,

    $$
    x \ra x^3
    $$
    
    is clearly a _function_, not a _value_. 
    So “$x^3$” and “$x \ra x^3$” are very (VERY) 
    different, qualitatively speaking.
    
    But including the arrow everywhere is 
    impractical and even pedantic, so, in the end,
    you might see us refer to an expression such as, 
    e.g.,
    
    $$
    x^3 + x^2
    $$
    
    as a “function”, arrow or no arrow. 


|> Section

    *Distinguishing “$f$” and “$f(x)$”.*
    The difference between
    
    $$
    \te{VX-11/78A}
    $$
    
    and
    
    $$
    \te{VX-11/78A}(x)
    $$
    
    is that the former is a

    __function__

    while the latter is a

    __value.__

    (Well, provided $x \in \{0, 1\}$, to make it 
    well-defined at all.) Likewise, if $f : \rr \ra \rr$, 
    the difference between
    
    $$
    f
    $$
    
    and 
    
    $$
    f(x)
    $$
    
    is that the former is a

    __function__

    while the latter is a

    __value.__

    Amusingly, though, if we add “$x \ra$” in front 
    of “$f(x)$” then we are back to considering a 

    __function__

    and which is namely the function whose rule is: 
    apply $f$. In fact,
    
    $$
    f = (x \ra f(x))
    $$
    
    where the above is _an equality between 
    functions_. (You cannot use this equality to

    __define__

    $f$ because that would lead to a circular 
    definition. But that doesn't make the equality 
    any less true. And btw, you can go “one layer 
    deeper”:
    
    $$
    f = (x \ra f(x)) = (x \ra (t \ra f(t))(x))
    $$
    
    ...where we use the fact that $f = (t \ra f(t))$ 
    in the second equality. You could keep going,
    replacing each time “$f$” by a self-referential
    expression, but the process is not intrinsically 
    useful.)


|> Section

    *Distinguishing “$x^3$” and “$x \ra x^3$”.*
    Technically,
    
    $$
    x^3
    $$
    
    is a _value_ (not a function) and the way
    logicians think of it, philosophically speaking, 
    is like so: at inception, every symbol has some 
    default value attached, absent any other context.
   
    By contrast,
   
    $$
    x \ra x^3
    $$
   
    is clearly a _function_, not a _value_. 
    So “$x^3$” and “$x \ra x^3$” are very (VERY) 
    different, qualitatively speaking.
   
    But including the arrow everywhere is 
    impractical and even pedantic, so, in the end,
    you might see us refer to an expression such as,
    e.g.,
   
    $$
    x^3 + x^2
    $$
   
    as a “function”, arrow or no arrow.


|> Section

    *Distinguishing “$x^3$” and “$x \ra x^3$”.*
    Technically,
    
    $$
    x^3
    $$
    
    is a _value_ (not a function) and the way 
    logicians think of it, philosophically speaking,
    is like so: at inception, every symbol has some 
    default value attached, absent any other context.
   
    By contrast,
   
    $$
    x \ra x^3
    $$
   
    is clearly a _function_, not a _value_. 
    So “$x^3$” and “$x \ra x^3$” are very (VERY) 
    different, qualitatively speaking.
   
    But including the arrow everywhere is 
    impractical and even pedantic, so, in the end,
    you might see us refer to an expression such as,
    e.g.,
   
    $$
    x^3 + x^2
    $$
   
    as a “function”, arrow or no arrow. 

|> Section

    *Polynomials.*
    A function $f$ of the form
    
    $$
    f(x) = a_kx^k + a_{k-1}x^{k-1} + \dots + a_2x^2 + a_1x + a_0
    $$
    
    is called a _polynomial_. Here
    
    $$
    a_0,\,a_1,\, \ldots,\, a_k \in \rr
    $$
    
    are arbitrary constants, also known as the 
    _coefficients_ of the polynomial. The _degree_ 
    of the polynomial is $k$, if $a_k \ne 0$. 
    (Otherwise, work your way down until you find
    a nonzero coefficient—if there are none, because 
    the polynomial is just the constant $0$, then 
    the degree is _minus infinity_.) (We're not 
    kidding.)

    For example,

    $$
    2x + \sqrt{2}
    $$

    is a polynomial of degree 1, and

    $$
    x^2 - 10
    $$

    is a polynomial of degree 2, and

    $$
    x^{100} + x^{99} + x^{98} + \dots + x^4 + x^3 + x^2 + x + 1
    $$

    is a polynomial of degree 100.

    Polynomials of low degree have their own 
    special names, as inventoried in the following 
    table:

    $$
    \begin{array}{c|c|c}
    \,\,\,\,\te{degree}\,\,\,\, & \te{name} & \,\,\,\,\te{example}\,\,\,\,\Rule{0pt}{0.8em}{0.5em} \\ \hline
    -\infty & \te{zero} & 0\Rule{0pt}{1.1em}{0.0em}\\
    \te{0} & \te{constant} & 1 + \sqrt{5^{\color{white}*\!\!\!}}\\
    \te{1} & \te{affine} & 10x - 1\\
    \te{2} & \,\,\,\,\te{quadratic}\,\,\,\, & x^2 - 1\\
    \te{3} & \te{cubic} & x^3 - 1\\
    \te{4} & \te{quartic} & 1 - x^4\\
    \te{5} & \te{quintic} & x^5
    \end{array}
    $$

    There is some confusion about the term
        
    __affine__
        
    for which the term
    
    __linear__

    is sometimes substituted. But if we say 
    “linear” we mean a function of the form 

    $$
    x \ra a_1x
    $$

    for a constant $a_1 \in \rr$. This is more 
    restricted than an affine function, because
    there is no constant $a_0$!

    |> Image
        src images/svg_affine_linear_cloud.svg


|> Section

    *Quadratic, linear, and constant terms.*
    To finish up on polynomials: the terms of degree
    $2$, $1$ and $0$ are called the _quadratic_, 
    _linear_, and _constant_ terms of the 
    polynomial, respectively. If you see

    $$
    a_7x^7 + a_6x^6 + a_5x^5 + a_4x^4 + a_3x^3 - a_2x^2 + a_1x + a_0
    $$

    |> ImageRight
        src images/svg_quadratic_linear_constant_cloud.svg

    then the quadratic term is $-a_2x^2$, not 
    $a_2x^2$, fyi.

    Note that the linear term can also be viewed as
    the “$x^1$ term” while the constant term can 
    also be viewed as the “$x^0$ term”; because

    $$
    x^1 = x
    $$

    |> ImageRight
        src images/svg_a1_x1_cloud.svg

    for all $x$, and 

    $$
    x^0 = 1
    $$

    |> ImageRight
        src images/svg_a0_x0_cloud.svg

    for all $x$ (even $x = 0$), namely.


|> Exercises

    |> Exercise
        
        How can you define the absolute value 
        function using “definition by cases”?

        |> Solution
            
            The absolute value function is 
            
            $$
            x \ra \begin{cases} x & \te{if $x \geq 0$,}\\ -x\!\!\up{1.2} & \te{if $x < 0$}\end{cases}
            $$
            
            because $-(-1) = 1$, $-(-5) = 5$, etc.


    |> Exercise

        How can you define the absolute value 
        function using an “ordinary” algebraic formula?

        |> Solution

            We have

            $$
            |x| = \sqrt{x^2}
            $$

            because $\sqrt{(-1)^2} = 1$, $\sqrt{(-5)^2} = 5$,
            etc. 

            |> Pause

            _Note 1._
            This definition is less ad-hoc than might seem, 
            being a 1-dimensional form of the Pythagorean 
            theorem.

    |> Exercise

        Evaluate:

        |> Grid
            cols 2
            sm_cols 1
            sm_cutoff 520
            center_on_overflow true
            place_items start
            with_padding false
            column_first true

            i. $(\lambda u.u^3)(0.5)$
                
            iii. $(\lambda t.t - 1)(100) \cdot (\lambda t.t + 1)(100)$

            ii. $(u \ra u^2)(x + 1)$

            iv. $(u \ra u^2)(a + b)$
                    

        |> Solution

            The answers are:

            |> Grid
                cols 2
                sm_cols 1
                sm_cutoff 520
                center_on_overflow true
                place_items start
                with_padding false

                i. $0.5^3 = 0.125$
                    
                iii. $(100 - 1) \cdot (100 + 1) = 9999$

                ii. $(x + 1)^2 = x^2 + 2x + 1$

                iv. $(a + b)^2 = a^2 + 2ab + b^2$

    |> Exercise
        
        The _floor_ of a real number $x$, written
        
        $$
        \lfloor x \rfloor,
        $$
        
        is the greatest integer less than or equal 
        to $x$. (Start at $x$ and travel left on the 
        number line until you meet an integer; but if
        $x$ is already an integer, stay there; the 
        place you land is $\lfloor x \rfloor$.)
       
        Sketch the graph $y = \floor{x}$.
       
        Secondly, find a formula for a function whose
        graph looks like this, where you are allowed
        to use “$\floor{x}$” in your formula:

        |> Image
            src images/svg_factory_roof_graph.svg

        |> Solution
        
            As $x$ grows, so does $\floor{x}$, but
            $\floor{x}$ only “levels up” each time $x$
            reaches a new integer, and “flatlines”
            otherwise; this gives rise to the following
            staircase-shaped graph:

            |> Image
                src images/svg_floor_graph.svg

            (For example, $\floor{1} = 1$ because the 
            greatest integer less than or equal to $1$ 
            is $1$, $\floor{-0.5} = -1$ because the greatest
            integer less than or equal to $-0.5$ is $-1$,
            and so on.)

            For the second part note that the following 
            two displacements, excerpted from the “factory 
            roof” graph in the statement, are equal:

            |> Image
                src images/svg_factory_roof_graph_with_displacement_arrows_worked.svg

            The red dot to the left of $x$ has $x$-coordinate
            $\floor{x}$, so the horizontal displacement is 
            $$
            x - \floor{x}
            $$
            so the equation of the graph is 
            $$
            y = x - \floor{x}
            $$
            because the $y$-coordinate _is_ the 
            vertical displacement, given that the vertical 
            displacement starts at $y = 0$, and because
            the vertical and horizontal displacements are
            equal.

    |> Exercise
        
        Find the formula for a function whose graph 
        looks like this, again using the floor function
        ‘$\lfloor \cdot \rfloor$’ as a building block:

        |> Image
            src images/svg_factory_roof_stretched_x2_graph_worked.svg

        |> Solution

            We would like to argue the correctness of 
            the following two-step process (divide the 
            input by $2$, apply the function from Exercise 
            4):

            |> Image
                src images/svg_factory_roof_stretched_and_compressed.svg

            Indeed, the two graphs featured above differ
            only by a horizontal dilation; dividing the 
            input by $2$ “undoes” the dilation, at which 
            point it suffices to apply the function pictured
            in the second graph; having declared our method 
            correct, the answer is thus...
            $$
            {x/2 - \lfloor x/2 \rfloor}
            $$
            ...as obtained by “sticking” $x/2$ (the halved 
            input) in place of “$x$” in “$\,x - \lfloor x \rfloor$”,
            the formula for the function from Exercise 4.

            |> Pause

            _Note 1._
            One can check the answer by typing “x/2 - floor(x/2)”
            in DESMOS. Viz:

            |> Image
                src images/png_desmos_composition_0_c.png
                width 1400px

            |> Pause

            _Note 2._
            Alternately, enter “f(x) = x - floor(x)” and
            then “f(x/2)”, viz:

            |> Image
                src images/png_desmos_composition_1_c.png
                width 1400px

            Or we can be even fancier:

            |> Image
                src images/png_desmos_composition_2_b.png
                width 1400px
            
            What you see above (the graph in orange) is 
            the so-called __composition__ of the functions 
            $\f$ and $g$; in more detail, if we switch the 
            “input tube” and “output tube” sides of a function...

            |> Image
                src images/svg_f_box_inverted.svg

            ...(compared to the drawing at the top of the
            chapter), then the composition of $\f$ and $g$,
            written
            $$
            {\f \circ g}
            $$
            and read
            
            __“$f$ of $\hlfbk{}g$”__
            
            (mathematicians have to invent a notation for 
            everything—that little circle “$\circ$” is called 
            the _composition operator_,
            by the way) is the function that you get by
            gluing $g$'s box to the right of $\f$'s box, 
            like so:

            |> Image
                src images/svg_f_box_g_box.svg

            In other words, $g$'s output is passed on to $\f$ 
            for further processing. (A certain movie called
            “The Human Centipede” comes to mind.)

            (To be perfectly clear,

            |> Image
                src images/svg_f_circle_g_and_f_box_g_box.svg

            $f \circ g$ is a _function_, defined as the
            above assemblage of “$g$ first, $f$ second”.)
    
            |> Pause

            _Note 3._
            For a formal definition of “$f \circ g$”—something
            not based on pictures—one need only specify what
            $f \circ g$ does to inputs. Specifically:
            $$
            \,{(f \circ g)(x) = f(g(x))}.
            $$
            (So that equation is a formal definition.) One can
            also clarify that
            $$
            {\dom f \circ g = \{x\, \in\, \dom g:\, g(x)\, \in\, \dom f\}}
            $$
            which is to say that the domain of $f \circ g$
            consists of all $x$ such that: *(i)* $g(x)$
            exists (a.k.a, “$x \in \dom g$”) and,
            *(ii)* $f(g(x))$ exists (a.k.a., “$g(x) \in \dom f$”).

            |> Pause

            _Note 4._
            Amusingly—or not—both sides of 
            $$
            {(f \circ g)(x) = f(g(x))}
            $$
            are read
        
            _|“$f$ of $\hlfbk{}g$ of $x\hspace{0.1em}$”|_
            
            since “$f \circ g$” is read “$f$ of $g\rt{0.1}$”,
            and “$f(\dots)$” is read “$f$ of ...”.
    
    |> Exercise

        Find formulas for functions whose graphs look 
        like these:
    
        |> Image
            src images/svg_factory_roof_x2_worked.svg

        |> Solution

            For the first graph, 
            |> del

                the 
            an answer is 
            $$
            2 \cdot(x/2 - \fl{x/2})
            $$
            which simplifies to
            $$
            x - 2\fl{x/2}
            $$
            because all we have to do is to multiply 
            Exercise 5's formula by $2$.

            For the second graph, an answer is
            $$
            x/3 - \fl{x/3}
            $$
            because the problem is similar to Exercise 5 
            except with a factor $3$ horizontal dilation.

            For the third graph, we will first stop to 
            find a formula  for the function depicted here:

            |> Image
                src images/svg_factory_roof_stretched_x3_translated_1_worked.svg
        
            And that formula is...
        
            |> Image
                src images/svg_factory_roof_stretched_x3_translated_1_explanation.svg

            ...iiiiiiiS...
            $$
            (x-1)/3 - \fl{(x-1)/3}
            $$
            as obtained by substituting “$x - 1$”  
            (the input, minus $1$) in place of “$x$” in 
            “$\,x/3 - \fl{x/3}$”, the formula for the 
            second graph. Then we multiply that by $3$ 
            (to go from ”
            
            |> InlineImage
                src images/svg_3_3_grid_1.svg
                width 0.9em

            ” to “

            |> InlineImage
                src images/svg_3_3_grid_2.svg
                width 0.9em

            ”, namely), meaning that the final answer is
            $$
            3 \cdot ((x-1)/3 - \fl{(x-1)/3})
            $$
            or
            $$   
            (x - 1) - 3\fl{(x-1)/3}
            $$
            after simplification. (Or just
            $$
            x - 1 - 3\fl{(x-1)/3}
            $$
            though we personally prefer the previous 
            form, it being more “talkative”.)


    |> Exercise

        If
        $$
        \cos \dblcol \rr \ra \rr
        $$
        (the “hollow dot colon” means that $\dom \cos = \rr$)
        is a function whose graph looks like so...

        |> Image
            src images/svg_cosine.svg

        ...then does the function...
        $$
        {x \ra \cos(1000x)}
        $$
        ...have a graph that looks like a bunch of very tight 
        bumps, or, instead, very flat _&_ spaced-out bumps??

        |> Solution

            Consider how to “read off” a value of $y = \cos(1000x)$
            from the graph $y = \cos(x)$:

            |> Image
                src images/svg_cosine_1000x_worked.svg

            By the first step, a

            __horizontal dilation by a factor 1000__ 
            
            maps the first graph onto the second graph—i.e.,
            a point
            $$
            (x, y)
            $$
            is on the first graph if and only the dilated
            point
            $$
            (1000x, y)
            $$
            is on the second graph. The first graph is
            therefore some very compressed thing, full of
            scrunched bumps!

            |> Pause

            _Note 1._
            One can also reason that a small change in $x$ 
            results in a large change in $1000x$, so that 
            $\cos(1000x)$ must “cycle” much faster through 
            values than $\cos(x)$ does.

    |> Exercise

        Rewrite
        
        $$
        \tag{A}
        (f \circ (g \circ h))(x)
        $$
        
        without using “$\circ$”, using only the 
        “definitional equation of function 
        composition”, which is namely
        
        $$
        \tag{AA}
        (r \circ s)(x) = r(s(x))
        $$
        
        (where $r$ and $s$ are functions); plz
        note that you will have to apply (AA) 
        _twice_, as each application of (AA) 
        makes _one_ copy of the symbol “$\circ$” 
        disappear, and (A) contains _two_ copies of 
        “$\circ$”!!

        |> Solution

            Setting “$r$” to “$f$” and “$s$” to “$(g \circ h)$” 
            in (AA) yields

            |> ImageLeft
                src images/svg_r_s_substitution_1_1.1em_cloud.svg
                popup true

            $$
            {(f \circ (g \circ h))(x) = \f((g \circ h)(x))}
            $$
            ...which already constitutes progress towards
            our goal, since only one copy of “$\circ$” 
            exists on the right-hand side! But
            $$
            {(g \circ h)(x) = g(h(x))}
            $$
            by the “definitional equation” again, so
            $$
            {f((g \circ h)(x)) = \f(g(h(x)))}
            $$
            ...and this completes the computation!
        
            |> Pause

            _Note 1._
            We can collect both steps of the computation 
            into a single string of equalities:

            |> Image
                src images/svg_first_victim_second_victim_1_1em_v2.svg

    |> Exercise

        Same question as Exercise 17, but for 
        “$f \circ (g \circ h)$” instead of 
        “$(f \circ g) \circ h$”.

        |> Solution

            We will again evaluate the “outer”
            composition operator first and the “inner”
            composition operator second, where the “outer”
            composition operator is the one that is fewer
            pairs of parentheses away from the outside 
            world:

            |> Image
                src images/svg_inner_outer_1_1em_v2.svg

            So the first step is...
            
            $$
            ((f \circ g) \circ h)(x) = (f \circ g)(h(x))
            $$
            
            ...by setting $r = f \circ g$, $s = h$ in
            the definitional equation, and the second step
            is...
            
            $$
            (f \circ g)(h(x)) = \f(g(h(x)))
            $$
            
            ...by setting $r = f$, $s = g$, and while
            replacing “$x$” by “$h(x)$”.
        
            |> Pause

            _Note 1._
            The fact that
            $$
            (f \circ (g \circ h))(x)
            $$
            and
            $$
            ((f \circ g) \circ h)(x)
            $$
            both evaluate to
            $$
            f(g(h(x)))
            $$
            actually implies that
            $$
            f \circ (g \circ h)
            $$
            and
            $$
            (f \circ g) \circ h
            $$
            are the same function; this function is namely
            the function that maps $x$ to $f(g(h(x)))$ for 
            all $x$ (or
            $$
            x \ra f(g(h(x)))
            $$
            in lambda notation).

            |> Pause

            _Note 2._
            Because of this, we can write
            $$
            f \circ g \circ h
            $$
            without any parentheses. (The point is: either
            way you parenthesize it you obtain the same 
            function, so why bother?)

            |> Pause

            _Note 3._
            The fact that

            $$
            {(a + b) + c = a + (b + c)}
            $$

            for all numbers $a$, $b$, $c$ is known as the

            __associativity__

            of addition; likewise, the fact that

            $$
            (ab)c = a(bc)
            $$

            for all numbers $a$, $b$, $c$ is known as the

            __associativity__

            of multiplication; and again likewise,
            the fact that

            $$
            {(f \circ g) \circ h  =  f \circ (g \circ h)}
            $$

            for all functions $f$, $g$, $h$ is known 
            as the

            __associativity__

            |> ImageRight
                src images/svg_associativity_cloud.svg

            of function composition.
        
            |> Pause

            _Note 4._
            One of the best ways to explain _&_ understand
            the associativity of function composition 
            uses this picture:

            |> Image
                src images/svg_truth_be_said.svg

            In the above $A$, $B$, $C$, $D$ are sets 
            while the arrows encode functions $f$, $g$ 
            and $h$ that, respectively in reverse order, 
            go from $D$ to $C$, $C$ to $B$, and $B$ to $A$. 
            For example,

            |> DisplayCentered 

                ${\Large h(}$
                
                |> InlineImage
                    src images/svg_composition_icon_clubs.svg
                    width 1rem
                    y_anchor -3px

                ${\Large{}) =}$
                
                |> InlineImage
                    src images/svg_composition_icon_die_3.svg
                    width 1.25rem
                    y_anchor -3px
                    margin_left .5rem
                    margin_right .5rem
            
            because the arrow that originates at 

            |> InlineImage
                src images/svg_composition_icon_clubs.svg
                width 1rem
                y_anchor -4px
                space_left true
                space_right true

            in set $D$ lands at 
            
            |> InlineImage
                src images/svg_composition_icon_die_3.svg
                width 1.25rem
                y_anchor -4px
                space_left true
                space_right true
            
            in set $C$,  and

            |> DisplayCentered 

                ${\Large g(h(}$

                |> InlineImage
                    src images/svg_composition_icon_clubs.svg
                    width 1rem
                    y_anchor -3px

                ${\Large{})){}=}$

                |> InlineImage
                    src images/svg_composition_icon_fence.svg
                    width 2.3rem
                    y_anchor -4px
                    space_left true
                    space_right true
                
            because, pursuing that path onwards, the arrow 
            that originates at
            
            |> InlineImage
                src images/svg_composition_icon_die_3.svg
                width 1.25rem
                y_anchor -4px
                space_left true
                space_right true

            in set $C$ lands at

            |> InlineImage
                src images/svg_composition_icon_fence.svg
                width 2.3rem
                y_anchor -4px
                space_left true
                space_right true

            in set $B$, etc.

            Under this representation one can “compute”
            $f \circ g \circ h$ by gluing arrows end-to-end.
            First, say, obliviate set $C$ in the middle 
            right, then do the same with set $B$ in the
            middle left:

            |> Image
                src images/svg_truth_be_said_right_first.svg
                width 2000px
                
            We can also get rid of $B$ first, $C$ second:

            |> Image
                src images/svg_truth_be_said_left_first.svg
                width 2000px

            The first order of computation corresponds
            to the parenthetization “$f \circ (g \circ h)$”
            while the second corresponds to the 
            parenthetization “$(f \circ g) \circ h$”.
            Intuitively, the reason they come out the 
            same (in “step 6”, bottom left) is because 
            each final arrow in the last diagram comes 
            from a path-of-arrows in the original 
            diagram, and the order in which the 
            waypoints along a path are “straightened” 
            (or “collapsed”) does not affect the origin 
            point or destination point of the final 
            arrow.

            |> Pause

            _Note 5._
            The last series of diagrams might leave one 
            with the impression that the composition of 
            two or more functions can be “precomputed”
            by looking ahead along the path of yellow arrows. 
            Just so you know, computers do not generically 
            do this. For reason, computers are not given 
            functions as tables of input-output values to 
            know by heart but rather as “recipes” (synonyms: 
            algorithms, code, programs) that allow them 
            to compute an output for any given input. 
            Moreover, there is no general way of flattening 
            two recipes into a single, shorter one—when 
            composing two functions the computer has, in 
            general, no choice but to diligently apply 
            each recipe in order—the first function first,
            the second function second.

            |> Pause

            _Note 6._
            We have taken for granted the fact that two 
            functions $f$ and $g$ are “equal” if and only
            if they produce the same outupt for every 
            input but this is a actually subtle thing 
            that has to do with how functions are defined 
            “under the hood”. Specifically, mathematicians 
            view functions as
            |> del

                long
            |> del

                lists of
            sets of ordered pairs; for example—conceptual
            cold water shock ahead—

            $$
            {\textrm{VX-11/78A} = \{(0, 3.5), (1, 2.5)\}}
            $$

            because VX-11/78A maps $0$ to $3.5$ and maps 
            $1$ to $2.5$. (The presence of an ordered pair
            
            $$
            (a, b)
            $$

            means that input $a$ produces output $b$.) So 
            two functions are equal if and only if they 
            are equal
            
            __as sets of ordered pairs__
            
            because the set of ordered pairs is the 
            underlying “thing” that the function is. In 
            particular, there is no notion of a “formula” 
            or of a “procedure” being attached to a 
            function, that might cause two functions to 
            be considered unequal even if they produce the 
            same output on every input—producing the same
            output on every input implies that the 
            |> del

                list of
            set of ordered pairs is equal, and, perforce, 
            that the two functions are equal!!

    |> Exercise
        
        Show that
        $$
        x^2 + 10x + 30
        $$
        can be written in the form 
        $$
        (x + \dots)^2 + \,\dots
        $$
        for some numbers “$\dots$” and “$\dots$”.

        |> Solution

            The answer is
            $$
            (x + 5)^2 + 5
            $$
            because
            $$
            \begin{align}
            \,\,\,\,(x + 5)^2
            &= x^2 + (2\cdot 5)x + 5^2 \\
            &= \up{1.4} x^2 + 10x + 25
            \end{align}
            $$
            and adding $5$ gives $x^2 + 10x + 30$.

    |> Exercise

        Solve Exercise 10 using algebra _&_
        variables.

        |> Solution

            Put an unknown “$U$” for the first set
            of dots and an unknown “$V$” for the 
            second set of dots. Then
            $$
            (x + U)^2 + V = x^2 + 10x + 30
            $$
            becomes the equation to satisfy.
            Expanding the left-hand side, we get:
            $$
            x^2 + 2Ux + U^2 + V = x^2 + 10x + 30.
            $$
            In order for this equation to hold as an 
            equality between polynomials (i.e., for all
            $x$) the coefficients of $x^2$ on both 
            sides of the equation must be equal, the 
            coefficients of $x$ on both sides of the
            equation must be equal, and the constant
            terms on boths of the equation must be 
            equal—this gives us
            $$
            1 = 1
            $$
            (equating the coefficients of $x^2$), and
            $$
            2U = 10
            $$
            (equating the coefficients of $x$), and 
            $$
            U^2 + V = 30
            $$
            (equating the constant terms). Only the
            latter two equations contain information.
            In particular,
            $$
            2U = 10
            $$
            implies $U = 5$, so $U^2 + V = 30$ becomes
            $25 + V = 30$, and 
            $V = 30 - 25 = 5$.
            So $U = V = 5$, as previously found.
            (But now we know that the solution is _unique_,
            because the only number $U$ that satisfies
            $$
            2U = 10
            $$
            is $U = 5$, and the only number $V$ that
            satisfies 
            $$
            25 + V = 30
            $$
            is $V = 5$.)

    |> Exercise

        Show that
        $$
        x^2 + 10x + 30 = 0
        $$
        (cf$.$ Exercise 21) has no solutions $x \in \rr$.

        |> Solution

            The equation is equivalent to
            $$
            (x + 5)^2 + 5 = 0
            $$
            by Exercise 21, but this implies
            $$
            (x + 5)^2 = -5
            $$
            which is an equation with no solution over
            the reals because the square of a real number
            is nonnegative.

    |> Exercise

        Show that 
        $$
        x^2 + 10x - 30 = 0
        $$
        has two solutions $x \in \rr$.

        |> Solution

            The equation can be written
            $$
            (x + 5)^2 - 55 = 0
            $$
            because $(x + 5)^2 = x^2 + 10x + 25$
            and $25 - 55 = -30$. Passing $55$ to the
            other side, we find
            $$
            (x + 5)^2 = 55
            $$
            which holds if and only if
            $$
            \,x + 5 = \pm\sqrt{55}
            $$
            or
            $$
            \,x = -5 \pm\sqrt{55}
            $$
            constituting two distinct solutions.

    |> Exercise

        What sequence of geometric transformations
        (rotations, translations, scalings, etc) 
        maps the curve
        $$
        {y = x^2}
        $$
        onto
        $$
        {y = Ax^2 + Bx}
        $$
        for constants $A$, $B$ such that $A \ne 0$? 

        |> Solution

            Write
            $$
            {Ax^2 + Bx}
            $$
            as
            $$
            {A\Big(x^2 + {B \over A}x\Big)}
            $$
            and then write
            $$
            {x^2 + {B \over A}x}
            $$
            as
            $$
            {\Big(x + {B \over 2A}\Big)^2 - {B^2 \over 4A^2}}
            $$
            so that, altogether,
            $$
            {Ax^2 + Bx}
            $$
            is rewritten
            $$
            {A\left[\Big(x + {B \over 2A}\Big)^2 - {B^2 \over 4A^2}\right]}
            $$
            that can be seen as descending from $y = x^2$ 
            in three steps:
            $$
            {y = x^2}
            $$
            $$
            {\downarrow}
            $$
            $$
            {y = \,\Big(x + {B \over 2A}\Big)^2}
            $$
            $$
            {\downarrow}
            $$
            $$
            {y = \,\Big(x + {B \over 2A}\Big)^2 - {B^2 \over 4A^2}}
            $$
            $$
            {\downarrow}
            $$
            $$
            {y = A\left[\Big(x + {B \over 2A}\Big)^2 - {B^2 \over 4A^2}\right]}.
            $$
            Three steps, three geometric transformations!
            The third step effects a

            __vertical scaling by $A$__

            i.e., vertically stretches the graph by
            a factor $A$, because we multiply the
            value of $y$ by $A$. The second step 
            effects a

            __vertical translation by ${-{B^2 \over 4A^2}}$__

            i.e., lowers the height of the entire
            graph by ${B^2 \over 4A^2}$, because we add 
            $-{B^2 \over 4A^2}$ to the value of $y$.
            The first step, on the other hand, is
            entirely different: it is a

            __preprocessing__

            step, in that we mess with the input (i.e., 
            $x$), instead of adding on (or “multiplying 
            on”) to the current value of $y$.     
       
            To understand how a preprocessing step 
            affects the shape of a graph, note that, more
            generally, a graph of the form
            $$
            {y = f(x + a)}
            $$
            (for some constant $a$) “fetches” values on
            the graph
            $$
            {y = f(x)}
            $$
            by going $a$ units to the right. The larger $a$
            is, thus, the further 
            $$
            {y = f(x + a)}
            $$
            drifts off to the left. For example,
            $$
            {y = f(x + 20)}
            $$
            has value $f(0)$ at $x = -20$, and if you replace
            $20$ with something larger, that position (i.e., 
            $x = -20$) drifts even further off to the left!
            In any case, the graph $y = f(x + a)$ is the
        
            __leftward__

            translate by $a$ units of $y = f(x)$
            and, as a consequence, the first step effects a
            
            __leftward translation by ${B\over 2A}$__
            
            of the curve $y = x^2$, or
        
            __horizontal translation by $-{B\over 2A}$__

            more elegantly put. (The second formulation
            doesn't assume a particular orientation of the 
            $x$-axis, that's why it's “more elegantly put”,
            in our opinion.)
       
            To recapitulate, the three transformations
            are,in order:

            |> List

                1. <span>horizontal</span> translation by $-{B \over 2A}$
            
                2. vertical translation by ${-{B^2 \over 4A^2}}$

                3. vertical scaling by $A$

            |> Pause

            _Note 1._
            You could do the vertical translation
            before the horizontal translation,
            geometrically it comes out the same.
            That order of geometric transformations 
            would correspond to the following sequence
            of algebraic transformations:
            $$
            {y = x^2}
            $$
            $$
            {\downarrow}
            $$
            $$
            {y = x^2 - {B^2 \over 4A^2}}
            $$
            $$
            {\downarrow}
            $$
            $$
            {y = \,\Big(x + {B \over 2A}\Big)^2 - {B^2 \over 4A^2}}
            $$
            $$
            {\downarrow}
            $$
            $$
            {y = A\left[\Big(x + {B \over 2A}\Big)^2 - {B^2 \over 4A^2}\right]}
            $$
            ...in which the _second_ step is a 
            preprocessing step. (I.e., a step that 
            replaces “$x$” with something else.)

    |> Exercise
        
        Let $x_0 \in \rr$, $y_0 \in \rr$ and 
        $a \in \rr$ with $y_0 \geq 0$, $a \ne 0.$
        If you apply these transformations...

        |> List

            1. vertical translation by $-y_0$

            2. horizontal translation by $x_0$

            3. vertical scaling by $a$

        ...to the curve $y = x^2$, what are the 
        roots of the final curve that you obtain?
        (Nb: _Roots_ are values of $x$ such 
        that $y = 0$.)
    
        |> Solution

            Start by noting that the point 
            $(\sqrt{y_0}, y_0)$ is on the curve $y = x^2$,
            as well as the point $(-\sqrt{y_0}, y_0)$, 
            because
            $(\sqrt{y_0})^2 = (-\sqrt{y_0})^2 =$ $y_0;$
            here is a sketch of the situation before 
            anything happens:
        
            |> Image
                src images/svg_x0_y0_step0.svg
                width 760px
        
            After vertically translating by $-y_0$ 
            the roots will therefore be at $x = \pm\sqrt{y_0}$:
        
            |> Image
                src images/svg_x0_y0_step1_alone.svg
                width 760px
        
            Then after horizontally translating by $x_0$
            the roots mosey over to $x = x_0\pm\sqrt{y_0}$:

            |> Image
                src images/svg_x0_y0_step2_alone.svg
                width 760px

            Lastly, vertical scaling does not affect 
            the position of the roots, because it 
            stretches the graph _about_ the $x$ 
            axis (here $a \approx 1.7$):

            |> Image
                src images/svg_x0_y0_step3_alone.svg
                width 760px

            So the roots are at: $x = x_0 \pm \sqrt{y_0}$.
            (Like we found them after the second step.)

    |> Exercise

        Use the results of the previous two exercises
        to find the value(s) of $x$ such that 
        $Ax^2 + Bx = 0$ for constants $A$, $B$ such 
        that $A \ne 0$.
    
        |> Solution

            Well,
            $$
            Ax^2 + Bx = 0
            $$
            obviously has solution $x = 0$ to start
            with, so we _don't need_ the previous 
            exercises for one of the roots at least—actually,
            $$
            Ax^2 + Bx = x(Ax + B)
            $$
            so the equation is equivalent to
            $$
            x(Ax + B) = 0
            $$
            and so one of the roots is 
            $$
            x = 0
            $$
            and the other root is the value of $x$
            such that
            $$
            Ax + B = 0
            $$
            which is $x = -B/A$. (In order for the product
            $$
            x(Ax + B)
            $$
            to be $0$ you either need the first term to
            be $0$, leading us to $x = 0$, or the second
            term to be $0$, leading us to $Ax + B = 0$—the 
            product of two things is $0$ if and only if
            one of the two things is $0$.)
        
            So the roots are $x = 0$ and $x = -B/A$.
        
            To complete the problem _as we were asked_, 
            however, we will use the fact that $y = Ax^2 + Bx$
            is obtained from $y = x^2$ by the following
            sequence of transformations (cf. Exercise 14):

            |> List

                1. vertical translation by $-{B^2 \over 4A^2}$

                2. <span>horizontal</span> translation by $-{B \over 2A}$

                3. vertical scaling by $A$

            (We put the vertical translation first.)
            By Exercise 15, the roots of $y = Ax^2 + Bx$ 
            are thus at
            $$
            \tag{A}
            x = -{B\over 2A} \pm \sqrt{B^2 \over 4A^2}
            $$
            which looks a little different than our 
            previous result of $x = 0$ and $x = -B/A$
            until you realize that
            $$
            \pm \sqrt{B^2 \over 4A^2} = \pm {B \over 2A}
            $$
            (because
            $$
            \left({B \over 2A}\right)^{\!2} = {B^2 \over 4A^2}
            $$
            and even though ${B\over 2A}$ could be 
            negative, the “$\pm$” on either side of 
            the equation means that the _set_ of values
            on either side of the equation is the same),
            so that (A) becomes
            $$
            x = -{B\over 2A} \pm {B \over 2A}
            $$
            and, on the one hand,
            $$
            -{B\over 2A} + {B \over 2A} = 0
            $$
            while, on the other hand,
            $$
            -{B\over 2A} - {B \over 2A} = -{2B\over 2A} = -{B\over A}
            $$
            so here too we find that the roots are $x = 0$
            and $x = -B/A$. (It must be the right answer!)

    |> Exercise

        True or false ($f$ and $g$ are functions):

        |> Grid
            cols 2
            sm_cols 1
            sm_cutoff 520
            center_on_overflow true
            place_items start
            with_padding false
            column_first true

            i. $f \circ g = (x \ra f(g(x)))$
                
            iii. $f \circ g = (x \ra g(f(x)))$

            ii. $g \circ f = (x \ra f(g(x)))$

            iv. $g \circ f = (x \ra g(f(x)))$

        |> Solution

            The true statements are i, iv, because
            $f \circ g$ is the function that maps an 
            input $x$ to $f(g(x))$, and symmetrically 
            for $g \circ f$. 

    |> Exercise

        If $f$ and $g$ are functions then we define 
        (and not just us but people in general)
        $$
        f + g
        $$
        to be
        $$
        t \ra f(t) + g(t)
        $$
        (use ‘$x$’ if you like), 
        i.e., to be the function that applies $f$ and
        $g$ separately and then takes the sum,
        and we define 
        $$
        fg
        $$
        to be
        $$
        z \ra f(z)g(z)
        $$
        (use ‘$t$’ if you like, hehe),
        i.e., to be the function that applies $f$ and 
        $g$ separately and then takes the product.
        (These definitions are similar to how we define
        $$
        f \circ g
        $$
        to be
        $$
        u \ra f(g(u))
        $$
        for the symbol “$\circ$”, except that now we 
        are defining the _sum_ and _product_ of functions,
        instead of their _composition_, namely.)
       
        Given these definitions, which of the following
        equalities hold, in general for all functions $f$,
        $g$ and $h$?
        $$f \circ (g + h) = (x \ra f(g(x)) + f(h(x)))$$
        $$f \circ (g + h) = (x \ra f(g(x) + h(x)))$$
        $$(g + h) \circ f = (x \ra h(f(x)) + g(f(x)))$$
        $$(g + h) \circ f = (x \ra (g + h)(f(x)))$$

        |> Solution

            The first equality is false because the 
            right-hand side is actually
            $$
            (f \circ g) + (f \circ h)
            $$
            not $f \circ (g + h)$; the second equality is
            true; the third equality is true even though you
            would expect the right-hand side to be written 
            $$
            (x \ra g(f(x)) + h(f(x)))
            $$
            with “$g$” and “$h$” switched (but addition is
            commutative, so it doesn't matter); the fourth
            equality is true: it is the definition of “$\circ$”.


    |> Exercise
        
        What sequence of geometric transformations 
        of length no more than 3 maps
        $$
        y = x^2
        $$
        onto
        $$
        y = Ax^2 + Bx + C
        $$
        for constants $A$, $B$, $C$ such that $A \ne 0$?

        |> Solution

            Write
            $$
            Ax^2 + Bx + C
            $$
            as
            $$
            A\Big(x^2 + {B \over A}x + {C\over A}\Big)
            $$
            and, similarly to Exercise 14, write
            $$
            x^2 + {B \over A}x
            $$
            as
            $$
            \Big(x + {B \over 2A}\Big)^2 - {B^2 \over 4A^2}
            $$
            so that, altogether, $Ax^2 + Bx + C$
            becomes
            $$
            A\left[\Big(x + {B \over 2A}\Big)^2 - {B^2 \over 4A^2} + {C\over A}\right]
            $$
            or 
            $$
            A\left[\Big(x + {B \over 2A}\Big)^2 - {B^2 - 4AC\over 4A^2}\right]
            $$
            by putting things on a common denominator.
            (We have endeavored to keep the minus sign 
            out front of the common denominator fraction 
            in order to maintain the most similarity with
            the term “$-{B^2\over 4A}$” of Exercise 14,
            that also has a minus sign out front.)
           
            By direct analogy with Exercise 14, the three
            transformations are thus...

            |> List

                1. horizontal translation by $-{B \over 2A}$

                2. vertical translation by ${-{B^2 - 4AC \over 4A^2}}$

                3. vertical scaling by $A$

            ...or...

            |> List

                1. vertical translation by ${-{B^2 - 4AC \over 4A^2}}$

                2. horizontal translation by $-{B \over 2A}$

                3. vertical scaling by $A$
            
            ...if we put the vertical translation first.

    |> Exercise
            
        What are the roots (i.e., solutions) $x$ of
        $$
        Ax^2 + Bx + C = 0
        $$
        for constants $A$, $B$, $C$ such that $A \ne 0$?

        |> Solution

            The curve
            $$
            y = Ax^2 + Bx + C
            $$
            is obtained from the curve $y = x^2$ by the following sequence of 
            transformations (cf. Exercise 19):

            |> List

                1. vertical translation by ${-{B^2 - 4AC \over 4A^2}}$
                2. horizontal translation by $-{B \over 2A}$
                3. vertical scaling by $A$

            _|~|_

            On the one hand, if
            $$
            {B^2 - 4AC \over 4A^2} < 0
            $$
            then
            $$
            -{B^2 - 4AC \over 4A^2} > 0
            $$
            and the vertical translation is _upward_, 
            the curve detaches from the $x$ axis never
            to see it again, and there are no roots!
           
            On the other hand, if
            $$
            {B^2 - 4AC \over 4A^2} \geq 0
            $$
            then the roots are given by
            $$
            x = -{B \over 2A} \pm \sqrt{B^2 - 4AC \over 4A^2}
            $$
            by Exercise 15. $\rightarrow$ ~The End~ $\leftarrow$

            |> Pause

            _Note 1._
            In fact,
            $$
            \pm\sqrt{B^2 - 4AC \over 4A^2} = \pm {\sqrt{B^2 - 4AC} \over 2A}
            $$
            (square both sides of the equation—in general,
            $$
            \pm E = \pm F
            $$
            as one set of two values equalling another 
            set of two values, if and only if
            $$
            |E| = |F|
            $$
            or 
            $$
            E^2 = F^2
            $$
            —so that's why we say “square both sides”),
            so the formula for the roots can also be written
            $$
            x = -{B \over 2A} \pm {\sqrt{B^2 - 4AC} \over 2A}
            $$
            or
            $$
            x = {{-B \pm \sqrt{B^2 - 4AC}} \over 2A}
            $$
            as briefly flashed by, e.g., in Chapter 1.

            |> Pause

            _Note 2._
            If
            $$
            {B^2 - 4AC \over 4A^2} < 0
            $$
            then
            $$
            \sqrt{B^2 - 4AC \over 4A^2}
            $$
            does not exist, alerting you to the absence 
            of roots, if you try to use the first formula 
            we gave. Also
            $$
            {B^2 - 4AC \over 4A^2} < 0\iff B^2 - 4AC < 0
            $$
            because $4A^2 > 0$ for all $A \ne 0$, 
            so the second set of formulas would alert you
            to the absence of roots in that case, as well.

    |> Exercise

        Summon your senses of extrapolation _&_ 
        imagination to evaluate this expression:
        $$
        (f \ra x \ra h \ra {f(x+h) - f(x)\over h})(x \ra x^2)(5)(0.1)
        $$
        (Hint: The answer is a specific real number.)

        |> Solution

            ...in an expression such as...
            $$
            (x \ra x^3)(6)
            $$
            ...we pair the $x$ with $6$...

            |> Image
                src images/svg_connection_x_6.svg
            
            ...and $6$ becomes the value to use for $x$
            in “$x^3$”:

            |> Image
                src images/svg_connection_x_6_in_computation.svg
            
            ...; in an expression such as...
            $$
            (x \ra y \ra x^3y)(6)
            $$
            ...we also pair the $x$ with $6$...

            |> Image
                src images/svg_connection_x_6_again.svg
            
            ...and $6$ becomes the value to use for $x$
            in “$y \ra x^3y$”:
        
            |> Image
                src images/svg_connection_x_6_again_in_computation.svg
            
            ...(in this case the result is not a number,
            but a function—a function is a mathematical 
            object like another, after all); in an 
            expression such as...
            
            $$
            (x \ra y \ra x^3y)(6)(8)
            $$
            
            ...we pair the $x$ with $6$ and the $y$ with $8$...
        
            |> Image
                src images/svg_connection_x_y_6_8.svg

            ...and $6$ and $8$ become respectively the 
            values to use for $x$ and $y$ in “$x^3y$”:
        
            |> Image
                src images/svg_connection_x_y_6_8_in_computation.svg
            
            ...; now in an expression such as...
            
            $$
            (f \ra x \ra h \ra {f(x+h) - f(x)\over h})(x \ra x^2)(5)(0.1)
            $$
            
            ...we pair the $f$ with $x \ra x^2$, the 
            $x$ with $5$, and the $h$ with $0.1$...
        
            |> Image
                src images/svg_connection_f_x_h.svg
            
            ...and $x \ra x^2$, $5$ and $0.1$ become
            respectively the values to use for $f$, $x$
            and $h$ in “${f(x + h) - f(x)\over h}$”:
        
            |> Image
                src images/svg_connection_f_x_h_in_computation.svg
            
            ...; evaluating...
            $$
            {(x \ra x^2)(5 + 0.1) - (x \ra x^2)(5) \over 0.1}
            $$
            ...we...
            $$
            {(x \ra x^2)(5.1) - (x \ra x^2)(5) \over 0.1}
            $$
            ...get...
            $$
            {5.1^2 - 5^2 \over 0.1}
            $$
            ...this...
            $$
            {26.01 - 25 \over 0.1}
            $$
            ..._thiiis_...
            $$
            {1.01 \over 0.1} = 1.01 \times 10 = 10.1
            $$
            ...result! (The answer is: ten point one.)

*/