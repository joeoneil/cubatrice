use serde::{Deserialize, Serialize};

use super::{
    converter::{Convert, Converter},
    cube::CubeType,
    faction::alt_kit::UpgradeToken,
    Item, Upgrade,
};

/// Transparent usize type for referring to specific colonies.
#[derive(
    Clone, Copy, Default, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub struct ColonyID(pub usize);

/// Which biome type a colony is. Some converters or upgrades care about
/// colonies of specific biomes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColonyType {
    /// Also known as 'Red' Planets
    Desert,
    /// Also known as 'White' Planets
    Ice,
    /// Also known as 'Green' Planets
    Jungle,
    /// Also known as 'Blue' Planets
    Ocean,
    /// Any planet type
    Any,
}

/// Colony which produces resources
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Colony {
    /// Name of the colony. Used only for display.
    pub name: String,
    /// unique id for the colony data.
    pub id: ColonyID,
    /// color of the colony. See ColonyType.
    pub typ: ColonyType,
    /// Internal converter for the planet. Resources generated and consumed
    /// are dependent on `conv`
    #[serde(flatten)]
    pub conv: Converter,
    /// Cost to upgrade. If the planet is already upgraded, this is None if
    /// the planet has been upgraded.
    pub up_cost: Option<(CubeType, usize)>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OldColony {
    pub name: String,
    pub id: ColonyID,
    pub typ: ColonyType,
    #[serde(flatten)]
    pub conv: Converter,
    pub up_cost: Option<Vec<Item>>,
}

impl Convert for Colony {
    fn input(&self) -> &[Item] {
        self.conv.input.as_slice()
    }

    fn output(&self) -> &[Item] {
        self.conv.output.as_slice()
    }

    fn upgrade(&mut self, data: &crate::state::GameData, opt: usize) {
        if self.id.0 >= 100 {
            // already upgraded
            return;
        }
        let new_data = data.colony.get(&ColonyID(self.id.0 + 100));
        if new_data.is_none() {
            return;
        }
        let Colony {
            name,
            id,
            typ,
            conv,
            up_cost,
        } = new_data.unwrap().clone();
        self.name = name;
        self.id = id;
        self.typ = typ;
        self.conv = conv;
        self.up_cost = up_cost;
    }

    fn upgradable(&self) -> bool {
        self.up_cost.is_some()
    }

    fn upgrade_opts(&self) -> Option<usize> {
        if self.upgradable() {
            Some(1)
        } else {
            None
        }
    }

    fn upgrade_cost(&self, alt: usize) -> Option<Upgrade> {
        if alt == 0 {
            self.up_cost.as_ref().map(|v| Upgrade::Cubes(v.0, v.1))
        } else {
            None
        }
    }

    fn upgrade_token(&self) -> Option<UpgradeToken> {
        if self.upgradable() {
            Some(UpgradeToken::Colony)
        } else {
            None
        }
    }

    fn color(&self) -> super::converter::Arrow {
        self.conv.color
    }
}
