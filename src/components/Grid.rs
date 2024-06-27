use crate::components::Span::Span;
use crate::constants::{TEXT_LEFT_PADDING, TEXT_RIGHT_PADDING};
use leptos::ev::resize;
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlCollection};

#[component]
pub fn Grid(
    cx: Scope,
    children: ChildrenFn,
    #[prop(default = 0)] margin_top: i16,
    #[prop(default = 0)] margin_bottom: i16,
    #[prop(optional)] id: usize,
    #[prop(default = 0)] cols: i16,
    #[prop(default = -1)] sm_cols: i16,
    #[prop(default = 520)] sm_cutoff: u16,
    #[prop(optional)] classes: &'static str,
    #[prop(default = "center")] place_items: &'static str,
    #[prop(default = "1rem")] gap: &'static str,
    #[prop(default = false)] center_on_overflow: bool,
    #[prop(default = true)] with_padding: bool,
    #[prop(default = false)] column_first: bool,
) -> impl IntoView {
    let children_count = children(cx)
        .nodes
        .into_iter()
        .filter(move |node| {
            if let Some(text) = node.as_text() {
                return text.content != r#""#.into_view(cx).as_text().unwrap().content;
            }
            return true;
        })
        .count();

    let (sm_activated, set_sm_activated) = create_signal(cx, false);

    create_effect(cx, move |_| {
        set_sm_activated(window().inner_width().unwrap().as_f64().unwrap() <= sm_cutoff as f64);

        window_event_listener(resize, move |_| {
            set_sm_activated(window().inner_width().unwrap().as_f64().unwrap() <= sm_cutoff as f64);
        });
    });

    create_effect(cx, move |_| {
        let parent = document().get_element_by_id(&format!("{id}")).unwrap();
        let children: HtmlCollection = parent.children();
        for i in 0..children.length() {
            let mut position = i as f64;

            //   if mode is column first , element new position is calculated by how many elements exist before it . An element x is considered before y if x.j < y.j or x.j = y.j and x.i < y.i
            if column_first && sm_activated() {
                let rows = (children_count as f64 / cols as f64).ceil();
                let element_row = ((i + 1) as f64 / cols as f64).ceil();
                let element_col = (i as i16 % cols) + 1;
                let preceding_elements_in_prev_cols = (element_col - 1) as f64 * rows;
                let preceding_elements_in_curr_col = (element_row - 1.0) as f64;
                position = preceding_elements_in_prev_cols + preceding_elements_in_curr_col;
            }

            if let Some(child) = children.item(i) {
                let _ = child.set_attribute("style", &format!("order: {}", position));
            }
        }
    });

    view! { cx,
      <span
        id=id
        class=move || {
            format!(
                "col-start-2 grid flex-wrap min-h-fit grid-cols-{} {} ",
                if sm_cols != -1 && sm_activated() { sm_cols } else { cols },
                classes,
            )
        }

        style=move || {
            format!(
                "margin-top: {}px;margin-bottom: {}px; animation: appear 2s ease 0s 1 normal forwards;place-items: {}; gap: {}; padding-left: {}; padding-right: {}",
                margin_top,
                margin_bottom,
                place_items,
                gap,
                if with_padding { TEXT_LEFT_PADDING } else { "0px" },
                if with_padding { TEXT_RIGHT_PADDING } else { "0px" },
            )
        }
      >

        <For
          each=move || {
              children(cx)
                  .nodes
                  .clone()
                  .into_iter()
                  .filter(move |node| {
                      if let Some(text) = node.as_text() {
                          return text.content != r#""#.into_view(cx).as_text().unwrap().content;
                      }
                      return true;
                  })
                  .enumerate()
          }

          key=|label| label.0
          view=move |cx, label| {
                match label.1 {
                    View::Component(com) => {
                        if center_on_overflow && (children_count as i16 % sm_cols) == 1
                            && label.0 == (children_count - 1)
                        {
                            return
                            view! { cx,
                                <Span classes="col-span-full sm:col-span-1 w-max">{com.children}</Span>
                            };
                        }
                        return
                        view! { cx, <Span classes="w-max">{com.children}</Span> };
                    }
                    _ => {
                        view! { cx, <Span>""</Span> }
                    }
                }
          }
        />

      </span>
    }
}
