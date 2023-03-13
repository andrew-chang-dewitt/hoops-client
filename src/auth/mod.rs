pub mod redirect;

use cfg_if::cfg_if;
use http::header::GetAll;
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
        use axum::body::Bytes;
        use http::header::{COOKIE, HeaderValue };
        use leptos::{use_context, server_fn::ServerFnError, Scope};
        use leptos_axum::RequestParts;
        use reqwest::Client;

        use crate::app::User;

        /// Check if the user is logged in by using a session auth cookie found in the Scope's
        /// RequestParts value. Returns Ok(Some(user)) if logged in, Ok(None) if not logged in, &
        /// Err(...) if there's any errors are encountered along the way.
        pub async fn is_logged_in(cx: Scope) -> Result<Option<User>, ServerFnError> {
            let req = match use_context::<RequestParts>(cx) {
                Some(req) => req,      // actual user request
                None => return Ok(None), // no request, building routes in main
            };
            let cookies = req.headers.get_all(COOKIE);
            log::trace!("cookies: {cookies:#?}");

            match get_token(cookies) {
                Ok(None) => Ok(None),                     // no token == not logged in
                Ok(Some(token)) => get_user(token).await, // if there is a token, try to get the user
                                                          // info to find out if they're logged in
                Err(err) => Err(err),
            }
        }

        fn get_token(cookies: GetAll<'_, HeaderValue>) -> Result<Option<String>, ServerFnError> {
            for cookie in cookies.iter() {
                match cookie.to_str() {
                    Ok(cookie_str) => {
                        if let Some(token) = get_cookie_value(cookie_str, "jwt") {
                            // only return early if there's a jwt value in the cookie, otherwise
                            // keep iterating through the remaining cookies
                            return Ok(Some(token));
                        }
                    },
                    Err(err) => return Err(ServerFnError::ServerError(err.to_string())),
                }
            }

            // if none of the cookies contain a jwt value, return None
            Ok(None)
        }

        fn get_cookie_value(cookie: &str, key: &str) -> Option<String> {
            // assumes cookie key/value pairs are delimited by `;` & searches for a given key
            cookie.split(';').find_map(|term| {
                let (k, v) = term.split_once('=').unwrap_or_default();
                if k.trim().eq(key) && !v.trim().is_empty() {
                    Some(v.to_string())
                } else {
                    None
                }
            })
        }

        pub async fn create_session_cookie(form_data: Bytes) -> Result<HeaderValue, ServerFnError> {
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
                    log::error!("Error submitting API request: {err:#?}");
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
                let token: Token = res.json().await.map_err(|err| {
                    log::error!("Error processing API response: {err:#?}");
                    ServerFnError::ServerError(String::from(
                            "Whoops, there was problem. Please try again.",
                    ))
                })?;

                // stuff token in cookie
                let cookie = HeaderValue::from_str(
                    &format!("jwt={}; Path=/; HttpOnly", token.access_token)
                ).map_err(|err| {
                    log::error!("Unable to create cookie from token.\ntoken: {token:#?}\nerror: {err:#?}");
                    ServerFnError::ServerError(String::from("An unknown error occurred."))
                })?;

                // return cookie
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

        pub async fn get_user(token: String) -> Result<Option<User>, ServerFnError> {
            let client = Client::new();
            let res = client.get("http://localhost:8000/user")
                .header("Authorization", format!("Bearer {token}"))
                .send()
                .await
                .map_err(|err| {
                    // log actual error to server console
                    log::error!("Error submitting API request: {err:#?}");
                    // obfuscate error sent to client
                    ServerFnError::ServerError(String::from(
                            "Whoops, there was problem. Please try again.",
                    ))
                })?;
            log::trace!("validation response: {res:#?}");

            // handle validation response
            let status_code = res.status();

            if status_code == 200 { // valid token
                let user: User = res.json().await.map_err(|err| {
                    // log error internally
                    log::error!("{err}");
                    ServerFnError::ServerError(String::from("Error parsing user data"))
                })?;
                log::trace!("user: {user:#?}");

                Ok(Some(user))
            } else if status_code == 401 { // unauthorized
                Ok(None)
            } else {
                Err(ServerFnError::ServerError(String::from("An unknown error occurred.")))
            }
        }
    }
}
