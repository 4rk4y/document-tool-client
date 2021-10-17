use super::sub_store;
use yewdux::prelude::*;

pub type StoreDispatchProps = DispatchProps<ReducerStore<Store>>;

#[derive(Clone)]
pub struct Store {
    pub sub_store: sub_store::Store,
}

pub enum Action {
    SubStoreAction(sub_store::Action),
}

impl Reducer for Store {
    type Action = Action;

    fn new() -> Self {
        Self {
            sub_store: sub_store::Store::default(),
        }
    }

    fn reduce(&mut self, action: Self::Action) -> Changed {
        match action {
            Action::SubStoreAction(action) => sub_store::reduce(&mut self.sub_store, action),
        }
    }
}
