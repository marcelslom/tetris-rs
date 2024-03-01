#[derive(Copy, Clone)]
pub enum Rotation {
    _0 = 0,
    R = 1,
    _2 = 2,
    L = 3
}

impl Rotation {

    pub fn next(&self, direction: RotationDirection) -> Self {
        match direction {
            RotationDirection::Clockwise => Self::next_clockwise(*self),
            RotationDirection::CounterClockwise => Self::next_counter_clockwise(*self),
        }
    }

    fn next_clockwise(rotation: Rotation) -> Self {
        match rotation {
            Rotation::_0 => Rotation::R,
            Rotation::R => Rotation::_2,
            Rotation::_2 => Rotation::L,
            Rotation::L => Rotation::_0,
        }
    }

    fn next_counter_clockwise(rotation: Rotation) -> Self {
        match rotation {
            Rotation::_0 => Rotation::L,
            Rotation::R => Rotation::_0,
            Rotation::_2 => Rotation::R,
            Rotation::L => Rotation::_2,
        }
    }
    
}

#[derive(Copy, Clone, PartialEq)]
pub enum RotationDirection {
    Clockwise, CounterClockwise
}