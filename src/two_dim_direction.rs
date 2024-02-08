use std::convert::TryFrom;

use super::Direction;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TwoDimDirection {
    East,
    West,
}

impl TwoDimDirection {
    #[must_use]
    pub fn is_default(self) -> bool {
        self == Self::default()
    }
}

impl Default for TwoDimDirection {
    fn default() -> Self {
        Self::East
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ConvertError {
    North,
    South,
    Here,
}

impl TryFrom<Direction> for TwoDimDirection {
    type Error = ConvertError;

    fn try_from(value: Direction) -> Result<Self, Self::Error> {
        match value {
            Direction::NorthEast | Direction::East | Direction::SouthEast => {
                Ok(TwoDimDirection::East)
            }
            Direction::SouthWest | Direction::West | Direction::NorthWest => {
                Ok(TwoDimDirection::West)
            }
            Direction::North => Err(ConvertError::North),
            Direction::South => Err(ConvertError::South),
            Direction::Here => Err(ConvertError::Here),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::{ConvertError, Direction, TwoDimDirection};

    #[test]
    fn south_east_to_two_dim() {
        let dir = TwoDimDirection::try_from(Direction::SouthEast);
        assert!(matches!(dir, Ok(TwoDimDirection::East)));
    }

    #[test]
    fn west_to_two_dim() {
        let dir = TwoDimDirection::try_from(Direction::West);
        assert!(matches!(dir, Ok(TwoDimDirection::West)));
    }

    #[test]
    fn north_to_two_dim() {
        let dir = TwoDimDirection::try_from(Direction::North);
        assert!(matches!(dir, Err(ConvertError::North)));
    }
}
