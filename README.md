# [Gossip protocol](https://en.wikipedia.org/wiki/Gossip_protocol)
A gossip protocol is a procedure or process of computer peer-to-peer communication that is based on the way that epidemics spread. Some distributed systems use peer-to-peer gossip to ensure that data is routed to all members of an ad-hoc network. Some ad-hoc networks have no central registry and the only way to spread common data is to rely to each member to pass it along to their neighbors.

## Gossip communication
The concept of gossip communication can be illustrated by the analogy of office workers spreading rumors. Let's say each hour the office workers congregate around the water cooler. Each employee pairs off with another, chosen at random, and shares the latest gossip. At the start of the day, Alice starts a new rumor: she comments to Bob that she believes that Charlie dyes his mustache. At the next meeting, Bob tells Dave, while Alice repeats the idea to Eve. After each water cooler rendezvous, the number of individuals who have heard the rumor roughly doubles (though this doesn't account for gossiping twice to the same person; perhaps Alice tries to tell the story to Frank, only to find that Frank already heard it from Dave). Computer systems typically implement this type of protocol with a form of random "peer selection": with a given frequency, each machine picks another machine at random and shares any hot rumors.

## Many variants and styles
There are probably hundreds of variants of specific Gossip-like protocols because each use-scenario is likely to be customized to the organization's specific needs.

For example, a gossip protocol might employ some of these ideas:
+ The core of the protocol involves periodic, pairwise, inter-process interactions.
+ The information exchanged during these interactions is of bounded size.
+ When agents interact, the state of at least one agent changes to reflect the state of the other.
+ Reliable communication is not assumed.
+ The frequency of the interactions is low compared to typical message latencies so that the protocol costs are negligible.
+ There is some form of randomness in the peer selection. Peers might be selected from the full set of nodes or from a smaller set of neighbors.
+ Due to the replication there is an implicit redundancy of the delivered information.

## Gossip protocol types
+ Dissemination protocols These use gossip to spread information; they basically work by flooding agents in the network, but in a manner that produces bounded worst-case loads:
1. The gossip occurs periodically and events don’t actually trigger the gossip. One concern here is the potentially __high latency__ from when the event occurs until it is delivered.
2. Background data dissemination protocols continuously gossip about information associated with the participating nodes. Typically, propagation latency isn’t a concern, perhaps because the information in question changes slowly or there is __no significant penalty__ for acting upon slightly stale data.

+ Protocol that compute aggregates. These compute a network-wide aggregate by sampling information at the nodes in the network and combining the values to arrive at a system-wide value – the largest value for some measurement nodes are making, smallest, etc. 
