use cfg_if::cfg_if;
use leptos::{
    component, create_resource, server, server_fn::ServerFn, server_fn::ServerFnError, view,
    IntoView, Scope, SignalGet,
};
use leptos_meta::*;
use leptos_router::{
    AProps, ActionForm, ActionFormProps, Route, RouteProps, Router, RouterProps, Routes,
    RoutesProps, A,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use uuid::Uuid;

use crate::leptos_server::create_server_action;

use crate::auth::redirect::{AuthGuard, AuthGuardProps};
use crate::components::input::{Input, InputProps, InputType};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use http::header::SET_COOKIE;
        use leptos::{ use_context };
        use leptos_axum::{redirect, RequestParts };

        use crate::auth::{ create_session_cookie, redirect::{CheckLoggedIn, ForceLogout } };

        pub fn register_server_functions() {
            _ = Login::register();
            _ = CheckLoggedIn::register();
            _ = ForceLogout::register();
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    id: Uuid,
    handle: String,
    full_name: String,
    preferred_name: String,
}

#[server(Login, "/api")]
async fn login(cx: Scope) -> Result<(), ServerFnError> {
    // get form data from request context
    let req = match use_context::<RequestParts>(cx) {
        Some(req) => req,      // actual user request
        None => return Ok(()), // no request, building routes in main
    };
    let form_data = req.body;

    let cookie = create_session_cookie(form_data).await?;

    // send cookie to client in response
    let res = use_context::<leptos_axum::ResponseOptions>(cx)
        .ok_or("Unable to get response object.")
        .map_err(|err| {
            log::error!("{}", &err);
            ServerFnError::ServerError(err.to_string())
        })?;
    res.append_header(SET_COOKIE, cookie);
    redirect(cx, "/protected");

    Ok(())
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start_axum.css"/>

        // sets the document title
        <Title text="Hoops | App"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/>}/>
                    <Route path="/login" view=|cx| view! { cx, <Login/>}/>
                    <Route path="/protected" view=|cx| view! { cx, <AuthGuard><Protected/></AuthGuard>}/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders a home page
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <h1>"Welcome!"</h1>
        <A href="/login">"log in"</A>
    }
}

/// Renders the login page
#[component]
fn Login(cx: Scope) -> impl IntoView {
    let login = create_server_action::<Login>(cx);

    view! {
        cx,
        <ActionForm action=login>
            <Input name={ String::from( "username" ) } label={ String::from( "Username:" ) }/>
            <Input name={ String::from( "password" ) } label={ String::from( "Password:" ) } input_type=InputType::Password />
            <button type="submit">"Login"</button>
        </ActionForm>

    }
}

/// Renders an example protected page
#[component]
fn Protected(cx: Scope) -> impl IntoView {
    // let fetcher = move |_| {
    //     let client = Client::new();
    //     let res = client.get()
    // };

    // let user_resource = create_resource(
    //     cx,
    //     move || data_action.version().get(),
    //     move |_| get_protected_page(cx),
    // );

    view! {
        cx,
        <p>"Protected!"</p>
    }
}
