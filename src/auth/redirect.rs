use cfg_if::cfg_if;
use leptos::{
    component, create_resource, create_server_action, create_signal, server,
    server_fn::{self, ServerFn, ServerFnError},
    view, ChildrenFn, IntoView, Scope, SignalGet, Suspense, SuspenseProps,
};
use urlencoding::encode;

use crate::app::User;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos_axum::redirect;
        use crate::auth::is_logged_in;
    }
}
cfg_if! {
    if #[cfg(not(feature = "ssr"))] {
        use leptos_router::{use_navigate, NavigateOptions };
    }
}

const LOGIN_PATH: &str = "/login";

/// This component is used to wrap any other component that shouldn't be rendered unless the user
/// is logged in with a valid token. If the user is not logged in, they should be redirected to the
/// `/login` page instead of rendering the given child.
#[component]
pub fn AuthGuard(cx: Scope, children: ChildrenFn) -> impl IntoView {
    let check_logged_in_action = create_server_action::<CheckLoggedIn>(cx);
    let checking_is_logged_in = create_resource(
        cx,
        move || (check_logged_in_action.version().get()),
        move |_| check_logged_in(cx),
    );

    let (err_msg, set_err_msg) = create_signal(cx, String::new());
    let check_result = move || {
        checking_is_logged_in.read(cx).map(|n| {
            log::trace!("check_logged_in result: {n:#?}");

            match n {
                // User is logged in, ok to render children
                Ok(Some(_)) => view! {
                    cx,
                    { children(cx) }
                }
                .into_view(cx),
                // User is not logged in, redirect to login page w/ "you have been logged out" error
                // message if not
                Ok(None) => {
                    set_err_msg(String::from("you have been logged out"));
                    let err_msg_str = &err_msg();
                    let err_msg_encoded = encode(err_msg_str);
                    let path = format!("{LOGIN_PATH}?{err_msg_encoded}");

                    // redirect using axum if ssr, or leptos_router if client
                    cfg_if! {
                        if #[cfg(feature = "ssr")] {
                            redirect(cx, &path);
                        } else {
                            let navigate = use_navigate(cx);
                            if let Err(err) = navigate(&path, NavigateOptions::default()) {
                                log::error!("There was an error redirecting to the login page: {err:#?}");
                            };
                        }
                    }

                    // render a fallback view to satisfy type checker on match arms & return
                    // results
                    view! {
                        cx,
                        <>"Unauthorized"</>
                    }
                    .into_view(cx)
                }
                // Error occurred while checking auth status, redirect to login page w/ "an error
                // occurred, please log in" message on error
                Err(err) => {
                    log::error!("Error checking if user is logged in: {err:#?}");
                    set_err_msg(String::from("an error occurred, please log in"));

                    // render a fallback view to satisfy type checker on match arms & return
                    // results
                    view! {
                        cx,
                        <>"Unauthorized"</>
                    }
                    .into_view(cx)
                }
            }
        })
    };

    view! {
        cx,
        <Suspense fallback={move || view!{cx, <>"Loading..."</>}}>
            {check_result()}
        </Suspense>
    }
}

#[component]
fn LoggedOut(cx: Scope, msg: String) -> impl IntoView {
    create_server_action::<ForceLogout>(cx).dispatch(ForceLogout { msg });

    view! {
        cx,
        <p>"Unauthorized"</p>
    }
    .into_view(cx)
}

#[server(CheckLoggedIn, "/api")]
pub async fn check_logged_in(cx: Scope) -> Result<Option<User>, ServerFnError> {
    is_logged_in(cx).await
}

#[server(ForceLogout, "/api")]
pub async fn force_logout(cx: Scope, msg: String) -> Result<(), ServerFnError> {
    log::info!("Forcing logout: {msg}...");
    todo!()
}
