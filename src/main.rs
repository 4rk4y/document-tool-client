use store::*;
use yew::prelude::*;
use yewdux::prelude::*;
use yewtil::NeqAssign;

mod page;
mod page_list;
mod store;
mod sub_store;

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
        let is_true = self.dispatch.state().sub_store.is_true;
        let toggle = self
            .dispatch
            .callback(|_| Action::SubStoreAction(sub_store::Action::Toggle));

        html! {
            <div>
                {is_true}
                <br/>
                <button onclick=toggle>{"Store test: toggle"}</button>
                <br/><br/>
                <page_list::PageList/>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<WithDispatch<Main>>();
}
