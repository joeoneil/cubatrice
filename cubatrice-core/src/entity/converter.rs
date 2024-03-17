use std::{fmt::Debug, hash::Hash};

use serde::{Deserialize, Serialize};

use crate::{state::GameData, Fraction};

use super::{cube::CubeType, faction::alt_kit::UpgradeToken, Item, Upgrade};

/// Transparent type for referring to a specific converter.
#[derive(
    Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct ConverterID(pub usize);

/// Used for determining when a converter can be run
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Arrow {
    /// Runs during economy phase (e.g. converters)
    White,
    /// Runs during trade phase (e.g. research teams, relic worlds)
    Purple,
    /// Stealing converter (Runs during Zeth Steal phase)
    Red,
}

/// Inner converter object used to generalize whenever a card needs a converter.
/// This allows other structs to include converters concisely.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Converter {
    pub color: Arrow,
    pub input: Vec<Item>,
    pub output: Vec<Item>,
}

/// Used as a generic converter. Specific types might be a planet, a converter
/// card, or a Faderan relic world, for example.
pub trait Convert: Debug {
    /// Gets the inputs used to run this converter. Converters with empty
    /// inputs run for free and are always run.
    fn input(&self) -> &[Item];

    /// Gets the current value of cubes and victory points used as inputs to
    /// this converter.
    fn input_value(&self) -> Fraction {
        let mut sum = Fraction::new(0, 1);
        for i in self.input() {
            match i {
                Item::Cubes(typ, qty) | Item::DonationCubes(typ, qty) => {
                    sum = sum + (typ.value() * (*qty) as isize)
                }
                _ => continue,
            }
        }
        sum
    }

    /// Gets the current value of cubes and victory points used as inputs to
    /// this converter, with cubes being adjusted for inflation based on a
    /// given rate and number of remaining turns. Turns remaining is 6 on the
    /// first confluence, as the converter can run 6 more times.
    fn input_value_adjusted(&self, interest_rate: Fraction, turns_left: usize) -> Fraction {
        let mut rate = Fraction::new(1, 1);
        for _ in 0..(turns_left - 1) {
            rate = rate * interest_rate;
        }
        let mut sum = Fraction::new(0, 1);
        for i in self.input() {
            match i {
                Item::Cubes(CubeType::Ship, qty) | Item::DonationCubes(CubeType::Ship, qty) => {
                    sum = sum + (*qty) as isize
                }
                Item::Cubes(CubeType::VictoryPoint, qty)
                | Item::DonationCubes(CubeType::VictoryPoint, qty) => {
                    sum = sum + (6 * qty) as isize
                }
                Item::Cubes(typ, qty) | Item::DonationCubes(typ, qty) => {
                    sum = sum + rate * (typ.value() * (*qty) as isize)
                }
                _ => continue,
            }
        }
        sum / rate
    }

    /// Gets the outputs produced when this converter is run. Converters with
    /// empty outputs may be used to flip a card, draw from a deck, upgrade a
    /// technology, or do something else unrepresentable by an Item.
    fn output(&self) -> &[Item];

    /// Gets the current value of cubes and victory points output by this
    /// converter.
    fn output_value(&self) -> Fraction {
        let mut sum = Fraction::new(0, 1);
        for i in self.output() {
            match i {
                Item::Cubes(typ, qty) | Item::DonationCubes(typ, qty) => {
                    sum = sum + (typ.value() * (*qty) as isize)
                }
                _ => continue,
            }
        }
        sum
    }

    /// Gets the current value of cubes and victory points output by this
    /// converter, with cubes being adjusted for inflation based on a given
    /// rate and number of remaining turns. Turns remaining is 6 on the first
    /// confluence, as the converter can run 6 more times.
    fn output_value_adjusted(&self, interest_rate: Fraction, turns_left: usize) -> Fraction {
        let mut rate = Fraction::new(1, 1);
        for _ in 0..(turns_left - 1) {
            rate = rate * interest_rate;
        }
        let mut sum = Fraction::new(0, 1);
        for i in self.output() {
            match i {
                Item::Cubes(CubeType::Ship, qty) | Item::DonationCubes(CubeType::Ship, qty) => {
                    sum = sum + (*qty) as isize;
                }
                Item::Cubes(CubeType::VictoryPoint, qty)
                | Item::DonationCubes(CubeType::VictoryPoint, qty) => {
                    sum = sum + (6 * qty) as isize
                }
                Item::Cubes(typ, qty) | Item::DonationCubes(typ, qty) => {
                    sum = sum + rate * (typ.value() * (*qty) as isize)
                }
                _ => continue,
            }
        }
        sum / rate
    }

    /// Checks whether the converter can be run for free. This is only the case
    /// when the the converter has no input cost.
    fn free(&self) -> bool {
        self.input().len() == 0
    }

    /// Modifies the converter, upgrading it. This should be done after removing
    /// input costs from the target player's state, as it will change the
    /// converter.
    fn upgrade(&mut self, _data: &GameData, _opt: usize);

    /// Checks whether the converter can be upgraded.
    fn upgradable(&self) -> bool;

    /// If the converter can be upgraded, how many upgrade options are there.
    /// If the converter is upgradable, this is guaranteed to return Some.
    fn upgrade_opts(&self) -> Option<usize>;

    /// The cost of upgrading this converter using a particular alternate.
    /// If the converter is upgradable and has greater than `alt` upgrade
    /// options, this is guaranteed to return Some.
    fn upgrade_cost(&self, alt: usize) -> Option<Upgrade>;

    /// The outputs of upgrading the converter with a given option, if any.
    fn upgrade_outputs(&self, _alt: usize) -> Option<&[Item]> {
        None
    }

    fn upgrade_token(&self) -> Option<UpgradeToken>;

    /// The color of the converter's arrow, used to determine when the
    /// converter can be run.
    fn color(&self) -> Arrow;
}
