use bevy::prelude::*;
use crate::game_state::GameStateInit;

pub struct PGCoreAssetsPlugin;

impl Plugin for PGCoreAssetsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, init_configs)
        // .add_systems(Update, track_configs.run_if(resource_exists::<ConfigHandles>.and(resource_exists::<ConfigReadyChecks>)))
        // .add_systems(OnEnter(GameStateInit::Assets), init_assets)
        // .add_systems(Update,  track_assets.run_if(resource_exists::<AssetReadyChecks>))
        ;
    }
}

// #[derive(Resource)]
// pub struct ConfigHandles {
//     pub settings: Handle<Settings>,
//     pub arrow:    Handle<ArrowConfig>,
//     pub nav:      Handle<NavConfig>,
//     pub maps:     Vec<Handle<MapMetadata>>,
//     pub scenes:   HashMap<String, Handle<SceneData>>
// }

// #[derive(Resource)]
// pub struct ConfigHandles<A: Asset> {
//     pub handles: Vec<Handle<A>>
// }


// #[derive(Resource)]
// pub struct ConfigHandles {
//     pub handles: Vec<AssetId<>
// }


fn init_configs(
    mut commands: Commands,
    ass:          Res<AssetServer>,
){

}

fn track_configs(

){

}

fn init_assets(

){

}

fn track_assets(

){

}