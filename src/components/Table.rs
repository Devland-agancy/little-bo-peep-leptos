use crate::constants::{MOBILE_MAX_WIDTH, TEXT_LEFT_PADDING, TEXT_RIGHT_PADDING};
use leptos::{html::Table, *};

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
    let (scaled_down, set_scaled_down) = create_signal(cx, true);

    create_effect(cx, move |_| {
        if window().inner_width().unwrap().as_f64().unwrap() <= MOBILE_MAX_WIDTH as f64
            && sm_cols.len() > 0
        {
            set_cols(sm_cols.clone())
        }
    });
    let table = create_node_ref::<Table>(cx);

    let calc_scale = move || {
        if let Some(table) = table() {
            let table_width = table.offset_width() as f64;
            let screen_width = window().inner_width().unwrap().as_f64().unwrap();
            if screen_width < table_width && scaled_down() {
                screen_width / (table_width + 32.0)
            } else {
                1.0
            }
        } else {
            1.0
        }
    };

    view! { cx,
      <div
        class=format!(
            "col-start-2 min-h-fit my-4 w-fit relative left-1/2 -translate-x-1/2 {}",
            classes,
        )
        style=format!("padding-left: {}; padding-right: {}", TEXT_LEFT_PADDING, TEXT_RIGHT_PADDING)
      >
        <table
          node_ref=table
          class="table-fixed max-w-full w-full transition-image-scale"
          class=("lines", move || lines)
          style=move || format!("margin-top: {}px ; transform: scale({}); {}", margin_top, calc_scale()  ,style)
          on:click=move |_|{
            set_scaled_down(!scaled_down())
          }
        >
          <colgroup>
            {_cols()
                .into_iter()
                .map(|w| {
                    view! { cx,
                      <col style=move || format!("min-width:{}px;width:{}px", w, w)
                      />
                    }
                })
                .collect_view(cx)}
          </colgroup>
          {children(cx)}
        </table>
      </div>
    }
}
