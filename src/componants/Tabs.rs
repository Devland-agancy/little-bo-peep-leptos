use leptos::{*,  html::Div, html::Svg};
use web_sys::Node;

#[component]
fn LabelsView(cx: Scope, vec: Vec<&'static str>, selected_tab: ReadSignal<usize>, set_selected_tab: WriteSignal<usize>) -> impl IntoView {
    
    let (_vec, set_vec) = create_signal(cx, vec);
    view! {cx, 
            <svg width="43" height="43" viewBox="0 0 43 43" fill="none" xmlns="http://www.w3.org/2000/svg" class="tab cursor-pointer" 
                on:click=move |_|{
                    if selected_tab() != 0 {
                        set_selected_tab( selected_tab() - 1 )
                    }}
            >
                <path class="overflow-visible" d="M35.4941 1H6.65545C3.53203 1 1 3.53203 1 6.65545V35.4941C1 38.6175 3.53203 41.1495 6.65545 41.1495H35.4941C38.6175 41.1495 41.1495 38.6175 41.1495 35.4941V6.65545C41.1495 3.53203 38.6175 1 35.4941 1Z" fill=move || format!("{}", if selected_tab() != 0 {"#EEFFAA"} else { "#888888" }) fill-opacity="0.4" stroke="black" stroke-width="3" stroke-miterlimit="3"/>
                <path d="M8 21L18 26.7735V15.2265L8 21ZM17 22H34V20H17V22Z" fill="black"/>
            </svg>
            <svg width="43" height="43" viewBox="0 0 43 43" fill="none" xmlns="http://www.w3.org/2000/svg" class="tab cursor-pointer rotate-180" 
            on:click=move |_|{
                if selected_tab() != _vec().len() - 1 {
                    set_selected_tab( selected_tab() + 1 )
                }}
            >
                <path class="overflow-visible" d="M35.4941 1H6.65545C3.53203 1 1 3.53203 1 6.65545V35.4941C1 38.6175 3.53203 41.1495 6.65545 41.1495H35.4941C38.6175 41.1495 41.1495 38.6175 41.1495 35.4941V6.65545C41.1495 3.53203 38.6175 1 35.4941 1Z" fill=move || format!("{}", if selected_tab() !=  _vec().len() - 1 {"#EEFFAA"} else { "#888888" }) fill-opacity="0.4" stroke="black" stroke-width="3" stroke-miterlimit="3"/>
                <path d="M8 21L18 26.7735V15.2265L8 21ZM17 22H34V20H17V22Z" fill="black"/>
            </svg>
           
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