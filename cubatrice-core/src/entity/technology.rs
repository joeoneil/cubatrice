use serde::{Deserialize, Serialize};

use super::{
    converter::{Arrow, Convert, Converter},
    cube::CubeType,
    faction::alt_kit::UpgradeToken,
    Item, Upgrade,
};

/// Transparent type for referring to techs.
#[derive(
    Clone, Copy, Default, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub struct TechID(pub usize);

impl TechID {
    pub fn upgrades_with(&self) -> Option<(TechID, TechID)> {
        match self.0 {
            1 => Some((TechID(2), TechID(6))),
            2 => Some((TechID(1), TechID(7))),
            3 => Some((TechID(4), TechID(5))),
            4 => Some((TechID(3), TechID(7))),
            5 => Some((TechID(3), TechID(6))),
            6 => Some((TechID(4), TechID(5))),
            7 => Some((TechID(1), TechID(2))),

            8 => Some((TechID(10), TechID(12))),
            9 => Some((TechID(11), TechID(13))),
            10 => Some((TechID(8), TechID(14))),
            11 => Some((TechID(10), TechID(12))),
            12 => Some((TechID(8), TechID(11))),
            13 => Some((TechID(9), TechID(14))),
            14 => Some((TechID(9), TechID(13))),

            15 => Some((TechID(17), TechID(21))),
            16 => Some((TechID(18), TechID(20))),
            17 => Some((TechID(19), TechID(21))),
            18 => Some((TechID(16), TechID(19))),
            19 => Some((TechID(18), TechID(20))),
            20 => Some((TechID(15), TechID(16))),
            21 => Some((TechID(15), TechID(17))),
            _ => None,
        }
    }
}

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
pub struct ConverterPrototype {
    pub id: TechID,
    pub name: String,
    #[serde(flatten)]
    pub conv: Converter,
}

impl Convert for ConverterPrototype {
    fn input(&self) -> &[Item] {
        self.conv.input.as_slice()
    }

    fn output(&self) -> &[Item] {
        self.conv.output.as_slice()
    }

    fn upgrade(&mut self, _data: &crate::state::GameData, _opt: usize) {
        // converter prototypes should not be upgraded, they aren't real
        // converters, and can't know (for example) which faction's set
        // of converters to actually pull from.
        //
        // upgradable, upgrade_opts, and upgrade_cost are implemented for this
        // type as ConverterPrototype is meant to be an inner field on a more
        // fleshed out converter.
    }

    fn upgradable(&self) -> bool {
        self.id.0 <= 21
    }

    fn upgrade_opts(&self) -> Option<usize> {
        if self.id.0 <= 21 {
            Some(2)
        } else {
            None
        }
    }

    fn upgrade_cost(&self, alt: usize) -> Option<Upgrade> {
        if alt == 0 {
            self.id.upgrades_with().map(|t| Upgrade::ConverterCard(t.0))
        } else if alt == 1 {
            self.id.upgrades_with().map(|t| Upgrade::ConverterCard(t.1))
        } else {
            None
        }
    }

    fn upgrade_token(&self) -> Option<UpgradeToken> {
        if self.id.0 <= 7 {
            Some(UpgradeToken::TierOne)
        } else if self.id.0 <= 14 {
            Some(UpgradeToken::TierTwo)
        } else {
            None
        }
    }

    fn color(&self) -> Arrow {
        self.conv.color
    }
}
