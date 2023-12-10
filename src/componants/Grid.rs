use leptos::*;

#[component]
pub fn Grid(
    cx: Scope,
    children: Children,
    #[prop(default = 0)] margin_top: i16,
    #[prop(default = 0)] margin_bottom: i16,
    #[prop(optional)] id: &'static str,
    #[prop(default = 0)] cols: i16,
    #[prop(default = -1)] sm_cols: i16,
    #[prop(optional)] classes: &'static str,
    #[prop(default = "center")] place_items: &'static str,
    #[prop(default = "1rem")] gap: &'static str,
    #[prop(default = false)] inner_borders: bool,
) -> impl IntoView {
    let (_cols, set_cols) = create_signal(cx, cols);

    create_effect(cx, move |_| {
        if window().inner_width().unwrap().as_f64().unwrap() <= 520_f64 && sm_cols > -1 {
            set_cols(sm_cols)
        }
    });

    view! { cx,
      <span
        id=id
        class=format!("col-start-2 px-4 grid flex-wrap min-h-fit {} {}", if inner_borders { "grid-inner-borders"} else { "" } ,classes)
        style=move || {
            format!(
                "grid-template-columns: repeat({}, 1fr) ;margin-top: {}px;margin-bottom: {}px; animation: appear 2s ease 0s 1 normal forwards;place-items: {}; gap: {}",
                _cols(),
                margin_top,
                margin_bottom,
                place_items,
                gap,
            )
        }
      >

        {children(cx)}
      </span>
    }
}
