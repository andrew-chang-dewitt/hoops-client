use cfg_if::cfg_if;
use leptos::{
    component, create_server_action, server,
    server_fn::{self, ServerFn, ServerFnError},
    view, IntoView, Params, Scope,
};
use leptos_router::{use_query, ActionForm, ActionFormProps, IntoParam, Params};

use crate::components::input::{Input, InputProps, InputType};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use http::header::SET_COOKIE;
        use leptos::use_context;
        use leptos_axum::{redirect, RequestParts};

        use crate::auth::create_session_cookie;
    }
}

/// Renders the login page
#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let params = use_query::<LoginParams>(cx);
    let msg = move || params().map_or_else(|err| Some(err.to_string()), |p| p.msg);
    let login = create_server_action::<Login>(cx);

    view! {
        cx,
        <div>{msg()}</div>
        <ActionForm action=login>
            <Input name={ String::from( "username" ) } label={ String::from( "Username:" ) }/>
            <Input name={ String::from( "password" ) } label={ String::from( "Password:" ) } input_type=InputType::Password />
            <button type="submit">"Login"</button>
        </ActionForm>

    }
}

#[derive(Params, PartialEq, Clone, Debug)]
struct LoginParams {
    #[params]
    msg: Option<String>,
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
    dbg!(&res);
    redirect(cx, "/protected");

    Ok(())
}
