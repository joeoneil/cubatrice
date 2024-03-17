use serde::{Deserialize, Serialize};

use crate::entity::{
    converter::{Arrow, Convert},
    Item, Upgrade,
};

use super::GenericStartingConverter;

/// Used by dyn Convert to know which upgrade token can upgrade a given
/// converter, if any.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UpgradeToken {
    /// Colony Upgrade token, of which 2 can be invented.
    Colony,
    /// Tier one upgrade token, of which 2 can be invented.
    TierOne,
    /// Tier two upgrade token, of which 1 can be invented.
    TierTwo,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KitConverter {
    pub l_conv: GenericStartingConverter,
    pub r_conv: GenericStartingConverter,
    pub output_cache: Vec<Item>,
}

impl KitConverter {
    fn update_cache(&mut self) {
        self.output_cache.clear();
        self.output_cache.extend_from_slice(self.l_conv.output());
        self.output_cache.extend_from_slice(self.r_conv.output());
    }
}

impl Convert for KitConverter {
    fn input(&self) -> &[Item] {
        self.l_conv.input()
    }

    fn output(&self) -> &[Item] {
        self.output_cache.as_slice()
    }

    fn upgrade(&mut self, data: &crate::state::GameData, opt: usize) {
        if opt < 2 && self.l_conv.upgradable() {
            self.l_conv.upgrade(data, opt)
        } else {
            self.r_conv.upgrade(
                data,
                if self.l_conv.upgradable() {
                    opt - 2
                } else {
                    opt
                },
            )
        }
    }

    fn upgradable(&self) -> bool {
        self.l_conv.upgradable() || self.r_conv.upgradable()
    }

    fn upgrade_opts(&self) -> Option<usize> {
        if self.l_conv.upgradable() && self.r_conv.upgradable() {
            Some(4)
        } else if self.upgradable() {
            Some(2)
        } else {
            None
        }
    }

    fn upgrade_cost(&self, alt: usize) -> Option<Upgrade> {
        if alt < 2 && self.l_conv.upgradable() {
            self.l_conv.upgrade_cost(alt)
        } else {
            self.r_conv.upgrade_cost(if self.l_conv.upgradable() {
                alt - 2
            } else {
                alt
            })
        }
    }

    fn upgrade_token(&self) -> Option<UpgradeToken> {
        None
    }

    fn color(&self) -> Arrow {
        Arrow::White
    }
}
