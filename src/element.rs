use yew::prelude::*;

enum DataType {
    Image = 0,
    Link,
    Text,
}

impl TryFrom<i32> for DataType {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            value if value == 0 => Ok(Self::Image),
            value if value == 1 => Ok(Self::Link),
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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                {format!("id: {},", self.props.id)}<br/>
                {format!("page_id: {},", self.props.page_id)}<br/>
                {format!("width: {},", self.props.width)}<br/>
                {format!("height: {},", self.props.height)}<br/>
                {format!("top: {},", self.props.top)}<br/>
                {format!("right: {},", self.props.right)}<br/>
                {format!("bottom: {},", self.props.bottom)}<br/>
                {format!("left: {},", self.props.left)}<br/>
                {format!("align: {},", self.props.align)}<br/>
                {format!("data_type: {},", self.props.data_type)}<br/>
                {format!("Data type: {}",
                    match DataType::try_from(self.props.data_type) {
                        Ok(DataType::Image) => "image",
                        Ok(DataType::Link) => "link",
                        Ok(DataType::Text) => "text",
                        Err(()) => "no such data type",
                    })
                }<br/>
                {format!("data: {},", self.props.data)}<br/><br/>
            </div>
        }
    }
}
