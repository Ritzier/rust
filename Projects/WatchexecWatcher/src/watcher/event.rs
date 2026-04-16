#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
    ConfigCreate,
    ConfigModify,
    ConfigRemove,
    FileCreate,
    FileModify,
    FileRemove,
    ConfigFileModify,
}
