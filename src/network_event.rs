
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;


pub trait NetworkEvent: Clone + Eq + Ord + Serialize + DeserializeOwned + Debug {}
