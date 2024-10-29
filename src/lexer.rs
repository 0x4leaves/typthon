#[derive(Debug, Clone, Copy)]
pub(crate) struct Position {
    pub(crate) line: usize,
    pub(crate) col: usize,
    pub(crate) byte_offset: usize,
}
