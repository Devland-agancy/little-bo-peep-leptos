use leptos::{*,  html::Div};
use web_sys::Node;

#[component]
fn LabelsView(cx: Scope, vec: Vec<&'static str>, selected_tab: ReadSignal<usize>, set_selected_tab: WriteSignal<usize>) -> impl IntoView {
    
    
    view! {cx, 
            <img  on:click=move |_|{
            if selected_tab() != 0 {
                set_selected_tab( selected_tab() - 1 )
            }}
            src="/images/left.svg"
            class="tab cursor-pointer" 
            />
            <img  on:click=move |_|{
                if selected_tab() != vec.len() - 1 {
                    set_selected_tab( selected_tab() + 1 )
                }}
                src="/images/left.svg"
                class="tab cursor-pointer rotate-180" 
            /> 
        }
}

#[component]
pub fn tabs(cx: Scope, labels: Vec<&'static str>,children: ChildrenFn) -> impl IntoView 
{
    let (selected_tab, set_selected_tab) = create_signal(cx, 0);
    
    
    view! {cx,
        <div class="text-xl flex items-center justify-center gap-2 col-start-2 hidden-on-startup mb-10">
            <LabelsView vec={labels.clone()}  selected_tab={selected_tab} set_selected_tab={set_selected_tab} />
        </div>
        <For
            each=move || children(cx)
                .nodes
                .into_iter()
                .enumerate()
            /*     .filter(move |f| f.0 == selected_tab()) */
            key=|label| label.0
            view=move |cx, label| {
                view! {cx,
                     <div class="col-start-2 relative transition-opacity duration-500"
                        class=("opacity-0", move || selected_tab() != label.0)
                        class=("h-0", move || selected_tab() != label.0)
                        class=("transition-none", move || selected_tab() != label.0)> {label.1} 
                     </div> 
                    }
            }
        />
        <div class="text-xl flex items-center justify-center gap-2 col-start-2 hidden-on-startup">
            <LabelsView vec={labels}  selected_tab={selected_tab} set_selected_tab={set_selected_tab} />
        </div>
    }
}

#[component]
pub fn TabElement(cx: Scope, children: ChildrenFn) -> impl IntoView {
        view! {cx, {children(cx)} }
}