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
        src images/63.svg

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
    important) for the result of passing an input $x$
    to a function $f$. For example, if the rule
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
    is the Greek letter “lambda”, giving its name to
    the topic) while the other notation writes
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
    you choose.*) (*As long as you don't collide with
    other existing variable names.)
*/
