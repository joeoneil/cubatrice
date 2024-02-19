use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{state::player::PlayerID, Fraction};

/// Transparent type for cube IDs
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
            CubeType::Ultratech => Fraction::new(3, 1),
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
            | CubeType::Ultratech => false,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cube {
    /// unique id for this cube
    pub id: CubeID,
    /// what type of cube this cube is
    pub typ: CubeType,
    pub owner: PlayerID,
    /// Whether this cube was a donation and who produced it
    /// (only matters for alt kjas)
    pub donation: Option<PlayerID>,
}

impl Cube {
    /// Create a new cube of a given type. ID should be unique per cube
    /// if individual tracking is desired, but this is not enforced.
    pub fn new(typ: CubeType, owner: PlayerID, donation: bool, id: CubeID) -> Self {
        Cube {
            id,
            typ,
            owner,
            donation: if donation { Some(owner) } else { None },
        }
    }

    pub fn value(&self) -> Fraction {
        self.typ.value()
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
        };
        write!(f, "\x1b[{}m{}\x1b[0m", color, text)
    }
}
