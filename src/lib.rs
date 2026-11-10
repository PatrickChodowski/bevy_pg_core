mod assets;
mod game_state;
mod player;
mod pointer;
mod types;
mod utils;

pub mod prelude {
    pub use crate::assets::{PGCoreAssetsPlugin, ConfigHandles, AssetHandles};
    pub use crate::game_state::{PGCoreStatesPlugin, GameState, GameStatePlay, GameStateInit, GameStateTransition};
    pub use crate::player::Player;
    pub use crate::utils::{AABB, rotate_point_2d};
    pub use crate::pointer::{Hoverables, PointerData};
    pub use crate::types::{TerrainChunk, WaterChunk, Tile, EditorAsset};
}
