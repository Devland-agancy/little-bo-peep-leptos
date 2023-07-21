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
            <Paragraph indent=true>
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
            <Paragraph indent=true>
                "Now we can ponder, say, "
            </Paragraph>
            <MathBlock>r#"$$\sqrt{0.5}$$"#</MathBlock>
            <Paragraph>
                "whose value is—by definition—the unique nonnegative solution to"
            </Paragraph>
            <MathBlock>r#"$$x^2 = 0.5$$"# "."</MathBlock>
            <Paragraph>
               "As beginners, there's nothing wrong with trying to solve this equation by trial and error. With "
                <Math>r#"$x = \frac{1}{4}$"# ","</Math>
                " for example, we find"
            </Paragraph>
            <MathBlock>
                r#"$$
                   x^2 = \frac{1}{2}\times\frac{1}{2} = \frac{1}{4} 
                $$"#
            </MathBlock>
        </Columns>
    }
}
