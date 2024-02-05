use leptos::*;

use crate::constants::MOBILE_BREAKPOINT;

#[component]
pub fn Table(
    cx: Scope,
    children: Children,
    #[prop(optional)] id: &'static str,
    #[prop(default = vec![])] cols: Vec<u16>,
    #[prop(default = vec![])] sm_cols: Vec<u16>,
    #[prop(default = 0)] margin_top: i32,
    #[prop(optional)] classes: &'static str,
    #[prop(default = "")] style: &'static str,
    #[prop(default = false)] lines: bool,
) -> impl IntoView {
    let (_cols, set_cols) = create_signal(cx, cols);

    create_effect(cx, move |_| {
        if window().inner_width().unwrap().as_f64().unwrap() <= MOBILE_BREAKPOINT as f64
            && sm_cols.len() > 0
        {
            set_cols(sm_cols.clone())
        }
    });

    view! { cx,
      <div class=format!(
          "col-start-2 px-4 min-h-fit my-4 w-full overflow-scroll scrollbar-hidden {}",
          classes,
      )>
        <table
          class="table-fixed"
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
