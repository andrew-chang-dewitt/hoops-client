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
        use async_trait::async_trait;
        use axum_sessions_auth::{Authentication, SessionSqlitePool};
        use reqwest::Client;
        use leptos::use_context;
        use sqlx::SqlitePool;

        pub type AuthSession = axum_sessions_auth::AuthSession<User, Uuid, SessionSqlitePool, SqlitePool>;

        pub fn register_server_functions() {
            _ = Login::register();
        }

        pub fn pool(cx: Scope) -> Result<SqlitePool, ServerFnError> {
            Ok(use_context::<SqlitePool>(cx)
                .ok_or("Pool missing")
                .map_err(|err| ServerFnError::ServerError(err.to_string()))?)
        }

        pub fn auth(cx: Scope) -> Result<AuthSession, ServerFnError> {
            Ok(use_context::<AuthSession>(cx)
                .ok_or("Auth session missing")
                .map_err(|err| ServerFnError::ServerError(err.to_string()))?)
        }

        impl User {
            pub async fn get(userid: Uuid, pool: &SqlitePool) -> anyhow::Result<Option<User>> {
                // get user by id from Sessions sqlite db
                // if they aren't there, attempt to get them from the API
                todo!()
            }
        }

        #[async_trait]
        impl Authentication<User, Uuid, SqlitePool> for User {
            async fn load_user(userid: Uuid, pool: Option<&SqlitePool>) -> Result<User, anyhow::Error> {
                let pool = pool.unwrap();

                User::get(userid, pool)
                    .await?
                    .ok_or_else(|| anyhow::anyhow!("Cannot get user"))
            }

            fn is_authenticated(&self) -> bool {
                // impl this by checking if a User has a valid Token
                todo!()
            }

            fn is_active(&self) -> bool {
                todo!()
            }

            fn is_anonymous(&self) -> bool {
                todo!()
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    user_id: Uuid,
    token: Token,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Token {
    access_token: String,
    token_type: TokenType,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum TokenType {
    #[serde(alias = "bearer")]
    Bearer,
}

#[server(Login, "/api")]
async fn login(cx: Scope) -> Result<(), ServerFnError> {
    // get form data from Request context
    match use_context::<leptos_axum::RequestParts>(cx) {
        Some(req) => {
            // form data is in request body
            let body = req.body;
            // then send to API verbatim to create a login token
            let client = Client::new();
            let res = client
                .post("http://localhost:8000/token")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(body)
                .send()
                .await
                .or_else(|err| {
                    println!("Error submitting API request: {err:#?}");
                    Err(ServerFnError::ServerError(String::from(
                        "Whoops, there was problem. Please try again.",
                    )))
                })?;
            // get token from response
            let token: Token = res.json().await.or_else(|err| {
                println!("Error processing API response: {err:#?}");
                Err(ServerFnError::ServerError(String::from(
                    "Whoops, there was problem. Please try again.",
                )))
            })?;
            // create new session w/ token in db so the token can be retrieved by a session cookie
            // then get session cookie & send to client

            Ok(())
        }
        None => Err(ServerFnError::ServerError(String::from(
            "No Request Received, this should never happen.",
        ))),
    }
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
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the login page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <h1>"Welcome!"</h1>
        <A href="/login">"log in"</A>
    }
}

/// Renders the login page of your application.
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
