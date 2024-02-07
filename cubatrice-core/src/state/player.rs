use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerID(pub usize);
