use leptos::{component, view, IntoView, Scope};
use leptos_router::{AProps, A};

use super::LOGOUT_PATH;

/// Renders an example protected page
#[component]
pub fn Protected(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <p>"Protected!"</p>
        <A href=LOGOUT_PATH>"Log out"</A>
    }
}
