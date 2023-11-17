use crate::componants::Article::*;
use crate::componants::ArticleTitle::*;
use crate::componants::Columns::*;
use crate::componants::Grid::*;
use crate::componants::Image::*;
use crate::componants::ImageLeft::*;
use crate::componants::ImageRight::*;
use crate::componants::List::*;
use crate::componants::Math::*;
use crate::componants::MathBlock::*;
use crate::componants::Paragraph::*;
use crate::componants::Solution::*;
use crate::componants::Span::*;
use crate::componants::Tabs::*;

use leptos::*;

#[component]
pub fn View(cx: Scope) -> impl IntoView {
    view! { cx,
      <Article>
        <ArticleTitle>"Chapter 1: A Few Refreshers"</ArticleTitle>
        <ArticleBody/>
      </Article>
    }
}

#[component]
fn ArticleBody(cx: Scope) -> impl IntoView {
    view! { cx,
      <Columns>
        <Paragraph>
          <Span bold=true>"Square Roots. "</Span>
          "You might remember that “minus times minus is plus” and that “plus times plus is plus”. (Why? The enemy of my enemy is my friend.) So any nonzero number multiplied by itself is positive. For example,"
        </Paragraph>
        <MathBlock height=Height::Fit>
          r#"$$
           (-2) \times (-2) = 4 
           $$"#
        </MathBlock>
        <Paragraph align=Align::Center>and</Paragraph>
        <MathBlock height=Height::Fit>
          r#"$$
           2 \times 2 = 4 
          $$"#
        </MathBlock>
        <Paragraph>
          "are both positive. But " <Math>r#"$\sqrt{4}$"#</Math> " is, by definition, the unique "
          <Span italic=true>"nonnegative "</Span> "solution to " <Math>r#"$x^2 = 4$"# "."</Math>
          " Hence, and whether you like it or not,"
        </Paragraph>
        <MathBlock>
          r#"$$
              \sqrt{(-2)^2} = 2 
          $$"# <ImageRight src="/images/svg_cloud_minus_two_squared.svg" pos_x="1rem" pos_y="-2rem">
            ""
          </ImageRight>
        </MathBlock>
        <Paragraph>
          "and, in particular, it is " <Span italic=true>"not "</Span> "true that"
        </Paragraph>
        <MathBlock>
          r#"$$
              \sqrt{x^{2}} \rt{0.1} = \rt{0.1} x
          $$"#
        </MathBlock>
        <Paragraph>"for every real number " <Math>r#"$x$"# "."</Math> " Instead we have"</Paragraph>
        <MathBlock>
          r#"$$
              \sqrt{x^{2}} \rt{0.1} = \rt{0.1} |x|
          $$"#
        </MathBlock>
        <Paragraph>
          "for every real number " <Math>r#"$x$"# ","</Math> " where " <Math>r#"$|x|$"#</Math>
          " denotes the absolute value of " <Math>r#"$x$"# "."</Math>
        </Paragraph>
        <Paragraph indent=Indent::Line>
          "(Nb: If ever you want to indicate both solutions of the equation "
          <Math>r#"$x^2 = 4$"#</Math> " you can always use the notation "
          <Math>r#"$\pm \sqrt{4}$"# "."</Math>
          " This is what happens, for example, in the  maybe-well-known formula"
        </Paragraph>
        <MathBlock>
          r#"$$
              x = {-b \pm \sqrt{b^2 - 4ac} \over 2a}
          $$"#
        </MathBlock>
        <Paragraph>
          "for the solutions to the quadratic equation " <Math>r#"$ax^2 + bx + c = 0$"# ".)"</Math>
        </Paragraph>
        <Paragraph indent=Indent::Line>"Now we can ponder, say, "</Paragraph>
        <MathBlock>r#"$$\sqrt{0.5}$$"#</MathBlock>
        <Paragraph>"whose value is—by definition—the unique nonnegative solution to"</Paragraph>
        <MathBlock>r#"$$x^2 = 0.5.$$"#</MathBlock>
        <Paragraph>
          "As beginners, there's nothing wrong with trying to solve this equation by trial and error. With "
          <Math>r#"$x = \frac{1}{4}$"# ","</Math> " for example, we find"
        </Paragraph>
        <MathBlock>
          r#"$$
             x^2 = \frac{1}{4}\times\frac{1}{4} = \frac{1}{16} 
          $$"#
        </MathBlock>
        <Paragraph>
          "so " <Math>r#"$x = \frac{1}{4}$"#</Math>
          " is not a solution of the equation, being apparently too small. Increasing "
          <Math>r#"$x$"#</Math> " to " <Math>r#"$x = \frac{1}{2}$"# ","</Math> " say, we find"
        </Paragraph>
        <MathBlock>
          r#"$$
             x^2 = \frac{1}{2}\times\frac{1}{2} = \frac{1}{4} 
          $$"#
        </MathBlock>
        <Paragraph>
          "which is better, since " <Math>r#"$1/4$"#</Math> " is closer to "
          <Math>r#"$1/2$"# ","</Math> " but still too small. Increasing " <Math>r#"$x$"#</Math>
          " by " <Math>r#"$1/4$"#</Math> " again, say, to " <Math>r#"$x = \frac{3}{4}$"# ","</Math>
          " we find"
        </Paragraph>
        <MathBlock>
          r#"$$
              x^2 = \frac{3}{4}\times\frac{3}{4} = \frac{9}{16}
          $$"#
        </MathBlock>
        <Paragraph>
          "which—surprise!—is actually pretty close to " <Math>r#"$1/2$"# ","</Math> " as "
          <Math>r#"$1/2 = 8/16$"# "."</Math> " And since " <Math>r#"$9/16 > 0.5$"#</Math> ", "
          <Math>r#"$ \sqrt{0.5} $"#</Math> " must be a little " <Span italic=true>"less"</Span>
          " than " <Math>r#"$\frac{3}{4} = 0.75$"# "."</Math>
        </Paragraph>
        <Paragraph indent=Indent::Line>
          "In last resort, and in reasonably good agreement with our
          observations, a calculator reveals that"
        </Paragraph>
        <MathBlock>
          r#"$$
          \sqrt{0.5} = 0.7071067...
          $$"#
        </MathBlock>
        <Paragraph>
          "where the decimals trail off with no pattern. (This number is irrational.) Even so, the fact that "
          <Math>r#"$\sqrt{0.5}$"#</Math> " is " <Span italic=true>"greater"</Span> " than "
          <Math>r#"$0.5$"#</Math> " is often perceived as counterintuitive."
        </Paragraph>
        <Paragraph indent=Indent::Line>
          "You can think of it this way: multiplying a value by " <Math>r#"$0.7071$"#</Math>
          ", or approximately " <Math>r#"$\sqrt{0.5}$"#</Math> ", is like taking "
          <Math>r#"$70.71\%$"#</Math> " of that value—for example, say, "
        </Paragraph>
        <MathBlock>
          r#"$$
          \begin{align}
          605 \cdot 0.7071 = 427.7955
          \end{align}
          $$"#
        </MathBlock>
        <Paragraph>
          " is " <Math>r#"$70.71\%$"#</Math> " of " <Math>r#"$605$"# ","</Math>
          " and so on—so if we multiply " <Span italic=true>twice</Span> " by "
          <Math>r#"$0.7071$"#</Math> " we obtain " <Math>"“" r#"$70.71\%$"#</Math> " of "
          <Math>r#"$70.71\%$"# "”"</Math> " and it just so happens that "
          <Math>"“" r#"$70.71\%$"#</Math> " of " <Math>r#"$70.71\%$"# "”"</Math> " is close to "
          <Math>r#"$50\%$"# "."</Math>
        </Paragraph>
        <Paragraph indent=Indent::Line>
          "The point is: if " <Math>"“" r#"$X\%$"#</Math> " of " <Math>r#"$X\%$"# "”"</Math>
          " equals " <Math>r#"$50\%$"# ","</Math> " then, of course, "
          <Math>r#"$\hspace{0.03em}X > 50$"#</Math> "—that much seems logical—and, with
          a little thought, the same phenomenon explains why " <Math>r#"$\sqrt{0.5} > 0.5$."#</Math>
        </Paragraph>
        <Paragraph margin_top=15>
          <Span bold=(true)>"Fractions and Division."</Span>
          " An elementary fraction, or division, such as "
        </Paragraph>
        <MathBlock>
          r#"$$
              {50 \over 2}
          $$"#
        </MathBlock>
        <Paragraph>"can be thought of in a few different ways:"</Paragraph>
        <Paragraph indent=Indent::Block>
          <List>
            <Item>"Fifty halves (i.e., " <Math>r#"$50 \times \frac{1}{2}$"#</Math> ")."</Item>
            <Item>
              "The size obtained when something of size fifty is divided into two equal parts (answer: "
              <Math>r#"$25$"#</Math> ")."
            </Item>
            <Item>
              "The number of times that " <Math>r#"$2$"#</Math> " goes into " <Math>r#"$50$"#</Math>
              " (answer: " <Math>r#"$25$"# ","</Math> " because it takes twenty-five "
              <Math>r#"$2$"# "s"</Math> " to make up " <Math>r#"$50$"# ")."</Math>
            </Item>
          </List>
        </Paragraph>
        <Paragraph>
          "But " <Math>r#"$50/2$"#</Math>
          " is a ratio of integers, which makes things particularly nice! For a ratio of decimals, such as, say, "
        </Paragraph>
        <MathBlock>
          r#"$$
          {1 \over 0.01}
          $$"#
        </MathBlock>
        <Paragraph>
          "our possible points of view are going to be more restricted. Thankfully, however, we can still characterize this fraction as the answer to the question “how many times does $0.01$ go into $1$?” as in the third option above. And, indeed,"
        </Paragraph>
        <MathBlock>
          r#"$$
          \begin{align*}
          
          {1 \over 0.01} \,&=\,100
          \end{align*}
          
          $$"#
        </MathBlock>
        <Paragraph>
          "because " <Math>r#"$0.01$"#</Math> " goes " <Math>r#"$100$"#</Math> " times into "
          <Math>r#"$1$"# "."</Math> " For that matter, "
        </Paragraph>
        <MathBlock margin_left=61 margin_right=0 height=Height::Fit>
          r#"$$
          \begin{align*}
          { 1 \over 0.001} &= 1000\textrm{,} \hspace{20px}\up{2}  \\
          { 1 \over 0.0001} &= 10000 \hspace{20px}\up{2}
          \,\textrm{(etc),}
          \end{align*}
          $$"#
        </MathBlock>
        <Paragraph>
          "by the same reasoning, which explains why dividing by smaller and smaller numbers produces larger and larger results (and, by extension, why dividing by $0$ is undefined)."
        </Paragraph>
        <Paragraph margin_top=15>
          <Span bold=(true)>"Note."</Span>
          " In general, the ratio of two decimal numbers can be turned into a ratio of integers by multiplying the ratio top and bottom by a suitable power of "
          <Math>r#"$10$"# "."</Math>
          " E.g.: "
        </Paragraph>
        <MathBlock>
          r#"$$ {1.42 \over 0.8} = {100 \cdot 1.42 \over 100 \cdot 0.8} = {142 \over 80} = {71 \over 40}. $$"#
        </MathBlock>
        <Paragraph>
          "This example was chosen randomly, and, if you allow, we would like to see how large "
          <Math>r#"$71/40$"#</Math> " really is (one second!): "
        </Paragraph>
        <MathBlock height=Height::Fit>
          r#"$$ \begin{align} {71 \over 40} \,&=\, {40 + 30 + 1 \over 40} \,=\, {40 \over 40} + {30 \over 40} + {1 \over 40}\\ \,&=\, 1 + {3 \over 4} + {1 \over 4}\!\cdot \!{1 \over 10}\rule{0pt}{1.5em}\\ \,&=\, 1 + 0.75 + 0.025 = 1.775\rule{0pt}{1.5em} \end{align} $$"#
        </MathBlock>
        <Paragraph>
          "...so we find, among others, that " <Math>r#"$71$"#</Math> " is exactly "
          <Math>r#"$77.5\%$"#</Math> " greater than " <Math>r#"$40$"# "."</Math>
          " (Interesting, no?)"
        </Paragraph>
        <Paragraph margin_top=15>
          <Span bold=(true)>"Distributivity."</Span>
          " As you might already know, a number that multiplies a sum can be brought “inside” the sum. For example, "
        </Paragraph>
        <MathBlock>
          r#"$$
              5(10 + 2) \,=\, 5\!\cdot\!10 \,+\, 5\!\cdot\!2
          $$"#
        </MathBlock>
        <Paragraph>"(five times twelve equals fifty plus ten), or "</Paragraph>
        <MathBlock>
          r#"$$
              a(b + c) = ab + ac
          $$"#
        </MathBlock>
        <Paragraph>
          "more generally. This property is known as the "
          <Span italic=true>"distributivity of multiplication over addition"</Span> ", or "
          <Span italic=true>"distributivity"</Span> " for short. "
        </Paragraph>
        <Paragraph indent=Indent::Line>
          "(We might finally clarify that " <Math>"‘" r#"$\cdot$"# "’"</Math>
          " means “times”, i.e., the same as " <Math>"‘" r#"$\times$"# "’"</Math>
          ". Moreover, when we write"
        </Paragraph>
        <MathBlock>
          r#"$$
          5\!\cdot\!10 \,+\, 5\!\cdot\!2
          $$"#
        </MathBlock>
        <Paragraph>"we really mean"</Paragraph>
        <MathBlock>
          r#"$$
          (5\!\cdot\!10) + (5\!\cdot\!2)
          $$"#
        </MathBlock>
        <Paragraph>"as opposed to something else, such as"</Paragraph>
        <MathBlock>
          r#"$$
          ((5\!\cdot\!10) + 5)\!\cdot\! 2,
          $$"#
        </MathBlock>
        <Paragraph>
          "because multiplication takes precedence over addition, by
              default.)"
        </Paragraph>
        <Paragraph indent=Indent::Line>
          "A little more generally, one has such identities as"
        </Paragraph>

        <MathBlock>
          r#"$$
          (a + b)(C + D) = aC + aD + bC + bD
          $$"#
        </MathBlock>
        <Paragraph>
          "that come from multiplying every term of the first parenthesis with every term of the second parenthesis. Indeed,"
          <ImageLeft
            src="/images/325.svg"
            hidden_in_mobile=true
            pos_y="-8rem"
            pos_x="1rem"
            squiggle_right="-0.8rem"
            squiggle_top="45%"
          >
            ""
          </ImageLeft>
        </Paragraph>
        <MathBlock>
          r#"$$
          (a + b)(C + D) = (a + b)C + (a + b)D
          $$"#
        </MathBlock>
        <Paragraph>"by one application of distributivity, while"</Paragraph>
        <MathBlock height=Height::Fit>
          r#"$$ \begin{align}
              (a + b)C = aC + bC \\
              (a + b)D = aD + bD\up{1.5}
              \end{align}
          $$"#
        </MathBlock>
        <Paragraph>"by distributivity again."</Paragraph>
        <Paragraph margin_top=15>
          <Span bold=true>"Example 1. "</Span>
          "One has"
        </Paragraph>
        <MathBlock height=Height::Fit arrow_position="4.4rem 0rem auto auto">
          r#"$$
              \begin{align}
              (10 + 2)(10 + 4) \,&=\, 10\!\cdot\!10 \,+\, 10\!\cdot\!4 \,+\, 2\!\cdot\!10 \,+\, 2\!\cdot\!4\\
                                  \,&=\, 100 \,+\, 40 \,+\, 20 \,+\, 8\\
                                  \,&=\, 168
              \end{align}
          $$"#
        </MathBlock>
        <Paragraph>"so " <Math>r#"$12 \space \times \space 14 = 168$"#</Math> "."</Paragraph>
        <Paragraph margin_top=15>
          <Span bold=true>"Example 2. "</Span>
          "One has"
        </Paragraph>
        <MathBlock height=Height::Fit arrow_position="4.4rem 0rem auto auto">
          r#"$$
              \begin{align}
              (10 + 3)(10 + 3) \,&=\, 10\!\cdot\!10 \,+\, 10\!\cdot\!3 \,+\, 3\!\cdot\!10 \,+\, 3\!\cdot\!3\\
                                  \,&=\, 100 \,+\, 30 \,+\, 30 \,+\, 9\\
                                  \,&=\, 169
              \end{align}
          $$"#
        </MathBlock>
        <Paragraph>"so " <Math>r#"$ 13 \space \times \space 13 = 169$"#</Math> "."</Paragraph>
        <Paragraph margin_top=15>
          "(The fact that " <Math>r#"$13 \times 13$"#</Math> " is exactly one
          greater than " <Math>r#"$12 \times 14$"#</Math> " is a bit curious indeed.)"
        </Paragraph>
        <Paragraph indent=Indent::Line>"If we start from the afore-mentioned identity"</Paragraph>
        <MathBlock>
          r#"$$
          (a + b)(C + D) \,=\, aC + bC + aD + bD
          $$"#
        </MathBlock>
        <Paragraph>"and set " <Math>r#"$C = a, \space D = b$"#</Math> ", we find"</Paragraph>
        <MathBlock>
          r#"$$
          (a + b)(a + b) \,=\, aa + ba + ab + bb
          $$"#
        </MathBlock>
        <Paragraph>"or, equivalently,"</Paragraph>
        <MathBlock>
          r#"$$
          (a + b)^2 = a^2 + 2ab + b^2
          $$"#
        </MathBlock>
        <Paragraph>
          "since " <Math>r#"$(a + b)(a + b) = (a + b)^2 $"#</Math> ", "
          <Math>r#"$ aa = a^2 $"#</Math> ", " <Math>r#"$ bb = b^2$"#</Math> ". (This
          is the" <Span italic=true>" binomial expansion of degree two, "</Span>
          "but such terminology
          is not very important at this stage.)"
        </Paragraph>
        <Paragraph margin_top=15>
          <Span bold=true>"Example 3. "</Span>
          "By the last formula (or “binomial expansion of degree two”),"
        </Paragraph>
        <MathBlock height=Height::Fit>
          r#"$$
          \begin{align}
              \up{1} (10 + 3)^2 \,&=\, 10\!\cdot\!10 \,+\, 2\!\cdot\!3\!\cdot\!10 \,+\, 3\!\cdot\!3 \\
              \up{1} \,&=\, 100 + 60 + 9 \\
              \up{1} \,&=\, 169
          \end{align}
          $$"#
        </MathBlock>
        <Paragraph>which agrees with Example 2.</Paragraph>
        <Paragraph margin_top=15>
          "On the other hand, setting " <Math>r#"$ C = a $"#</Math> ", "
          <Math>r#"$ D = -b $"#</Math> " in"
        </Paragraph>
        <MathBlock>
          r#"$$
              (a + b)(C + D) = aC + aD + bC + bD
          $$"#
        </MathBlock>
        <Paragraph>"gives"</Paragraph>
        <MathBlock>
          r#"$$
              (a + b)(a + (-b)) = aa + a(-b) + ba + b(-b)
          $$"#
        </MathBlock>
        <Paragraph>"or, less pedantically,"</Paragraph>
        <MathBlock>
          r#"$$
          (a + b)(a - b) = aa - ab + ba - bb
          $$"#
        </MathBlock>
        <Paragraph>"or"</Paragraph>
        <MathBlock>
          r#"$$
              \,\,\,(a + b)(a - b) = a^2 - b^2
          $$"#
        </MathBlock>
        <Paragraph>
          "since " <Math>r#"$- ab + ba = 0$"#</Math> ", " <Math>r#"$aa = a^2$"#</Math> ", "
          <Math>r#"$bb = b^2$"#</Math> ". Note that"
        </Paragraph>
        <MathBlock>
          r#"$$
          \,\,\,a^2 - b^2
          $$"#
        </MathBlock>
        <Paragraph>"is"</Paragraph>
        <Paragraph align=Align::Center>
          <Span italic=true>"a difference of squares"</Span>
        </Paragraph>
        <Paragraph margin_top=15>
          "whence" <Span italic=true>a "a difference of squares can always be factored"</Span> ". "
          "(Factored as" <Math>r#"$(a + b)(a - b)$"#</Math> ", that is.)"
          " (PS: “Factored” means “written as a product”.)"
        </Paragraph>
        <Paragraph margin_top=15>
          <Span bold=true>"Example 4. "</Span>
          "Since"
        </Paragraph>
        <MathBlock>
          r#"$$
              19 = 100 - 81 = 10^2 - 9^2
          $$"#
        </MathBlock>
        <Paragraph>
          "is a difference of squares, $19$ can be factored. (On the other
              hand $19$ is a prime number, but nevermind.)"
        </Paragraph>
        <Paragraph margin_top=15>
          <Span bold=true>"Example 5. "</Span>
          "The algebraic expression "
        </Paragraph>
        <MathBlock>
          r#"$$
          1 - x^2
          $$"#
        </MathBlock>
        <Paragraph>"can be factored, because"</Paragraph>
        <MathBlock>
          r#"$$
              1 = 1^2
          $$"#
        </MathBlock>
        <Paragraph>"implies that"</Paragraph>
        <MathBlock>
          r#"$$
              1 - x^2
          $$"#
        </MathBlock>
        <Paragraph>"truly is “a difference of squares”. And, indeed,"</Paragraph>
        <MathBlock>
          r#"$$
          1 - x^2 = (1 - x)(1 + x)
          $$"#
        </MathBlock>
        <Paragraph>"as per “" <Math>r#"$ a^2 - b^2 = (a - b)(a + b) $"#</Math> "”."</Paragraph>
        <Paragraph margin_top=15>
          "In relation to distributivity, we should also mention the
          simple but important fact that multiplying a difference by " <Math>r#"$-1$"#</Math>
          <Span italic=true>" reverses "</Span> "the difference. That is,"
        </Paragraph>
        <MathBlock>
          r#"$$
          (-1)(a - b) \,=\, b - a
          $$"#
        </MathBlock>
        <Paragraph>"or, for short,"</Paragraph>
        <MathBlock>
          r#"$$
              -(a - b) \,=\, b - a
          $$"#
        </MathBlock>
        <Paragraph>"because, indeed,"</Paragraph>
        <MathBlock height=Height::Fit>
          r#"$$
              \begin{align}
              (-1)(a - b) \,&=\, (-1)(a + (-b)) \\
              \,&=\, (-1)a + (-1)(-b) \\
              \,&=\, -a + b
              \end{align}
          $$"#
        </MathBlock>
        <Paragraph>"by distributivity (used in the second step)."</Paragraph>
        <Paragraph margin_top=15>
          <Span bold=true>"Example 6. "</Span>
          "We have "
          <Math>r#"$-(10 - 3) = 3 - 10$"#</Math>
          ".
          (Because "
          <Math>r#"$ -7 = -7 $"#</Math>
          ", as it would be, haha.)"
        </Paragraph>
        <Paragraph margin_top=15>
          <Span bold=true>"Epilogue. "</Span>
          "Do you remember the near miss between"
        </Paragraph>
        <MathBlock>
          r#"$$
              12\cdot 14 \,=\, 168
          $$"#
        </MathBlock>
        <Paragraph>"and"</Paragraph>
        <MathBlock>
          r#"$$
              13 \cdot 13 \,=\, 13^2 \,=\, 169
          $$"#
        </MathBlock>
        <Paragraph>"...? Well if you observe, additionally, that"</Paragraph>
        <MathBlock height=Height::Fit>
          r#"$$
              \begin{align}
              11\,\cdot\,13 &= 12^2 - 1\\
          
              10\,\cdot\,12 &= 11^2 - 1\\
          
              9\,\cdot\,11 &= 10^2 - 1
              \end{align}
          $$"#
        </MathBlock>
        <Paragraph>
          "(etc) you might become suspicious of a pattern! But the
          mystery is rather thin: we have"
        </Paragraph>
        <MathBlock>
          r#"$$
              (n - 1)(n + 1) \,=\, n^2 - 1
          $$"#
        </MathBlock>
        <Paragraph>
          "for" <Span italic=true>" every "</Span> "real number " <Math>r#"$n$"#</Math>
          " because of the formula"
        </Paragraph>
        <MathBlock>
          r#"$$
              (a - b)(a + b) \,=\, a^2 - b^2
          $$"#
        </MathBlock>
        <Paragraph>"for a difference of squares!"</Paragraph>
        <Paragraph margin_top=15>
          <Span bold=true>"Vocabulary. "</Span>
          "A pair of algebraic expressions of the form"
        </Paragraph>
        <MathBlock>
          r#"$$
              a + b,\, a - b
          $$"#
        </MathBlock>
        <Paragraph>
          "is called a " <Span italic=true>"conjugate pair"</Span> ". For example,"
        </Paragraph>
        <MathBlock>
          r#"$$
              n + 1,\, n - 1
          $$"#
        </MathBlock>
        <Paragraph>"is a conjugate pair, as is"</Paragraph>
        <MathBlock>
          r#"$$
              \sqrt{3} + \sqrt{2},\,\, \sqrt{3} - \sqrt{2}
          $$"#
        </MathBlock>
        <Paragraph>
          "and so on. (Generally speaking, conjugate pairs are good things to multiply together.)"
        </Paragraph>
        <Image
          src="/images/seperator.png"
          height=50
          imageClasses="w-[160px]"
          containerClasses="flex items-center"
        />
        <Tabs labels=vec!["1", "2"]>
          <TabElement>
            <Paragraph classes="animate-appear-slow">
              <Span bold=true>"Exercise 1. "</Span>
              "True or false (and, if possible, explain):"
            </Paragraph>
            <Grid classes="animate-appear-slow" margin_top=15 cols=3 sm_cols=2>
              <Span>"a. " <Math>r#"$ 0.9^2 < 0.9 $"#</Math></Span>
              <Span>"b. " <Math>r#"$ \sqrt{0.01} = 0.1 $"#</Math></Span>
              <Span>
                "c. "
                <Math>r#"$ \sqrt[2]{\up{0.8}\sqrt[3]{2}} = \sqrt[3]{\up{0.8}\sqrt[2]{2}} $"#</Math>
              </Span>
              <Span>"d. " <Math>r#"${\sqrt{2} \over \up{0.55}2} = \sqrt{0.5}$"#</Math></Span>
              <Span>"e. " <Math>r#"$ {1 \over \sqrt{2}} = \sqrt{0.5} $"#</Math></Span>
              <Span>"f. " <Math>r#"$ 2^{30} > 1000^3 $"#</Math></Span>
              <Span>"g. " <Math>r#"$ {1 \over 0.95} > 1.05 $"#</Math></Span>
              <Span>"h. " <Math>r#"$ (-1)^{101} = -1 $"#</Math></Span>
              <Span _class="col-span-full sm:col-span-1">
                "i. " <Math>r#"$ {100 \over \up{0.5}99} < {101 \over \up{0.5}100} $"#</Math>
              </Span>
            </Grid>

            <Solution>
              <Paragraph>"Part by part:"</Paragraph>
              <Paragraph margin_top=15>"a. (True) We have"</Paragraph>
              <MathBlock>
                r#"$$
                    0.9^2 = {9 \over 10}\cdot{9 \over 10} = {81 \over 100} = 0.81
                $$"#
              </MathBlock>
              <Paragraph>"and " <Math>r#"$ 0.81 < 0.9 $"#</Math> "."</Paragraph>

              <Paragraph margin_top=15>"b. (True) We have"</Paragraph>
              <MathBlock>
                r#"$$
                0.1^2 = {1 \over 10} \cdot {1 \over 10} = {1 \over 100} = 0.01,
                $$"#
              </MathBlock>
              <Paragraph>
                "and " <Math>r#"$ 0.1 $"#</Math> " is nonnegative, so "
                <Math>r#"$ \sqrt{0.01} = 0.1 $"#</Math> "."
              </Paragraph>
              <Paragraph margin_top=15>
                "c. (True) In fact, " <Math>r#"$ \sqrt[2]{\up{0.75}\sqrt[3]{2}} $"#</Math> " and "
                <Math>r#"$ \sqrt[3]{\up{0.75}\sqrt[2]{2}} $"#</Math> " are both equal to "
                <Math>r#"$ \sqrt[6]{\up{0.6}2} $"#</Math> ". To convince yourself, note that"
              </Paragraph>
              <MathBlock id="sqrt-1" height=Height::Fit arrow_position="8.5rem 0rem auto auto">
                r#"$$
                    \begin{align}
                    &\,\, (\sqrt[2]{\up{0.75}\sqrt[3]{2}}\rt{0.1})^6 \\
                    =&\,\, \up{1.3}
                    \sqrt[2]{\up{0.75}\sqrt[3]{2}} \times
                    \sqrt[2]{\up{0.75}\sqrt[3]{2}} \times
                    \sqrt[2]{\up{0.75}\sqrt[3]{2}} \times
                    \sqrt[2]{\up{0.75}\sqrt[3]{2}} \times
                    \sqrt[2]{\up{0.75}\sqrt[3]{2}} \times
                    \sqrt[2]{\up{0.75}\sqrt[3]{2}}\qquad\\
                    =&\,\, \up{1.3}
                    (\gbk\sqrt[2]{\up{0.75}\sqrt[3]{2}} \times
                    \sqrt[2]{\up{0.75}\sqrt[3]{2}}\rt{0.11}) \times
                    (\gbk\sqrt[2]{\up{0.75}\sqrt[3]{2}} \times
                    \sqrt[2]{\up{0.75}\sqrt[3]{2}}\rt{0.11}) \times
                    (\gbk\sqrt[2]{\up{0.75}\sqrt[3]{2}} \times
                    \sqrt[2]{\up{0.75}\sqrt[3]{2}}\rt{0.11}) \\
                    =& \,\, \up{1.3} (\sqrt[3]{\up{0.64}2}\rt{0.1}) \times (\sqrt[3]{\up{0.64}2}\rt{0.1}) \times
                    (\sqrt[3]{\up{0.64}2}\rt{0.1})\\
                    =& \,\, \up{1.4} 2
                    \end{align}
                $$"#
                <ImageLeft
                  src="/images/17.svg"
                  pos_y="3.5rem"
                  pos_x="-3.2rem"
                  hidden_in_mobile=true
                  squiggle_right="-4rem"
                  squiggle_top="30%"
                >
                  ""
                </ImageLeft>
              </MathBlock>
              <Paragraph>"and"</Paragraph>
              <MathBlock id="sqrt-2" height=Height::Fit arrow_position="8.5rem 0rem auto auto">
                r#"$$
                    \begin{align}
                    &\,\, (\sqrt[3]{\up{0.75}\sqrt[2]{2}}\rt{0.1})^6 \\
                    =&\,\, \up{1.3}
                    \sqrt[3]{\up{0.75}\sqrt[2]{2}} \times
                    \sqrt[3]{\up{0.75}\sqrt[2]{2}} \times
                    \sqrt[3]{\up{0.75}\sqrt[2]{2}} \times
                    \sqrt[3]{\up{0.75}\sqrt[2]{2}} \times
                    \sqrt[3]{\up{0.75}\sqrt[2]{2}} \times
                    \sqrt[3]{\up{0.75}\sqrt[2]{2}}\\
                    =& \,\, \up{1.3}
                    (\gbk\sqrt[3]{\up{0.75}\sqrt[2]{2}} \times
                    \sqrt[3]{\up{0.75}\sqrt[2]{2}} \times
                    \sqrt[3]{\up{0.75}\sqrt[2]{2}}\rt{0.11}) \times
                    (\gbk\sqrt[3]{\up{0.75}\sqrt[2]{2}} \times
                    \sqrt[3]{\up{0.75}\sqrt[2]{2}} \times
                    \sqrt[3]{\up{0.75}\sqrt[2]{2}}\rt{0.11})\\
                    =&\,\, \up{1.3}
                    \sqrt[2]{\up{0.65}2} \times \sqrt[2]{\up{0.65}2}\\
                    =&\,\, \up{1.4} 2
                    \end{align}
                $$"#
                <ImageLeft
                  src="/images/18.svg"
                  hidden_in_mobile=true
                  squiggle_right="-4rem"
                  squiggle_top="37%"
                  pos_y="3.2rem"
                  pos_x="-3.2rem"
                >
                  ""
                </ImageLeft>
              </MathBlock>
              <Paragraph>
                "so "
                <Math>
                  r#"$  (\sqrt[2]{\up{0.76}\sqrt[3]{2}}\rt{0.1})^6 =
                  (\sqrt[3]{\up{0.76}\sqrt[2]{2}}\rt{0.1})^6 = 2 $"# "."
                </Math>
              </Paragraph>
              <Paragraph indent=Indent::Line>
                "Technically, however, a number $x$ such that"
              </Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                    x^6 = 2
                $$"#
              </MathBlock>
              <Paragraph>
                "is not necessarily " <Math>r#"$ \sqrt[6]{\up{0.6}2} $"#</Math> ", because
                " <Math>r#"$ x = -\sqrt[6]{\up{0.6}2} $"#</Math> " satisfies this equation
                as well!"
              </Paragraph>
              <Paragraph indent=Indent::Line>
                "The last step, therefore, is to note that "
                <Math>r#"$ \sqrt[2]{\up{0.76}\sqrt[3]{2}} $"#</Math> " and "
                <Math>r#"$ \sqrt[3]{\up{0.76}\sqrt[2]{2}} $"#</Math> " are both"
                <Span italic=true>" nonnegative "</Span> "numbers (taken as obvious),
                and which implies that they are the " <Span italic=true>"unique nonnegative"</Span>
                " solution to " <Math>r#"$ x^6 = 2 $"#</Math> "."
              </Paragraph>
              <Paragraph margin_top=15>"d. (True) In general, "</Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                    {\sqrt{x} \over \sqrt{y}} = \sqrt{\up{0.7}x \over y}
                $$"#
              </MathBlock>
              <Paragraph>
                "for all " <Math>r#"$ x \geq 0 $"#</Math> " " , " " <Math>r#"$ y > 0 $"#</Math>
                " (you need each root to be defined),
                so"
              </Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                    {\sqrt{2} \over 2} = {\sqrt{2} \over \sqrt{4}} = \sqrt{\up{0.8}2 \over 4} = \sqrt{0.5}
                $$"#
              </MathBlock>
              <Paragraph>"...ta-daa!"</Paragraph>
              <Paragraph margin_top=15>
                <Span italic=true>"Note 1."</Span>
                " One can also proceed by “direct verification”:"
              </Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                    \left({\sqrt{2} \over 2}\right)^{\!2} = {\sqrt{2} \over 2}\cdot{\sqrt{2} \over 2}
                    = {\sqrt{2}\cdot\sqrt{2} \over 4} = {2 \over 4} = 0.5.
                $$"#
              </MathBlock>
              <Paragraph>
                "(This, together with the fact that " <Math>r#"${\sqrt{2} \over 2}$"#</Math>
                " is not negative, establishes that "
                <Math>r#"${\sqrt{2} \over 2} = \sqrt{0.5}$"#</Math> ".)"
              </Paragraph>
              <Paragraph margin_top=15>
                "e. (True) Using the “"
                <Math>r#"${\sqrt{x} \over \sqrt{y}} = \sqrt{\up{0.7}x \over y}$"#</Math>
                "” identity:"

              </Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                    {1 \over \sqrt{2}} = {\sqrt{1} \over \sqrt{2}} = \sqrt{\up{0.8}1 \over 2} = \sqrt{0.5}.
                $$"#
              </MathBlock>
              <Paragraph>"Or by direct verification:"</Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                    \left({1 \over \sqrt{2}}\right)^{\!2} = {1 \over \sqrt{2}}\cdot{1 \over \sqrt{2}}
                    = {1 \over \sqrt{2}\cdot\sqrt{2}} = {1 \over 2} = 0.5.
                $$"#
              </MathBlock>
              <Paragraph>
                " (And " <Math>r#"$1 \over \sqrt{2}$"#</Math>
                " is nonnegative.) Or by reducing to part d:"
              </Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                    {1 \over \sqrt{2}} = {\sqrt{2} \over \sqrt{2} \cdot \sqrt{2}} = {\sqrt{2} \over 2}.
                $$"#
              </MathBlock>
              <Paragraph>
                "(The point being: we already know that "
                <Math>r#"${\sqrt{2} \over 2} = \sqrt{0.5}$"#</Math> " by part d.)"
              </Paragraph>
              <Paragraph margin_top=15>"f. (True) We have"</Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                    2^{30} = 2^{10} \times 2^{10} \times 2^{10} = (2^{10})^3
                $$"#
              </MathBlock>
              <Paragraph>"and"</Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                (2^{10})^3 = (1024)^3 > 1000^3.
                $$"#
              </MathBlock>
              <Paragraph>
                <Span italic=true>"Note 2."</Span>
                " The first ten or so powers of "
                <Math>r#"$ 2 $"#</Math>
                " are worth knowing by heart (here's "
                <Span italic=true>"eleven"</Span>
                " powers, mind you): "
              </Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                    \begin{array}{c|c}
                    \,\,\,\,n\,\,\,\, & 2^n\dn{0.3} \\ \hline
                    0 & 1 \up{1.1}\\
                    1 & 2 \\
                    2 & 4 \\
                    3 & 8 \\
                    4 & 16 \\
                    5 & 32 \\
                    6 & 64 \\
                    7 & 128 \\
                    8 & 256 \\
                    9 & 512 \\
                    10 & 1024
                    \end{array}
                $$"#
              </MathBlock>
              <Paragraph>"Among which, the fact that"</Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                2^{10} \approx 10^3
                $$"#
              </MathBlock>
              <Paragraph>
                "can be particularly useful to know!
                For example, if a 1-millimeter-thick napkin is folded 50 times over, doubling
                the width each time, one obtains something of thickness"
              </Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                2^{50}\fw\te{mm} = (2^{10})^5\fw\te{mm} \approx (10^3)^5\fw\te{mm} = 10^{15}\fw\te{mm}.
                $$"#
              </MathBlock>
              <Paragraph>"As"</Paragraph>
              <MathBlock id="104" height=Height::Fit>
                r#"$$
                1\fw\te{mm} = 10^{-6}\fw\te{km}
                $$"#
                <ImageRight
                  src="/images/104.svg"
                  width=600
                  pos_y="-8.2rem"
                  pos_x="4rem"
                  children_y="47%"
                  children_x="34%"
                >
                  <Math>
                    r#"$
                        10^{15} \space \te{mm} = 10^{15}(10^{-6} \space \te{km})
                    $"#
                  </Math>
                </ImageRight>
              </MathBlock>
              <Paragraph>"this is"</Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                    10^{9}\fw\te{km}
                $$"#
              </MathBlock>

              <Paragraph>
                "or " <Span italic=true>"one billion"</Span>
                " kilometers. By comparison, the distance from the Earth to the Sun is a mere "
                <Math>r#"$150$"#</Math> " million kilometers."
              </Paragraph>
              <Paragraph indent=Indent::Line>
                "(The point being: that we could go from the relatively mysterious"
              </Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                \te{“}2^{50}\fw\te{mm}\te{”}
                $$"#
              </MathBlock>
              <Paragraph>"to the relatively less mysterious"</Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                \te{“}\fw10^{15}\te{mm}\te{”}
                $$"#
              </MathBlock>
              <Paragraph>
                "by the approximation  " <Math>r#"$2^{10} \approx 10^3$"#</Math> ".)"
              </Paragraph>
              <Paragraph margin_top=15>
                "g. (True) As an inequality can be multiplied on both sides
                by a positive number while preserving the inequality, one has"
              </Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                \begin{align}
                    & {1 \over 0.95} > 1.05\\
                    \iff & 1 > 1.05 \cdot 0.95\up{1.4}\\
                    \iff & 1 > (1 + 0.05)(1 - 0.05)\up{1.4}\\
                    \iff & 1 > 1 - 0.05^2\up{1.4}
                \end{align}
                $$"#
              </MathBlock>
              <Paragraph>
                "(using the fact that " <Math>r#"$(1+x)(1-x) = 1-x^2$"#</Math> ", of"
              </Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                    \te{“}\,(a+b)(a-b) = a^2-b^2\,\te{”}
                $$"#
              </MathBlock>
              <Paragraph>
                "fame), and since the " <Span italic=true>"last"</Span> " inequality is true,
                the " <Span italic=true>"first"</Span> " inequality is true! (Recall that"
                <span class="whitespace-nowrap">" “" <Math>r#"$\!\iff\!$"#</Math> "” "</span>
                "means “if and only if”.)"
              </Paragraph>
              <Paragraph margin_top=15>
                <Span italic=true>"Note 3."</Span>
                " More generally, even though "
              </Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                    {1 \over 1 - \epsilon} > 1 + \epsilon
                $$"#
              </MathBlock>
              <Paragraph>
                "for any small " <Math>r#"$ \epsilon > 0$"#</Math> ", the number "
                <Math>r#"$1 + \epsilon$"#</Math> " remains a good approximation to "
                <Math>r#"${1 \over 1 - \epsilon}$"#</Math> ". For example,"
              </Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                    %{1 \over 0.99}
                    1.01
                $$"#
              </MathBlock>
              <Paragraph>"is a good approximation to"</Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                    %1.01
                    {1 \over 0.99}
                $$"#
              </MathBlock>
              <Paragraph>"while"</Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                  1.001
                  %{1 \over 0.999}
                $$"#
              </MathBlock>
              <Paragraph>"is a good approximation to"</Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                %1.001
                \,{1 \over 0.999},
                $$"#
              </MathBlock>
              <Paragraph>"etc."</Paragraph>
              <Paragraph margin_top=15>
                "h. (True) Here are the first few powers of $-1$ (note how each additional multiplication by $-1$ simply changes the sign of the previous result):"
              </Paragraph>
              <MathBlock height=Height::Fit child_tag="span" arrow_position="13.3rem 0rem 0rem 90%">
                <div class="w-full">
                  <Grid margin_top=15 margin_bottom=15 cols=3 place_items="end" classes="w-fit">

                    <Span>
                      <Math>r#"$ (-1)^1 = $"#</Math>
                    </Span>
                    <Span>
                      <Math>r#"$ (-1) = $"#</Math>
                    </Span>
                    <Span>
                      <Math>r#"$ -1 $"#</Math>
                    </Span>

                    <Span>
                      <Math>r#"$ (-1)^2 = $"#</Math>
                    </Span>
                    <Span>
                      <Math>r#"$ (-1)\times (-1) = $"#</Math>
                    </Span>
                    <Span>
                      <Math>r#"$ 1 $"#</Math>
                    </Span>

                    <Span>
                      <Math>r#"$ (-1)^3 = $"#</Math>
                    </Span>
                    <Span>
                      <Math>r#"$ (-1)\times(-1)\times (-1) = $"#</Math>
                    </Span>
                    <Span>
                      <Math>r#"$ -1 $"#</Math>
                    </Span>

                    <Span>
                      <Math>r#"$ (-1)^4 = $"#</Math>
                    </Span>
                    <Span>
                      <Math>r#"$ (-1)\times(-1)\times(-1)\times(-1) = $"#</Math>
                    </Span>
                    <Span>
                      <Math>r#"$ 1 $"#</Math>
                    </Span>

                    <Span>
                      <Math>r#"$ (-1)^5 = $"#</Math>
                    </Span>
                    <Span>
                      <Math>r#"$ \,\,\,(-1)\times(-1)\times(-1)\times(-1)\times(-1) = $"#</Math>
                    </Span>
                    <Span>
                      <Math>r#"$ -1 $"#</Math>
                    </Span>

                  </Grid>
                </div>
              </MathBlock>
              <Paragraph margin_top=15>
                "(Etc.) Obviously, even powers of " <Math>r#"$ (-1) $"#</Math> " are equal to "
                <Math>r#"$ 1 $"#</Math> ", while odd powers of " <Math>r#"$ (-1) $"#</Math>
                " are equal to " <Math>r#"$ -1 $"#</Math> ". As " <Math>r#"$ -101 $"#</Math>
                " is odd, " <Math>r#"$ (-1)^{101} $"#</Math> " is
                " <Math>r#"$ -1 $"#</Math> "."
              </Paragraph>
              <Paragraph margin_top=15>"(False) We have"</Paragraph>
              <MathBlock height=Height::Fit>
                r#"$${100 \over 99} = {99 + 1 \over 99} = 1 + {1 \over 99}$$"#
              </MathBlock>
              <Paragraph>"and"</Paragraph>
              <MathBlock height=Height::Fit>
                r#"$${101 \over 100} = {100 + 1 \over 100} = 1 + {1 \over 100}$$"#
              </MathBlock>
              <Paragraph>
                "so the smaller of the two fractions is " <Math>r#"$ {101 \over 100} $"#</Math>
                ", since " <Math>r#"${1 \over 100} < {1 \over 99}$"#</Math> "."
              </Paragraph>
              <Paragraph margin_top=15>
                <Span italic=true>"Note 4."</Span>
                " The difference"
              </Paragraph>
              <MathBlock height=Height::Fit>r#"$${1 \over 99} - {1 \over 100}$$"#</MathBlock>
              <Paragraph>
                "is interesting in its own right, being connected to a famous infinite sum."
              </Paragraph>
              <Paragraph indent=Indent::Line>
                "To visualize this sum, picture a hare poised at " <Math>r#"$x = 0 $"#</Math>
                " on the number line. This hare runs forward by one unit and backwards by half a unit, stopping at the number"
              </Paragraph>
              <MathBlock height=Height::Fit>r#"$$1 - \frac{1}{2}$$"#</MathBlock>
              <Paragraph>
                "by virtue of this back-and-forth movement. The hare then proceeds to run forward by "
                <Span italic=true>"half"</Span> " a unit and back by a "
                <Span italic=true>"third"</Span> " of a unit, stopping at"
              </Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                \begin{align}
                &\, \left(1 - {1 \over 2}\right)\\
                + \,&\, \left({1 \over 2} - {1 \over 3}\right)_{\color{white} a_{a_a}\!\!\!\!\!\!\!\!\!\!}\\
                \hline
                = \,&\, \left(1 - {1 \over 3}\right)^{\color{white} a^{a^a}}
                \end{align}
                $$"#
              </MathBlock>
              <Paragraph>
                "for another break. Keeping with this pattern, the hare then stops at"
              </Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                \begin{align}
                    &\, \left(1 - {1 \over 2}\right)\\
                    + \,&\, \left({1 \over 2} - {1 \over 3}\right)\\
                    + \,&\, \left({1 \over 3} - {1 \over 4}\right)_{\color{white} a_{a_a}\!\!\!\!\!\!\!\!\!\!}\\
                    \hline
                    = \,&\, \left(1 - {1 \over 4}\right)^{\color{white} a^{a^a}}
                \end{align}
                $$"#
              </MathBlock>
              <Paragraph>"and then at"</Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                \begin{align}
                    &\, \left(1 - {1 \over 2}\right)\\
                    + \,&\, \left({1 \over 2} - {1 \over 3}\right)\\
                    + \,&\, \left({1 \over 3} - {1 \over 4}\right)\\
                    + \,&\, \left({1 \over 4} - {1 \over 5}\right)_{\color{white} a_{a_a}\!\!\!\!\!\!\!\!\!\!}\\
                    \hline
                    = \,&\, \left(1 - {1 \over 5}\right)^{\color{white} a^{a^a}}
                \end{align}
                $$"#
              </MathBlock>
              <Paragraph>
                "and so on. Clearly, the successive positions at which the hare stops
                are approaching the number " <Math>r#"$ 1 $"#</Math>
                " from the left, pointing to the fact that the " <Span italic=true>"infinite"</Span>
                " sum"
              </Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                \begin{align}
                    &\, \left(1 - {1 \over 2}\right)\\
                    + \,&\, \left({1 \over 2} - {1 \over 3}\right)\\
                    + \,&\, \left({1 \over 3} - {1 \over 4}\right)\\
                    + \,&\, \left({1 \over 4} - {1 \over 5}\right)\\
                    + \,&\, \left({1 \over 5} - {1 \over 6}\right)\\
                    + \,&\, \left({1 \over 6} - {1 \over 7}\right)\\
                    + \,&\, \,\,\,\,\,\,\,\,\dots\up{1.3}
                \end{align}
                $$"#
              </MathBlock>
              <Paragraph>
                "is “equal” (in some sense) to " <Math>r#"$ 1 $"#</Math>
                ". But how much, exactly, is the " <Math>r#"$ n $"#</Math> "-th term"
              </Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                {1 \over n} - {1 \over n+1}
                $$"#
              </MathBlock>
              <Paragraph>
                "of the sum? (By the way, this " <Math>r#"$ n $"#</Math> "-th term is the difference
                " <Math>r#"${1 \over 99} - {1 \over 100}$"#</Math> " for "
                <Math>r#"$ n = 99 $"#</Math>
                ", which is how we came to be reminded of this infinite sum in the first place.) Well..."
              </Paragraph>
              <MathBlock id="svg-24" height=Height::Fit>
                r#"$$
                \begin{align}
                    {1 \over n} - {1 \over n+1} &= {1 \over n}\cdot{n+1 \over n+1}\, - \,
                    {1 \over n+1}\cdot{n \over n}\up{1.5}\\
                    &= {n+1 \over n(n+1)} - {n \over n(n+1)}\up{1.5}\\
                    &= {1 \over n(n+1)}\up{1.7}
                \end{align}
                $$"#
                <ImageRight
                  src="/images/24.svg"
                  pos_y="-4.8rem"
                  children_x="30%"
                  children_y="25.5%"
                  squiggle_left="-1.2rem"
                  squiggle_top="30%"
                  hidden_in_mobile=true
                  width=497
                >
                  <MathBlock height=Height::Fit>
                    r#"$$
                    \begin{align}
                        {1 \over 99} - {1 \over 100}
                        &= {1 \over 99}\cdot{100 \over 100}\, - \,{1 \over 100}\cdot{99 \over 99}\up{1.5}\\
                        &= {100 \over 99\cdot 100} - {99 \over 99\cdot 100}\up{1.5}\\
                        &= {1 \over 99\cdot 100}\up{1.5}
                        \end{align}
                    $$"#
                  </MathBlock>
                </ImageRight>
              </MathBlock>
              <Paragraph>"...it's that much. (For example,"</Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                {1 \over 1} - {1 \over 2} = {1 \over 1 \cdot 2} = {1 \over 2}
                $$"#
              </MathBlock>
              <Paragraph>"and"</Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                {1 \over 2} - {1 \over 3} = {1 \over 2 \cdot 3} = {1 \over 6}
                $$"#
              </MathBlock>
              <Paragraph>"and so on.) So the infinite sum"</Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                \begin{align}
                    &\, \left(1 - {1 \over 2}\right)\\
                    + \,&\, \left({1 \over 2} - {1 \over 3}\right)\\
                    + \,&\, \left({1 \over 3} - {1 \over 4}\right)\\
                    + \,&\, \left({1 \over 4} - {1 \over 5}\right)\\
                    + \,&\, \left({1 \over 5} - {1 \over 6}\right)\\
                    + \,&\, \left({1 \over 6} - {1 \over 7}\right)\\
                    + \,&\, \,\,\,\,\,\,\,\,\dots\up{1.3}\dn{1}\\ \hline
                    = \,&\, 1\up{1.5}
                \end{align}
                $$"#
              </MathBlock>
              <Paragraph>"can also be written"</Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                {1 \over 1 \cdot 2} + {1 \over 2 \cdot 3} + {1 \over 3 \cdot 4} + {1 \over 4 \cdot 5} + {1 \over 5 \cdot 6} + \dots \,=\, 1
                $$"#
              </MathBlock>
              <Paragraph>"(or"</Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                {1 \over 2} + {1 \over 6} + {1 \over 12} + {1 \over 20} + {1 \over 30} + \dots \,=\, 1
                $$"#
              </MathBlock>
              <Paragraph>
                "equivalently) which is not obvious at first glance, and kind of interesting!"
              </Paragraph>
              <Paragraph margin_top=15>
                <Span italic=true>"Note 5."</Span>
                " The fact that"
              </Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                {1 \over n} - {1 \over n+1} = {1 \over n(n+1)}
                $$"#
              </MathBlock>
              <Paragraph>
                "means, in particular, that " <Math>r#"${1 \over n} - {1 \over n+1}$"#</Math>
                " is roughly " <Math>r#"${1 \over n^2}$"#</Math> " for large " <Math>r#"$n$"#</Math>
                ", which is sometimes handy to know.
                For example,"
              </Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                {1 \over 10} - {1 \over 11}
                $$"#
              </MathBlock>
              <Paragraph>"is approximately " <Math>r#"$1/10^2 = 0.01$"#</Math> ", while"</Paragraph>
              <MathBlock height=Height::Fit>
                r#"$$
                {1 \over 100} - {1 \over 101}
                $$"#
              </MathBlock>
              <Paragraph>
                "is approximately " <Math>r#"$1/100^2 = 0.01^2 = 0.0001$"#</Math> ", etc."
              </Paragraph>

            </Solution>
          </TabElement>
          <TabElement>
            <Paragraph classes="animate-appear-slow">
              <Span bold=true>"Exercise 2."</Span>
              " In the solution to Exercise 1, we observed how the difference"
            </Paragraph>
            <MathBlock>
              r#"$$
                  {1 \over n} - {1 \over n+1}
              $$"#
            </MathBlock>
            <Paragraph classes="animate-appear-slow">
              "is " <Span italic=true>"roughly "</Span> <Math>r#"$ 1/n^2 $"#</Math> " for large "
              <Math>r#"$ n $"#</Math> ". But how far off
              is this exactly? (I.e., what is the difference between "
              <Math>r#"${1 \over n} - {1 \over n+1}$"#</Math> " and "
              <Math>r#"${1 \over n^2}$"#</Math> "?) And " <Span italic=true>"roughly"</Span>
              " how much is this far-offness, for large " <Math>r#"$n$"#</Math> "?"
            </Paragraph>

            <Solution>
              <Paragraph margin_top=15>
                "Since"
                <MathBlock>
                  r#"$$
                  {1 \over n} - {1 \over n+1} = {1 \over n(n+1)}
                  $$"#
                  <ImageRight
                    src="/images/25.svg"
                    children_y="10%"
                    children_x="37%"
                    pos_x="4.5rem"
                    pos_y="-1.2rem"
                    width=362
                  >
                    <MathBlock>
                      r#"$$
                      {1 \over 99\cdot 100}
                      $$"#
                    </MathBlock>
                  </ImageRight>
                </MathBlock> "(as previously computed) is smaller than"
              </Paragraph>
              <Paragraph>
                <MathBlock>
                  r#"$$
                  {1 \over n^2}
                  $$"#
                  <ImageRight
                    src="/images/26.svg"
                    children_y="15%"
                    children_x="33%"
                    pos_x="5rem"
                    pos_y="-1.9rem"
                    width=362
                  >
                    <MathBlock>
                      r#"$$
                      {1 \over 99\cdot 100}
                      $$"#
                    </MathBlock>
                  </ImageRight>
                </MathBlock>
                "we will compute the difference"

                <MathBlock>
                  r#"$$
                  {1 \over n^2} - {1 \over n(n+1)}
                  $$"#
                </MathBlock>
                "as opposed to the “other” difference"
                <MathBlock>
                  r#"$$
                  {1 \over n(n+1)} - {1 \over n^2}
                  $$"#
                </MathBlock>
              </Paragraph>

              <Paragraph>
                "to avoid a minus sign in the result. (Computing the second difference and having a minus sign does not constitute a mistake, however.) Having said this, the difference is:"
                <MathBlock height=Height::Fit>
                  r#"$$
                  \begin{align}
                  {1 \over n^2} - {1 \over n(n+1)} &= {1 \over n^2} \cdot {n+1 \over n+1} - {1 \over n(n+1)} \cdot
                  {n \over n}\\
                  &= {n+1 \over n^2(n+1)} - {n \over n^2(n+1)}\up{1.5}\\
                  &= {1 \over n^2(n+1)}\up{1.5}
                  \end{align}
                  $$"#
                  <ImageRight
                    src="/images/27.svg"
                    children_y="40%"
                    children_x="20%"
                    pos_x="-1.6rem"
                    pos_y="-5.2rem"
                    hidden_in_mobile={true}
                    width=607

                  >
                    <MathBlock>
                      r#"$$
                      \begin{align}
                      {1 \over 99^2} - {1 \over 99\cdot 100} &=
                      {1 \over 99^2}\cdot{100 \over 100}\, - \,
                      {1 \over 99\cdot 100}\cdot{99 \over 99}\\
                      &= {100 \over 99^2 \cdot 100} - {99 \over 99^2 \cdot 100}\up{1.5}\\
                      &= {1 \over 99^2\cdot 100}\up{1.5}
                      \end{align}
                      $$"#
                    </MathBlock>
                  </ImageRight>
                </MathBlock>
                "which, for large "<Math>"$n$"</Math>", is roughly"
                <MathBlock>
                  r#"$$ {1 \over n^3} $$"#
                </MathBlock>
              </Paragraph>
              <Paragraph margin_top=15>
                "since "<Math>r#"$ n^2(n+1) \approx n^3 $"#</Math>" for large "<Math>"$n$"</Math>"."
              </Paragraph>

              <Paragraph margin_top=15>
                <Span italic={true}>"Example 1. "</Span>
                "Above, we estimated"
                <MathBlock height=Height::Fit>
                  r#"$$
                  {1 \over 10} - {1 \over 11}
                  $$"#
                </MathBlock>
                "to be roughly "<Math>"$1/100 = 0.01$"</Math>", but "<Math>"$1/100$"</Math>" is bigger than the actual value of
                "<Math>r#"${1 \over 10\cdot 11} = {1 \over 110}$"#</Math>" by"
                <MathBlock>
                  r#"$$ {1 \over 10^2\cdot 11}$$"#
                </MathBlock>
              </Paragraph>

              <Paragraph margin_top=15>
                "or close to "<Math>"$1/10^3 = 0.001$"</Math>". (So"
                <MathBlock height=Height::Fit>
                  r#"$$
                  {1 \over 10} - {1 \over 11}
                  $$"#
                </MathBlock>
                "is about "<Math>"$0.01$"</Math>", while making an error of about "<Math>"$0.001$"</Math>".)
                (In fact,"
                <MathBlock>
                  r#"$$ {1 \over 10^2\cdot 11}$$"#
                </MathBlock>
                "is"
                <Span>"less"</Span>
                "than "<Math>"$1/10^3$"</Math>", so the error is <i>less</i> than "<Math>"$0.001$"</Math>".)"
              </Paragraph>
            </Solution>
          </TabElement>
        </Tabs>

      </Columns>
    }
}
