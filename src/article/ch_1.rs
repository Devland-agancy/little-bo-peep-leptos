use super::*;
use leptos::*;

pub fn view(cx: Scope) -> impl IntoView {
    view! {cx,
        <Article>
            <ArticleTitle />
            <ArticleBody />
        </Article>
    }
}

#[component]
fn ArticleTitle(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="lg:grid lg:grid-cols-[1fr_32.5rem_1fr]">
            <h1 class="lg:col-start-2 text-4xl p-4">
                "Chapter 1: A Few Refreshers"
            </h1>
        </div>
    }
}

#[component]
fn ArticleBody(cx: Scope) -> impl IntoView {
    view! {cx,
        <Columns>
            <Paragraph>
                <span class="font-baskerville-bold indent-0">
                    "Square Roots. "
                </span>
                "You might remember that “minus times minus is plus” and that “plus times plus is plus”. (Why? The enemy of my enemy is my friend.) So any nonzero number multiplied by itself is positive. For example,"
            </Paragraph>
            <MathBlock>
                r#"$$(-2) \times (-2) = 4\qquad\textrm{and}\qquad 2 \times 2 = 4$$"#
            </MathBlock>
            <Paragraph>
                "are both positive. But "
                <Math>r#"$\sqrt{4}$"#</Math>
                " is, by definition, the unique "
                <span class="font-baskerville-italic">"nonnegative "</span>
                "solution to "
                <Math>r#"$x^2 = 4$"#</Math>
                ". Hence, and whether you like it or not,"
            </Paragraph>
        </Columns>
    }
}
