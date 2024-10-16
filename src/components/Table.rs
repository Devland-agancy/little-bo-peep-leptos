use crate::constants::{MOBILE_SCREEN_MAX_WIDTH, TEXT_X_PADDING};
use leptos::{html::Table, *};

#[component]
pub fn Table(
    children: Children,
    #[prop(default = vec![])] cols: Vec<u16>,
    #[prop(default = vec![])] sm_cols: Vec<u16>,
    #[prop(optional)] classes: &'static str,
    #[prop(default = "")] style: &'static str,
    #[prop(default = false)] lines: bool,
) -> impl IntoView {
    let (_cols, set_cols) = create_signal(cols);
    let (scaled_down, set_scaled_down) = create_signal(true);

    create_effect(move |_| {
        if window().inner_width().unwrap().as_f64().unwrap() <= MOBILE_SCREEN_MAX_WIDTH as f64
            && sm_cols.len() > 0
        {
            set_cols.set(sm_cols.clone())
        }
    });
    let table = create_node_ref::<Table>();

    let calc_scale = move || {
        if let Some(table) = table.get() {
            let table_width = table.offset_width() as f64;
            let screen_width = window().inner_width().unwrap().as_f64().unwrap();
            if screen_width < table_width && scaled_down.get() {
                screen_width / (table_width + 32.0)
            } else {
                1.0
            }
        } else {
            1.0
        }
    };

    let calc_margin = move || {
        if let Some(table) = table.get() {
            let table_width = table.offset_width() as f64;
            let screen_width = window().inner_width().unwrap().as_f64().unwrap();
            if screen_width < table_width && scaled_down.get() {
                -((1.0 - (screen_width / (table_width + 32.0))) / 2.0) * 100.0
            } else {
                0.0
            }
        } else {
            0.0
        }
    };

    view! {
      <div
        class=format!(
            "col-start-2 min-h-fit w-fit relative left-1/2 -translate-x-1/2 {}",
            classes,
        )
        style=format!("padding-inline: {}", TEXT_X_PADDING)
      >
        <table
          node_ref=table
          class="table-fixed max-w-full w-full transition-image-scale"
          class=("lines", move || lines)
          style=move || format!("transform: scale({}); margin-block: {}%;{}", calc_scale(), calc_margin() ,style)
          on:click=move |_|{
            set_scaled_down.set(!scaled_down.get())
          }
        >
          <colgroup>
            {_cols.get_untracked()
                .into_iter()
                .map(|w| {
                    view! {
                      <col style=move || format!("min-width:{}px;width:{}px", w, w)
                      />
                    }
                })
                .collect_view()}
          </colgroup>
          {children()}
        </table>
      </div>
    }
}
