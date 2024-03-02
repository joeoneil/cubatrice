use serde::{Deserialize, Serialize};

use crate::entity::{
    colony::ColonyID,
    converter::ConverterID,
    cube::{CubeID, CubeType},
    faction::FactionType,
    technology::TechID,
};

use super::{player::PlayerID, Phase};

/// Transparent type for referring to records
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecordID(pub usize);

/// A record of a GameEvent used to modify gamestate
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RecordType {
    // TODO
    // I know I'm going to need more record types later. There's no way to
    // represent debts currently, or recurring payments, or other things of
    // that nature.
    /// Cube portion of a trade. Transfers cube ownership between players
    TradeCubes {
        /// Player 'A' in the trade
        a: PlayerID,
        /// Player 'B' in the trade
        b: PlayerID,
        /// Cubes (currently) owned by A, transfered to B.
        a_cubes: Vec<CubeID>,
        /// cubes (currently) owned by B, transfered to A.
        b_cubes: Vec<CubeID>,
    },
    /// Colony portion of a trade. Transfers colony ownership between players
    TradeColony {
        /// Player 'A' in the trade
        a: PlayerID,
        /// Player 'B' in the trade
        b: PlayerID,
        /// Colonies (currently) owned by A, transferred to B.
        a_colony: Vec<ColonyID>,
        /// Colonies (currently) owned by B, transferred to A.
        b_colony: Vec<ColonyID>,
    },
    /// Converter portion of a trade. Transfers converter ownership between
    /// players
    TradeConverter {
        /// Player 'A' in the trade
        a: PlayerID,
        /// Player 'B' in the trade
        b: PlayerID,
        /// Converters (currently) owned by A, transferred to A temporarily
        a_converter: Vec<ConverterID>,
        /// Converters (currently) owned by B, transferred to B temporarily
        b_converter: Vec<ConverterID>,
        /// Whether the trade is permanent or temporary.
        permanent: bool,
    },
    /// Creates a player with a given faction, adding them and all of their
    /// resources to the game.
    CreatePlayer {
        player: PlayerID,
        faction: FactionType,
    },
    /// Applies a retrocontinuity token to a converter, producing its outputs
    /// during the trade phase instead of the economy phase.
    Retrocontinuity { converter: ConverterID },
    /// Changes the current game phase to the specified phase. Must be the
    /// phase immediately following the current phase.
    ChangePhase { to: Phase },
    /// Represents a player's bid for colonies and tech teams. Bids must be
    /// made at the same time. Base Kjas may optionally bid for two colonies,
    /// and Alt Faderan may optionally bid for two research teams.
    Bid {
        player: PlayerID,
        for_colony: usize,
        for_colony_kjas: Option<usize>,
        for_tech: usize,
        for_tech_faderan: Option<usize>,
    },
    /// A Player taking a colony after bidding
    TakeColony {
        player: PlayerID,
        /// Colony is None if the player passed on taking a colony.
        /// If some, it is an index into colony_bid_track
        colony: Option<usize>,
    },
    /// A Player taking a research team after bidding
    TakeResearch {
        player: PlayerID,
        /// Tech is None if the player passed on taking a tech.
        /// If some, it is an index into tech_bid_track
        tech: Option<usize>,
    },
    /// A Player inventing a technology
    InventTech {
        player: PlayerID,
        tech: TechID,
        cost: CubeType,
    },
    /// The Yengii player licensing a technology to another player
    License { player: PlayerID, tech: TechID },
}

/// A Record along with its ID.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecordGroup {
    pub id: RecordID,
    pub typ: RecordType,
}
