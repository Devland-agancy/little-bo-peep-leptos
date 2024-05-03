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
    notation  that enables one to 
    define a function without giving it a name, 
    such as “$f$”. In fact there are two different 
    mainstream notations, in this instance.
    One notation writes
    $$
    \lambda{x}.x^2
    $$
    to mean 
    “the function that maps $x$ to $x^2$” 
    (and by the way,
    $$
    \lambda{z}.z^2
    $$
    is <i>the same</i> function, because 
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
    “$f(\dots) = \dots$

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
    the same <i>function</i>, no matter what symbol 
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

    The graph has only two points, because VX-78/11A is defined
    at only two values. One point is...
        
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
