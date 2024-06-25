use crate::components::Article::*;
use crate::components::ArticleTitle::*;
use crate::components::Columns::*;
use crate::components::Exercises::*;
use crate::components::Grid::*;
use crate::components::Image::*;
use crate::components::ImageLeft::*;
use crate::components::ImageLink::*;
use crate::components::ImageRight::*;

use crate::components::Indent::Indent;
use crate::components::InlineImage::*;
use crate::components::List::*;
use crate::components::Math::*;
use crate::components::MathBlock::*;
use crate::components::Paragraph::*;
use crate::components::Section::*;
use crate::components::SectionDivider::*;
use crate::components::StarDivider::*;

use crate::components::Solution::*;
use crate::components::Space::*;
use crate::components::Span::*;
use crate::components::Table::Table;

use crate::constants::MENU_ITEMS;
use elm_to_view::elm;
use leptos::*;

#[component]
pub fn View(cx: Scope) -> impl IntoView {
    view! { cx,
      <ArticleTitle label=MENU_ITEMS[0].0/>
      <Columns>
        <ArticleBody/>
      </Columns>
    }
}

#[component]
fn ArticleBody(cx: Scope) -> impl IntoView {
    elm! {
      cx,
      "file:/src/content/ch_1_emu.rs"
    }
}
