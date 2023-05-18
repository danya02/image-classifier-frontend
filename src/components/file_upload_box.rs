use web_sys::{HtmlInputElement, FileList, File};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileUploadProps {
    #[prop_or_default]
    pub on_image: Callback<File>,
}


#[function_component]
pub fn FileUploadBox(props: &FileUploadProps) -> Html {

    let on_image = props.on_image.clone();
    let on_file_drop: Callback<DragEvent> = Callback::from(move |event: DragEvent| {
        event.prevent_default();
        let files = event.data_transfer().unwrap().files();
        if let Some(files) = files {
            if let Some(file) = files.get(0) {
                on_image.emit(file);
            }
        }
    });

    let on_image = props.on_image.clone();
    let on_file_select: Callback<Event> = Callback::from(move |event: Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        if let Some(files) = input.files() {
            if let Some(file) = files.get(0) {
                on_image.emit(file);
            }
        }

    });

    html! {
        <div class="card" ondrop={on_file_drop}
            ondragover={Callback::from(|event: DragEvent| {
                event.prevent_default();
            })}
            ondragenter={Callback::from(|event: DragEvent| {
                event.prevent_default();
            })}>
            <div class="card-body">
                <label for="file-upload">
                        <i class="fa fa-cloud-upload"></i>
                        <p>{"Drop your images here or click to select"}</p>
                </label>
                <input
                    id="file-upload"
                    type="file"
                    accept="image/*"
                    onchange={on_file_select}
                />
            </div>
        </div>
    }
}
