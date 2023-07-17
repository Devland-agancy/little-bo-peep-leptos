use leptos::*;

#[component]
pub fn Article(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="pt-14 lg:pt-20">
            <div class="absolute flex justify-center align-center animate-appear w-full overflow-hidden" id="Article">
                <div class="w-full md:w-192 lg:w-full transition duration-300 lg:overflow-visible lg:translate-x-0">
                    <div class="font-baskerville w-full">
                        <ArticleTitle />
                        <ArticleBody />
                    </div>
                </div>
            </div>
        </div>
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
                "nonnegative "
                "solution to "
                <Math>r#"$x^2 = 4$"#</Math>
                ". Hence, and whether you like it or not,"
            </Paragraph>
        </Columns>
    }
}

#[component]
fn Columns(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="relative indent-0 text-xl sm:leading-relaxed -translate-x-[1000px] lg:translate-x-0 grid grid-cols-[1000px_100%_1000px] lg:grid lg:grid-cols-[1fr_32.5rem_1fr]">
            {children(cx)}
        </div>
    }
}

#[component]
fn Math(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="inline-block">
            {children(cx)}
        </div>
    }
}

#[component]
fn MathBlock(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="indent-0 text-xl min-h-[4rem] flex items-center justify-center col-start-2">
            {children(cx)}
        </div>
    }
}

#[component]
fn Paragraph(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <span class="col-start-2 px-4">
            {children(cx)}
        </span>
    }
}
