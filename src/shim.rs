/// Minimal shim for `StructureType` when the real screeps-game-api is not available.
/// Mirrors the enum from screeps-game-api for standalone / bench usage.

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum StructureType {
    Spawn = 0,
    Extension = 1,
    Road = 2,
    Wall = 3,
    Rampart = 4,
    KeeperLair = 5,
    Portal = 6,
    Controller = 7,
    Link = 8,
    Storage = 9,
    Tower = 10,
    Observer = 11,
    PowerBank = 12,
    PowerSpawn = 13,
    Extractor = 14,
    Lab = 15,
    Terminal = 16,
    Container = 17,
    Nuker = 18,
    Factory = 19,
    InvaderCore = 20,
}
