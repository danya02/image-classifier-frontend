use std::{collections::HashMap, rc::Rc};

use base64::Engine;
use gloo::timers::callback::Interval;
use log::{debug, info};
use reqwest::{multipart::Part, header::HeaderMap};
use yew::prelude::*;

use crate::components::{
    file_upload_box::{FileDetails, FileUploadBox},
    image_analysis_row::AnalysisReportRow, alert::Alert,
};

//const ANALYSIS_URL: &str = "http://localhost:5000/analyze";
const ANALYSIS_URL: &str = "http://10.13.37.252:5000/analyze";

const UPLOAD_URL: &str = "http://10.13.37.252:5000/save";

#[derive(serde::Deserialize, Debug, Clone, PartialEq, serde::Serialize)]
pub struct ImageAnalysisData {
    #[serde(rename = "overall_class")]
    pub overall_classification: HashMap<String, f64>,
}

#[derive(Clone, PartialEq)]
pub struct ImageAnalysisStatus {
    pub data: Rc<FileDetails>,
    pub outcome: ImageAnalysisOutcome,
}

type RequestId = usize;

pub type AnalysisResponse = HashMap<String, ImageAnalysisData>;

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
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
    uploading: Vec<Rc<FileDetails>>,
    alerts: Vec<Html>,
}

pub enum ImageAnalysisViewMsg {
    NewImageUploaded(FileDetails),
    TimerTick,
    AnalysisRequestCompleted(usize, Result<HashMap<String, ImageAnalysisData>, String>),
    DeleteImageRow(ImageAnalysisStatus),
    StartUpload(Rc<FileDetails>),
    FinishUpload(Rc<FileDetails>, Result<(), String>),
}

impl Component for ImageAnalysisView {
    type Message = ImageAnalysisViewMsg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let _clock_handle = {
            let link = ctx.link().clone();
            Interval::new(1000, move || {
                link.send_message(ImageAnalysisViewMsg::TimerTick)
            })
        };
        let s = Self {
            requests_sent: 0,
            images: vec![],
            _clock_handle,
            uploading: vec![],
            alerts: vec![],
        };

        s
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut image_rows = vec![];
        for image in self.images.iter() {
            let img = image.clone();
            let on_delete = ctx
                .link()
                .callback(move |_i| ImageAnalysisViewMsg::DeleteImageRow(img.clone()));
            let img = image.clone();
            let on_upload = ctx
                .link()
                .callback(move |_i| ImageAnalysisViewMsg::StartUpload(img.clone().data));

            image_rows.push(html!(
                <AnalysisReportRow image={image.clone()} {on_delete} {on_upload}/>
            ));
        }

        let on_image = ctx
            .link()
            .callback(|i| ImageAnalysisViewMsg::NewImageUploaded(i));

        let uploading_state = if self.uploading.is_empty() {
            html!()
        } else {
            html!(
                <p>{format!("Uploading {} images...", self.uploading.len())}<div class="spinner-border" role="status"></div></p>
            )
        };

        let mut json_labels = HashMap::new();
        let mut csv_labels = String::new();
        for img in self.images.iter() {
            json_labels.insert(img.data.name.clone(), img.outcome.clone());
            let label = match &img.outcome {
                ImageAnalysisOutcome::Analyzed(res) => {
                    let max = res.overall_classification.iter().fold( ("unknown", 0.0f64), |(ok,ov), (nk,nv)| {if nv > &ov {(nk,*nv)} else {(ok,ov)}} ).0;
                    max.to_string()
                },
                _ => "unknown".to_string()
            };
            csv_labels.extend(format!("{};{}\n", img.data.name, label).chars());
        }

        let json_labels = serde_json::to_string(&json_labels).unwrap();
        let json_labels = format!("data:application/json;base64,{}", base64::engine::general_purpose::STANDARD.encode(json_labels));
        let csv_labels = format!("data:text/csv;base64,{}", base64::engine::general_purpose::STANDARD.encode(csv_labels));

        html! {
        <div>
            <FileUploadBox {on_image} />
            {uploading_state}
            <div class="row">
                <a href={json_labels} download="labels.json" class="btn btn-success col mx-2">{"Export all as JSON"}</a>
                <a href={csv_labels} download="labels.csv" class="btn btn-primary col mx-2">{"Export labels only as CSV"}</a>
            </div>
            {for self.alerts.clone()}
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
            }
            ImageAnalysisViewMsg::TimerTick => self.collect_pending(ctx),
            ImageAnalysisViewMsg::AnalysisRequestCompleted(idx, data) => {
                self.analysis_request_completed(idx, data);
                true
            }
            ImageAnalysisViewMsg::DeleteImageRow(img) => {
                self.images.retain(|f| f != &img);
                true
            }
            ImageAnalysisViewMsg::StartUpload(imgdata) => {
                self.uploading.push(imgdata.clone());
                ctx.link().send_future(async move {
                    let client = reqwest::Client::new();
                    let body = reqwest::multipart::Form::new().part(
                        "f[]",
                        Part::bytes((&imgdata).data.clone())
                            .file_name((&imgdata).name.clone())
                            .mime_str(&imgdata.file_type)
                            .unwrap(),
                    ).part("tags", Part::text("test").file_name(""));

                    let request = client.post(UPLOAD_URL).multipart(body).send().await;

                    let result = match request {
                        Ok(resp) => match resp.error_for_status() {
                            Ok(_resp) => Ok(()),
                            Err(why) => Err(format!("Error status in upload: {why}")),
                        },
                        Err(why) => Err(format!("Error sending upload request: {why}")),
                    };

                    ImageAnalysisViewMsg::FinishUpload(imgdata, result)
                });
                true
            }
            ImageAnalysisViewMsg::FinishUpload(imgdata, res) => {
                self.uploading.retain(|x| x != &imgdata);
                match res {
                    Ok(_) => {
                        self.alerts.push(html!(
                            <Alert style="success" text={format!("Successfully uploaded {}!", imgdata.name)} />
                        ))
                    },
                    Err(why) => {
                        self.alerts.push(html!(
                            <Alert style="danger" text={format!("Failed to upload {}: {why}", imgdata.name)} />
                        ))
                    },
                };
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
                let request = client.post(ANALYSIS_URL).multipart(body).send().await;
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
