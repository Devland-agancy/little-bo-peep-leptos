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
    is “$f$.

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
    you choose.*) (*As long as you don't collide 
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
    completely arbitrary
    
    In fact, there are only two “ground rules” to 
    respect in order for something to qualify as a 
    function: *(i)* to output
        
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

        |> Table
            cols vec![27, 170, 27, 230]

            |> tr
                
                |> td
                
                    i.
                
                |> td
                
                    $(\lambda u.u^3)(0.5)$
                
                |> td
                
                    iii.

                |> td
                
                    $(\lambda t.t - 1)(100) \cdot (\lambda t.t + 1)(100)$
            
            |> tr
                margin-top 10pt

                |> td
                
                    ii.

                |> td
                
                    $(u \ra u^2)(x + 1)$

                |> td
                
                    iv.

                |> td
                
                    $(u \ra u^2)(a + b)$


        |> Solution

            The answers are:

            |> Table
                cols vec![27, 230, 27, 230]

                |> tr
                    
                    |> td
                    
                        i.
                    
                    |> td
                    
                        $0.5^3 = 0.125$
                    
                    |> td
                    
                        iii.

                    |> td
                    
                        $(100 - 1) \cdot (100 + 1) = 9999$
                
                |> tr

                    |> td
                    
                        ii.

                    |> td
                    
                        $(x + 1)^2 = x^2 + 2x + 1$

                    |> td
                    
                        iv.

                    |> td
                    
                        $(a + b)^2 = a^2 + 2ab + b^2$


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

*/