use cfg_if::cfg_if;
use leptos::{component, server, view, IntoView, Scope, ServerFn, ServerFnError};
use leptos_meta::*;
use leptos_router::{
    AProps, ActionForm, ActionFormProps, Route, RouteProps, Router, RouterProps, Routes,
    RoutesProps, A,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::leptos_server::create_server_action;

use crate::components::input::{Input, InputProps, InputType};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::auth::Token;

        pub fn register_server_functions() {
            _ = Login::register();
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
    let req = use_context::<leptos_axum::RequestParts>(cx).ok_or(ServerFnError::ServerError(
        String::from("An unknown error occurred."),
    ))?;
    let form_data = req.body;

    // get auth cookie
    let auth = auth(cx)?;
    let cookie = auth.get_session_cookie(form_data);

    // set cookie header in response
    todo!();

    // Ok(())
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
                    <Route path="/protected" view=|cx| view! { cx, <Protected/>}/>
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
    view! {
        cx,
        <h1>"Welcome user!"</h1>
        <p>"This is a protected page"</p>
    }
}
