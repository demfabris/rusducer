use std::path::Path;

pub enum DispatchError<P>
where
    P: AsRef<Path>,
{
    OverwriteError(P),
    PermissionError(P),
    FileDoesNotExistsError(P),
    FsPermissionError(P, P),
}
