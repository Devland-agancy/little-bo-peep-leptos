use leptos::{html::{ Div}, ev::{resize}, *};
use leptos_use::use_event_listener;
use crate::page::state::PageState;
use rand::Rng;

#[derive(PartialEq)]
pub enum Height {
    Fit,
    Small,
}
#[component]
pub fn MathBlock(
    cx: Scope,
    children: Children,
    #[prop(default = "")] id: &'static str,
    #[prop(default = Height::Small)] height: Height,
    #[prop(default = 12)] margin_right: i16,
    #[prop(default = 12)] margin_left: i16,
    #[prop(default = "-0.25rem 0.2rem auto auto")] arrow_position: &'static str,
    #[prop(default = false)] arrow_hidden: bool,

) -> impl IntoView {
    let node_ref = create_node_ref::<Div>(cx);
    let (is_wide, set_is_wide) = create_signal(cx, false);
    let set_page_state =
    use_context::<WriteSignal<PageState>>(cx).unwrap();
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let show_right = move || page_state() == PageState::ShowRight;
    let set_right_image_x_pos = use_context::<WriteSignal<f64>>(cx).unwrap();
    
    let (margin_left_active, set_margin_left_active) = create_signal(cx, true);

    let dots = vec!["dot", "dot 1", "dot 2", "dot 3", "dot 4", "dot 5"];
    let mut rng = rand::thread_rng();
    let randomDot = dots[rng.gen_range(0..=5)];

     create_effect(cx, move |_|{
        if node_ref().is_some() {
            let math_box = node_ref().unwrap().get_elements_by_tag_name("mjx-math").item(0);
            if math_box.is_some(){

                let math_box_width = math_box.unwrap().client_width() as f64;
                let window_width = window().inner_width().unwrap().as_f64().unwrap();
                if math_box_width + margin_left as f64 - 2_f64 > window_width {
                    request_animation_frame(move || {
                        set_margin_left_active(false);
                        log!("falseeee");
                        if math_box_width - 2_f64  > window_width{
                            set_is_wide(true)
                        }
                    });
                }
            }
        }
    }); 
    create_effect(cx, move |_|{
        let _ = use_event_listener(cx, window(), resize, move |_| {
            if node_ref().is_some() {
                let math_box_width = node_ref().unwrap().get_elements_by_tag_name("mjx-math").item(0).unwrap().client_width() as f64;
                let window_width = window().inner_width().unwrap().as_f64().unwrap();
                if math_box_width + margin_left as f64 - 2_f64 > window_width {
                    set_margin_left_active(false);
                    if math_box_width - 2_f64  > window_width{
                        set_is_wide(true)
                    }
                }/* else if math_box_width >= window_width {
                    set_margin_left_active(true);
                    set_is_wide(false);
                } */
                else{
                    set_is_wide(false);
                    set_margin_left_active(true);
                }
            }
        });
    });
    view! {cx,
        <div
            node_ref=node_ref
            id=id
            class="mathblock text-xl flex items-center justify-center col-start-2 hidden-on-startup relative"
            class=("h-20", height == Height::Small)
            class=("h-fit", height == Height::Fit)

            style=format!("margin-right: {}px", margin_right)
            style=move || format!("margin-left: {}px; margin-right: {}px", if margin_left_active() {margin_left}else{0}, if margin_left_active() {margin_right}else{0})

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
            <img src="/images/cream.svg" class="ml-auto h-3"/>
        </div>
        </div>
    }
}