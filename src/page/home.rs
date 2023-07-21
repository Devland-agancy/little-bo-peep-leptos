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
                <ul class="text-3xl">
                    <li>
                         <Link href="article/ch_1">
                            "Chapter 1: A Few Refreshers"
                        </Link>
                    </li>
                </ul>
            </Paragraph>
        </Columns>
    }
}
