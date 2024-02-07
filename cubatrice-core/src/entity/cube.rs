use std::{fmt::Display, sync::Mutex};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::{state::player::PlayerID, Fraction};

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CubeID(usize);

lazy_static! {
    static ref NEXT_CUBE_ID: Mutex<CubeID> = Mutex::new(CubeID(0));
}

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
    pub fn new(typ: CubeType, owner: PlayerID, donation: bool) -> Self {
        NEXT_CUBE_ID.lock().unwrap().0 += 1;
        Cube {
            id: *NEXT_CUBE_ID.lock().unwrap(),
            typ,
            owner,
            donation: if donation { Some(owner) } else { None },
        }
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
