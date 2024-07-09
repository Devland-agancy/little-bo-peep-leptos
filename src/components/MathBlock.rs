use leptos::*;

#[derive(PartialEq)]
pub enum Height {
    Fit,
    Small,
}
#[component]
pub fn MathBlock(
    cx: Scope,
    children: Children,
    #[prop(default = "")] id: &'static str,
    #[prop(default = Height::Small)] _height: Height,
    #[prop(default = 0)] margin_right: i16,
    #[prop(default = 0)] margin_left: i16,
) -> impl IntoView {
    view! { cx,
      <div
        id=id
        class="mathblock block-element text-xl col-start-2 hidden-on-startup relative h-fit"

        style=format!("margin-right: {}px", margin_right)
        style=move || {
            format!(
                "margin-left: {}px; margin-right: {}px",
                margin_left,
                margin_right,
            )
        }
      >

        {children(cx)}
      </div>
      <span style="display: block; height: 0"></span>
    }
}
