
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

// Type of Peers.
pub trait NetworkEvent: Clone + Eq + Ord + Serialize + DeserializeOwned + Debug {}
