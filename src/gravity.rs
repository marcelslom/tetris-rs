#[derive(Clone, Copy)]
pub enum Gravity {
    Normal,
    SoftDrop,
    HardDrop,
}

impl Gravity {
    pub fn value(&self) -> f32 {
        match self {
            Gravity::Normal => 1f32 / 64f32,
            Gravity::SoftDrop => 0.7f32,
            Gravity::HardDrop => 20f32,
        }
    }
}