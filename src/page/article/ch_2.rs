use crate::components::Article::*;
use crate::components::ArticleTitle::*;
use crate::components::Columns::*;
use crate::components::Image::*;
use crate::components::ImageLeft::*;
use crate::components::ImageRight::*;

use crate::components::Math::*;
use crate::components::MathBlock::*;

use crate::components::Exercises::*;
use crate::components::ImageLink::*;
use crate::components::Indent::*;
use crate::components::InlineImage::*;
use crate::components::Paragraph::*;
use crate::components::Section::*;
use crate::components::SectionDivider::*;
use crate::components::Solution::*;
use crate::components::Span::*;
use crate::components::StarDivider::*;
use crate::components::Table::*;

use crate::constants::MENU_ITEMS;

use elm_to_view::elm;
use leptos::*;

#[component]
pub fn View(cx: Scope) -> impl IntoView {
    view! { cx,
      <ArticleTitle on_mobile=MENU_ITEMS[1].1 label=MENU_ITEMS[1].0/>
      <Columns>
        <ArticleBody/>
      </Columns>
    }
}

#[component]
fn ArticleBody(cx: Scope) -> impl IntoView {
    elm! {
      cx,
      "file:/src/content/ch_2_emu.rs"
    }
}
