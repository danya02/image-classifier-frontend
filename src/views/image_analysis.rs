use std::{collections::HashMap, rc::Rc};

use gloo::timers::callback::Interval;
use log::{debug, info};
use reqwest::multipart::Part;
use yew::prelude::*;

use crate::components::{
    file_upload_box::{FileDetails, FileUploadBox},
    image_analysis_row::AnalysisReportRow,
    layout::column_layout::IntoColumns,
};

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct ImageAnalysisData {
    #[serde(rename="overall_class")]
    pub overall_classification: HashMap<String, f64>,
}

#[derive(Clone, PartialEq)]
pub struct ImageAnalysisStatus {
    pub data: Rc<FileDetails>,
    pub outcome: ImageAnalysisOutcome,
}

type RequestId = usize;

pub type AnalysisResponse = HashMap<String, ImageAnalysisData>;

#[derive(Clone, Debug, PartialEq)]
pub enum ImageAnalysisOutcome {
    WaitingToSend,
    WaitingForResponse(RequestId),
    Analyzed(ImageAnalysisData),
    Error(String),
}

pub struct ImageAnalysisView {
    requests_sent: usize,
    images: Vec<ImageAnalysisStatus>,
    _clock_handle: Interval,
}

pub enum ImageAnalysisViewMsg {
    NewImageUploaded(FileDetails),
    TimerTick,
    AnalysisRequestCompleted(usize, Result<HashMap<String, ImageAnalysisData>, String>),
}

impl Component for ImageAnalysisView {
    type Message = ImageAnalysisViewMsg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let _clock_handle = {
            let link = ctx.link().clone();
            Interval::new(100, move || link.send_message(ImageAnalysisViewMsg::TimerTick))
        };
        let s = Self {
            requests_sent: 0,
            images: vec![],
            _clock_handle,
        };
        
        s
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut image_rows = vec![];
        for image in self.images.iter() {
            image_rows.push(html!(
                <AnalysisReportRow image={image.clone()} />
            ));
        }

        let on_image = ctx.link().callback(|i| ImageAnalysisViewMsg::NewImageUploaded(i));
        html! {
        <div>
            <FileUploadBox {on_image} />
            <div>
                {image_rows}
            </div>
        </div>
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ImageAnalysisViewMsg::NewImageUploaded(img) => {
                self.on_image(img);
                true
            },
            ImageAnalysisViewMsg::TimerTick => self.collect_pending(ctx),
            ImageAnalysisViewMsg::AnalysisRequestCompleted(idx, data) => {
                self.analysis_request_completed(idx, data);
                true
            },

        }
    }
}

impl ImageAnalysisView {
    fn on_image(&mut self, i: FileDetails) {
        log::info!("Received image {}", i.name);
        let status = ImageAnalysisStatus {
            data: Rc::new(i),
            outcome: ImageAnalysisOutcome::WaitingToSend,
        };
        self.images.push(status);
    }

    fn analysis_request_completed(
        &mut self,
        idx: usize,
        data: Result<HashMap<String, ImageAnalysisData>, String>,
    ) {
        info!("Received data: {idx} {data:?}");
        for item in self.images.iter_mut() {
            debug!("checking {} {:?}", item.data.name, item.outcome);
            if let ImageAnalysisOutcome::WaitingForResponse(i) = item.outcome {
                if i != idx {
                    debug!("ignoring {}", item.data.name);
                    continue;
                }
                info!("Updated {}", item.data.name);
                if let Ok(file_data) = &data {
                    // Find the item with this file name in the response
                    if let Some(item_data) = file_data.get(&item.data.name) {
                        item.outcome = ImageAnalysisOutcome::Analyzed(item_data.clone());
                    } else {
                        item.outcome = ImageAnalysisOutcome::Error(
                            "Server seems to have ignored the provided image?!".to_owned(),
                        );
                    }
                } else if let Err(why) = &data {
                    item.outcome = ImageAnalysisOutcome::Error(why.to_string())
                };
            }
        }
    }

    fn collect_pending(&mut self, ctx: &Context<Self>) -> bool {
        // Collect all images waiting to be sent, then split that into groups of 5.
        let mut request_idxs_to_send = vec![];
        let chunk_size = 5;

        {
            let mut to_send = vec![];
            for img in self.images.iter_mut() {
                //debug!("Found: {}", img.data.name);
                if matches!(img.outcome, ImageAnalysisOutcome::WaitingToSend) {
                    info!("Found pending: {}", (&img).data.name);
                    to_send.push(img);
                }
            }

            // Mark the items inside the chunks to be uploaded.
            for batch in to_send.chunks_mut(chunk_size) {
                let request_idx = self.requests_sent;
                self.requests_sent += 1;
                for i in batch.iter_mut() {
                    i.outcome = ImageAnalysisOutcome::WaitingForResponse(request_idx);
                }
                request_idxs_to_send.push(request_idx);
            }
        }

        if request_idxs_to_send.is_empty() {
            return false;
        }


        let mut to_send: HashMap<usize, Vec<ImageAnalysisStatus>> = HashMap::new();
        for img in self.images.iter() {
            if let ImageAnalysisOutcome::WaitingForResponse(i) = img.outcome {
                if request_idxs_to_send.contains(&i) {
                    to_send
                        .entry(i)
                        .and_modify(|x| x.push(img.clone()))
                        .or_insert(vec![img.clone()]);
                }
            }
        }

        for (request_idx, out_batch) in to_send {
            ctx.link().send_future(async move {
                let client = reqwest::Client::new();
                let mut body = reqwest::multipart::Form::new();
                for img in out_batch {
                    body = body.part(
                        "f[]",
                        Part::bytes(img.data.data.clone())
                            .file_name(img.data.name.clone())
                            .mime_str(&img.data.file_type)
                            .unwrap(),
                    );
                }

                info!("Sending request {request_idx}");
                let request = client
                    .post("http://10.13.37.252:5000/analyze")
                    .multipart(body)
                    .send()
                    .await;
                let request_outcome = if let Ok(resp) = request {
                    let data: Result<AnalysisResponse, _> = resp.json().await;
                    if let Ok(data) = data {
                        Ok(data)
                    } else {
                        Err(format!(
                            "Data returned was not valid JSON: {}",
                            data.unwrap_err()
                        ))
                    }
                } else {
                    Err(format!(
                        "Error while sending request: {}",
                        request.unwrap_err()
                    ))
                };
                info!("Received response for {request_idx}: {request_outcome:?}");
                ImageAnalysisViewMsg::AnalysisRequestCompleted(request_idx, request_outcome)
            });
        }

        true
    }
}
