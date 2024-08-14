/*
|> Section

    *Terminology.*
    The expression below is called a _power_; 
    the number at the bottom of the power is called
    the _base_ (of the power); the number at the top
    is called the _exponent_:

    |> Image
        src images/svg_base_exponent.svg

    The whole expression is read $\mathit{10}$ _to the power_ $\mathit{3}$, and the
    general process of taking a power is called _exponentiation_.

    *Integer powers of 10.*
    We define
    $$
    \Large 10^{\hspace{0.2ex}n}
    $$
    as follows, if $n$ is a nonnegative integer: start
    from $1$ and multiply by $10$ $n$ times. We also define
    $$
    \Large 10^{-n}
    $$
    as follows, if $n$ is a positive integer: start from $1$ and divide
    by $10$ $n$ times. 

    For example,
    $$
    \Large 10^4 = 1 \times 10 \times 10 \times 10 \times 10 = 10000
    $$
    $$
    \Large 10^3 = 1 \times 10 \times 10 \times 10 = 1000
    $$
    $$
    \Large 10^2 = 1 \times 10 \times 10 = 100
    $$
    $$
    \Large 10^1 = 1 \times 10 = 10
    $$
    $$
    \Large 10^0 = 1 = 1
    $$

    |> ImageLeft
        src images/svg_ten_to_the_0_cloud.svg
        
    (where, in the last line, $1$ is multiplied by $10$ <i>zero times</i>,
    as per the exponent, which is zero) by the first definition, while
    $$
    \Large 10^{-1} = 1\,/\,10 = 0.1
    $$
    $$
    \Large 10^{-2} = (1\,/\, 10)\,/\,10 = 0.01
    $$
    $$
    \Large 10^{-3} = ((1\,/\, 10)\,/\,10)\,/\,10 = 0.001
    $$
    $$
    \Large 10^{-4} = (((1\,/\, 10)\,/\,10)\,/\, 10)\,/\, 10 = 0.0001
    $$
    by the second definition.
            
    As $n$ successive divisions
    by $10$ is the same as one division by $10^n$, one also has
    $$
    \Large 10^{-n} = {1 \over 10^{\hspace{0.2ex}n}}\tag{*}
    $$
    for every positive integer $n$, which gives an alternate means of computing $10^{-n}$.
    Moreover, (*) actually holds for
    
    __every__
    
    integer $n$, which is mildly important. In more
    detail, (*) holds for $n = 0$ by inspection, and (*) 
    is equivalent to the identity

    |> ImageLeft
        src images/svg_zero_verification_cloud.svg

    $$
    \Large 10^{-n}10^n = 1 \tag{**}
    $$

    which holds for $n$ if and only if it holds for
    $-n$. (By which we mean: replacing “$n$” by “$-n$”
    in (**) lands you right back on (**), due to the fact
    that $-{(-n)} = n$.) (So, namely, if (**) holds for all
    positive values of $\hspace{0.05em}n$, then it holds
    for all negative values of $n$, as well.)

*/