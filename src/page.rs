use super::element;
use super::store::*;
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
use yewdux::prelude::*;
use yewtil::NeqAssign;

pub enum Msg {
    Request,
    Response(Result<PageDetails, Error>),
}

#[derive(Deserialize)]
pub struct PageDetails {
    id: i32,
    title: String,
    elements: Vec<Element>,
}

#[derive(Deserialize)]
struct Element {
    id: i32,
    page_id: i32,
    width: f32,
    height: f32,
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
    align: i32,
    data_type: i32,
    data: String,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub dispatch: StoreDispatchProps,
    pub id: i32,
}

impl DispatchPropsMut for Props {
    type Store = Storage;

    fn dispatch(&mut self) -> &mut DispatchProps<Self::Store> {
        &mut self.dispatch
    }
}

pub type PageWithDispatch = WithDispatch<Page>;

pub struct Page {
    props: Props,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    page_details: Option<PageDetails>,
}

impl Component for Page {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::Request);
        Self {
            props,
            link,
            fetch_task: None,
            page_details: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Request => {
                self.fetch_task = Some(
                    FetchService::fetch(
                        yew::services::fetch::Request::get(format!(
                            "http://127.0.0.1:8000/page?_id={}",
                            self.props.id
                        ))
                        .body(Nothing)
                        .unwrap(),
                        self.link.callback(
                            |response: Response<Json<Result<PageDetails, Error>>>| {
                                let Json(body) = response.into_body();
                                Msg::Response(body)
                            },
                        ),
                    )
                    .unwrap(),
                );
            }
            Msg::Response(body) => {
                if let Ok(body) = body {
                    self.page_details = Some(body)
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.id != props.id {
            self.props.id = props.id;
            self.link.send_message(Msg::Request);
        }
        self.props.dispatch.neq_assign(props.dispatch)
    }

    fn view(&self) -> Html {
        match &self.page_details {
            Some(page_details) => html! {
                <div>
                    {"Page:"}<br/>
                    {format!("id: {}, title: {}", page_details.id, page_details.title)}<br/><br/>
                    {
                        page_details.elements.iter().map(|element| html! {
                            <element::Element
                                id={element.id}
                                page_id={element.page_id}
                                width={element.width}
                                height={element.height}
                                top={element.top}
                                right={element.right}
                                bottom={element.bottom}
                                left={element.left}
                                align={element.align}
                                data_type={element.data_type}
                                data={element.data.to_string()}
                            />
                        }).collect::<Html>()
                    }
                </div>
            },
            None => html! {
              <div>{"No data"}</div>
            },
        }
    }
}
