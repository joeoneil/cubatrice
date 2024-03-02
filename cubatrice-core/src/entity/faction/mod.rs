use serde::{Deserialize, Serialize};

use super::Item;
use crate::Fraction;

/// Which faction a player is playing.
// Support for custom ones might be  possible in the future, but it'd require
// an architectural shift away from this enum and, honestly, is probably more
// trouble than it's worth.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FactionType {
    /// Colony bids are halved, but colonies purchased by them produce double
    /// outputs.
    CaylionCore,
    /// Has interest converters that can be run by anyone other than the Eni Et
    /// Starting economy is relatively weak to compensate.
    EniEtCore,
    /// Has a deck of relic worlds that can be drawn, with relatively powerful
    /// bonuses. Requires adaptability in strategy. Also has aknowledgement tokens
    /// that give the faderan victory points when others invent tech.
    FaderanCore,
    /// Has a very powerful starting economy, but all converters require fleet
    /// support.
    ImdrilCore,
    /// Starting converters are in pairs. Can play nullspace colonies for
    /// yellow cubes.
    KitCore,
    /// Can put special tile converters into play. Counts as an extra player
    /// for the colony bid track, and can split their bid in two for planets.
    KjasCore,
    /// Starting converters can consume any color of cube of the right shape
    /// and produce wild outputs. Converters upgrade to produce wild outputs.
    UnityCore,
    /// Techs invented by the yengii are not shared. They may license techs
    /// to other players, and earn a reduced sharing bonus for inventing.
    YengiiCore,
    /// Players that trade with the zeth are safe. Anyone not marked safe can
    /// be stolen from by the Zeth during the steal phase.
    ZethCore,
    /// Creates projects which other players can vote on.
    CaylionAlt,
    /// Creates service tokens which reduce the input cost of white converters
    /// by half, rounded up.
    EniEtAlt,
    /// Does not received shared technology, does not share technology.
    /// Invents at half cost, and sells teams with regret. Players with
    /// regret go last in bidding.
    FaderanAlt,
    /// Creates factories which can be placed on colonies to produce additional
    /// resources. Requires fleet support to run converters. Starts with 5
    /// fleet support, and cannot gain any more.
    ImdrilAlt,
    /// Starting converters are in pairs. Creates research teams. Kit
    /// Research teams cost an additional ultratech if invented by the Kit.
    KitAlt,
    /// Starting resources are donations. Starting converters produce donations
    /// and are run off of other players' donations.
    KjasAlt,
    /// Rolls dice to determine which starting converters can be run. Produces
    /// wild outputs.
    UnityAlt,
    /// Sells Jii Constraints. Players with constraints cannot use that color
    /// cube as inputs to converters or to invent techs.
    YengiiAlt,
    /// Each other player starts with an 'undersireable' which they own but
    /// only the Zeth can run.
    ZethAlt,
}

impl FactionType {
    pub fn bid_tiebreaker(&self) -> Fraction {
        match *self {
            Self::KitCore => Fraction::new(100, 1),
            Self::KitAlt => Fraction::new(10, 1),
            Self::ImdrilCore => Fraction::new(8, 1),
            Self::KjasAlt => Fraction::new(15, 2),
            Self::FaderanCore => Fraction::new(7, 1),
            Self::YengiiAlt => Fraction::new(13, 2),
            Self::KjasCore => Fraction::new(6, 1),
            Self::ImdrilAlt => Fraction::new(11, 2),
            Self::YengiiCore => Fraction::new(5, 1),
            Self::ZethAlt => Fraction::new(9, 2),
            Self::UnityCore => Fraction::new(4, 1),
            Self::UnityAlt => Fraction::new(22, 7),
            Self::EniEtCore => Fraction::new(3, 1),
            Self::ZethCore => Fraction::new(2, 1),
            Self::EniEtAlt => Fraction::new(3, 2),
            Self::CaylionCore => Fraction::new(1, 1),
            Self::CaylionAlt => Fraction::new(0, 1),
            Self::FaderanAlt => Fraction::new(-1, 1),
        }
    }

    pub fn colony_support(&self) -> usize {
        match *self {
            Self::CaylionCore => 3,
            Self::EniEtCore => 3,
            Self::FaderanCore => 4,
            Self::ImdrilCore => 0,
            Self::KitCore => 100,
            Self::KjasCore => 6,
            Self::UnityCore => 1,
            Self::YengiiCore => 3,
            Self::ZethCore => 3,
            Self::CaylionAlt => 3,
            Self::EniEtAlt => 3,
            Self::FaderanAlt => 8,
            Self::ImdrilAlt => 0,
            Self::KitAlt => 3,
            Self::KjasAlt => 5,
            Self::UnityAlt => 3,
            Self::YengiiAlt => 3,
            Self::ZethAlt => 0,
        }
    }

    pub fn name(&self) -> &'static str {
        match *self {
            Self::CaylionCore => "Caylion Plutocracy",
            Self::EniEtCore => "Eni Et Ascendancy",
            Self::FaderanCore => "Faderan Conclave",
            Self::ImdrilCore => "Im'dril Nomads",
            Self::KitCore => "Kt'zr'kt'rtl Adhocracy",
            Self::KjasCore => "Kjasjavikalimm Directorate",
            Self::UnityCore => "Unity",
            Self::YengiiCore => "Yengii Society",
            Self::ZethCore => "Zeth Anocracy",
            Self::CaylionAlt => "Caylion Collaborative",
            Self::EniEtAlt => "Eni Et Engineers",
            Self::FaderanAlt => "Society of Falling Light",
            Self::ImdrilAlt => "Grand Fleet",
            Self::KitAlt => "Kt'zr'kt'rtl Technophiles",
            Self::KjasAlt => "Kjasjavikalimm Independent Nations",
            Self::UnityAlt => "Deep Unity",
            Self::YengiiAlt => "Yengii Jii",
            Self::ZethAlt => "Charity Syndicate",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match *self {
            Self::CaylionCore | Self::CaylionAlt => "Caylion",
            Self::EniEtCore | Self::EniEtAlt => "Eni Et",
            Self::FaderanCore | Self::FaderanAlt => "Faderan",
            Self::ImdrilCore | Self::ImdrilAlt => "Imdril",
            Self::KitCore | Self::KitAlt => "Kit",
            Self::KjasCore | Self::KjasAlt => "Kjas",
            Self::UnityCore | Self::UnityAlt => "Unity",
            Self::YengiiCore | Self::YengiiAlt => "Yengii",
            Self::ZethCore | Self::ZethAlt => "Zeth",
        }
    }

    pub fn difficulty(&self) -> usize {
        match *self {
            Self::FaderanAlt => 7,
            Self::YengiiCore => 6,
            Self::EniEtCore => 5,
            Self::UnityCore => 5,
            Self::ImdrilAlt => 5,
            Self::KitAlt => 5,
            Self::ZethAlt => 5,
            Self::ImdrilCore => 4,
            Self::EniEtAlt => 4,
            Self::KjasAlt => 4,
            Self::YengiiAlt => 4,
            Self::FaderanCore => 3,
            Self::ZethCore => 3,
            Self::CaylionAlt => 3,
            Self::CaylionCore => 2,
            Self::KjasCore => 2,
            Self::UnityAlt => 2,
            Self::KitCore => 1,
        }
    }

    pub fn impact(&self) -> usize {
        match *self {
            Self::CaylionAlt => 3,
            Self::ZethCore => 2,
            Self::FaderanAlt => 2,
            Self::YengiiAlt => 2,
            Self::EniEtCore => 1,
            Self::KitAlt => 1,
            Self::ZethAlt => 1,
            Self::EniEtAlt => 0,
            Self::ImdrilAlt => 0,
            Self::KjasAlt => 0,
            Self::UnityAlt => 0,
            _ => 0,
        }
    }

    pub fn bifurcate(&self) -> FactionType {
        match *self {
            Self::CaylionCore => Self::CaylionAlt,
            Self::EniEtCore => Self::EniEtAlt,
            Self::FaderanCore => Self::FaderanAlt,
            Self::ImdrilCore => Self::ImdrilAlt,
            Self::KitCore => Self::KitAlt,
            Self::KjasCore => Self::KjasAlt,
            Self::YengiiCore => Self::YengiiAlt,
            Self::UnityCore => Self::UnityAlt,
            Self::ZethCore => Self::ZethAlt,
            Self::CaylionAlt => Self::CaylionCore,
            Self::EniEtAlt => Self::EniEtCore,
            Self::FaderanAlt => Self::FaderanCore,
            Self::ImdrilAlt => Self::ImdrilCore,
            Self::KitAlt => Self::KitCore,
            Self::KjasAlt => Self::KjasCore,
            Self::YengiiAlt => Self::YengiiCore,
            Self::UnityAlt => Self::UnityCore,
            Self::ZethAlt => Self::ZethCore,
        }
    }

    pub fn core() -> Vec<Self> {
        vec![
            Self::CaylionCore,
            Self::EniEtCore,
            Self::FaderanCore,
            Self::ImdrilCore,
            Self::KitCore,
            Self::KjasCore,
            Self::YengiiCore,
            Self::UnityCore,
            Self::ZethCore,
        ]
    }

    pub fn bifurcation() -> Vec<Self> {
        vec![
            Self::CaylionAlt,
            Self::EniEtAlt,
            Self::FaderanAlt,
            Self::ImdrilAlt,
            Self::KitAlt,
            Self::KjasAlt,
            Self::YengiiAlt,
            Self::UnityAlt,
            Self::ZethAlt,
        ]
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartingResources(pub FactionType, pub Vec<Item>);
