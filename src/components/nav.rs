use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component]
pub fn Nav() -> Html {
    let current_route: Option<Route> = use_route();
    let link_to = |whence: Route, name: &str| -> Html {
        let is_current_route = current_route.contains(&whence);
        if is_current_route {
            html! {
              <Link<Route> classes="nav-link active" to={whence}>{name}</Link<Route>>
            }
        } else {
            html! {
              <Link<Route> classes="nav-link" to={whence}>{name}</Link<Route>>
            }
        }
    };

    html! {
      <nav class="navbar navbar-expand-lg bg-body-tertiary">
        <div class="container-fluid">
          <Link<Route> classes="navbar-brand" to={Route::Home}>{"Image classifier frontend"}</Link<Route>>
          <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav" aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
            <span class="navbar-toggler-icon"></span>
          </button>
          <div class="collapse navbar-collapse" id="navbarNav">
            <div class="navbar-nav">
            {link_to(Route::Home, "Home")}
            {link_to(Route::Search, "Search")}

            </div>
          </div>
        </div>
      </nav>
    }
}
