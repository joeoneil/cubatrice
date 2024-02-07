use serde::{Deserialize, Serialize};

use super::{
    converter::{Convert, Converter},
    cube::CubeType,
};

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TechID(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TechCost {
    pub typ: CubeType,
    pub qty: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Technology {
    pub id: TechID,
    pub cost: Vec<TechCost>,
    pub name: String,
    pub invents: Option<String>,
    pub tier: usize,
    pub invent_reward: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
