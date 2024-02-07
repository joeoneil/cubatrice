use std::fmt::Display;

use serde::{Deserialize, Serialize};

use self::{colony::ColonyType, cube::CubeType, technology::TechID};

pub mod colony;
pub mod converter;
pub mod cube;
pub mod faction;
pub mod technology;

/// Item is used in a lot of places where we need a generic item. For example,
/// the inputs / outputs of converters, upgrades for converters,
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Item {
    /// Some number of cubes with a given type and quantity
    Cubes(CubeType, usize),
    /// Some number of cubes with a given type and quantity that must be traded
    /// away this trade phase.
    DonationCubes(CubeType, usize),
    /// A Colony of a given type (or any type). Only seen as inputs for purple
    /// converters.
    Colony(ColonyType),
    /// Only seen in starting items. Some factions start with a T1 research team.
    /// AltCaylion starts with a T2 research team.
    ResearchTeamTier(usize),
    /// Only seen in starting items. AltKjas starts with specifically Universal
    /// Translator
    ResearchTeamSpecific(TechID),
    /// Used as inputs for upgrade converters. Takes a converter owned by the same
    /// player for all except AltKjas starting converters, which upgrade using
    /// converters owned by any other player.
    Converter(TechID),
    /// A number of victory points.
    VictoryPoint(usize),
    /// A number of victory points which must be traded away this trade phase.
    DonationVictoryPoint(usize),
    /// Some Token. see token for more detailed documentation.
    Token(Token),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Token {
    /// Given by the Faderan to other factions to acknowledge the help of
    /// the Faderan. Creates a victory point and is returned to the Faderan
    /// whenever a player holding an acknowledgement invents a technology.
    Acknowledgement,
    /// Envoy token. Given along with resources by the Zeth. Having multiple
    /// envoys makes you an easier target for the Zeth, and are not removed
    /// when the Zeth steal from you.
    Envoy,
    /// Comes attatched to research teams invented by the Society of Dying
    /// light, cannot be traded. Converts to negative 1 VP at the end of the
    /// game. Players with regret go last in bidding.
    Regret,
    /// Alt Eni Et Service Token. Halves the input cost of white converters
    /// (rounded up) and cannot be removed once the converter has been run.
    Service,
    /// Cross colonization token used by the Charity Syndicate to upgrade
    /// starting converters & cards. Placed on planets by the Syndicate
    /// and returned when the colony is consumed. Can only be held by the
    /// Zeth.
    CrossColonization,
}

impl Token {
    /// Whether the token is intended to be limited to a certain
    /// quantity and, if so, how many are allowed to exist at once.
    pub fn quantity_limited(&self) -> Option<usize> {
        match self {
            Self::Envoy => Some(7),
            Self::CrossColonization => Some(3),
            Self::Acknowledgement => None,
            Self::Regret => None,
            Self::Service => Some(17),
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Cubes(typ, qty) => write!(f, "{} {}", qty, typ),
            Self::DonationCubes(typ, qty) => write!(f, "{} \x1b[35m[D]\x1b[0m{}", qty, typ),
            Self::VictoryPoint(qty) => write!(f, "{} VP", qty),
            Self::DonationVictoryPoint(qty) => write!(f, "{} \x1b[35m[D]\x1b[0mVP", qty),
            _ => write!(f, "{:?}", self),
        }
    }
}
