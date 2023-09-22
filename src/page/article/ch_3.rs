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
            <ImageRight
            translate="(-2rem, 1.5rem)"
            src="/images/svg_cloud_minus_two_squared.svg"
        />
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
           
        
        </Columns>
    }
}
