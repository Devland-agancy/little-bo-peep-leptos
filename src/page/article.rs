#![allow(unused_imports)]
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
use render_chapters::render_article_modules;

render_article_modules! {"chapters"}
render_article_modules! {"bootcamps"}
