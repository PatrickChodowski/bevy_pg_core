mod assets;
mod camera;
mod game_state;
mod player;
mod pointer;
mod utils;

pub mod prelude {
    pub use crate::assets::{PGCoreAssetsPlugin, ConfigHandles, AssetHandles};
    pub use crate::camera::{PGCoreCameraPlugin, InputState, MainCamera, 
        fly_cam_controller, FlyCamController, is_camera_dev, is_camera_player, CameraModeChanged};
    pub use crate::game_state::{PGCoreStatesPlugin, GameState, GameStatePlay, GameStateInit, GameStateTransition};
    pub use crate::player::Player;
    pub use crate::utils::{AABB, rotate_point_2d};
    pub use crate::pointer::{Hoverables, PointerData};
}
