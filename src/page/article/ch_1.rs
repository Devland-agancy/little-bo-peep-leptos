use super::*;

#[component]
pub fn View(cx: Scope) -> impl IntoView {
    view! {cx,
        <Article>
            <ArticleTitle>"Chapter 1: A Few Refreshers"</ArticleTitle>
            <ArticleBody />
        </Article>
    }
}

#[component]
fn ArticleBody(cx: Scope) -> impl IntoView {
    view! {cx,
        <Columns>
        <div class="h-0 relative col-start-1"> <div class=" w-[1500px] h-[100000px] bg-red-700"></div>
       
        </div>
        <Paragraph>
            <Span bold=true>"Square Roots. "</Span>
            "You might remember that “minus times minus is plus” and that “plus times plus is plus”. (Why? The enemy of my enemy is my friend.) So any nonzero number multiplied by itself is positive. For example,"
        </Paragraph>
        <MathBlock height=Height::Fit>
            r#"$$
             (-2) \times (-2) = 4 
             $$"#
        </MathBlock>
        <Paragraph align=Align::Center>
          and
        </Paragraph>
        <MathBlock height=Height::Fit>
            r#"$$
             2 \times 2 = 4 
            $$"#
        </MathBlock>
        <Paragraph>
            "are both positive. But "
            <Math>r#"$\sqrt{4}$"#</Math>
            " is, by definition, the unique "
            <Span italic=true>"nonnegative "</Span>
            "solution to "
            <Math>r#"$x^2 = 4$"# "."</Math>
            " Hence, and whether you like it or not,"
        </Paragraph>
        <MathBlock>
            r#"$$
                \sqrt{(-2)^2} = 2 
            $$"#
        </MathBlock>
        <ImageRight
            translate="(-2rem, 1.5rem)"
            src="/images/svg_cloud_minus_two_squared.svg"
        />
        <Paragraph>
            "and, in particular, it is "
            <Span italic=true>"not "</Span>
            "true that"
        </Paragraph>
        <MathBlock>
            r#"$$
                \sqrt{x^{2}} \rt{0.1} = \rt{0.1} x
            $$"#
        </MathBlock>
        <Paragraph>
            "for every real number "
            <Math>r#"$x$"# "."</Math>
            " Instead we have"
        </Paragraph>
        <MathBlock>
            r#"$$
                \sqrt{x^{2}} \rt{0.1} = \rt{0.1} |x|
            $$"#
        </MathBlock>
        <Paragraph>
            "for every real number "
            <Math>r#"$x$"# ","</Math>
            " where "
            <Math>r#"$|x|$"#</Math>
            " denotes the absolute value of "
            <Math>r#"$x$"# "."</Math>
        </Paragraph>
        <Paragraph indent=Indent::Line>
            "(Nb: If ever you want to indicate both solutions of the equation "
            <Math>r#"$x^2 = 4$"#</Math>
            " you can always use the notation "
            <Math>r#"$\pm \sqrt{4}$"# "."</Math>
            " This is what happens, for example, in the  maybe-well-known formula"
        </Paragraph>
        <MathBlock>
            r#"$$
                x = {-b \pm \sqrt{b^2 - 4ac} \over 2a}
            $$"#
        </MathBlock>
        <Paragraph>
            "for the solutions to the quadratic equation "
            <Math>r#"$ax^2 + bx + c = 0$"# ".)"</Math>
        </Paragraph>
        <Paragraph indent=Indent::Line>
            "Now we can ponder, say, "
        </Paragraph>
        <MathBlock>r#"$$\sqrt{0.5}$$"#</MathBlock>
        <Paragraph>
            "whose value is—by definition—the unique nonnegative solution to"
        </Paragraph>
        <MathBlock>r#"$$x^2 = 0.5.$$"#</MathBlock>
        <Paragraph>
           "As beginners, there's nothing wrong with trying to solve this equation by trial and error. With "
            <Math>r#"$x = \frac{1}{4}$"# ","</Math>
            " for example, we find"
        </Paragraph>
        <MathBlock>
            r#"$$
               x^2 = \frac{1}{4}\times\frac{1}{4} = \frac{1}{16} 
            $$"#
        </MathBlock>
        <Paragraph>
            "so "
            <Math>r#"$x = \frac{1}{4}$"#</Math>
            "is not a solution of the equation, being apparently too small. Increasing "
            <Math>r#"$x$"#</Math>
            " to "
            <Math>r#"$x = \frac{1}{2}$"# ","</Math>
            " say, we find"
        </Paragraph>
        <MathBlock>
            r#"$$
               x^2 = \frac{1}{2}\times\frac{1}{2} = \frac{1}{4} 
            $$"#
        </MathBlock>
        <Paragraph>
            "which is better, since "
            <Math>r#"$1/4$"#</Math>
            " is closer to "
            <Math>r#"$1/2$"# ","</Math>
            " but still too small. Increasing "
            <Math>r#"$x$"#</Math>
            " by "
            <Math>r#"$1/4$"#</Math>
            " again, say, to "
            <Math>r#"$x = \frac{3}{4}$"# ","</Math>
            " we find"
        </Paragraph>
        <MathBlock>
            r#"$$
                x^2 = \frac{3}{4}\times\frac{3}{4} = \frac{9}{16}
            $$"#
        </MathBlock>
        <Paragraph>
            "which—surprise!—is actually pretty close to "
            <Math>r#"$1/2$"# ","</Math>
            " as "
            <Math>r#"$1/2 = 8/16$"# "."</Math>
            " And since "
            <Math>r#"$9/16 > 0.5$"# ", "</Math>
            <Math>r#"$\sqrt{0.5}$"#</Math>
            " must be a little "
            <Span italic=true>"less"</Span>
            " than "
            <Math>r#"$\frac{3}{4} = 0.75$"# "."</Math>
        </Paragraph>
        <Paragraph indent=Indent::Line>
            "In last resort, and in reasonably good agreement with our
            observations, a calculator reveals that"
        </Paragraph>
        <MathBlock>
            r#"$$
            \sqrt{0.5} = 0.7071067...
            $$"#
        </MathBlock>
        <Paragraph>
            "where the decimals trail off with no pattern. (This number is irrational.) Even so, the fact that "
            <Math>r#"$\sqrt{0.5}$"#</Math>
            " is "
            <Span italic=true>"greater"</Span>
            " than "
            <Math>r#"$0.5$"#</Math>
            " is often perceived as counterintuitive."
        </Paragraph>
        <Paragraph indent=Indent::Line>
            "You can think of it this way: multiplying a value by "
            <Math>r#"$0.7071$"# ","</Math>
            " or approximately "
            <Math>r#"$\sqrt{0.5}$"# ","</Math>
            " is like taking "
            <Math>r#"$70.71\%$"#</Math>
            " of that value—for example, say, "
        </Paragraph>
        <MathBlock>
            r#"$$
                605 \cdot 0.7071 = 427.7955
            $$"#
        </MathBlock>
        <Paragraph>
            " is "
            <Math>r#"$70.71\%$"#</Math>
            " of "
            <Math>r#"$605$"# ","</Math>
            " and so on—so if we multiply "
            <Span italic=true>twice</Span>
            " by "
            <Math>r#"$0.7071$"#</Math>
            " we obtain "
            <Math>"“" r#"$70.71\%$"#</Math>
            " of "
            <Math>r#"$70.71\%$"# "”"</Math>
            " and it just so happens that "
            <Math>"“" r#"$70.71\%$"#</Math>
            " of "
            <Math>r#"$70.71\%$"# "”"</Math>
            " is close to "
            <Math>r#"$50\%$"# "."</Math>
        </Paragraph>
        <Paragraph indent=Indent::Line>
            "The point is: if "
            <Math>"“" r#"$X\%$"#</Math>
            " of "
            <Math>r#"$X\%$"# "”"</Math>
            " equals "
            <Math>r#"$50\%$"# ","</Math>
            " then, of course, "
            <Math>r#"$\hspace{0.03em}X > 50$"#</Math>
            "—that much seems logical—and, with
            a little thought, the same phenomenon explains why "
            <Math>r#"$\sqrt{0.5} > 0.5$."#</Math>
        </Paragraph>
        <Paragraph margin_top=15>
            <Span bold=(true)>"Fractions and Division."</Span>
            " An elementary fraction, or division, such as "
        </Paragraph>
        <MathBlock>
            r#"$$
                {50 \over 2}
            $$"#
        </MathBlock>
        <Paragraph>
            "can be thought of in a few different ways:"
        </Paragraph>
        <Paragraph indent=Indent::Block>
            <List>
                <Item>
                    "Fifty halves (i.e., "
                    <Math>r#"$50 \times \frac{1}{2}$"#</Math>
                    ")."
                </Item>
                <Item>
                    "The size obtained when something of size fifty is divided into two equal parts (answer: "
                    <Math>r#"$25$"#</Math>
                    ")."
                </Item>
                <Item>
                    "The number of times that "
                    <Math>r#"$2$"#</Math>
                    " goes into "
                    <Math>r#"$50$"#</Math>
                    " (answer: "
                    <Math>r#"$25$"# ","</Math>
                    " because it takes twenty-five "
                    <Math>r#"$2$"# "s"</Math>
                    " to make up "
                    <Math>r#"$50$"# ")."</Math>
                </Item>
            </List>
        </Paragraph>
        <Paragraph>
            "But "
            <Math>r#"$50/2$"#</Math>
            " is a ratio of integers, which makes things particularly nice! For a ratio of decimals, such as, say, "
        </Paragraph>
        <MathBlock>
            r#"$$
            {1 \over 0.01}
            $$"#
        </MathBlock>
        <Paragraph>
            "our possible points of view are going to be more restricted. Thankfully, however, we can still characterize this fraction as the answer to the question “how many times does $0.01$ go into $1$?” as in the third option above. And, indeed,"
        </Paragraph>
        <MathBlock>
            r#"$$
            \begin{align*}

            {1 \over 0.01} \,&=\,100
            \end{align*}

            $$"#
        </MathBlock>
        <Paragraph>
            "because "
            <Math>r#"$0.01$"#</Math>
            " goes "
            <Math>r#"$100$"#</Math>
            " times into "
            <Math>r#"$1$"# "."</Math>
            " For that matter, "
        </Paragraph>
        <MathBlock margin_left=61 height=Height::Fit>
            r#"$$
            \begin{align*}
            { 1 \over 0.001} &= 1000\textrm{,} \hspace{20px}\up{2}  \\
            { 1 \over 0.0001} &= 10000 \hspace{20px}\up{2}
            \,\textrm{(etc),}
            \end{align*}
            $$"#
        </MathBlock>
        <Paragraph>
            "by the same reasoning, which explains why dividing by smaller and smaller numbers produces larger and larger results (and, by extension, why dividing by $0$ is undefined)."
        </Paragraph>
        <Paragraph margin_top=15>
            <Span bold=(true)>"Note."</Span>
            " In general, the ratio of two decimal numbers can be turned into a ratio of integers by multiplying the ratio top and bottom by a suitable power of "
            <Math>r#"$10$"# "."</Math>
            " E.g.: "
        </Paragraph>
        <MathBlock>
            r#"$$ {1.42 \over 0.8} = {100 \cdot 1.42 \over 100 \cdot 0.8} = {142 \over 80} = {71 \over 40}. $$"#
        </MathBlock>
        <Paragraph>
            "This example was chosen randomly, and, if you allow, we would like to see how large "
            <Math>r#"$71/40$"#</Math>
            " really is (one second!): "
        </Paragraph>
        <MathBlock height=Height::Fit>
            r#"$$ \begin{align} {71 \over 40} \,&=\, {40 + 30 + 1 \over 40} \,=\, {40 \over 40} + {30 \over 40} + {1 \over 40}\\ \,&=\, 1 + {3 \over 4} + {1 \over 4}\!\cdot \!{1 \over 10}\rule{0pt}{1.5em}\\ \,&=\, 1 + 0.75 + 0.025 = 1.775\rule{0pt}{1.5em} \end{align} $$"#
        </MathBlock>
        <Paragraph>
            "...so we find, among others, that "
            <Math>r#"$71$"#</Math>
            " is exactly "
            <Math>r#"$77.5\%$"#</Math>
            " greater than "
            <Math>r#"$40$"# "."</Math>
            " (Interesting, no?)"
        </Paragraph>
        <Paragraph margin_top=15>
            <Span bold=(true)>"Distributivity."</Span>
            " As you might already know, a number that multiplies a sum can be brought “inside” the sum. For example, "
        </Paragraph>
        <MathBlock>
            r#"$$
                5(10 + 2) \,=\, 5\!\cdot\!10 \,+\, 5\!\cdot\!2
            $$"#
        </MathBlock>
        <Paragraph>
            "(five times twelve equals fifty plus ten), or "
        </Paragraph>
        <MathBlock>
            r#"$$
                a(b + c) = ab + ac
            $$"#
        </MathBlock>
        <Paragraph>
            "more generally. This property is known as the "
            <Span italic=true>"distributivity of multiplication over addition"</Span>
            ", or "
            <Span italic=true>"distributivity"</Span>
            " for short. "
        </Paragraph>
        <Paragraph indent=Indent::Line>
            "(We might finally clarify that "
            <Math>"‘" r#"$\cdot$"# "’"</Math>
            " means “times”, i.e., the same as "
            <Math>"‘" r#"$\times$"# "’"</Math>
            ". Moreover, when we write"
        </Paragraph>
        <MathBlock>
            r#"$$
            5\!\cdot\!10 \,+\, 5\!\cdot\!2
            $$"#
        </MathBlock>
        <Paragraph>
            "we really mean"
        </Paragraph>
        <MathBlock>
            r#"$$
            (5\!\cdot\!10) + (5\!\cdot\!2)
            $$"#
        </MathBlock>
        <Paragraph>
            "as opposed to something else, such as"
        </Paragraph>
        <MathBlock>
            r#"$$
            ((5\!\cdot\!10) + 5)\!\cdot\! 2,
            $$"#
        </MathBlock>
        <Paragraph>
            "because multiplication takes precedence over addition, by
                default.)"
        </Paragraph>
        <Paragraph indent=Indent::Line>
            "A little more generally, one has such identities as"
        </Paragraph>

        <MathBlock>
            r#"$$
            (a + b)(C + D) = aC + aD + bC + bD
            $$"#
        </MathBlock>
        <ImageLeft
            translate="(0rem,1.7rem)"
            src="/images/325.svg"
            hidden_in_mobile=true
        />
        <Paragraph>
            "that come from multiplying every term of the first parenthesis with every term of the second parenthesis. Indeed,"
        </Paragraph>
        <MathBlock>
            r#"$$
            (a + b)(C + D) = (a + b)C + (a + b)D
            $$"#
        </MathBlock>
        <Paragraph>
            "by one application of distributivity, while"
        </Paragraph>
        <MathBlock height=Height::Fit>
            r#"$$ \begin{align}
                (a + b)C = aC + bC \\
                (a + b)D = aD + bD\up{1.5}
                \end{align}
            $$"#
        </MathBlock>
        <Paragraph>
            "by distributivity again."
        </Paragraph>
        <Paragraph margin_top=15 >
            <Span bold=true>"Example 1. "</Span> "One has"
        </Paragraph>
        <MathBlock height=Height::Fit>
            r#"$$
                \begin{align}
                (10 + 2)(10 + 4) \,&=\, 10\!\cdot\!10 \,+\, 10\!\cdot\!4 \,+\, 2\!\cdot\!10 \,+\, 2\!\cdot\!4\\
                                    \,&=\, 100 \,+\, 40 \,+\, 20 \,+\, 8\\
                                    \,&=\, 168
                \end{align}
            $$"#
        </MathBlock>
        <Paragraph>
            "so $12" r#"$$ \space  \times \space  $$"# "14 = 168$."
        </Paragraph>
        <Paragraph margin_top=15>
            <Span bold=true>"Example 2. "</Span> "One has"
        </Paragraph>
        <MathBlock height=Height::Fit>
            r#"$$
                \begin{align}
                (10 + 3)(10 + 3) \,&=\, 10\!\cdot\!10 \,+\, 10\!\cdot\!3 \,+\, 3\!\cdot\!10 \,+\, 3\!\cdot\!3\\
                                    \,&=\, 100 \,+\, 30 \,+\, 30 \,+\, 9\\
                                    \,&=\, 169
                \end{align}
            $$"#
        </MathBlock>
        <Paragraph>
            "so $13" r#"$$ \space  \times \space  $$"# "13 = 160$."
        </Paragraph>
        <Paragraph  margin_top=15>
            "(The fact that $13" r#"$$   \times   $$"# " 13$ is exactly one
                greater than $12" r#"$$   \times   $$"# " 14$ is a bit curious indeed.)"
        </Paragraph>
        <Paragraph indent=Indent::Line>
            "If we start from the afore-mentioned identity"
        </Paragraph>
        <MathBlock>
            r#"$$
            (a + b)(C + D) \,=\, aC + bC + aD + bD
            $$"#
        </MathBlock>
        <Paragraph>
        "and set $C = a$, $D = b$, we find"
        </Paragraph>
        <MathBlock>
            r#"$$
            (a + b)(a + b) \,=\, aa + ba + ab + bb
            $$"#
        </MathBlock>
        <Paragraph>
        "or, equivalently,"
        </Paragraph>
        <MathBlock>
            r#"$$
            (a + b)^2 = a^2 + 2ab + b^2
            $$"#
        </MathBlock>
        <Paragraph>
            "since $(a + b)(a + b) = (a + b)^2$, $aa = a^2$ and $bb = b^2$. (This
            is the" <Span italic=true>" binomial expansion of degree two, "</Span> "but such terminology
            is not very important at this stage.)"
        </Paragraph>
        <Paragraph margin_top=15>
        <Span bold=true>"Example 3. "</Span> "By the last formula (or “binomial expansion of degree two”),"
        </Paragraph>
        <MathBlock height=Height::Fit>
            r#"$$
            \begin{align}
                \up{1} (10 + 3)^2 \,&=\, 10\!\cdot\!10 \,+\, 2\!\cdot\!3\!\cdot\!10 \,+\, 3\!\cdot\!3 \\
                \up{1} \,&=\, 100 + 60 + 9 \\
                \up{1} \,&=\, 169
            \end{align}
            $$"#
        </MathBlock>
        <Paragraph>
            which agrees with Example 2.
        </Paragraph>
        <Paragraph margin_top=15>
           "On the other hand, setting $C = a$, $D = -b$ in"
        </Paragraph>
        <MathBlock>
            r#"$$
                \,\,\,(a + b)(C + D) = aC + aD + bC + bD
            $$"#
        </MathBlock>
        <Paragraph>
           "gives"
        </Paragraph>
        <MathBlock>
            r#"$$
                \,\,\,(a + b)(a + (-b)) = aa + a(-b) + ba + b(-b)
            $$"#
        </MathBlock>
        <Paragraph>
            "or, less pedantically,"
        </Paragraph>
        <MathBlock>
            r#"$$
            \,\,\,(a + b)(a - b) = aa - ab + ba - bb
            $$"#
        </MathBlock>
        <Paragraph>
            "or"
        </Paragraph>
        <MathBlock>
            r#"$$
                \,\,\,(a + b)(a - b) = a^2 - b^2
            $$"#
        </MathBlock>
        <Paragraph>
            "since" <Math>r#"$- ab + ba = 0$"#</Math> ", " <Math>r#"$aa = a^2$"#</Math> ", " <Math>r#"$bb = b^2$"#</Math>". Note that"
        </Paragraph>
        <MathBlock>
            r#"$$
            \,\,\,a^2 - b^2
            $$"#
        </MathBlock>
        <Paragraph>
            "is"
        </Paragraph>
        <Paragraph align=Align::Center>
            <Span italic=true>"a difference of squares"</Span>
        </Paragraph>
        <Paragraph margin_top=15>
        "whence" <Span italic=true>a "a difference of squares can always be factored"</Span>"."
		"(Factored as" <Math>r#"$(a + b)(a - b)$"#</Math> ", that is.)"
		"(PS: “Factored” means “written as a product”.)"
        </Paragraph>
        <Paragraph margin_top=15>
            <Span bold=true>"Example 4. "</Span> "Since"
        </Paragraph>
        <MathBlock>
            r#"$$
                19 = 100 - 81 = 10^2 - 9^2
            $$"#
        </MathBlock>
        <Paragraph>
            "is a difference of squares, $19$ can be factored. (On the other
                hand $19$ is a prime number, but nevermind.)"
        </Paragraph>
        <Paragraph margin_top=15>
            <Span bold=true>"Example 5. "</Span> "The algebraic expression "
        </Paragraph>
        <MathBlock>
            r#"$$
            1 - x^2
            $$"#
        </MathBlock>
        <Paragraph>
            "can be factored, because"
        </Paragraph>
        <MathBlock>
            r#"$$
                1 = 1^2
            $$"#
        </MathBlock>
        <Paragraph>
            "implies that"
        </Paragraph>
        <MathBlock>
            r#"$$
                1 - x^2
            $$"#
        </MathBlock>
        <Paragraph>
            "truly is “a difference of squares”. And, indeed,"
        </Paragraph>
        <MathBlock>
            r#"$$
            1 - x^2 = (1 - x)(1 + x)
            $$"#
        </MathBlock>
        <Paragraph>
            "as per “" <Math>r#"$ a^2 - b^2 = (a - b)(a + b) $"#</Math> "”."
        </Paragraph>
        <Paragraph>
            "In relation to distributivity, we should also mention the
            simple but important fact that multiplying a difference by" <Math>r#"$-1$"#</Math> <Span italic=true>"reverses"</Span>
            "the difference. That is,"
        </Paragraph>
        <MathBlock>
            r#"$$
            (-1)(a - b) \,=\, b - a
            $$"#
        </MathBlock>
        <Paragraph>
            "or, for short,"
        </Paragraph>
        <MathBlock>
            r#"$$
                -(a - b) \,=\, b - a
            $$"#
        </MathBlock>
        <Paragraph>
            "because, indeed,"
        </Paragraph>
        <MathBlock>
            r#"$$
                \begin{align}
                (-1)(a - b) \,&=\, (-1)(a + (-b)) \\
                \,&=\, (-1)a + (-1)(-b) \\
                \,&=\, -a + b
                \end{align}
            $$"#
        </MathBlock>
        <Paragraph>
            "by distributivity (used in the second step)."
        </Paragraph>
        <Paragraph margin_top=15>
            <Span bold=true>"Example 6. "</Span> "We have " <Math>r#"$-(10 - 3) = 3 - 10$"#</Math> ". 
            (Because "<Math>r#"$ -7 = -7 $"#</Math> ", as it would be, haha.)"
        </Paragraph>
        <Paragraph margin_top=15>
            <Span bold=true>"Epilogue. "</Span> "Do you remember the near miss between"
        </Paragraph>
        <MathBlock>
            r#"$$
                12\cdot 14 \,=\, 168
            $$"#
        </MathBlock>
        <Paragraph >
           "and"
        </Paragraph>
        <MathBlock>
            r#"$$
                13 \cdot 13 \,=\, 13^2 \,=\, 169
            $$"#
        </MathBlock>
        <Paragraph >
           "...? Well if you observe, additionally, that"
        </Paragraph>
        <MathBlock height=Height::Fit>
            r#"$$
                \begin{align}
                11\,\cdot\,13 &= 12^2 - 1\\
        
                10\,\cdot\,12 &= 11^2 - 1\\
        
                9\,\cdot\,11 &= 10^2 - 1
                \end{align}
            $$"#
        </MathBlock>
        <Paragraph >
            "(etc) you might become suspicious of a pattern! But the
            mystery is rather thin: we have"
        </Paragraph>
        <MathBlock>
            r#"$$
                (n - 1)(n + 1) \,=\, n^2 - 1
            $$"#
        </MathBlock>
        <Paragraph >
           "for" <Span italic=true>"every"</Span> "real number " <Math>r#"n$"#</Math> " because of the formula"
        </Paragraph>
        <MathBlock>
            r#"$$
                (a - b)(a + b) \,=\, a^2 - b^2
            $$"#
        </MathBlock>
        <Paragraph >
            "for a difference of squares!"
        </Paragraph>
        <Paragraph margin_top=15>
            <Span bold=true>"Vocabulary. "</Span> "A pair of algebraic expressions of the form"
        </Paragraph>
        <MathBlock>
            r#"$$
                a + b,\, a - b
            $$"#
        </MathBlock>
        <Paragraph>
            "is called a " <Span italic=true>"conjugate pair"</Span>". For example,"
        </Paragraph>
        <MathBlock>
            r#"$$
                n + 1,\, n - 1
            $$"#
        </MathBlock>
        <Paragraph>
            "is a conjugate pair, as is"
        </Paragraph>
        <MathBlock>
            r#"$$
                \sqrt{3} + \sqrt{2},\,\, \sqrt{3} - \sqrt{2}
            $$"#
        </MathBlock>
        <Paragraph margin_top=15>
            "and so on. (Generally speaking, conjugate pairs are good things to multiply together.)"
        </Paragraph>
        <Image src="/images/exercises.png" height=130/>
        <Tabs>
            <TabElement scroll_to="#exo_1" label="1" />
            <TabElement scroll_to="#exo_1" label="2" />
        </Tabs>
        <Paragraph id="exo_1" margin_top=15>
            "and so on. (Generally speaking, conjugate pairs are good things to multiply together.)"
        </Paragraph>
    </Columns>
    }
}
