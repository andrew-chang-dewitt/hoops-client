use leptos::{component, view, IntoView, Scope};
use leptos_meta::*;
use leptos_router::{Router, RouterProps};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::routes::{Routes, RoutesProps};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    id: Uuid,
    handle: String,
    full_name: String,
    preferred_name: String,
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
                <Routes />
            </main>
        </Router>
    }
}
