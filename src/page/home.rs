use super::*;

#[component]
pub fn View(cx: Scope) -> impl IntoView {
    view! {cx,
        <Article>
            <ArticleTitle>"Chapters"</ArticleTitle>
            <ArticleBody />
        </Article>
    }
}

#[component]
fn ArticleBody(cx: Scope) -> impl IntoView {
    view! {cx,
        <Columns>
            <Paragraph>
                <ul class="leading-9 text-2xl lg:text-3xl">
                    <li>
                         <Link href="article/ch_1">
                            "Chapter 1: A Few Refreshers"
                        </Link>
                    </li>
                    <li>
                         <Link href="article/ch_2">
                            "Chapter 2: Slopes"
                        </Link>
                    </li>
                </ul>
            </Paragraph>
        </Columns>
    }
}
