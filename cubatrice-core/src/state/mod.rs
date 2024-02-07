use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

use anyhow::Error;

use crate::{
    entity::{
        colony::{Colony, ColonyID},
        converter::Convert,
        cube::Cube,
        faction::{FactionType, StartingResources},
        technology::{ConverterPrototype, TechID, Technology},
        Item,
    },
    DATA_DIR,
};

use self::player::PlayerID;

/// I don't think this module is actually necessary, but I'm not deleting it
/// yet in case I want to move some information out into player structs.
///
/// For the time being though this has no use.
pub mod player;

/// Records are applied to a game state in order to construct it. Records
/// are atomic and reversable. After applying any record, the game will not be
/// in an illegal state, and any record can be undone to attain the previous
/// game state.
pub mod record;

/// Which phase the game is currently in
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Phase {
    /// Initialization phase before the game has started. GameState is still
    /// setting up in this phase, creating starting resources, adding players
    /// etc.
    #[default]
    Init,
    /// During the trade phase, players trade with each other and run purple
    /// converters.
    Trade,
    /// During economy, the engine runs all marked white converters.
    Economy,
    /// During colony bid, players decide on which colony to purchase, if
    /// any.
    ColonyBid,
    /// During tech bid, players decide on which tech teams to bid if any.
    /// If players bid a quantity of ships in colony bid, and later passed
    /// their bid, they cannot use those ships during this techbid phase.
    /// They can use those ships on future colony and tech bids.
    TechBid,
    /// If the Zeth Anocracy is in the game, they run stealing converters
    /// during this phase, stealing resources from any players that didn't
    /// trade with them during the previous phase.
    ZethSteal,
    /// Debts are resolved during this phase, including future value and
    /// recurring debts.
    Resolution,
    /// All points are totalled and a winner is decided. In reality, everyone
    /// wins because a lot of cubes were pushed.
    Finish,
}

/// Used to track the state of the game. Modified indirectly and atomically by
/// applying (and unapplying) records. Unapplying a record that was never
/// applied is a logic error.
#[derive(Debug, Default)]
pub struct GameState {
    phase: Phase,
    data: GameData,
    licenses: BTreeMap<PlayerID, Vec<TechID>>,
    unmarked_cards: BTreeMap<PlayerID, Vec<Box<dyn Convert>>>,
    marked_card: BTreeMap<PlayerID, Vec<Box<dyn Convert>>>,
    cubes: BTreeMap<PlayerID, Vec<Cube>>,
    colonies: BTreeMap<PlayerID, Vec<Colony>>,
    factions: BTreeMap<PlayerID, FactionType>,
}

impl GameState {
    /// Constructs a new empty game state with a given game data. Data will be
    /// used as the source of truth for game cards (converters, colonies,
    /// starting resources, etc.)
    pub fn new(data: GameData) -> Self {
        Self {
            data,
            ..Self::default()
        }
    }

    /// Sets the game data for a given game.
    pub fn set_game_data(&mut self, data: GameData) {
        self.data = data;
    }
}

/// Used as the source of truth for game data. This is not static to allow for
/// custom data from Unity buffs to completely custom factions.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct GameData {
    pub colony: HashMap<ColonyID, Colony>,
    pub tech: HashMap<TechID, Technology>,
    pub tech_prototype: HashMap<TechID, ConverterPrototype>,
    pub tech_converter: HashMap<&'static str, Vec<ConverterPrototype>>,
    pub start_resources: HashMap<FactionType, Vec<Item>>,
}

impl GameData {
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads all data into this gameData object. not all data may be necessary
    /// so if size becomes an issue, use individual loads for data you need.
    pub fn load_all(&mut self) -> Result<(), Error> {
        self.load_colonies()?;
        self.load_tech()?;
        self.load_resources()?;
        for f in [
            FactionType::CaylionCore,
            FactionType::EniEtCore,
            FactionType::FaderanCore,
            FactionType::ImdrilCore,
            FactionType::KitCore,
            FactionType::KjasCore,
        ] {
            // eventually this will be FactionType::core() or FactionType::all()
            // but not all the factions have their converters documented (yet)
            self.load_faction(f)?;
        }
        Ok(())
    }

    /// Loads all colony data from `DATA_DIR/colony.json`
    pub fn load_colonies(&mut self) -> Result<(), Error> {
        let ser = fs::read_to_string(format!("{}/colony.json", *DATA_DIR))?;
        let obj: Vec<Colony> = serde_json::from_str(ser.as_str())?;
        for c in obj {
            self.colony.insert(c.id, c);
        }
        Ok(())
    }

    /// Loads all tech and prototype data from `DATA_DIR/technology.json` and
    /// `DATA_DIR/prototypes.json`
    pub fn load_tech(&mut self) -> Result<(), Error> {
        let ser = fs::read_to_string(format!("{}/technology.json", *DATA_DIR))?;
        let ser2 = fs::read_to_string(format!("{}/prototypes.json", *DATA_DIR))?;
        let obj: Vec<Technology> = serde_json::from_str(ser.as_str())?;
        let obj2: Vec<ConverterPrototype> = serde_json::from_str(ser2.as_str())?;
        for t in obj {
            self.tech.insert(t.id, t);
        }
        for p in obj2 {
            self.tech_prototype.insert(p.id, p);
        }
        Ok(())
    }

    /// Loads all starting resources from `DATA_DIR/startResources.json`
    pub fn load_resources(&mut self) -> Result<(), Error> {
        let ser = fs::read_to_string(format!("{}/startResources.json", *DATA_DIR))?;
        let obj: Vec<StartingResources> = serde_json::from_str(ser.as_str())?;
        for s in obj {
            self.start_resources.insert(s.0, s.1);
        }
        Ok(())
    }

    /// Loads a specific faction's starting converters and tech converters
    /// from `DATA_DIR/techConverters/{faction}.json` and
    /// `DATA_DIR/startConverters/{faction}.json`. This also loads faction
    /// specific data such as relic worlds, jii constraints, nullspace
    /// colonies, or other things not represented by starting converters.
    pub fn load_faction(&mut self, f: FactionType) -> Result<(), Error> {
        let ser = fs::read_to_string(format!(
            "{}/techConverters/{}.json",
            *DATA_DIR,
            f.short_name()
        ))?;
        let obj: Vec<ConverterPrototype> = serde_json::from_str(ser.as_str())?;
        self.tech_converter.insert(f.short_name(), obj);
        Ok(())
    }
}
