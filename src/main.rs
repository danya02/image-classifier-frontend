#![feature(option_result_contains)]

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::layout::main_container::Main;
use crate::components::nav::Nav;
use crate::views::image_analysis::ImageAnalysisView;
use crate::views::not_found::NotFound;

mod components;
mod views;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <ImageAnalysisView /> }
        }
        Route::NotFound => {
            html! { <NotFound /> }
        }
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Nav />
            <Main>
                    <Switch<Route> render={switch} />
            </Main>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
