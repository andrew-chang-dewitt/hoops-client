use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub token_type: TokenType,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenType {
    #[serde(alias = "bearer")]
    Bearer,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        // use std::{fmt, task::{Context, Poll}};

        // use tower::Layer;
        // use tower_service::Service;
        use axum::{body::Bytes, http::header::{COOKIE, SET_COOKIE}};
        use http::header::HeaderValue;
        use leptos::{use_context, ServerFnError, Scope};
        use reqwest::Client;

        use crate::app::User;

        pub fn auth(cx: Scope) -> Result<AuthSession, ServerFnError> {
            use_context::<AuthSession>(cx)
                .ok_or("Auth session missing")
                .map_err(|err| {
                    println!("{err}");
                    ServerFnError::ServerError(err.to_string())
                })
        }

        #[derive(Clone)]
        pub struct AuthSession {
            pub current_user: Option<User>,
            pub token: Option<Token>,
        }

        impl AuthSession {
            pub async fn get_session_cookie(&self, form_data: Bytes) -> Result<HeaderValue, ServerFnError> {
                // get token from api
                let client = Client::new();
                let res = client
                    .post("http://localhost:8000/token")
                    .header("content-type", "application/x-www-form-urlencoded")
                    .body(form_data)
                    .send()
                    .await
                    .map_err(|err| {
                        // log actual error to server console
                        println!("Error submitting API request: {err:#?}");
                        // obfuscate error sent to client
                        ServerFnError::ServerError(String::from(
                                "Whoops, there was problem. Please try again.",
                        ))
                    })?;

                // handle response
                let status = res.status();

                // happy path
                if status == 200 {
                    // get token from response
                    self.token = Some( res.json().await.map_err(|err| {
                        println!("Error processing API response: {err:#?}");
                        ServerFnError::ServerError(String::from(
                                "Whoops, there was problem. Please try again.",
                        ))
                    })? );
                    // get user with token
                    // TODO: not sure how I want to do this. Maybe it's time to refactor API calls
                    // into another module? Should probably just impl something super ugly inline
                    // here then refactor that plus the above call into the new api module.

                    // stuff token in cookie
                    let cookie = HeaderValue::from_str(
                        &format!("user_id={}; jwt={}; Path=/; HttpOnly", self.current_user.id, token.access_token)
                    ).map_err(|err| {
                        println!("Unable to create cookie from token:");
                        dbg!(token);
                        dbg!(err);
                        ServerFnError::ServerError(String::from("An unknown error occurred."))
                    })?;
                    dbg!(&cookie);

                    // send cookie to client in response

                    Ok(cookie)
                }
                // unauthorized
                else if status == 401 {
                    Err(ServerFnError::ServerError(String::from(
                        "Bad username or password. Please correct it and try again.",
                    )))
                }
                // everything else
                else {
                    dbg!(&res);
                    Err(ServerFnError::ServerError(String::from(
                        "An unknown error occurred.",
                    )))
                }
            }
        }

        // #[derive(Clone)]
        // pub struct AuthLayer {}

        // impl AuthLayer {
        //     pub fn new() -> Self {
        //         Self {}
        //     }
        // }

        // impl Default for AuthLayer {
        //     fn default() -> Self {
        //         Self::new()
        //     }
        // }

        // impl<S> Layer<S> for AuthLayer {
        //     type Service = AuthService<S>;

        //     fn layer(&self, service: S) -> Self::Service {
        //         AuthService { service }
        //     }
        // }

        // #[derive(Clone)]
        // pub struct AuthService<S> {
        //     service: S,
        // }

        // impl<S, Request> Service<Request> for AuthService<S>
        // where
        //     S: Service<Request>,
        //     Request: fmt::Debug,
        // {
        //     type Response = S::Response;
        //     type Error = S::Error;
        //     type Future = S::Future;

        //     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        //         self.service.poll_ready(cx)
        //     }

        //     fn call(&mut self, request: Request) -> Self::Future {
        //         self.service.call(request)
        //     }
        // }
    }
}
