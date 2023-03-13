use cfg_if::cfg_if;
use leptos::{
    component, create_resource, create_server_action, server,
    server_fn::{self, ServerFn, ServerFnError},
    use_context, view, ChildrenFn, IntoView, Scope, SignalGet, Suspense, SuspenseProps,
};
use urlencoding::encode;

use crate::app::User;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use http::header::SET_COOKIE;
        use leptos_axum::{ redirect, ResponseOptions };

        use crate::auth::{create_logout_cookie, is_logged_in };
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
                    let err_msg = String::from("you have been logged out");

                    // render a fallback view to satisfy type checker on match arms & return
                    // results
                    view! {
                        cx,
                        <Logout msg={err_msg} />
                    }
                    .into_view(cx)
                }
                // Error occurred while checking auth status, redirect to login page w/ "an error
                // occurred, please log in" message on error
                Err(err) => {
                    log::error!("Error checking if user is logged in: {err:#?}");
                    let err_msg = String::from("an error occurred, please log in");

                    // render a fallback view to satisfy type checker on match arms & return
                    // results
                    view! {
                        cx,
                        <Logout msg={err_msg} />
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
fn Logout(cx: Scope, msg: String) -> impl IntoView {
    let logout_action = create_server_action::<DestroyCookie>(cx);
    let logout_resource = create_resource(
        cx,
        move || (logout_action.version().get()),
        move |_| server_destroy_cookie(cx),
    );
    let logout_result = move || {
        logout_resource.read(cx).map(|n| match n {
            Ok(()) => redirect_to_login(cx, &msg),
            Err(err) => {
                log::error!("Error logging out: {err:#?}");
                redirect_to_login(cx, &msg)
            }
        })
    };

    view! {
        cx,
        <Suspense fallback={move || view! {
            cx,
            <p>"You will be taken back to the login page momentarily..."</p>
        }}>
            <>{logout_result()}</>
        </Suspense>
    }
}

fn redirect_to_login(cx: Scope, msg: &str) {
    let err_msg_encoded = encode(msg);
    let path = format!("{LOGIN_PATH}?msg={err_msg_encoded}");

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
}

#[server(CheckLoggedIn, "/api")]
pub async fn check_logged_in(cx: Scope) -> Result<Option<User>, ServerFnError> {
    is_logged_in(cx).await
}

#[server(DestroyCookie, "/api")]
pub async fn server_destroy_cookie(cx: Scope) -> Result<(), ServerFnError> {
    destroy_cookie(cx);
    Ok(())
}

#[cfg(feature = "ssr")]
fn destroy_cookie(cx: Scope) {
    // remove cookie
    let res = match use_context::<ResponseOptions>(cx) {
        Some(r) => r,
        None => return,
    };
    res.append_header(
        SET_COOKIE,
        create_logout_cookie().expect("to create cookie"),
    );
}
