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
        <Paragraph>
            <Bold>"Square Roots. "</Bold>
            "You might remember that “minus times minus is plus” and that “plus times plus is plus”. (Why? The enemy of my enemy is my friend.) So any nonzero number multiplied by itself is positive. For example,"
        </Paragraph>
        <Image src="/images/charts.svg" height=800 />
        <MathBlock>
            r#"$$(-2) \times (-2) = 4\qquad\textrm{and}\qquad 2 \times 2 = 4$$"#
        </MathBlock>
        <Paragraph>
            "are both positive. But "
            <Math>r#"$\sqrt{4}$"#</Math>
            " is, by definition, the unique "
            <Italic>"nonnegative "</Italic>
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
            <Italic>"not "</Italic>
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
            <Italic>"less"</Italic>
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
            <Italic>"greater"</Italic>
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
            <Italic>twice</Italic>
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
        <Paragraph>
            <Bold>"Fractions and Division."</Bold>
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
            {1 \over 0.01} \,=\,100
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
        <MathBlock>
            r#"$$
            { 1 \over 0.001} = 1000,\qquad{1 \over 0.0001} = 10000,\quad\,\,\,\,\textrm{(etc)}
            $$"#
        </MathBlock>
        <Paragraph>
            "by the same reasoning, which explains why dividing by smaller and smaller numbers produces larger and larger results (and, by extension, why dividing by $0$ is undefined)."
        </Paragraph>
        <Paragraph>
            <Bold>"Note."</Bold>
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
        <Paragraph>
            <Bold>"Distributivity."</Bold>
            " As you might already know, a number that multiplies a sum can be bought “inside” the sum. For example, "
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
            <Italic>"distributivity of multiplication over addition"</Italic>
            ", or "
            <Italic>"distributivity"</Italic>
            " for short. "
        </Paragraph>
        <Paragraph>
            "(We might finally clarify that "
            <Math>"‘" r#"$\cdot$"# "’"</Math>
            " means “times”, i.e., the same as "
            <Math>"‘" r#"$\times$"# "’"</Math>
            " Moreover, when we write"
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
        <ImageLeft
            translate="(2rem,1.7rem)"
            src="/images/325.svg"
        />
        <MathBlock>
            r#"$$
            (a + b)(C + D) = (a + b)C + (a + b)D
            $$"#
        </MathBlock>
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
        <MathBlock>
            r#"$$
            (a + b)C = aC + bC\\
            (a + b)D = aD + bD
            $$"#
        </MathBlock>
        <Paragraph>
            "by distributivity again."
        </Paragraph>
        </Columns>
    }
}
