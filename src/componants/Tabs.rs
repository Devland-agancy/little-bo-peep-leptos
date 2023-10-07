use leptos::{*,  html::Div};
use web_sys::Node;
#[component]
pub fn tabs(cx: Scope, labels: Vec<&'static str>,children: ChildrenFn) -> impl IntoView 
{
    let (selected_tab, set_selected_tab) = create_signal(cx, 0);
    
    let lables = labels
                            .into_iter()
                            .enumerate()
                            .map(|(i, el)| {
                                view! {cx,  <div  on:click=move |_|{
                                        set_selected_tab(i);
                                        let maths = document().get_elements_by_class_name("hidden-on-startup");
                                        /*for i in 0..maths.length() {
                                            let _ = maths.item(i).unwrap().class_list().remove_1("hidden-on-startup");
                                            let _ = maths.item(i).unwrap().class_list().add_1("animate-appear");
                                             let load_math = r#"MathJax.Hub.Queue(["Typeset",MathJax.Hub]);"#;
                                            let mut script = document().create_element("script");
                                            script.as_mut().unwrap().set_inner_html(load_math);
                                            document().head().unwrap().append_child(&script.unwrap() as &Node);
                                        } */
                                       
                                    }
                                  class="math_reloader cursor-pointer hover:font-bold"
                                  class=("selected", move || selected_tab() == i ) >{el}</div>
                                }
                            })
                            .collect_view(cx);

   
    
    view! {cx,
        <div
            class="text-xl flex items-center justify-center gap-10 col-start-2 hidden-on-startup"
        >
            {lables} 
        </div>
        <For
            each=move || children(cx)
                .nodes
                .into_iter()
                .enumerate()
                .filter(move |f| f.0 == selected_tab())
            key=|label| label.0
            view=move |cx, label| {
                view! {cx, {label.1} }
            }
        />
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