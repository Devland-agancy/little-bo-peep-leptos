use crate::constants::{MOBILE_MAX_WIDTH, TEXT_LEFT_PADDING, TEXT_RIGHT_PADDING};
use leptos::*;

#[component]
pub fn Table(
    cx: Scope,
    children: Children,
    #[prop(default = vec![])] cols: Vec<u16>,
    #[prop(default = vec![])] sm_cols: Vec<u16>,
    #[prop(default = 0)] margin_top: i32,
    #[prop(optional)] classes: &'static str,
    #[prop(default = "")] style: &'static str,
    #[prop(default = false)] lines: bool,
) -> impl IntoView {
    let (_cols, set_cols) = create_signal(cx, cols);

    create_effect(cx, move |_| {
        if window().inner_width().unwrap().as_f64().unwrap() <= MOBILE_MAX_WIDTH as f64
            && sm_cols.len() > 0
        {
            set_cols(sm_cols.clone())
        }
    });

    view! { cx,
      <div
        class=format!(
            "col-start-2 min-h-fit my-4 w-fit relative left-1/2 -translate-x-1/2 {}",
            classes,
        )
        style=format!("padding-left: {}; padding-right: {}", TEXT_LEFT_PADDING, TEXT_RIGHT_PADDING)
      >
        <table
          class="table-fixed max-w-full w-full"
          class=("lines", move || lines)
          style=move || format!("margin-top: {}px ;{}", margin_top, style)
        >
          <colgroup>
            {_cols()
                .into_iter()
                .map(|w| {
                    view! { cx,
                      <col style=move || format!("min-width:{}px;width:{}px", w, w) width=w/>
                    }
                })
                .collect_view(cx)}
          </colgroup>
          {children(cx)}
        </table>
      </div>
    }
}
