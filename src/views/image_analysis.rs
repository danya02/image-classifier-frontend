use std::{collections::HashMap, rc::Rc};

use base64::Engine;
use gloo::timers::callback::Interval;
use log::{debug, info};
use reqwest::multipart::Part;
use yew::prelude::*;

use crate::components::{
    alert::Alert,
    file_upload_box::{FileDetails, FileUploadBox},
    image_analysis_row::AnalysisReportRow,
};

use crate::root_url;

//const ANALYSIS_URL: &str = "http://localhost:5000/analyze";
const ANALYSIS_URL: &str = concat!(root_url!(), "/analyze");


const UPLOAD_URL: &str = concat!(root_url!(), "/save");

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
    #[serde(rename = "waiting_to_send")]
    WaitingToSend,
    #[serde(rename = "waiting_for_response")]
    WaitingForResponse(RequestId),
    #[serde(rename = "analyzed")]
    Analyzed(ImageAnalysisData),
    #[serde(rename = "error")]
    Error(String),
}

pub struct ImageAnalysisView {
    requests_sent: usize,
    images: Vec<ImageAnalysisStatus>,
    _clock_handle: Interval,
    uploading: Vec<Rc<FileDetails>>,
    alerts: Vec<Html>,
}

#[derive(Clone)]
pub enum ImageAnalysisViewMsg {
    NewImageUploaded(FileDetails),
    TimerTick,
    AnalysisRequestCompleted(usize, Result<HashMap<String, ImageAnalysisData>, String>),
    DeleteImageRow(ImageAnalysisStatus),
    StartUploadAll,
    StartUpload(Rc<FileDetails>, ImageAnalysisOutcome),
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
            let on_upload = ctx.link().callback(move |_i| {
                ImageAnalysisViewMsg::StartUpload(img.clone().data, img.clone().outcome)
            });

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
                <p class="mb-3">{format!("Uploading {} images...", self.uploading.len())}<div class="spinner-border" role="status"></div></p>
            )
        };

        let mut json_labels = HashMap::new();
        let mut csv_labels = String::from("name;class\n");
        for img in self.images.iter() {
            json_labels.insert(img.data.name.clone(), img.outcome.clone());
            let label = match &img.outcome {
                ImageAnalysisOutcome::Analyzed(res) => {
                    let max = res
                        .overall_classification
                        .iter()
                        .fold(("unknown", 0.0f64), |(ok, ov), (nk, nv)| {
                            if nv > &ov {
                                (nk, *nv)
                            } else {
                                (ok, ov)
                            }
                        })
                        .0;
                    max.to_string()
                }
                _ => "unknown".to_string(),
            };
            csv_labels.extend(format!("{};{}\n", img.data.name, label).chars());
        }

        let json_labels = serde_json::to_string(&json_labels).unwrap();
        let json_labels = format!(
            "data:application/json;base64,{}",
            base64::engine::general_purpose::STANDARD.encode(json_labels)
        );
        let csv_labels = format!(
            "data:text/csv;base64,{}",
            base64::engine::general_purpose::STANDARD.encode(csv_labels)
        );

        html! {
        <div>
            <FileUploadBox {on_image} />
            {uploading_state}
            <div class="row mb-3">
                <a href={json_labels} download="labels.json" class="btn btn-success col mx-2">{"Export all as JSON"}</a>
                <a href={csv_labels} download="labels.csv" class="btn btn-primary col mx-2">{"Export labels only as CSV"}</a>
            </div>
            <div class="row mb-3">
                <button class="btn btn-warning col mx-2" onclick={ctx.link().callback(|_| ImageAnalysisViewMsg::StartUploadAll)}>{"Upload all to archive"}</button>
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
            ImageAnalysisViewMsg::StartUpload(imgdata, imgoutcome) => {
                self.uploading.push(imgdata.clone());
                let max_tag = if let ImageAnalysisOutcome::Analyzed(res) = &imgoutcome {
                    res.overall_classification
                        .iter()
                        .fold(("unknown", 0.0f64), |(ok, ov), (nk, nv)| {
                            if nv > &ov {
                                (nk, *nv)
                            } else {
                                (ok, ov)
                            }
                        })
                        .0
                        .to_string()
                } else {
                    "unknown_tag".to_string()
                };

                ctx.link().send_future(async move {
                    let client = reqwest::Client::new();
                    let body = reqwest::multipart::Form::new()
                        .part(
                            "f[]",
                            Part::bytes((&imgdata).data.clone())
                                .file_name((&imgdata).name.clone())
                                .mime_str(&imgdata.file_type)
                                .unwrap(),
                        )
                        .part("tags", Part::text(max_tag).file_name(""))
                        .part(
                            "analysis",
                            Part::text(serde_json::to_string(&imgoutcome).unwrap()),
                        );

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
            ImageAnalysisViewMsg::StartUploadAll => {
                info!("Uploading all!!");
                let msgs: Vec<ImageAnalysisViewMsg> = self.images.iter().map(|i: &ImageAnalysisStatus| ImageAnalysisViewMsg::StartUpload(i.data.clone(), i.outcome.clone())).collect();
                ctx.link().send_message_batch(msgs);
                false
            }
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
        let chunk_size = 1;

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
