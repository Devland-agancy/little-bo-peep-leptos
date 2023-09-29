use leptos::{html::{Div}, *};

#[component]
pub fn Solution(cx: Scope, children: Children) -> impl IntoView {
    let (visible, set_visible) = create_signal(cx, false);
    let (content_height, set_content_height) = create_signal(cx, 0);
    let node_ref = create_node_ref::<Div>(cx);
    create_effect(cx, move |_|{
        if node_ref().is_some() {
            if visible() {
                set_content_height(node_ref().unwrap().offset_height())
            }else{
                set_content_height(0)
            }
        }
    });

    view! {cx,
        <div
            class="px-4 my-10 relative col-start-2"
        >
            <img 
                class="w-2/3 mx-auto cursor-pointer" 
                src="/images/solution.png"  
                on:click=move |_| {
                    set_visible(!visible());
                }
            />
        </div>
        <div
            class="col-start-2 px-4 transition-[height] duration-1000 overflow-y-clip relative"
            class=("pointer-events-none", move || !visible())
            class=("animated-height-full", move || visible())
            style=move || format!("height: {}px", content_height() + if visible() { 40 } else { 0 })
        >
            <div  
                node_ref=node_ref
                class="transition-all duration-1000"
                class=("-translate-y-full", move || !visible()) 
            >
                {children(cx)}
            </div>

        </div>

    }
}