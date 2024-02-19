use serde::{Deserialize, Serialize};

use super::{
    converter::{Convert, Converter},
    cube::CubeType,
};

/// Transparent type for referring to techs.
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TechID(pub usize);

/// Alternate cost for technology
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TechCost {
    pub typ: CubeType,
    pub qty: usize,
}

/// Technology card. Tech cards have multiple possible costs, one and only
/// one of which must be paid to invent the technology. Technologies are
/// initially only available the inventor, but are shared in the next
/// confluence with all other players except under special circumstances
/// involving Base Yengii and Alt Faderan.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Technology {
    /// Unique ID for this technology
    pub id: TechID,
    /// List of alternate costs for this technology. Tier 1-3 techs will have
    /// 2 choices, and Tier 4 techs will havec 3.
    pub cost: Vec<TechCost>,
    /// Name of the technology.
    pub name: String,
    /// Name of the invented technology. Tier 4 technologies do not invent any
    /// converters, so this is None for Tier 4 techs.
    pub invents: Option<String>,
    /// Techs are released in tier order. Tier 1 first, then 2, etc. There are
    /// 4 tiers.
    pub tier: usize,
    /// The reward for inventing the technology, in addition to the sharing
    /// bonus.
    pub invent_reward: usize,
}

/// A converter without additional information, such as who owns it or
/// additional faction data (such as imdril fleet cost).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
// TODO: Does this need to be prototype? other data (ownership, borrowing,
// fleet requirements, runnable by, etc.) can maybe be handled in encapsulating
// Game state? I'll decide this later but currently leaning toward this bit
// being all that I need.
pub struct ConverterPrototype {
    pub id: TechID,
    #[serde(flatten)]
    pub conv: Converter,
}

impl Convert for ConverterPrototype {
    fn input(&self) -> &[super::Item] {
        self.conv.input.as_slice()
    }

    fn output(&self) -> &[super::Item] {
        self.conv.output.as_slice()
    }

    fn upgradable(&self) -> bool {
        false
    }

    fn upgrade_opts(&self) -> Option<usize> {
        None
    }

    fn upgrade_cost(&self, _alt: usize) -> Option<&[super::Item]> {
        None
    }

    fn color(&self) -> super::converter::Arrow {
        self.conv.color
    }
}
