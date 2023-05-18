use yew::prelude::*;

use yew::{html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ColumnProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn IntoColumns(props: &ColumnProps) -> Html {
    html! {
        <div class="row">
            { for props.children.iter().map(|x| html!{ <div class="col">{x}</div> }) }
        </div>
    }
}
