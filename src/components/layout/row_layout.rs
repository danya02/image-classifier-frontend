use yew::prelude::*;

use yew::{html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Row {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn AsRows(props: &Row) -> Html {
    html! {
        <div class="container">
        { for props.children.iter().map(|x| html!{ <div class="row">{x}</div> }) }
        </div>
    }
}
