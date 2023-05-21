use log::debug;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::alert::Alert;

#[derive(serde::Deserialize)]
pub struct SearchResult {
    filename: String,
    delete: String,
    download: String,
    uuid: String,
    update: String,
    tags: Vec<String>,
}

pub enum SearchState {
    Results(Vec<SearchResult>),
    Running,
    Error(String),
}

pub struct Search {
    state: SearchState,
    query: String,
}

pub enum SearchMsg {
    SetSearchQuery(String),
    RunSearch,
    RecvResults(Result<Vec<SearchResult>, String>),
}

fn get_text(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

use crate::root_url;

const SEARCH_URL: &str = concat!(root_url!(), "/image");


impl Component for Search {
    type Message = SearchMsg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            state: SearchState::Results(vec![]),
            query: "".to_string(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let results = match &self.state {
            SearchState::Results(items) => {
                let mut imgs = vec![];
                for item in items {
                    imgs.push(html!(
                        <div class="card col">
                            <img class="img-card-top" src={item.download.clone()} />
                            <p class="card-footer">{item.filename.clone()}</p>
                        </div>
                    ));
                }
                html!(
                    <>
                        <p>{format!("Showing {} results", items.len())}</p>
                        <div class="row row-cols-4">
                            {imgs}
                        </div>
                    </>
                )
            }
            SearchState::Running => html! {
                <p>{format!("Searching")}<div class="spinner-border" role="status"></div></p>
            },
            SearchState::Error(why) => html! {
                <Alert style="danger" text={why.clone()} />
            },
        };

        html! {
            <div class="container">
                <div class="row">
                    <div class="col">
                        <div class="input-group mb-3">
                            <input type="text" class="form-control" placeholder="tag1 tag2 tag3 ..." oninput={ctx.link().callback(|ev: InputEvent| SearchMsg::SetSearchQuery(get_text(ev)))}/>
                            <button class="btn btn-outline-primary" type="button" onclick={ctx.link().callback(|_| SearchMsg::RunSearch)}>{"Search"}</button>
                        </div>
                    </div>
                </div>
                <div class="row">
                    {results}
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SearchMsg::SetSearchQuery(s) => {
                debug!("Now searching for {s:?}");
                self.query = s;
                false
            }
            SearchMsg::RunSearch => {
                self.state = SearchState::Running;
                let query = self.query.replace(" ", "+");
                ctx.link().send_future(async move {
                    let client = reqwest::Client::new();

                    let query = urlencoding::encode(&query);
                    let request = client
                        .get(&format!("{SEARCH_URL}?tags={}", query))
                        .send()
                        .await;

                    let result = match request {
                        Ok(resp) => match resp.error_for_status() {
                            Ok(resp) => match resp.json().await {
                                Ok(d) => Ok(d),
                                Err(why) => Err(format!("Error in search JSON: {why}")),
                            },
                            Err(why) => Err(format!("Error status in search: {why}")),
                        },
                        Err(why) => Err(format!("Error sending search request: {why}")),
                    };

                    SearchMsg::RecvResults(result)
                });
                true
            }
            SearchMsg::RecvResults(res) => {
                match res {
                    Ok(ent) => self.state = SearchState::Results(ent),
                    Err(why) => self.state = SearchState::Error(why),
                };
                true
            }
        }
    }
}
