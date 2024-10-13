use leptos::*;

#[component]
pub fn InlineImage(
    src: &'static str,
    #[prop(default = "")] width: &'static str,
    #[prop(default = "")] height: &'static str,
    #[prop(default = "")] y_anchor: &'static str,
    #[prop(default = "")] margin_left: &'static str,
    #[prop(default = "")] margin_right: &'static str,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] style: &'static str,
    #[prop(default = false)] space_left: bool,
    #[prop(default = false)] space_right: bool,
) -> impl IntoView {
    view! {
      <Show
          fallback=move || ""
          when=move || space_left
        >
        " "
      </Show>
      <img
        class=move || format!("bg-cover inline-block relative {class}")
        src=src
        style=move || {
            format!(
                "width: {}; height: {}; top: {}; margin-left: {}; margin-right: {}; {style}",
                width,
                height,
                y_anchor,
                margin_left,
                margin_right,
            )
        }
      />
      <Show
          fallback=move || ""
          when=move || space_right
        >
        " "
      </Show>
    }
}
