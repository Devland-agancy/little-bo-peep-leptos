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
        <Bold>"The Definition. "</Bold>
        "The "
        <Italic>"slope "</Italic>
        "of a line is a mathematical measure of how “steep” a line is. Here are a few examples (for an explanation of the values, see below):"
        </Paragraph>
        </Columns>
    }
}
