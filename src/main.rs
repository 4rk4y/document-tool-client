use store::*;
use yew::prelude::*;
use yew_router::components::RouterAnchor;
use yew_router::prelude::Router;
use yew_router::*;
use yewdux::prelude::*;
use yewtil::NeqAssign;

mod element;
mod page;
mod page_list;
mod store;
mod sub_store;

#[derive(Clone, Switch)]
pub enum MainRoute {
    #[to = "/page/{id}"]
    Page(i32),
    #[to = "/"]
    Main,
}

pub type MainRouterAnchor = RouterAnchor<MainRoute>;

struct Main {
    dispatch: StoreDispatchProps,
}

impl Component for Main {
    type Message = ();
    type Properties = StoreDispatchProps;

    fn create(dispatch: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { dispatch }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        self.dispatch.neq_assign(dispatch)
    }

    fn view(&self) -> Html {
        html! {
            <div style="display: flex; justify-content: center;">
                <div style="width: 800px;">
                    <div style="display: flex; justify-content: space-between; font-size: 20px;">
                        <MainRouterAnchor route=MainRoute::Page(1)>{"Main"}</MainRouterAnchor>
                        <MainRouterAnchor route=MainRoute::Main>{"All pages"}</MainRouterAnchor>
                    </div>
                    <Router<MainRoute>
                        render = Router::render(|switch: MainRoute| {
                            match switch {
                                MainRoute::Main => html! {<page_list::PageList/>},
                                MainRoute::Page(id) => html! {
                                    <page::PageWithDispatch
                                        dispatch={DispatchProps::default()}
                                        id={id}
                                    />
                                },
                            }
                        })
                    />
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<WithDispatch<Main>>();
}
