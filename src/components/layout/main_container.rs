use yew::prelude::*;

use yew::{html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct MainProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Main(props: &MainProps) -> Html {
    html! {
        <div class="container">
            { for props.children.iter() }
        </div>
    }
}
