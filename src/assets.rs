use bevy::prelude::*;
use bevy::asset::LoadedUntypedAsset;
use crate::game_state::GameStateInit;

pub struct PGCoreAssetsPlugin;

impl Plugin for PGCoreAssetsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, init)
        // .add_systems(OnEnter(GameStateInit::Configs), init_configs)  -> belongs to the game
        .add_systems(Update,  track_configs.run_if(in_state(GameStateInit::Configs)))
        // .add_systems(OnEnter(GameStateInit::Assets), init_assets)  -> belongs to the game
        .add_systems(Update,  track_assets.run_if(in_state(GameStateInit::Assets)))
        // .add_systems(OnEnter(GameStateInit::Loaded), on_loaded) -> belongs to the game
        .add_systems(OnExit(GameStateInit::Loaded), on_exit_loaded)
        ;
    }
}

fn on_exit_loaded(
    mut commands: Commands
){
    commands.remove_resource::<ConfigHandles>();
    commands.remove_resource::<AssetHandles>();
}

fn init(
    mut next_state: ResMut<NextState<GameStateInit>>
){  
    info!(" [ASSETS] Start loading Configs");
    next_state.set(GameStateInit::Configs);
}


#[derive(Resource)]
pub struct ConfigHandles {
    pub data: Vec<Handle<LoadedUntypedAsset>>
}

#[derive(Resource)]
pub struct AssetHandles {
    pub data: Vec<Handle<LoadedUntypedAsset>>
}

fn track_configs(
    ass:                         Res<AssetServer>,
    config_handles:              Option<Res<ConfigHandles>>,
    mut next_game_state_init:    ResMut<NextState<GameStateInit>>,
){

    let Some(config_handles) = config_handles else {return;};
    if config_handles.data.len() == 0 {
        return;
    }

    let mut loaded_count: usize = 0;
    for handle in config_handles.data.iter(){
        if let Some(asset_load_state) = ass.get_load_state(handle) {
             if asset_load_state.is_loaded(){
                loaded_count += 1;
             }
        }
    }
    if loaded_count == config_handles.data.len(){
        info!(" [ASSETS]: Loaded {} configs", loaded_count);
        next_game_state_init.set(GameStateInit::Assets);
    }
}




fn track_assets(
    ass:                         Res<AssetServer>,
    assets_handles:              Option<Res<AssetHandles>>,
    mut next_game_state_init:    ResMut<NextState<GameStateInit>>,
){

    let Some(assets_handles) = assets_handles else {return;};
    if assets_handles.data.len() == 0 {
        return;
    }

    let mut loaded_count: usize = 0;
    for handle in assets_handles.data.iter(){
        if let Some(asset_load_state) = ass.get_load_state(handle) {
             if asset_load_state.is_loaded(){
                loaded_count += 1;
             }
        }
    }
    if loaded_count == assets_handles.data.len(){
        info!(" [ASSETS]: Loaded {} assets", loaded_count);
        next_game_state_init.set(GameStateInit::Loaded);
    }
}