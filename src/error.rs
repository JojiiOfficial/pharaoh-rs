use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PharaohError {
    IndexOutOfBonuds(usize),
    InvalidAlignmentFormat,
}

impl Display for PharaohError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for PharaohError {}
