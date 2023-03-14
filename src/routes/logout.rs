use leptos::{component, view, IntoView, Scope};
use leptos_router::{AProps, A};

use crate::auth::{Logout as LogoutAction, LogoutProps as LogoutActionProps};

/// Uses auth::Logout to log the user out on render
#[component]
pub fn Logout(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <LogoutAction msg={String::from("You have been successfully logged out.")} />
    }
}
