#[derive(Clone, Copy, Debug, Default)]
pub enum Alignment {
    #[default]
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub struct RowSettings {
    pub spacing: usize,
}
