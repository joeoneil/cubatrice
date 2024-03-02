use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{state::player::PlayerID, Fraction};

/// Transparent type for cube IDs
#[derive(
    Clone, Default, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub struct CubeID(pub usize);

/// Different types of cube. Some cubes exist only virtually, as inputs or
/// outputs on cards. Physical cubes that players can own can only be of
/// certain types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CubeType {
    /// Ships
    Ship,
    /// White Cubes
    Culture,
    /// Green Cubes
    Food,
    /// Brown Cubes
    Industry,
    /// Small Gray Cubes
    UnitySmall,
    /// Any Small Cube (including unity)
    AnySmall,
    /// Any Small Cube (except unity)
    AnySmallNonUnity,
    /// Yellow Cubes
    Power,
    /// Blue Cubes
    Biotech,
    /// Black Cubes
    Information,
    /// Large Gray Cubes
    UnityLarge,
    /// Any Large Cube (including unity)
    AnyLarge,
    /// Any Large Cube (except unity)
    AnyLargeNonUnity,
    /// Honey
    Ultratech,
    /// Victory points.
    VictoryPoint,
}

impl CubeType {
    /// Gets the suggested raw value of this cube type. Perceived value of
    /// cubes may change based on supply and demand.
    pub fn value(&self) -> Fraction {
        match *self {
            CubeType::Culture
            | CubeType::Food
            | CubeType::Industry
            | CubeType::Ship
            | CubeType::UnitySmall
            | CubeType::AnySmall
            | CubeType::AnySmallNonUnity => Fraction::new(1, 1),
            CubeType::Power
            | CubeType::Biotech
            | CubeType::Information
            | CubeType::UnityLarge
            | CubeType::AnyLarge
            | CubeType::AnyLargeNonUnity => Fraction::new(3, 2),
            CubeType::Ultratech | CubeType::VictoryPoint => Fraction::new(3, 1),
        }
    }

    /// Checks if a cube type is a 'virtual cube'. Virtual cubes can only exist
    /// as inputs or outputs of converters, and should never be instantiated.
    pub fn is_virtual(&self) -> bool {
        match *self {
            CubeType::Ship
            | CubeType::Culture
            | CubeType::Food
            | CubeType::Industry
            | CubeType::UnitySmall
            | CubeType::Power
            | CubeType::Biotech
            | CubeType::Information
            | CubeType::UnityLarge
            | CubeType::Ultratech
            | CubeType::VictoryPoint => false,
            CubeType::AnySmall
            | CubeType::AnySmallNonUnity
            | CubeType::AnyLarge
            | CubeType::AnyLargeNonUnity => true,
        }
    }

    /// checks if rhs is a valid cube if self is the input of a converter
    pub fn matches(self, rhs: Self) -> bool {
        if self == rhs {
            true
        } else {
            match self {
                CubeType::Culture | CubeType::Food | CubeType::Industry => {
                    rhs == CubeType::UnitySmall
                }
                CubeType::AnySmallNonUnity => {
                    rhs == CubeType::Culture || rhs == CubeType::Food || rhs == CubeType::Industry
                }
                CubeType::AnySmall => {
                    rhs == CubeType::Culture
                        || rhs == CubeType::Food
                        || rhs == CubeType::Industry
                        || rhs == CubeType::UnitySmall
                }
                CubeType::Power | CubeType::Biotech | CubeType::Information => {
                    rhs == CubeType::UnityLarge
                }
                CubeType::AnyLargeNonUnity => {
                    rhs == CubeType::Power
                        || rhs == CubeType::Biotech
                        || rhs == CubeType::Information
                }
                CubeType::AnyLarge => {
                    rhs == CubeType::Power
                        || rhs == CubeType::Biotech
                        || rhs == CubeType::Information
                        || rhs == CubeType::UnityLarge
                }
                _ => false,
            }
        }
    }
}

/// represents an individual cube. We want to keep track of IDs so that cubes
/// can be traced through the whole economy later.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Cube {
    /// what type of cube this cube is
    pub typ: CubeType,
    /// Whether this cube was originally a donation and who produced it
    /// (only matters for alt kjas)
    pub donation: Option<PlayerID>,
}

impl Cube {
    /// Create a new cube of a given type. ID should be unique per cube
    /// if individual tracking is desired, but this is not enforced.
    pub fn new(typ: CubeType, donation: Option<PlayerID>) -> Self {
        Cube { typ, donation }
    }

    pub fn value(&self) -> Fraction {
        self.typ.value()
    }
}

/// A Record of some number of cubes
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CubeRecord {
    food: isize,
    culture: isize,
    industry: isize,
    small_wild: isize,
    biotech: isize,
    power: isize,
    information: isize,
    large_wild: isize,
    ultratech: isize,
    ships: isize,
    points: isize,
}

impl CubeRecord {
    pub fn split(self) -> (Self, Self) {
        (self.clone().gt_zero(), (-self).gt_zero())
    }

    fn gt_zero(self) -> Self {
        Self {
            food: self.food.max(0),
            culture: self.culture.max(0),
            industry: self.industry.max(0),
            small_wild: self.small_wild.max(0),
            biotech: self.biotech.max(0),
            power: self.power.max(0),
            information: self.information.max(0),
            large_wild: self.large_wild.max(0),
            ultratech: self.ultratech.max(0),
            ships: self.ships.max(0),
            points: self.points.max(0),
        }
    }

    fn value(&self) -> Fraction {
        Fraction::new(1, 1)
            * (self.food + self.culture + self.industry + self.small_wild + self.ships)
            + Fraction::new(3, 2) * (self.biotech + self.power + self.information + self.large_wild)
            + Fraction::new(3, 1) * (self.ultratech + self.points)
    }

    fn vp_value(&self) -> Fraction {
        Fraction::new(1, 6)
            * (self.food + self.culture + self.industry + self.small_wild + self.ships)
            + Fraction::new(1, 4) * (self.biotech + self.power + self.information + self.large_wild)
            + Fraction::new(1, 2) * (self.ultratech)
            + Fraction::new(1, 1) * (self.points)
    }

    pub fn count_type(&self, typ: CubeType) -> isize {
        match typ {
            CubeType::Ship => self.ships,
            CubeType::Food => self.food,
            CubeType::Culture => self.culture,
            CubeType::Industry => self.industry,
            CubeType::UnitySmall => self.small_wild,
            CubeType::AnySmall => self.food + self.culture + self.industry + self.small_wild,
            CubeType::AnySmallNonUnity => self.food + self.culture + self.industry,
            CubeType::Power => self.power,
            CubeType::Information => self.information,
            CubeType::Biotech => self.biotech,
            CubeType::UnityLarge => self.large_wild,
            CubeType::AnyLarge => self.power + self.information + self.biotech + self.large_wild,
            CubeType::AnyLargeNonUnity => self.power + self.information + self.biotech,
            CubeType::Ultratech => self.ultratech,
            CubeType::VictoryPoint => self.points,
        }
    }
}

impl PartialOrd for CubeRecord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value().cmp(&other.value()))
    }
}

impl std::ops::Neg for CubeRecord {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            food: -self.food,
            culture: -self.culture,
            industry: -self.industry,
            small_wild: -self.small_wild,
            biotech: -self.biotech,
            power: -self.power,
            information: -self.information,
            large_wild: -self.large_wild,
            ultratech: -self.ultratech,
            ships: -self.ships,
            points: -self.points,
        }
    }
}

impl From<&[Cube]> for CubeRecord {
    fn from(value: &[Cube]) -> Self {
        value.iter().collect()
    }
}

impl<'a> FromIterator<&'a Cube> for CubeRecord {
    fn from_iter<T: IntoIterator<Item = &'a Cube>>(iter: T) -> Self {
        let mut s = Self::default();
        for v in iter {
            match v.typ {
                CubeType::Ship => s.ships += 1,
                CubeType::Culture => s.culture += 1,
                CubeType::Food => s.food += 1,
                CubeType::Industry => s.industry += 1,
                CubeType::UnitySmall => s.small_wild += 1,
                CubeType::Biotech => s.biotech += 1,
                CubeType::Power => s.power += 1,
                CubeType::Information => s.information += 1,
                CubeType::UnityLarge => s.large_wild += 1,
                CubeType::VictoryPoint => s.points += 1,
                _ => {}
            }
        }
        s
    }
}

impl Display for CubeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (color, text) = match *self {
            Self::Ship => ("31", "Ship"),
            Self::Culture => ("37", "Culture"),
            Self::Food => ("32", "Food"),
            Self::Industry => ("33", "Industry"),
            Self::UnitySmall => ("90", "Small Wild"),
            Self::AnySmall => ("35", "Any Small"),
            Self::AnySmallNonUnity => ("35", "Any Small (Zeth)"),
            Self::Power => ("93", "Power"),
            Self::Biotech => ("94", "Biotech"),
            Self::Information => ("97", "Information"),
            Self::UnityLarge => ("90", "Large Wild"),
            Self::AnyLarge => ("35", "Any Large"),
            Self::AnyLargeNonUnity => ("35", "Any Large (Zeth)"),
            Self::Ultratech => ("33", "Ultratech"),
            Self::VictoryPoint => ("35", "Victory Point"),
        };
        write!(f, "\x1b[{}m{}\x1b[0m", color, text)
    }
}
