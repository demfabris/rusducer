use crate::action::Action;

pub struct Worker {
    pub dry: bool,
    pub silent: bool,
    pub delay: u32,
    pub actions: Option<Vec<Action>>,
}

impl Worker {
    pub fn new() -> Self {
        Self {
            dry: false,
            silent: false,
            delay: 0,
            actions: None,
        }
    }
}
