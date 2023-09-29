use leptos::*;

#[component]
pub fn Grid(
    cx: Scope,
    children: Children,
    #[prop(default = 0)] margin_top: i16,
    #[prop(optional)] id: &'static str,
    #[prop(default = 0)] cols: i16,
    #[prop(default = -1)] sm_cols: i16,

) -> impl IntoView {

    let (_cols, set_cols) = create_signal(cx, cols);

    create_effect(cx, move |_|{
        if window().inner_width().unwrap().as_f64().unwrap() <= 640_f64 && sm_cols > -1 {
            set_cols(sm_cols)
        }
    });

    view! {cx,
        <span
            id=id
            class="col-start-2 px-4 grid flex-wrap min-h-fit gap-4 place-items-center"
            style=move || format!("grid-template-columns: repeat({}, 1fr) ;margin-top: {}px", _cols(), margin_top)
        >
            {children(cx)}
        </span>
    }
}