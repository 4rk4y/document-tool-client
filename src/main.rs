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
}

struct Main {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    pages: Option<Vec<Page>>,
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
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match &self.pages {
            Some(pages) => html! {
              <div>{"Pages:"}{
                pages.iter().map(|page| html! {
                  <div>{"id: "}{&page.id}{", title: "}{&page.title}</div>
                }).collect::<Html>()
              }</div>
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
