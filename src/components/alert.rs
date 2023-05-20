use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AlertProps {
    pub text: String,
    pub style: String,
}

#[function_component]
pub fn Alert(props: &AlertProps) -> Html {
    let alert_kind = format!("alert-{}", props.style);
    let is_dismissed = use_state(|| false);
    let is_dismissedc = is_dismissed.clone();
    let set_dismissed = Callback::from(move |_| is_dismissedc.set(true));
    if *is_dismissed {
        html!()  // Yes, this means that dismissed alerts will leak memory and not show up
    } else {
        html! {
            <div class={classes!("alert", "alert-dismissable", alert_kind)}>
                {&props.text}

                <button type="button" class="btn-close" onclick={set_dismissed} />
            </div>
        }
    }
}
