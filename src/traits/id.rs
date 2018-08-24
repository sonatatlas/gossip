// 
//  Id divide into Pub and Secret; 
//
//  udtrokia@gmail.com
//


use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::hash::Hash;

use Proof;

pub trait PublicId: Clone + Eq + Ord + Hash + Serialize + DeserializeOwned + Debug {
    type Signature: Clone + Eq + Ord + Hash + Serialize + DeserializeOwned + Debug;
    fn verify_signature(&self, signature: &Self::Signature, data: &[u8]) -> bool;
}


// fn define
pub trait SecretId {
    type PublicId: PublicId;
    
    fn public_id(&self) -> &Self::PublicId;
    fn sign_detached(&self, data: &[u8]) -> <Self::PublicId as PublicId>::Signature;
    fn create_proof(&self, data: &[u8]) -> Proof<Self::PublicId> {
        Proof {
            public_id: self.public_id().clone(),
            signature: self.sign_detached(data),
        }
    }
}


mod test {
    #[test]
    fn it_works() {
        
    }
}

