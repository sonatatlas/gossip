use std::collections::HashMap;
use std::time::Duration;
    
type NodeId = String;
type StoreKey = String;
type NodeStatus = u8;
type StateEvent =  u8;
type NodeInfoMap = HashMap<NodeId, NodeInfo>;
type NodeValueMap = HashMap<NodeId, NodeValue>;
type StoreMap = HashMap<StoreKey, Vec<u8>>;

// DEFAULT_GOSSIP_INTERVA: Duration = Duration::new(2, 0);
// DEFAULT_PUSH_PULL_INTERVAL: Duration = Duration::new(2, 0);
// DEFAULT_PROBE_INTERVAL: Duration = Duration::new(5, 0);
// DEFAULT_PROBE_TIMEOUT: Duration = Duration::from_millis(200);
// DEFAULT_QUORUM_TIMEOUT: Duration = Duration::new(60, 0);
// DEFAULT_GOSSIP_VERSION: String =  String::from("v1");
// GOSSIP_VERSION_2: String =  String::from("v2");

// NODE_STATUS_INVALID
// NODE_STATUS_UP;
// NODE_STATUS_DOWN;
// NODE_STATUS_NEVER_GOSSIPED;
// NODE_STATUS_NOT_IN_QUORUM;
// NODE_STATUS_SUSPECT_NOT_IN_QUORUM;
// 
// SELF_ALIVE;
// NODE_ALIVE;
// SELF_LEAVE;
// NODE_LEAVE;
// UPDATE_CLUSTER_SIZE;
// TIMEOUT;

// struct NodeUpdate {
//     Addr: String,
//     QurumMember: bool
// }
// 
// struct NodeMetaInfo {
//     ClustrId: String,
//     GossipVersion: String,
//     Id: String,
//     GenNumber: u64,
//     LastUpdateTs: Duration    
// }
// 
// struct NodeInfo {
//     Id: NodeId,
//     GenNumber: u64,
//     LastUpdateTs: Duration,
//     WaitForGenUpdateTs: Duration,
//     Status: NodeStatus,
//     Value: StoreMap,
//     QorumMember: bool
// }
// 
// struct NodeValue {
//     Id: NodeId,
//     GenNumber: u64,
//     LastUpdateTs: Duration,
//     Status: NodeStatus,
//     Value: Vec<u8>
// }
// 
// struct GossipIntervals {
//     GossipInterval: Duration,
//     PushPullInnterval: Duration,
//     ProbeInterval: Duration,
//     ProbeTimeout: Duration,
//     QuorumTimeout: Duration
// }
