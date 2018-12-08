use std::collections::HashMap;

pub type NodeID = String;

/// Node struct
#[derive(Default, Debug)]
pub struct Node {
    pub id: NodeID,
    pub storage: HashMap<i32, String>,
    pub node_list: Vec<NodeID>,
}

impl Node {
    pub fn emit() {}    
    pub fn receive() {}
}
