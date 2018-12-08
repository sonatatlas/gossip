extern crate gossip;
use gossip::node::Node;
use std::collections::HashMap;

fn main(){
    let mut storage = HashMap::new();
    storage.insert(0, "test_storage".to_string());
    
    let node = Node {
        id: "test_id".to_string(),
        storage: storage,
        node_list: vec!["test_0".to_string()],
    };
    
    println!("{:?}", node);
}
