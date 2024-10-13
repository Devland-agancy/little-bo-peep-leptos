use leptos::*;

#[component]
pub fn List( #[prop(default = true)] indent: bool, children: Children) -> impl IntoView {
    view! { 
      <ol class=("ml-6", indent) class="px-4 list-decimal">
        {children()}
      </ol>
    }
}

#[component]
pub fn Item( children: Children) -> impl IntoView {
    view! { <li>{children()}</li> }
}
