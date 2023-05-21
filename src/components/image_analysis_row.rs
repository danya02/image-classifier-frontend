use yew::prelude::*;

use crate::{
    components::image_display::{get_image_url, ImageDisplayBox},
    views::image_analysis::{ImageAnalysisOutcome, ImageAnalysisStatus},
};

#[derive(Properties, PartialEq)]
pub struct AnalysisReportProps {
    pub image: ImageAnalysisStatus,
    pub on_delete: Callback<MouseEvent>,
    pub on_upload: Callback<MouseEvent>,
}

fn placeholder() -> Html {
    html! {
        <div>
            <h1>{"Predicted label: "}<span class="placeholder col-6 text-success fw-bolder"></span> <div class="spinner-border" role="status"></div></h1>
            <table class="table table-striped">
                <thead><tr><th>{"Category"}</th><th>{"Confidence"}</th></tr></thead>
                <tr><th><span class="placeholder col-4 text-bg-secondary"></span></th><td><span class="placeholder col-7 text-bg-secondary"></span></td></tr>
                <tr><th><span class="placeholder col-5 text-bg-secondary"></span></th><td><span class="placeholder col-7 text-bg-secondary"></span></td></tr>
                <tr><th><span class="placeholder col-3 text-bg-secondary"></span></th><td><span class="placeholder col-7 text-bg-secondary"></span></td></tr>
            </table>
        </div>
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
                let mut max_prob_class = "???";
                for (k, v) in first {
                    category_rows
                        .push(html!(<tr class="text-bg-success"><th>{k}</th><td>{v}</td></tr>));
                    max_prob_class = k;
                    break;
                }
                for (k, v) in others {
                    category_rows.push(html!(<tr class=""><th>{k}</th><td>{v}</td></tr>));
                }

                html! {
                    <div>
                    <h1>{"Predicted label: "}<span class="text-success fw-bolder">{max_prob_class}</span></h1>
                    <table class="table table-striped">
                            <thead><tr><th>{"Category"}</th><th>{"Confidence"}</th></tr></thead>
                            { category_rows }
                        </table>
                    </div>
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
            <div class="col-8">{get_analysis_result(&props.image)}</div>
            <div class="col-2">
                <div class="row row-cols-1">
                    <button class="btn btn-success col mb-2" onclick={&props.on_upload}>{"Upload to Archive"}</button>
                    <button class="btn btn-danger col mb-2" onclick={&props.on_delete}>{"Delete"}</button>
                    <a class="btn btn-primary col mb-2" href={get_image_url(&*props.image.data)} download={props.image.data.name.clone()}>{"Download as file"}</a>
                </div>
            </div>
        </div>
    }
}
