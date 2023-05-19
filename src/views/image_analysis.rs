use yew::prelude::*;

use crate::components::{file_upload_box::{FileUploadBox, FileDetails}, layout::column_layout::IntoColumns};

#[function_component]
pub fn ImageAnalysisView() -> Html {
    let img = use_state(|| String::new());
    let imgc = img.clone();
    let on_image = Callback::from(move |i: FileDetails| {
        log::info!("Received {}", i.name);
        imgc.set(i.name);
    });
    html! {
        <IntoColumns>
            <FileUploadBox {on_image} />
            <p>{img.to_string()}</p>
        </IntoColumns>
    }
}
