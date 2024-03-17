use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProjectState {
    /// Project has been activated by the Caylion, and cards have been
    /// distributed
    Active,
    /// Project has all the necessary votes, and is awaiting approval
    /// from the Caylion.
    Pending,
    /// Project does not have all of the requisite votes, and needs more votes
    /// before it can be activated.
    Idle,
    /// Project is on the reverse of a currently available project, and cannot
    /// be voted on this turn.
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProjectID(pub usize);
