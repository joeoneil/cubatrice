use crate::entity::{
    colony::ColonyType,
    converter::{Arrow, Convert},
    cube::CubeType,
    Item, Upgrade,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RelicWorld {
    GiftOfTheDuruntai,
    ContextualIntegratorCache,
    AutomatedTransportNetwork,
    RelicDetector,
    LibraryOfEntelechy,
    TransmutiveDecomposer,
    NalgorianGrindstone,
    StarsRuin,
    ParadiseConverter,
    BarianTradeArmada,
    ThilsDemiring,
    TheGrandArmilla,
}

const ATN_OUT: [Item; 1] = [Item::Cubes(CubeType::Food, 1)];

const LIBRARY_OUT: [Item; 4] = [
    Item::DonationCubes(CubeType::VictoryPoint, 1),
    Item::DonationCubes(CubeType::Information, 1),
    Item::DonationCubes(CubeType::Food, 1),
    Item::DonationCubes(CubeType::Culture, 1),
];

const TRANSMUTIVE_IN: [Item; 1] = [Item::Cubes(CubeType::Ship, 2)];
const TRANSMUTIVE_OUT: [Item; 4] = [
    Item::Cubes(CubeType::Ultratech, 1),
    Item::Cubes(CubeType::Power, 1),
    Item::Cubes(CubeType::Industry, 1),
    Item::Cubes(CubeType::Food, 1),
];

const NALGORIAN_IN: [Item; 1] = [Item::Cubes(CubeType::VictoryPoint, 1)];
const NALGORIAN_OUT: [Item; 4] = [
    Item::Cubes(CubeType::Biotech, 1),
    Item::Cubes(CubeType::Information, 1),
    Item::Cubes(CubeType::Industry, 1),
    Item::Cubes(CubeType::Food, 1),
];

const PARADISE_IN: [Item; 1] = [Item::Colony(ColonyType::Any)];
const PARADISE_OUT: [Item; 1] = [Item::Cubes(CubeType::VictoryPoint, 2)];

const THILS_IN: [Item; 2] = [
    Item::Cubes(CubeType::Information, 1),
    Item::Cubes(CubeType::Power, 1),
];
const THILS_OUT: [Item; 1] = [Item::Cubes(CubeType::VictoryPoint, 2)];

const ARMILLA_OUT: [Item; 1] = [Item::Cubes(CubeType::Ship, 4)];

impl Convert for RelicWorld {
    fn input(&self) -> &[Item] {
        match *self {
            Self::TransmutiveDecomposer => &TRANSMUTIVE_IN,
            Self::NalgorianGrindstone => &NALGORIAN_IN,
            Self::ParadiseConverter => &PARADISE_IN,
            Self::ThilsDemiring => &THILS_IN,
            _ => &[],
        }
    }

    fn color(&self) -> crate::entity::converter::Arrow {
        match *self {
            Self::ParadiseConverter => Arrow::Purple,
            _ => Arrow::White,
        }
    }

    fn output(&self) -> &[Item] {
        match *self {
            Self::AutomatedTransportNetwork => &ATN_OUT,
            Self::LibraryOfEntelechy => &LIBRARY_OUT,
            Self::TransmutiveDecomposer => &TRANSMUTIVE_OUT,
            Self::NalgorianGrindstone => &NALGORIAN_OUT,
            Self::ParadiseConverter => &PARADISE_OUT,
            Self::ThilsDemiring => &THILS_OUT,
            Self::TheGrandArmilla => &ARMILLA_OUT,
            _ => &[],
        }
    }

    fn upgradable(&self) -> bool {
        false
    }

    fn upgrade_opts(&self) -> Option<usize> {
        None
    }

    fn upgrade_cost(&self, _alt: usize) -> Option<Upgrade> {
        None
    }

    fn upgrade(&mut self, _data: &crate::state::GameData, _opt: usize) {}

    fn upgrade_token(&self) -> Option<super::alt_kit::UpgradeToken> {
        None
    }
}
