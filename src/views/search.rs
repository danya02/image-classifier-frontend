use yew::prelude::*;

#[function_component]
pub fn Search() -> Html {
    let run_search = Callback::from(|_| {});

    html! {
        <div class="container">
            <div class="row">
                <div class="col">
                    <div class="input-group mb-3">
                        <input type="text" class="form-control" placeholder="tag1 tag2 tag3 ..." />
                        <button class="btn btn-outline-primary" type="button" onclick={run_search}>{"Search"}</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
