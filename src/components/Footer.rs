use crate::constants::HAMBURGER_MENU_HEIGHT;
use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
      <footer class="slice"
            style=move || format!("height: {HAMBURGER_MENU_HEIGHT}px")
      >
      </footer>
    }
}
