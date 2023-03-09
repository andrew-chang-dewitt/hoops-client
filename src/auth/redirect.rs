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
        move || check_logged_in_action.version().get(),
        move |_| check_logged_in(cx),
    );
    let is_logged_in = move || checking_is_logged_in.read(cx);

    match is_logged_in() {
        Some(Ok(maybe_user)) => {
            if let Some(_) = maybe_user {
                // render children if logged in
                view! {
                    cx,
                    {children(cx)}
                }
                .into_view(cx)
            } else {
                // redirect to login page w/ "you have been logged out" error message if not
                view! {
                    cx,
                    <LoggedOut msg=String::from( "you have been logged out" ) />
                }
                .into_view(cx)
            }
        }
        // redirect to login page w/ "an error occurred, please log in" message on error
        Some(Err(err)) => {
            log::error!("Error checking if user is logged in: {err:#?}");

            view! {
                cx,
                <LoggedOut msg=String::from( "an error occurred, please log in" ) />
            }
            .into_view(cx)
        }
        // resource fetcher still pending
        _ => view! {
            cx,
            <p>"Loading..."</p>
        }
        .into_view(cx),
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
