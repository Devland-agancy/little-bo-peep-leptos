use leptos::{html::{ Div}, ev::{resize}, *};
use leptos_use::use_event_listener;
use crate::page::state::PageState;
use web_sys::{ UiEvent};

#[derive(PartialEq)]
pub enum Height {
    Fit,
    Small,
}
#[component]
pub fn MathBlock(
    cx: Scope,
    children: Children,
    #[prop(default = Height::Small)] height: Height,
    #[prop(default = 0)] margin_right: i16,
    #[prop(default = 12)] margin_left: i16,
    #[prop(default = "-0.25rem 0 auto auto")] arrow_position: &'static str,
    #[prop(default = false)] arrow_hidden: bool,

) -> impl IntoView {
    let node_ref = create_node_ref::<Div>(cx);
    let (is_wide, set_is_wide) = create_signal(cx, false);
    let set_page_state =
    use_context::<WriteSignal<PageState>>(cx).unwrap();
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let show_right = move || page_state() == PageState::ShowRight;
    let set_right_image_x_pos = use_context::<WriteSignal<f64>>(cx).unwrap();

     create_effect(cx, move |_|{
        if node_ref().is_some() {
            let math_box = node_ref().unwrap().get_elements_by_tag_name("mjx-math").item(0);
            if math_box.is_some(){

                let math_box_width = math_box.unwrap().client_width() as f64;
                let window_width = window().inner_width().unwrap().as_f64().unwrap();
                if math_box_width + 24_f64 > window_width {
                    request_animation_frame(move || set_is_wide(true) )  ;
                }
            }
        }
    }); 
    create_effect(cx, move |_|{
        let cleanup = use_event_listener(cx, window(), resize, move |evt: UiEvent| {
            if node_ref().is_some() {
                let math_box_width = node_ref().unwrap().get_elements_by_tag_name("mjx-math").item(0).unwrap().client_width() as f64;
                let window_width = window().inner_width().unwrap().as_f64().unwrap();
                if math_box_width + 24_f64 > window_width {
                    set_is_wide(true);
                }else{
                    set_is_wide(false);
                }
            }
        });
    });
    view! {cx,
        <div
            node_ref=node_ref
            class="mathblock text-xl flex items-center justify-center col-start-2 hidden-on-startup relative"
            class=("h-20", height == Height::Small)
            class=("h-fit", height == Height::Fit)

            style=format!("margin-right: {}px", margin_right)
            style=format!("margin-left: {}px", margin_left)

        >
            {children(cx)}
            <div
                on:click=move |e| {
                    e.stop_propagation();
                    if page_state() == PageState::ShowArticle {
                        set_page_state.update(|value| *value = PageState::ShowRight);
                        set_right_image_x_pos(80_f64);
                    }
                } 
                class="block cursor-pointer absolute h-full w-10"
                class=("hidden", move || !is_wide() | arrow_hidden)
                style=move || format!("inset: {}", arrow_position)
             >
            <img src="/images/cartoon_arrow.svg" class="ml-auto"/>
        </div>
        </div>
    }
}