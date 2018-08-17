GUIDE OF PARSEC
================


### Tree

```
.
├── block.rs
├── dump_graph.rs
├── error.rs
├── gossip
│   ├── cause.rs
│   ├── content.rs
│   ├── event.rs
│   ├── messages.rs
│   ├── mod.rs
│   └── packed_event.rs
├── hash.rs
├── id.rs
├── lib.rs
├── meta_vote.rs
├── mock.rs
├── network_event.rs
├── parsec.rs
├── peer_manager.rs
├── round_hash.rs
└── vote.rs

```


### Structure

+ Parsec

<!---------------|----------------|------------------>
| Struct         | Trait          | Tool             |
|----------------|----------------|------------------|
| block          | id             | dump-graph(func) |
| hash           | network_event  | error(error)     |
| mock           |                | meta-votes(enum) |
| parsec         |                | gossip(mod)      |
| peer-manager   |                |                  |
| round-hash     |                |                  |
| vote           |                |                  |
|----------------|----------------|------------------|
| __mod-gossip__ | __mod-gossip__ | __mod-gossip__   |
| content        |                | cause(enum)      |
| event          |                |                  |
| message        |                |                  |
| packed-event   |                |                  |
|----------------|----------------|------------------|






