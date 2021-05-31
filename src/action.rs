use crate::config::ActionSpec;

pub enum Kind {
    Move,
    Copy,
}

pub struct Action {
    id: String,
    description: String,
    files: Vec<String>,
    destination: String,
    kind: Kind,
    replace: bool,
}

pub trait Dispatchable {
    fn dispatch(&self) -> fs_extra::error::Result<u64>;
}

impl Dispatchable for Action {
    fn dispatch(&self) -> fs_extra::error::Result<u64> {
        let mut options = fs_extra::dir::CopyOptions::new();
        options.overwrite = self.replace;

        match self.kind {
            Kind::Move => fs_extra::move_items(&self.files[..], &self.destination, &options),
            Kind::Copy => fs_extra::copy_items(&self.files[..], &self.destination, &options),
        }
    }
}

impl From<ActionSpec> for Action {
    fn from(spec: ActionSpec) -> Self {
        // Self {
        //     id: spec.action.unwrap_or(String::from("None given")),
        //     description: spec.description.unwrap_or(String::from("None given")),

        // }
        todo!()
    }
}
