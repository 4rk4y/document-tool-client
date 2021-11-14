use super::MainRoute;
use super::MainRouterAnchor;
use yew::prelude::*;

enum DataType {
    Image = 0,
    PageRoute,
    Text,
}

impl TryFrom<i32> for DataType {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            value if value == 0 => Ok(Self::Image),
            value if value == 1 => Ok(Self::PageRoute),
            value if value == 2 => Ok(Self::Text),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Properties)]
pub struct Props {
    pub id: i32,
    pub page_id: i32,
    pub width: f32,
    pub height: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
    pub align: i32,
    pub data_type: i32,
    pub data: String,
}

pub struct Element {
    props: Props,
}

impl Component for Element {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.id != props.id {
            self.props = props;
            return true;
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            match DataType::try_from(self.props.data_type) {
                Ok(DataType::Image) => html! {<>{"image"}</>},
                Ok(DataType::PageRoute) => {
                    let mut data = self.props.data.split(",");

                    let id: &str = match data.next() {
                        Some(id) => id,
                        None => return html! {},
                    };

                    let id: i32 = match id.parse() {
                        Ok(id) => id,
                        Err(_) => return html! {},
                    };

                    let title: &str = match data.next() {
                        Some(title) => title,
                        None => return html! {},
                    };

                    html! {
                        <div>
                            <MainRouterAnchor route=MainRoute::Page(id)>{title}</MainRouterAnchor>
                        </div>
                    }
                },
                Ok(DataType::Text) => {
                    let width = match self.props.width {
                        0.0 => "".to_string(),
                        width => format!("width: {}px;", width),
                    };

                    let height = match self.props.height {
                        0.0 => "".to_string(),
                        height => format!("height: {}px;", height),
                    };

                    let top = match self.props.top {
                        0.0 => "".to_string(),
                        top => format!("margin-top: {}px;", top),
                    };
                    let right = match self.props.right {
                        0.0 => "".to_string(),
                        right => format!("margin-right: {}px;", right),
                    };

                    let bottom = match self.props.bottom {
                        0.0 => "".to_string(),
                        bottom => format!("margin-bottom: {}px;", bottom),
                    };

                    let left = match self.props.left {
                        0.0 => "".to_string(),
                        left => format!("margin-left: {}px;", left),
                    };

                    let align = format!(
                        "text-align: {};",
                        match self.props.align {
                            0 => "center",
                            1 => "inherit",
                            2 => "justify",
                            3 => "left",
                            4 => "right",
                            _ => "",
                        }
                    );

                    html! {
                        <div style=format!("{}{}{}{}{}{}{}",
                            width,
                            height,
                            top,
                            right,
                            bottom,
                            left,
                            align,
                        )>
                            {self.props.data.to_string()}
                        </div>
                    }
                },
                Err(()) => html! {<>{"no such data type"}</>},
            }
        }
    }
}
