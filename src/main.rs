use anyhow::Error;
use serde::Deserialize;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::{
        fetch::{FetchTask, Response},
        FetchService,
    },
};

enum Msg {
    Request,
    Response(Result<Vec<Page>, Error>),
    Title(String),
    AddPage,
}

struct Main {
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
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::Request);
        Self {
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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let title_callback = self
            .link
            .callback(|event: InputData| Msg::Title(event.value));

        let add_page = self.link.callback(|_| Msg::AddPage);

        match &self.pages {
            Some(pages) => html! {
                <div>
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
                </div>
            },
            None => html! {
              <div>{"No data"}</div>
            },
        }
    }
}

fn main() {
    yew::start_app::<Main>();
}
