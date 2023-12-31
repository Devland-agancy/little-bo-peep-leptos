use crate::componants::Article::*;
use crate::componants::ArticleTitle::*;
use crate::componants::Columns::*;
use crate::componants::ImageLeft::*;
use crate::componants::ImageRight::*;
use crate::componants::MathBlock::*;
use crate::componants::Paragraph::*;
use crate::componants::Span::*;

use leptos::*;

#[component]
pub fn View(cx: Scope) -> impl IntoView {
    view! { cx,
        <ArticleTitle label="Chapter 2: Slopes"/>
        <ArticleBody/>
    }
}

#[component]
fn ArticleBody(cx: Scope) -> impl IntoView {
    view! { cx,
      <Columns>
        <Paragraph>
          <Span bold=true>"The Definition. "</Span>
          "The "
          <Span italic=true bold=true>
            "slope "
          </Span>
          "of a line is a mathematical measure of how “steep” a line is. Here are a few examples (for an explanation of the values, see below):"
        </Paragraph>
        <ImageRight src="/images/svg_cloud_minus_two_squared.svg">""</ImageRight>
        <ImageLeft src="/images/325.svg" hidden_in_mobile=true>
          ""
        </ImageLeft>
        <Paragraph>"or, less pedantically,"</Paragraph>
        <MathBlock>"$$ (-2)^2 = 2 $$"</MathBlock>
        <Paragraph>"or"</Paragraph>

        <Paragraph>"since"</Paragraph>
        <ImageRight src="/images/svg_cloud_minus_two_squared.svg">""</ImageRight>

        <Paragraph>"is"</Paragraph>
        <Paragraph align=Align::Center>
          <Span italic=true>"a difference of squares"</Span>
        </Paragraph>
        <Paragraph>
          <Span bold=true>"The Definition. "</Span>
          "The "
          <Span italic=true bold=true>
            "slope "
          </Span>
          "of a line is a mathematical measure of how “steep” a line is. Here are a few examples (for an explanation of the values, see below):"
        </Paragraph>
        <Paragraph>
          <Span bold=true>"The Definition. "</Span>
          "The "
          <Span italic=true bold=true>
            "slope "
          </Span>
          "of a line is a mathematical measure of how “steep” a line is. Here are a few examples (for an explanation of the values, see below):"
        </Paragraph>
        <Paragraph>
          <Span bold=true>"The Definition. "</Span>
          "The "
          <Span italic=true bold=true>
            "slope "
          </Span>
          "of a line is a mathematical measure of how “steep” a line is. Here are a few examples (for an explanation of the values, see below):"
        </Paragraph>
        <Paragraph>
          <Span bold=true>"The Definition. "</Span>
          "The "
          <Span italic=true bold=true>
            "slope "
          </Span>
          "of a line is a mathematical measure of how “steep” a line is. Here are a few examples (for an explanation of the values, see below):"
        </Paragraph>
        <Paragraph>
          <Span bold=true>"The Definition. "</Span>
          "The "
          <Span italic=true bold=true>
            "slope "
          </Span>
          "of a line is a mathematical measure of how “steep” a line is. Here are a few examples (for an explanation of the values, see below):"
        </Paragraph>
        <Paragraph>
          <Span bold=true>"The Definition. "</Span>
          "The "
          <Span italic=true bold=true>
            "slope "
          </Span>
          "of a line is a mathematical measure of how “steep” a line is. Here are a few examples (for an explanation of the values, see below):"
        </Paragraph>
        <Paragraph>
          <Span bold=true>"The Definition. "</Span>
          "The "
          <Span italic=true bold=true>
            "slope "
          </Span>
          "of a line is a mathematical measure of how “steep” a line is. Here are a few examples (for an explanation of the values, see below):"
        </Paragraph>

      </Columns>
    }
}
