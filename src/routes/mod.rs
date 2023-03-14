use cfg_if::cfg_if;
use leptos::{component, view, IntoView, Scope, ServerFn, ServerFnError};
use leptos_router::{Route, RouteProps, Routes as LeptosRoutes, RoutesProps as LeptosRoutesProps};

use crate::auth::{AuthGuard, AuthGuardProps};

mod home;
use home::{HomePage, HomePageProps};
mod login;
use login::{Login, LoginProps};
mod logout;
use logout::{Logout, LogoutProps};
mod protected;
use protected::{Protected, ProtectedProps};

// List all routes as constant string slices
pub const LOGIN_PATH: &str = "/login";
pub const LOGOUT_PATH: &str = "/logout";

#[component]
pub fn Routes(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <LeptosRoutes>
            <Route path="" view=|cx| view! { cx, <HomePage/>}/>
            <Route path=LOGIN_PATH view=|cx| view! { cx, <Login/>}/>
            <Route path=LOGOUT_PATH view=|cx| view! { cx, <Logout/>}/>
            <Route path="/protected" view=|cx| view! { cx, <AuthGuard><Protected/></AuthGuard>}/>
        </LeptosRoutes>
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub fn register_server_fns() -> Result<(), ServerFnError> {
            login::Login::register()?;
            Ok(())
        }
    }
}
