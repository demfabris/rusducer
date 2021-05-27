use crate::errors::DispatchError;
use std::{
    ffi::OsString,
    fs::{copy, metadata},
    path::Path,
};

enum Kind {
    Move,
    Copy,
    Delete,
}

pub struct Action {
    files: Vec<OsString>,
    destination: OsString,
    kind: Kind,
}

pub trait Dispatchable {
    fn dispatch(&self);
}

impl Dispatchable for Action {
    fn dispatch(&self) {
        match self.kind {
            Kind::Move => todo!(),
            Kind::Copy => todo!(),
            Kind::Delete => todo!(),
        }
    }
}

fn try_execute<P>(origin: P, destination: P, replace: bool) -> Result<(), DispatchError<P>>
where
    P: AsRef<Path> + Copy,
{
    let destination_exists = metadata(destination).is_ok();
    let origin_exists = metadata(origin).is_ok();

    if !replace && destination_exists {
        Err(DispatchError::OverwriteError(destination))
    } else if !origin_exists {
        Err(DispatchError::FileDoesNotExistsError(origin))
    } else {
        match copy(origin, destination) {
            Err(_) => Err(DispatchError::FsPermissionError(origin, destination)),
            _ => Ok(()),
        }
    }
}
