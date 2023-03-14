use leptos::{component, view, IntoView, Scope};
use leptos_router::{AProps, A};

/// Renders a home page
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <h1>"Welcome!"</h1>
        <A href="/login">"log in"</A>
    }
}
