use std::path::Path;

pub struct DispatchError {
    kind: ErrorKind,
}

pub enum ErrorKind {
    OverwriteError,
    PermissionError,
    FileDoesNotExistsError,
    FsPermissionError,
}

pub type Result<T> = ::std::result::Result<T, DispatchError>;
