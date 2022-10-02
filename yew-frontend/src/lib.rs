use std::ops::Deref;

use gloo::console::{log};
use yew::prelude::*;

mod components;
use components::molecules::custom_form::Data;
use components::molecules::custom_form::CustomForm;

#[derive(Debug,PartialEq,Clone,Default)]
pub struct User{
    pub username:String,
    pub language:String,
}

#[function_component(App)]
pub fn app()->Html {
    let name = "my name";
    log!("name: ",name);

    let user_state = use_state(User::default);
    let custom_form_submit = {
        let user_state = user_state.clone();
        Callback::from(move |data:Data| {
            let mut user = user_state.deref().clone();
            user.username = data.username;
            user.language = data.language;
            user_state.set(user);
        })
    };

    html!{
        <>
        <h1>{"Hello world"}</h1>
        <CustomForm onsubmit={custom_form_submit} />

        </>
        
    }

}