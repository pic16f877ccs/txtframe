/// Text alignment within the frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Algn {
    /// Align left.
    #[default]
    Left,
    /// Align center.
    Centr,
    /// Align right.
    Right,
}
