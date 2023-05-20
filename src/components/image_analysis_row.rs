use std::{collections::HashMap, rc::Rc};

use gloo::net::http::Request;
use reqwest::{header::CONTENT_TYPE, multipart::Part};
use yew::{prelude::*, suspense::use_future};

use crate::{
    components::{image_display::ImageDisplayBox, layout::column_layout::IntoColumns},
    views::image_analysis::{ImageAnalysisOutcome, ImageAnalysisStatus},
};

use super::file_upload_box::FileDetails;

#[derive(Properties, PartialEq)]
pub struct AnalysisReportProps {
    pub image: ImageAnalysisStatus,
}

fn placeholder() -> Html {
    html! {
        <table class="table table-striped">
            <thead><tr><th>{"Category"}</th><th>{"Confidence"}</th></tr></thead>
            <tr><th><span class="placeholder col-4 text-bg-secondary"></span></th><td><span class="placeholder col-7 text-bg-secondary"></span></td></tr>
            <tr><th><span class="placeholder col-5 text-bg-secondary"></span></th><td><span class="placeholder col-7 text-bg-secondary"></span></td></tr>
            <tr><th><span class="placeholder col-3 text-bg-secondary"></span></th><td><span class="placeholder col-7 text-bg-secondary"></span></td></tr>
        </table>
    }
}

#[function_component]
pub fn AnalysisReportRow(props: &AnalysisReportProps) -> Html {
    fn get_analysis_result(img: &ImageAnalysisStatus) -> Html {
        match &img.outcome {
            ImageAnalysisOutcome::WaitingToSend => html! {
                <div>
                    <p>{"Waiting to send..."}</p>
                    {placeholder()}
                </div>
            },
            ImageAnalysisOutcome::WaitingForResponse(idx) => html! {
                <div>
                    <p>{format!("Waiting to get response (request index is {idx})...")}</p>
                    {placeholder()}
                </div>
            },
            ImageAnalysisOutcome::Analyzed(res) => {
                let mut category_rows = vec![];
                let mut items: Vec<(&String, &f64)> = res.overall_classification.iter().collect();
                items.sort_by(|(_k1, v1), (_k2, v2)| {
                    v2.partial_cmp(v1).unwrap_or(std::cmp::Ordering::Equal)
                }); // v2 with v1 because want desc order
                let (first, others) = items.split_at(1); // first is [0;1) = [0;0].
                for (k, v) in first {
                    category_rows
                        .push(html!(<tr class="text-bg-success"><th>{k}</th><td>{v}</td></tr>));
                }
                for (k, v) in others {
                    category_rows.push(html!(<tr class=""><th>{k}</th><td>{v}</td></tr>));
                }

                html! {
                    <table class="table table-striped">
                        <thead><tr><th>{"Category"}</th><th>{"Confidence"}</th></tr></thead>
                        <tr><th><span class="placeholder col-4"></span></th><td><span class="placeholder col-6"></span></td></tr>
                        { category_rows }
                    </table>
                }
            }
            ImageAnalysisOutcome::Error(e) => html! {
                <div class="alert alert-danger">{e}</div>
            },
        }
    }

    html! {
        <div class="row">
            <ImageDisplayBox image_data={props.image.data.clone()} class={classes!("col-2")}/>
            <div class="col-10">{get_analysis_result(&props.image)}</div>
        </div>
    }
}
