use yewdux::prelude::*;

pub type StoreDispatchProps = DispatchProps<ReducerStore<Store>>;

#[derive(Clone)]
pub struct Store {
    pub is_true: bool,
}

pub enum Action {
    Toggle,
}

impl Reducer for Store {
    type Action = Action;

    fn new() -> Self {
        Self { is_true: false }
    }

    fn reduce(&mut self, action: Self::Action) -> Changed {
        match action {
            Action::Toggle => {
                self.is_true = !self.is_true;
                true
            }
        }
    }
}
