use std::collections::HashMap;

use base64::{Engine};
use gloo::file::callbacks::FileReader;
use gloo::file::File;
use web_sys::{DragEvent, Event, FileList, HtmlInputElement};
use yew::html::TargetCast;
use yew::{html, Callback, Component, Context, Html};
use yew::prelude::*;

#[derive(Clone)]
pub struct FileDetails {
    pub name: String,
    pub file_type: String,
    pub data: Vec<u8>,
}

pub enum Msg {
    Loaded(String, String, Vec<u8>),
    Files(Vec<File>),
}

pub struct FileUploadBox {
    readers: HashMap<String, FileReader>,
    file: Option<FileDetails>,
}

#[derive(Properties, PartialEq)]
pub struct FileUploadProps {
    #[prop_or_default]
    pub on_image: Callback<FileDetails>,
}

impl Component for FileUploadBox {
    type Message = Msg;
    type Properties = FileUploadProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            file: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file_name, file_type, data) => {
                let file_details = FileDetails {
                    data,
                    file_type,
                    name: file_name.clone(),
                };
                self.readers.remove(&file_name);
                ctx.props().on_image.emit(file_details.clone());
                self.file = Some(file_details);
                true
            }
            Msg::Files(files) => {
                for file in files.into_iter() {
                    let file_name = file.name();
                    let file_type = file.raw_mime_type();

                    let task = {
                        let link = ctx.link().clone();
                        let file_name = file_name.clone();

                        gloo::file::callbacks::read_as_bytes(&file, move |res| {
                            link.send_message(Msg::Loaded(
                                file_name,
                                file_type,
                                res.expect("failed to read file"),
                            ))
                        })
                    };
                    self.readers.insert(file_name, task);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let preview = if let Some(f) = &self.file {
            Self::view_file(&f)
        } else {
            html!(
                <img class="card-img-top" src="https://via.placeholder.com/400" />
            )
        };

        html! {
        <div class="card"
            id="drop-container"
            ondrop={ctx.link().callback(|event: DragEvent| {
                event.prevent_default();
                let files = event.data_transfer().unwrap().files();
                Self::upload_files(files)
            })}
            ondragover={Callback::from(|event: DragEvent| {
                event.prevent_default();
            })}
            ondragenter={Callback::from(|event: DragEvent| {
                event.prevent_default();
            })}>
            { preview }

            <div class="card-body">
                <input
                    id="file-upload"
                    type="file"
                    accept="image/*"
                    multiple={false}
                    onchange={ctx.link().callback(move |e: Event| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        Self::upload_files(input.files())
                    })}
                />
            </div>
        </div>
        }
    }
}

impl FileUploadBox {
    fn view_file(file: &FileDetails) -> Html {
        html! {
            <>
                <img class="card-img-top" src={Self::get_image_url(file)} />
                <p class="preview-name">{ format!("{}", file.name) }</p>
            </>
        }
    }

    fn get_image_url(file: &FileDetails) -> String {
        format!("data:{};base64,{}", file.file_type, base64::engine::general_purpose::STANDARD.encode(&file.data))
    }

    fn upload_files(files: Option<FileList>) -> Msg {
        let mut result = Vec::new();

        if let Some(files) = files {
            let files = js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(File::from);
            result.extend(files);
        }
        Msg::Files(result)
    }
}
