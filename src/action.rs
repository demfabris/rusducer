use crate::config::ActionSpec;
use nanoid::nanoid;

pub enum Kind {
    Move,
    Copy,
    Undefined(String),
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
            Kind::Undefined(_) => Err(fs_extra::error::Error::new(
                fs_extra::error::ErrorKind::Interrupted,
                "Undefined action",
            )),
        }
    }
}

impl From<&ActionSpec> for Action {
    fn from(spec: &ActionSpec) -> Self {
        Self {
            id: spec.action.clone().unwrap_or(nanoid!()),
            description: spec.description.unwrap_or(String::from("")),
            files: spec.files.unwrap_or(Vec::new()),
            destination: spec.destination.unwrap_or(String::from("")),
            kind: match spec.action {
                Some(name) if name.contains("COPY") => Kind::Copy,
                Some(name) if name.contains("MOVE") => Kind::Move,
                Some(name) => Kind::Undefined(name),
                None => Kind::Undefined(String::from("")),
            },
            replace: spec.replace.unwrap_or(false),
        }
    }
}
