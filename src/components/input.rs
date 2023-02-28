use leptos::typed_builder::*;
use leptos::*;

/// Properties that can be passed to the [Input] component.
#[derive(TypedBuilder)]
pub struct InputProps {
    /// The field's name. Used as the input element's id & name as well as to link the label element
    /// to the input element.
    pub name: String,
    /// The field's label text.
    pub label: String,
    /// The input type, can be any of [InputType]. Defaults to [InputType::Text] if unset.
    #[builder(default, setter(strip_option))]
    pub input_type: Option<InputType>,
}

#[allow(non_snake_case)]
pub fn Input(cx: Scope, props: InputProps) -> impl IntoView {
    let InputProps {
        name,
        label,
        input_type,
    } = props;

    let input_type_str: String;

    if let Some(given_type) = input_type {
        input_type_str = match given_type {
            InputType::Text => String::from("text"),
            InputType::Password => String::from("password"),
        }
    } else {
        input_type_str = String::from("text")
    }

    view! {
        cx,
        <label for=&name>{&label}</label>
        <input id=&name name=&name type=input_type_str />
    }
}

pub enum InputType {
    Text,
    Password,
}
