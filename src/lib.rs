#[macro_use]
extern crate serde_derive;
extern crate serde;

mod traits {
    pub mod id;
    pub mod network_event;
}

mod structs {
    pub mod proof;
}


use traits::id::PublicId;
use structs::proof::Proof;
