use std::rc::Rc;

use base64::Engine;
use yew::prelude::*;

use super::file_upload_box::FileDetails;

#[derive(Properties, PartialEq)]
pub struct ImageDisplayProps {
    pub image_data: Rc<FileDetails>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn ImageDisplayBox(props: &ImageDisplayProps) -> Html {
    view_file(props)
}

fn view_file(props: &ImageDisplayProps) -> Html {
    html! {
        <div class={props.class.clone()}>
            <div class="card">
                <img class="card-img-top" src={get_image_url(&*props.image_data)} />
                <p class="card-footer">{ format!("{}", &*props.image_data.name) }</p>
            </div>
        </div>
    }
}

pub fn get_image_url(file: &FileDetails) -> String {
    format!(
        "data:{};base64,{}",
        file.file_type,
        base64::engine::general_purpose::STANDARD.encode(&file.data)
    )
}
