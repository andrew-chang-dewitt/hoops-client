use std::fmt::Display;

use leptos::{typed_builder::TypedBuilder, view, Children, IntoAttribute, IntoView, Scope};
use leptos_router::{use_resolved_path, ToHref};

pub enum HttpMethod {
    Get,
    Post,
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Get => String::from("GET"),
            Self::Post => String::from("POST"),
        };

        write!(f, "{string}")
    }
}

pub enum FormEnctype {
    XWwwFormUrlEncoded,
    Json,
}

#[derive(TypedBuilder)]
pub struct FormProps<Action> {
    /// The url to submit the form to
    action: Action,
    /// The submission method to use, defaults to GET
    #[builder(default = HttpMethod::Get)]
    method: HttpMethod,
    /// The submission encoding type to use, defaults to application/x-www-form-urlencoded
    // #[builder(default, setter(strip_option))]
    // enctype: Option<FormEnctype>,
    /// Enable or disable autocomplete, defaults to enabled
    // #[builder(default, setter(strip_option))]
    // autocomplete: Option<bool>,
    /// Component children
    children: Children,
}

#[allow(non_snake_case)]
pub fn Form<Action>(cx: Scope, props: FormProps<Action>) -> impl IntoView
where
    Action: ToHref + 'static,
{
    let FormProps {
        action,
        method,
        children,
    } = props;

    // use leptos router's method to ensure given action is valid & store it as a Signal
    let resolved_action = use_resolved_path(cx, move || action.to_href()());

    view! {
        cx,
        <form
            // get the action from the resolved Signal as a String
            action=move || resolved_action.get()
            method=method.to_string()
        >
            {children(cx)}
        </form>
    }
}

#[cfg(test)]
mod tests {
    use leptos::mount_to_body;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // use crate::components::test_utils::*;

    use super::*;

    mod default_attributes {
        use web_sys::Element;

        use super::*;

        fn setup<Action>(action: Action) -> Option<Element>
        where
            Action: ToHref + 'static,
        {
            mount_to_body(move |cx| {
                view! {
                    cx,
                    <Form action=action>
                        <label for="username">"Username:"</label>
                        <input id="username" type="text" />
                        <button type="submit">"Submit"</button>
                    </Form>
                }
            });

            let document = leptos::document();
            document.query_selector("form").unwrap()
        }

        #[wasm_bindgen_test]
        pub fn it_uses_the_given_action() {
            let form = setup("/some/action").unwrap();

            assert_eq!(
                form.get_attribute("action"),
                Some(String::from("/expected/action")),
                "Form should use the given action."
            );
        }

        #[wasm_bindgen_test]
        pub fn it_contains_the_expected_children() {
            let form = setup("/some/action").unwrap();

            assert_eq!(
                form.child_element_count(),
                3,
                "Form should contain only the given number of children"
            );
        }

        #[wasm_bindgen_test]
        pub fn it_defaults_to_get_method() {
            let form = setup("/some/action").unwrap();

            assert_eq!(
                form.get_attribute("method"),
                Some(String::from("GET")),
                "Form should default to GET method."
            );
        }

        #[wasm_bindgen_test]
        pub fn it_defaults_to_enabling_autocomplete() {
            let form = setup("/some/action").unwrap();

            assert_eq!(
                form.get_attribute("autocomplete"),
                Some(String::from("true")),
                "Form should default to autocomplete being enabled."
            );
        }

        #[wasm_bindgen_test]
        pub fn it_defaults_to_url_encoding() {
            let form = setup("/some/action").unwrap();

            assert_eq!(
                form.get_attribute("enctype"),
                Some(String::from("application/x-www-form-urlencoded")),
                "Form should use url encoding by default."
            );
        }
    }
}
