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
            >""</ImageLeft>
            <Paragraph>
            "or, less pedantically,"
            </Paragraph>
              <MathBlock>
              "$$ (-2)^2 = 2 $$"
            </MathBlock>
            <Paragraph>
                "or"
            </Paragraph>
          
            <Paragraph>
                "since" 
            </Paragraph>
            <ImageRight
            translate="(-2rem, 1.5rem)"
            src="/images/svg_cloud_minus_two_squared.svg"
        />
            
            <Paragraph>
                "is"
            </Paragraph>
            <Paragraph align=Align::Center>
                <Span italic=true>"a difference of squares"</Span>
            </Paragraph>
            <Paragraph>
            <Span bold=true>"The Definition. "</Span>
            "The "
            <Span italic=true bold=true>"slope "</Span>
            "of a line is a mathematical measure of how “steep” a line is. Here are a few examples (for an explanation of the values, see below):"
            </Paragraph>  <Paragraph>
            <Span bold=true>"The Definition. "</Span>
            "The "
            <Span italic=true bold=true>"slope "</Span>
            "of a line is a mathematical measure of how “steep” a line is. Here are a few examples (for an explanation of the values, see below):"
            </Paragraph>  <Paragraph>
            <Span bold=true>"The Definition. "</Span>
            "The "
            <Span italic=true bold=true>"slope "</Span>
            "of a line is a mathematical measure of how “steep” a line is. Here are a few examples (for an explanation of the values, see below):"
            </Paragraph>  <Paragraph>
            <Span bold=true>"The Definition. "</Span>
            "The "
            <Span italic=true bold=true>"slope "</Span>
            "of a line is a mathematical measure of how “steep” a line is. Here are a few examples (for an explanation of the values, see below):"
            </Paragraph>  <Paragraph>
            <Span bold=true>"The Definition. "</Span>
            "The "
            <Span italic=true bold=true>"slope "</Span>
            "of a line is a mathematical measure of how “steep” a line is. Here are a few examples (for an explanation of the values, see below):"
            </Paragraph>  <Paragraph>
            <Span bold=true>"The Definition. "</Span>
            "The "
            <Span italic=true bold=true>"slope "</Span>
            "of a line is a mathematical measure of how “steep” a line is. Here are a few examples (for an explanation of the values, see below):"
            </Paragraph>  <Paragraph>
            <Span bold=true>"The Definition. "</Span>
            "The "
            <Span italic=true bold=true>"slope "</Span>
            "of a line is a mathematical measure of how “steep” a line is. Here are a few examples (for an explanation of the values, see below):"
            </Paragraph>
        
        </Columns>
    }
}