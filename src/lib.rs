#[macro_use]
extern crate serde_derive;
extern crate serde;

mod id;
mod proof;
mod network_event;

pub use id::{PublicId};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
