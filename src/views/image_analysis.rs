use web_sys::File;
use yew::prelude::*;

use crate::components::{file_upload_box::FileUploadBox, layout::column_layout::IntoColumns};

#[function_component]
pub fn ImageAnalysisView() -> Html {
    let on_image = Callback::from(|i: File| {
        log::info!("Received {}", i.name());
    });
    html! {
        <IntoColumns>
            <FileUploadBox {on_image}/>
            <FileUploadBox />
        </IntoColumns>
    }
}
