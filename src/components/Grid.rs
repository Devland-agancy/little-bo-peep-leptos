use crate::components::Span::Span;
use leptos::*;

#[component]
pub fn Grid(
    cx: Scope,
    children: ChildrenFn,
    #[prop(default = 0)] margin_top: i16,
    #[prop(default = 0)] margin_bottom: i16,
    #[prop(optional)] id: &'static str,
    #[prop(default = 0)] cols: i16,
    #[prop(default = -1)] sm_cols: i16,
    #[prop(optional)] classes: &'static str,
    #[prop(default = "center")] place_items: &'static str,
    #[prop(default = "1rem")] gap: &'static str,
    #[prop(default = false)] center_on_overflow: bool,
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

    view! { cx,
      <span
        id=id
        class=format!("col-start-2 px-4 grid flex-wrap min-h-fit grid-cols-{} sm:grid-cols-{} {} ", if sm_cols == -1 { cols } else { sm_cols } , cols , classes)
        style=move || {
            format!(
                "margin-top: {}px;margin-bottom: {}px; animation: appear 2s ease 0s 1 normal forwards;place-items: {}; gap: {}",
                margin_top,
                margin_bottom,
                place_items,
                gap,
            )
        }
      >

      <For
        each=move || {
          children(cx).nodes
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
            View::Component(com) =>  {
              if center_on_overflow && (children_count as i16 % sm_cols) == 1 && label.0 == (children_count -1) {
                  return view! {
                  cx,
                  <Span classes="col-span-full sm:col-span-1">
                    {com.children}
                  </Span>
                };
              }
              return com.into_view(cx);
            },
          _ => view!{cx, <Span>""</Span>}
          }
        }
      />
      </span>
    }
}
