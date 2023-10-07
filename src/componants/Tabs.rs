use leptos::{*,  html::Div};
use web_sys::Node;

fn render_labels(cx: Scope, vec: &Vec<&'static str>, selected_tab: ReadSignal<usize>, set_selected_tab: WriteSignal<usize>) -> leptos::View {
    let res = vec.into_iter()
    .enumerate()
    .map(|(i, el)| {
        view! {cx,  <div  on:click=move |_|
                set_selected_tab(i)
          class="tab cursor-pointer px-4 py-2 rounded border-2 border-black"
          class=("active", move || selected_tab() == i ) >{*el}</div>
        }
    })
    .collect_view(cx);
    res
}

#[component]
pub fn tabs(cx: Scope, labels: Vec<&'static str>,children: ChildrenFn) -> impl IntoView 
{
    let (selected_tab, set_selected_tab) = create_signal(cx, 0);
    
    let lables = render_labels(cx, &labels, selected_tab, set_selected_tab);
    let labeles_bottom = render_labels(cx, &labels, selected_tab, set_selected_tab);
   
    
    view! {cx,
        <div class="text-xl flex items-center justify-center gap-2 col-start-2 hidden-on-startup mb-10">
            {lables} 
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
                     <div class="col-start-2 transition-[height] duration-1000 overflow-y-clip      relative"
                        class=("hidden", move || selected_tab() != label.0)> {label.1} 
                     </div> 
                    }
            }
        />
        <div class="text-xl flex items-center justify-center gap-2 col-start-2 hidden-on-startup">
            {labeles_bottom} 
        </div>
    }
}

#[component]
pub fn TabElement(cx: Scope, children: ChildrenFn) -> impl IntoView {
    /* let node_ref = create_node_ref::<Div>(cx);
    let (is_selected, set_is_selected) = create_signal(cx, false);
   
    create_effect(cx, move |_|{
            if node_ref().is_some() {
                let parent = node_ref().unwrap().parent_element();
                 if parent.is_some() {
                    let parent_class = parent.unwrap().class_list();
                    log!("b: {}", parent_class.contains("selected"));
                    set_is_selected(parent_class.contains("selected"))
                }
            }
        }); */
        view! {cx, {children(cx)} }
    /* view! {cx,
        <div
            on:click=move |_| {
                if node_ref().is_some() {
                    let parent = node_ref().unwrap().parent_element();
                    if parent.is_some() {
                        let parent_class = parent.unwrap().class_list();
                        log!("b: {}", parent_class.contains("selected"));
                        set_is_selected(parent_class.contains("selected"))
                    }
                }
            }
            node_ref=node_ref
            class="cursor-pointer hover:font-bold"
        >
            {label}
        </div>
       <Show
            when=is_selected
            fallback=|_| ()
        >
            {children(cx)}
        </Show> 
    } */
}