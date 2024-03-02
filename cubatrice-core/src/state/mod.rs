use std::{
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::Error;
use serde::{Deserialize, Serialize};

use crate::{
    entity::{
        colony::{Colony, ColonyID},
        converter::{Convert, ConverterID},
        cube::{Cube, CubeID, CubeRecord, CubeType},
        faction::{FactionType, StartingResources},
        technology::{ConverterPrototype, TechID, Technology},
        Item,
    },
    Deck, DATA_DIR,
};

use self::{
    player::PlayerID,
    record::{Record, RecordID, RecordType},
};

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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Confluence(pub usize);

impl Default for Confluence {
    fn default() -> Self {
        Confluence(1)
    }
}

/// Used to track the state of the game. Modified indirectly and atomically by
/// applying (and unapplying) records. Unapplying a record that was never
/// applied is a logic error.
#[derive(Debug, Default)]
pub struct GameState {
    /// Which phase the game is currently in.
    phase: Phase,
    /// Which confluence the game is currently in
    confluence: Confluence,
    /// Game data for specific converters, techs, planets, etc.
    data: GameData,

    /// Techs waiting to be shared at the sharing phase.
    to_share: Vec<TechID>,
    /// Which techs the yengii hold the license to
    yengii_techs: Vec<TechID>,

    /// The current deck of technologies, shuffled then sorted by tier.
    tech_deck: Deck<TechID>,
    /// The current deck of planets, shuffled.
    colony_deck: Deck<ColonyID>,

    /// A Map from converter IDs to real converters.
    converters: HashMap<ConverterID, Box<dyn Convert>>,
    /// A Map from converters to their current owners.
    converter_owners: HashMap<ConverterID, PlayerID>,
    /// If converters are temporarily transferred, their original owners
    /// are listed here, and will be returned at after the economy phase.
    original_owners: HashMap<ConverterID, PlayerID>,
    /// whether the converter can be traded. If not it will be in this hashset.
    untradable_converters: HashSet<ConverterID>,

    /// Map from cubeID to each cube
    cubes: HashMap<CubeID, Cube>,
    /// Map from CubeID to its owner
    cube_owners: HashMap<CubeID, PlayerID>,

    /// Who owns which tech team, if not yet invented.
    tech_team_owners: HashMap<TechID, PlayerID>,

    /// How many victory points each player has.
    victory_points: HashMap<PlayerID, usize>,

    /// How many ships a player bid for colonies, and an optional second bid.
    player_colony_bid: HashMap<PlayerID, (usize, Option<usize>)>,
    /// How many ships a player bid for techs, and an optional second bid.
    player_tech_bid: HashMap<PlayerID, (usize, Option<usize>)>,
    /// Which colonies are on the bid track. If colonies are not in the process
    /// of being doled out, all options will be Some.
    colony_bid_track: Vec<Option<ColonyID>>,
    /// The order of players colony bids.
    colony_bid_order: Vec<PlayerID>,
    /// The order of players tech bids.
    tech_bid_order: Vec<PlayerID>,
    /// Which techs are on the bid track. If techs are not in the process of
    /// being doles out, all options will be Some.
    tech_bid_track: Vec<Option<ColonyID>>,

    /// The colonies that exist in the game
    colonies: HashMap<ColonyID, Colony>,
    /// Who owns which colony, if it exists
    colony_owners: HashMap<ColonyID, PlayerID>,
    /// The particular faction a player is.
    factions: HashMap<PlayerID, FactionType>,

    next_cube_id: CubeID,
    next_converter_id: ConverterID,
    next_record_id: RecordID,

    records: Vec<Record>,
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

    pub fn validate(&self, rec: &Record) -> bool {
        match &rec.typ {
            RecordType::CreatePlayer { player, faction } => {
                // check that this player ID doesn't exist, and that
                // nobody has selected this faction yet.
                self.factions
                    .iter()
                    .filter(|(p, f)| **p == player || **f == faction || **f == faction.bifurcate())
                    .count()
                    == 0
            }
            RecordType::ChangePhase { to } => match to {
                // check that we don't skip a phase. There has to be a more
                // idiomatic way to do this.
                Phase::Init => false,
                Phase::Trade => self.phase == Phase::Init || self.phase == Phase::ZethSteal,
                Phase::Economy => self.phase == Phase::Trade,
                Phase::ColonyBid => self.phase == Phase::Economy,
                Phase::TechBid => self.phase == Phase::ColonyBid,
                Phase::ZethSteal => self.phase == Phase::TechBid,
                Phase::Resolution => self.phase == Phase::Economy,
                Phase::Finish => self.phase == Phase::Resolution,
            },
            RecordType::TradeCubes {
                a,
                b,
                a_cubes,
                b_cubes,
            } => {
                // check that each player owns all cubes involved.
                a != b &&
                a_cubes
                    .iter()
                    .all(|c| self.cube_owners.get(c).is_some_and(|id| *id == a))
                    && b_cubes
                        .iter()
                        .all(|c| self.cube_owners.get(c).is_some_and(|id| *id == b))
            }
            RecordType::TradeColony {
                a,
                b,
                a_colony,
                b_colony,
            } => {
                a != b &&
                a_colony
                    .iter()
                    .all(|c| self.colony_owners.get(c).is_some_and(|id| *id == a))
                    && b_colony
                        .iter()
                        .all(|c| self.colony_owners.get(c).is_some_and(|id| *id == b))
            }
            RecordType::TradeConverter {
                a,
                b,
                a_converter,
                b_converter,
                .. // we don't care about whether a trade is permanent
            } => {
                a != b &&
                a_converter.iter().all(|c| {
                    !self.untradable_converters.contains(c)
                        && self.converter_owners.get(c).is_some_and(|id| *id == a)
                }) && b_converter.iter().all(|c| {
                    !self.untradable_converters.contains(c)
                        && self.converter_owners.get(c).is_some_and(|id| *id == b)
                })
            }
            RecordType::Bid {
                player, for_colony, for_colony_kjas, for_tech, for_tech_faderan
            } => {
                let ships = self.cube_owners.iter().filter(|(_, v)| player == **v).filter_map(|(k, _)| self.cubes.get(k)).filter(|c| c.typ == CubeType::Ship).count();
                // player has not bid for colonies yet
                !self.player_colony_bid.contains_key(&player) 
                    // player has not bid for techs yet
                    && !self.player_tech_bid.contains_key(&player)
                    // check that the player if the player bid twice, that they
                    // are kjas and their bid is split evenly.
                    && !for_colony_kjas.is_some_and(|b| self.factions.get(&player).unwrap_or(&FactionType::KitCore) != &FactionType::KjasCore || b.max(for_colony) - b.min(for_colony) > 1)
                    // similar to above, but with alt faderan
                    && !for_tech_faderan.is_some_and(|b| self.factions.get(&player).unwrap_or(&FactionType::KitCore) != &FactionType::FaderanAlt || b.max(for_tech) - b.min(for_tech) > 1)
                    // check that the player can afford the bid.
                    && ships >= (for_colony + for_colony_kjas.unwrap_or(0) + for_tech + for_tech_faderan.unwrap_or(0))
            }
            RecordType::TakeColony { player, colony } => {
                self.colony_bid_order.get(0).is_some_and(|p| *p == player) && 
                colony.map(|i| self.colony_bid_track.get(i).is_some()).unwrap_or(true)
            }
            RecordType::TakeResearch { player, tech } => {
                self.tech_bid_order.get(0).is_some_and(|p| *p == player) && 
                tech.map(|i| self.tech_bid_track.get(i).is_some()).unwrap_or(true)
            }
            RecordType::InventTech { player, tech, cost } => {
                self.tech_team_owners.get(&tech).is_some_and(|p| *p == player) &&
                    self.data.tech.get(&tech).is_some_and(|t| t.cost.iter().find(|t| t.typ == cost).is_some_and(|c| self.get_player_cubes(player).count_type(c.typ) >= c.qty as isize))
            }
            _ => todo!(),
        }
    }

    pub fn apply(&mut self, rec: Record) {}

    pub fn get_player_cubes(&self, id: PlayerID) -> CubeRecord {
        self.cube_owners
            .iter()
            .filter(|(_, v)| **v == id)
            .filter_map(|(k, _)| self.cubes.get(k))
            .collect()
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

    pub fn preloaded() -> Result<Self, Error> {
        let mut gd = Self::default();
        gd.load_all()?;
        Ok(gd)
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
            FactionType::YengiiCore,
            FactionType::ZethCore,
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
