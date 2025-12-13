
#[derive(Copy, Clone, Debug)]
pub enum Axis { X, Y }

impl Axis {
    pub fn other(&self) -> Self {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::X
        }
    }
}
