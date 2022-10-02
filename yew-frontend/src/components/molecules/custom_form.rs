use std::ops::Deref;

use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_input::TextInput;
use yew::prelude::*;

#[derive(Default,Clone)]
pub struct Data{
    pub username:String,
    pub language: String,
    pub count: u32,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<Data>,
}

#[function_component(CustomForm)]
pub fn custom_form(props: &Props) -> Html {
    let state = use_state(|| Data::default());
    let cloned_state = state.clone();
    let username_change = Callback::from(move |username: String| {
        cloned_state.set(
            Data{
                username,
                ..cloned_state.deref().clone()
            }
        )
    });

    let cloned_state = state.clone();
    let form_onsubmit = props.onsubmit.clone();
    let onsubmit = Callback::from(move |event:FocusEvent| {
        event.prevent_default();
        let data:Data = cloned_state.deref().clone();
        form_onsubmit.emit(data);
    });

    html! {
        <form onsubmit={onsubmit}>
            <TextInput name="username" handle_onchange={username_change} />
        </form>
    }
}
