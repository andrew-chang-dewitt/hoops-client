use cfg_if::cfg_if;
use leptos::{
    component, create_resource, create_server_action, server,
    server_fn::{self, ServerFn, ServerFnError},
    use_context, view, IntoView, Scope, SignalGet, Suspense, SuspenseProps,
};

use super::guard::redirect_to_login;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos_axum::ResponseOptions;
        use http::header::SET_COOKIE;

        use super::create_logout_cookie;
    }
}

#[component]
pub fn Logout(cx: Scope, msg: String) -> impl IntoView {
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
