pub mod article;
pub mod home;
pub mod state;
use leptos::{ev::{click, touchend, EventDescriptor, touchstart, resize}, html::{Img, Div}, *};
use leptos_use::use_event_listener;
use state::PageState;
use std::time::Duration;

use web_sys::{ScrollBehavior, ScrollToOptions, Touch, Event, UiEvent};

#[component]
pub fn Article(cx: Scope, children: Children) -> impl IntoView {
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let set_page_state = use_context::<WriteSignal<PageState>>(cx).unwrap();
    let show_right = move || page_state() == PageState::ShowRight;
    let show_left = move || page_state() == PageState::ShowLeft;
    let show_article = move || page_state() == PageState::ShowArticle;
    let right_image_x_pos = use_context::<ReadSignal<f64>>(cx).unwrap();
    let article_node = create_node_ref::<Div>(cx);

    create_effect(cx, move |_| {
        let mut options = ScrollToOptions::new();
        options.behavior(ScrollBehavior::Smooth);
        if show_right() {
            options.left(right_image_x_pos());
            window().scroll_with_scroll_to_options(&options);
            set_timeout(move || {
                let _ = article_node().unwrap().style("transition", "none");
                let _ = article_node().unwrap().style("transform", "translateX(100%)");

                options.left(right_image_x_pos() + window().inner_width().unwrap().as_f64().unwrap());
                options.behavior(ScrollBehavior::Instant);
                window().scroll_with_scroll_to_options(&options);
            }, Duration::from_millis(800)); 
        } else {
            options.left(0_f64);
            window().scroll_with_scroll_to_options(&options);
          
        };
    });

    create_effect(cx, move |_| {
        let article_mode = move || -> () {
            log!("clicked");
            if show_right() {
                let mut options = ScrollToOptions::new();
                /* options.behavior(ScrollBehavior::Smooth);
                options.left(right_image_x_pos());

                set_timeout( || { */
                    if window().inner_width().unwrap().as_f64().unwrap() > window().scroll_x().unwrap() {
                        let _ = article_node().unwrap().style("transition",  "all 0.3s ease 0.1s"); 
                        options.left(0_f64);

                    }else{
                        let _ = article_node().unwrap().style("transition", "none"); 
                        options.left( window().scroll_x().unwrap() - window().inner_width().unwrap().as_f64().unwrap());

                    }
                    options.behavior(ScrollBehavior::Instant);
                    window().scroll_with_scroll_to_options(&options);
                    let _ = article_node().unwrap().style("transform", "none");
                    set_timeout(move || {
                        options.behavior(ScrollBehavior::Smooth);
                        options.left(0_f64);
                        window().scroll_with_scroll_to_options(&options);
                        set_timeout(
                            move || set_page_state.update(|value| *value = PageState::ShowArticle),
                            Duration::from_secs(1),
                        )
                    }, Duration::from_millis(100));
               /*  }, Duration::from_millis(100)); */

            } else if show_left() {
                set_page_state.update(|value| *value = PageState::ShowArticle)
            }
        };
        let _ = use_event_listener(cx, window(), click, move |_|article_mode());
    });
    // for right_images we autoscroll to their position
    view! { cx,
        <div
         class="pt-14 xl:pt-20 overscroll-none ">
            <div
                node_ref=article_node
                class="absolute flex justify-center align-center w-full pb-14 min-h-screen"
                class=("overflow-hidden", show_article)
                id="Article"
            >
                <div
                    class="w-full transition duration-300 sm:overflow-visible sm:translate-x-0"
                    // for left image we transle based on image width
                    style=move || {
                        if show_left() {
                            format!("transform: translateX(100%)")
                        }  else { "".to_string() }
                    }
                >
                    <div class="font-baskerville w-full">
                        {children(cx)}
                    </div>
                </div>
                <ColumnButton />
            </div>
        </div>
        <MathJaxTypeset/>
    }
}

#[derive(PartialEq)]
pub enum Indent {
    None,
    Line,
    Block,
}

#[derive(PartialEq)]
pub enum Align {
    None,
    Center,
    Right,
}

#[component]
pub fn Paragraph(
    cx: Scope,
    children: Children,
    #[prop(default = Indent::None)] indent: Indent,
    #[prop(default = Align::None)] align: Align,
    #[prop(default = 0)] margin_top: i16,
) -> impl IntoView {
    view! {cx,
        <span
            class="col-start-2 px-4"
            class=("indent-10", indent == Indent::Line)
            class=("pl-10", indent == Indent::Block)
            class=("text-center", align == Align::Center)
            class=("text-right", align == Align::Right)
            class=("text-left", align == Align::None)
            style=format!("margin-top: {}px", margin_top)

        >
            {children(cx)}
        </span>
    }
}

#[component]
fn Span(
    cx: Scope,
    #[prop(default = false)] bold: bool,
    #[prop(default = false)] italic: bool,
    children: Children,
) -> impl IntoView {
    view! {cx,
        <span class=("font-baskerville-italic", italic)
              class=("font-baskerville-bold", bold)
        >{children(cx)}</span>
    }
}

#[component]
pub fn Columns(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="relative text-xl sm:leading-relaxed -translate-x-[1000px] sm:translate-x-0 grid grid-cols-[1000px_100%_1000px] sm:grid sm:grid-cols-[1fr_30.5rem_1fr]">
            {children(cx)}
        </div>
    }
}

#[component]
fn ColumnButton(cx: Scope) -> impl IntoView {
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let set_page_state = use_context::<WriteSignal<PageState>>(cx).unwrap();

    let show_right = move || page_state() == PageState::ShowRight;
    let show_left = move || page_state() == PageState::ShowLeft;
    let show_article = move || page_state() == PageState::ShowArticle;

    view! {cx,
        <div

            class="z-40 transition duration-300 lg:hidden absolute grid grid-cols-4 justify-end items-center w-full h-full lg:translate-0"
            style="-webkit-tap-highlight-color: transparent;"
            class=("opacity-0", show_article)
            class=("pointer-events-none", show_article)
            class=("opacity-100", show_right)
            class=("-translate-x-3/4", show_right)
            class=("lg:-translate-x-[85%]", show_right)
            class=("opacity-100", show_left)
            class=("translate-x-3/4", show_left)
            class=("lg:translate-x-[85%]", show_left)
        >
        </div>
    }
}

#[component]
fn ArticleTitle(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="sm:grid sm:grid-cols-[1fr_30.5rem_1fr]">
            <h1 class="sm:col-start-2 text-3xl sm:text-4xl p-4">
                {children(cx)}
            </h1>
        </div>
    }
}

#[component]
fn MathJaxTypeset(cx: Scope) -> impl IntoView {
    view! { cx,
        <script>
            window.MathJax.typesetPromise().then(() => {
                document.querySelectorAll(".hidden-on-startup").forEach((elem) => {
                  elem.classList.remove("hidden-on-startup");
                  elem.classList.add("animate-appear");
                });
            });
        </script>
    }
}

#[component]
fn Math(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <span class="w-fit h-0 inline-flex items-baseline indent-0 hidden-on-startup">
            {children(cx)}
        </span>
    }
}

#[derive(PartialEq)]
pub enum Height {
    Fit,
    Small,
}

#[component]
fn MathBlock(
    cx: Scope,
    children: Children,
    #[prop(default = Height::Small)] height: Height,
    #[prop(default = 0)] margin_right: i16,
    #[prop(default = 0)] margin_left: i16,
) -> impl IntoView {
    let node_ref = create_node_ref::<Div>(cx);
    let (is_wide, set_is_wide) = create_signal(cx, false);
    let set_page_state =
    use_context::<WriteSignal<PageState>>(cx).unwrap();
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let show_right = move || page_state() == PageState::ShowRight;

     create_effect(cx, move |_|{
        if node_ref().is_some() {
            let math_box_width = node_ref().unwrap().get_elements_by_tag_name("mjx-math").item(0).unwrap().client_width() as f64;
            let window_width = window().inner_width().unwrap().as_f64().unwrap();
            if math_box_width > window_width {
               request_animation_frame(move || set_is_wide(true) )  ;
            }
        }
    }); 
    create_effect(cx, move |_|{
        let cleanup = use_event_listener(cx, window(), resize, move |evt: UiEvent| {
            if node_ref().is_some() {
                let math_box_width = node_ref().unwrap().get_elements_by_tag_name("mjx-math").item(0).unwrap().client_width() as f64;
                let window_width = window().inner_width().unwrap().as_f64().unwrap();
                if math_box_width > window_width {
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
            class="relative indent-0 text-xl flex items-center justify-center col-start-2 hidden-on-startup"
            class=("h-20", height == Height::Small)
            class=("h-fit", height == Height::Fit)
            style=format!("margin-right: {}px", margin_right)
            style=format!("margin-left: {}px", margin_left)

        >
            {children(cx)}
            <div
                on:click=move |e| {
                    e.stop_propagation();
                    set_page_state.update(|value| *value = PageState::ShowRight);
                } 
                class="block cursor-pointer absolute h-full right-0 top-0 w-4 transition-opacity"
                class=("pointer-events-none", move || page_state() == PageState::ShowRight)
                style=move || format!("opacity: {}", if is_wide() && page_state() == PageState::ShowArticle { "1" } else { "0" } )
            >
                <img src="/images/construction.png" class="h-full w-full" />
            </div>
        </div>
    }
}

#[component]
fn ImageRight(cx: Scope, translate: &'static str, src: &'static str) -> impl IntoView {
    let set_page_state =
        use_context::<WriteSignal<PageState>>(cx).expect("set_page_state context to exist");
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let show_right = move || page_state() == PageState::ShowRight;

    let set_right_image_x_pos = use_context::<WriteSignal<f64>>(cx).unwrap();
    let image_ref = create_node_ref::<Img>(cx);
    view! {cx,
        <div class="col-start-3 h-0 flex items-center justify-start">
            <button
                on:click=move |e| {
                    e.stop_propagation();
                    set_page_state.update(|value| *value = match value {
                        PageState::ShowArticle => PageState::ShowRight,
                        _ => PageState::ShowArticle
                    });
                    set_right_image_x_pos.update(|val| *val = f64::from(image_ref().unwrap().get_bounding_client_rect().left() - 50_f64))
                }
                style=move || format!("transform: translate{}", translate)
                class="flex shrink-0 transition-opacity duration-300 lg:transition-none lg:opacity-100 lg:pointer-events-none z-10"
                class=("pointer-events-none", show_right)
            >
                <img src=src node_ref=image_ref />
            </button>
        </div>
    }
}

#[component]
fn ImageLeft(
    cx: Scope,
    translate: &'static str,
    src: &'static str,
    #[prop(default = false)] hidden_in_mobile: bool,
) -> impl IntoView {
    let set_page_state =
        use_context::<WriteSignal<PageState>>(cx).expect("set_page_state context to exist");
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();
    let show_left = move || page_state() == PageState::ShowLeft;

    view! {cx,
        <div class="col-start-1 h-0 flex items-center justify-end relative">
            <button
                on:click=move |e| {
                    e.stop_propagation();
                    set_page_state.update(|value| *value = match value {
                    PageState::ShowArticle => PageState::ShowLeft,
                    _ => PageState::ShowArticle
                    });
                }
                style=move || format!("transform: translate{}", translate)
                class="flex shrink-0 transition-opacity duration-300 lg:transition-none lg:opacity-100 lg:pointer-events-none z-10"
                class=("pointer-events-none", show_left)
            >
                <img src=src />
                <Show fallback=|_| () when=move || hidden_in_mobile >
                    <div class="block sm:hidden absolute right-[-1.9rem] top-[42%]">
                        <img src="/images/squiggle.png" class="h-11" />
                    </div>
                </Show>
            </button>
        </div>
    }
}

#[component]
fn Link(cx: Scope, href: &'static str, children: Children) -> impl IntoView {
    view! {cx,
        <a
            href=href
            class="text-stone-900 hover:text-sky-800"
        >
            {children(cx)}
        </a>
    }
}

#[component]
fn List(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <ol class="p-4 list-decimal">
            {children(cx)}
        </ol>
    }
}

#[component]
fn Item(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <li>
            {children(cx)}
        </li>
    }
}

#[component]
fn Image(cx: Scope, src: &'static str, height: u32) -> impl IntoView {
    let page_state = use_context::<ReadSignal<PageState>>(cx).unwrap();

    view! {cx,

        <div
            class="px-4 my-10 relative col-start-2  scrollbar-hidden md:overflow-visible"
            style= move || format!("height: {}px", height)
            class=("overflow-x-scroll", move || page_state() == PageState::ShowArticle )
            class=("translate-x-full", move || page_state() == PageState::ShowRight )
            class=("-translate-x-full", move || page_state() == PageState::
            ShowLeft )

        >
            <img
                src=src style= move ||  format!("height: {}px", height)
                class="max-w-none md:absolute md:-translate-x-1/2 md:left-1/2 m-auto"
            />
        </div>

    }
}
