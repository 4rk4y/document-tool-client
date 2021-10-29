use anyhow::Error;
use serde::Deserialize;
use store::*;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::{
        fetch::{FetchTask, Response},
        FetchService,
    },
};
use yewdux::prelude::*;
use yewtil::NeqAssign;

mod page;
mod store;
mod sub_store;

enum Msg {
    Request,
    Response(Result<Vec<Page>, Error>),
    Title(String),
    AddPage,
}

struct Main {
    dispatch: StoreDispatchProps,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    pages: Option<Vec<Page>>,
    title: String,
}

#[derive(Deserialize)]
struct Page {
    id: i32,
    title: String,
}

impl Component for Main {
    type Message = Msg;
    type Properties = StoreDispatchProps;

    fn create(dispatch: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::Request);
        Self {
            dispatch,
            link,
            fetch_task: None,
            pages: None,
            title: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Request => {
                self.fetch_task = Some(
                    FetchService::fetch(
                        yew::services::fetch::Request::get("http://127.0.0.1:8000")
                            .body(Nothing)
                            .unwrap(),
                        self.link
                            .callback(|response: Response<Json<Result<Vec<Page>, Error>>>| {
                                let Json(body) = response.into_body();
                                Msg::Response(body)
                            }),
                    )
                    .unwrap(),
                );
            }
            Msg::Response(body) => {
                if let Ok(body) = body {
                    self.pages = Some(body)
                }
            }
            Msg::Title(value) => {
                self.title = value;
            }
            Msg::AddPage => {
                self.fetch_task = Some(
                    FetchService::fetch(
                        yew::services::fetch::Request::post(format!(
                            "http://127.0.0.1:8000?_title={}",
                            self.title
                        ))
                        .body(Nothing)
                        .unwrap(),
                        self.link
                            .callback(|_response: Response<Result<String, Error>>| Msg::Request),
                    )
                    .unwrap(),
                );
            }
        }
        true
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        self.dispatch.neq_assign(dispatch)
    }

    fn view(&self) -> Html {
        let title_callback = self
            .link
            .callback(|event: InputData| Msg::Title(event.value));

        let add_page = self.link.callback(|_| Msg::AddPage);

        let is_true = self.dispatch.state().sub_store.is_true;
        let toggle = self
            .dispatch
            .callback(|_| Action::SubStoreAction(sub_store::Action::Toggle));

        match &self.pages {
            Some(pages) => html! {
                <div>
                    {is_true}
                    <br/>
                    <button onclick=toggle>{"Store test: toggle"}</button>
                    <br/><br/>
                    <form>
                        <label for="title">{"Title: "}</label>
                        <input
                            id="title"
                            type="text"
                            value={self.title.clone()}
                            oninput=title_callback
                        />
                    </form>
                    <button onclick=add_page>{"Add page"}</button>
                    <br/><br/>
                    <div>
                        {"Pages:"}{
                            pages.iter().map(|page| html! {
                                <div>{"id: "}{&page.id}{", title: "}{&page.title}</div>
                            }).collect::<Html>()
                        }
                    </div>
                    <br/>
                    <div>
                        {"Page:"}{
                            match pages.first() {
                                Some(page) => html! {
                                    <page::PageWithDispatch
                                        dispatch={DispatchProps::default()}
                                        id={page.id}
                                    />
                                },
                                None => html! {
                                    <div>{"No data"}</div>
                                },
                            }
                        }
                    </div>
                </div>
            },
            None => html! {
              <div>{"No data"}</div>
            },
        }
    }
}

fn main() {
    yew::start_app::<WithDispatch<Main>>();
}
