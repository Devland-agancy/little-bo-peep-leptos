use super::*;

#[component]
pub fn View(cx: Scope) -> impl IntoView {
    view! {cx,
        <Article>
            <ArticleTitle>"Chapter 2: Slopes"</ArticleTitle>
            <ArticleBody />
        </Article>
    }
}

#[component]
fn ArticleBody(cx: Scope) -> impl IntoView {
    view! {cx,
        <Columns>
            <Paragraph>
            <Span bold=true>"The Definition. "</Span>
            "The "
            <Span italic=true bold=true>"slope "</Span>
            "of a line is a mathematical measure of how “steep” a line is. Here are a few examples (for an explanation of the values, see below):"
            </Paragraph>
            <ImageRight
                translate="(-2rem, 1.5rem)"
                src="/images/svg_cloud_minus_two_squared.svg"
            />
            <ImageLeft
                src="/images/325.svg"
                translate="(0rem, 1.5rem)"
                hidden_in_mobile=true
            />
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
        " (PS: “Factored” means “written as a product”.)"
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
            <ImageRight
                translate="(-2rem, 1.5rem)"
                src="/images/svg_cloud_minus_two_squared.svg"
            />
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
            <Paragraph margin_top=15>
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
            <MathBlock height=Height::Fit>
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
              "for" <Span italic=true>"every"</Span> "real number " <Math>r#"$n$"#</Math> " because of the formula"
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
        </Columns>
    }
}
