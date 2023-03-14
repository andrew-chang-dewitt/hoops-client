use leptos::*;

#[component]
pub fn Input(
    cx: Scope,
    name: String,
    label: String,
    #[prop(default = InputType::Text)] input_type: InputType,
) -> impl IntoView {
    let input_type_str: String = input_type.into();

    view! {
        cx,
        <label for=&name>{&label}</label>
        <input id=&name name=&name type=&input_type_str />
    }
}

pub enum InputType {
    Text,
    Password,
    Hidden,
}

impl Into<String> for InputType {
    fn into(self) -> String {
        match self {
            InputType::Text => String::from("text"),
            InputType::Password => String::from("password"),
            InputType::Hidden => String::from("hidden"),
        }
    }
}
