use yewdux::store::Changed;

#[derive(Clone)]
pub struct Store {
    pub is_true: bool,
}

impl Default for Store {
    fn default() -> Self {
        Self { is_true: false }
    }
}

pub enum Action {
    Toggle,
}

pub fn reduce(store: &mut Store, action: Action) -> Changed {
    match action {
        Action::Toggle => {
            store.is_true = !store.is_true;
            true
        }
    }
}
