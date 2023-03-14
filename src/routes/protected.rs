use leptos::{component, view, IntoView, Scope};

/// Renders an example protected page
#[component]
pub fn Protected(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <p>"Protected!"</p>
    }
}
