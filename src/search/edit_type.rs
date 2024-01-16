#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditType {
    Mutate,
    Insert,
    Delete,
}
