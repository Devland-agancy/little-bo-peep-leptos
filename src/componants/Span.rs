use leptos::*;

#[component]
pub fn Span(
    cx: Scope,
    #[prop(default = "")] _class: &'static str,
    #[prop(default = false)] bold: bool,
    #[prop(default = false)] italic: bool,
    children: Children,
) -> impl IntoView {
    view! { cx,
      <span class=move || {
          format!(
              "{} {} {}",
              _class,
              if italic { "font-baskerville-italic" } else { "" },
              if bold { "font-baskerville-bold" } else { "" },
          )
      }>{children(cx)}</span>
    }
}
