use cfg_if::cfg_if;
use leptos::*;

use crate::app::User;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::auth::is_logged_in;
    }
}
cfg_if! {
    if #[cfg(not(feature = "ssr"))] {
        use leptos_router::NavigateOptions;
    }
}

/// This component is used to wrap any other component that shouldn't be rendered unless the user
/// is logged in with a valid token. If the user is not logged in, they should be redirected to the
/// `/login` page instead of rendering the given child.
#[component]
pub fn AuthGuard(cx: Scope, children: Children) -> impl IntoView {
    let check_logged_in_action = create_server_action::<CheckLoggedIn>(cx);
    let checking_is_logged_in = create_resource(
        cx,
        move || (check_logged_in_action.version().get()),
        move |_| check_logged_in(cx),
    );

    let (err_msg, set_err_msg) = create_signal(cx, String::new());
    let check_result = move || {
        checking_is_logged_in
            .read(cx)
            .map(|n| -> bool {
                match n {
                    // User is logged in
                    Ok(Some(_)) => true,
                    // User is not logged in, redirect to login page w/ "you have been logged out" error
                    // message if not
                    Ok(None) => {
                        set_err_msg(String::from("you have been logged out"));
                        false
                    }
                    // Error occurred while checking auth status, redirect to login page w/ "an error
                    // occurred, please log in" message on error
                    Err(err) => {
                        log::error!("Error checking if user is logged in: {err:#?}");
                        set_err_msg(String::from("an error occurred, please log in"));
                        false
                    }
                }
            })
            .unwrap_or(false)
    };

    // TODO: this clearly won't work, maybe distill check_result into an option<bool> instead, then
    // use a Show inside the suspense to optionally render the children?
    // TODO: FIXME: maybe not this ^^? Show feels wrong, it'll render LoggedOut when still pending
    // instead of waiting to render...
    // TODO: also add redirect logic using leptos_axum::navigate && leptos_router::use_navigate for
    // ssr & not(ssr) contexts too in the Ok(None) & Err match arms
    // see https://github.com/Indrazar/auth-sessions-example/blob/14817a048995a96ef1105abf502ad3e2b923b302/src/pages/components/redirect.rs#L41
    // for example of navigation
    view! {
        cx,
        <Suspense fallback={move || view!{cx, <>"Loading..."</>}}>
            <Show
                when={move || check_result()}
                fallback={move |cx| view! { cx, <LoggedOut msg={err_msg()}/> }}
            >
                {children(cx)}
            </Show>
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
