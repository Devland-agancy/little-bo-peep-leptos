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
        y top
        offset_y 1em
        offset_x -5em
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
