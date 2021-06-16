use crate::{
    action::{Action, Dispatchable},
    config::ActionSpec,
};

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

    pub fn with_actions(&mut self, action_specs: Option<&Vec<ActionSpec>>) {
        if let Some(actions) = &action_specs {
            let mut parsed_actions: Vec<Action> = Vec::new();

            for action in *actions {
                parsed_actions.push(action.into());
            }

            self.actions = Some(parsed_actions);
        } else {
            self.actions = None;
            println!("No action found");
        }
    }

    pub fn try_dispatch(&self) -> fs_extra::error::Result<u64> {
        if self.dry {
            return Ok(0);
        }

        if let Some(actions) = self.actions.clone() {
            for action in actions {
                action.dispatch()?;
            }
        }

        Ok(0)
    }
}
