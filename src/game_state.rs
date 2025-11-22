use bevy::prelude::*;
use serde::{Serialize, Deserialize};

pub struct PGCoreStatesPlugin;

impl Plugin for PGCoreStatesPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_state::<GameState>()
        .add_sub_state::<GameStatePlay>()
        .add_sub_state::<GameStateInit>()
        .add_sub_state::<GameStateTransition>()
        ;
    }
}


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, SystemSet, Serialize, Deserialize)]
#[states(scoped_entities)]
pub enum GameState {
    #[default]
    Init,
    Menu,
    Play,
    Transition // -> for Loading Saves
}


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::Init)]
#[states(scoped_entities)]
pub enum GameStateInit {
    #[default]
    Init,
    Configs, // beginning of the game, reading settings
    Assets, // beginning of the game, reading assets
    Loaded
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::Play)]
#[states(scoped_entities)]
pub enum GameStatePlay {
    #[default]
    Running,
    Editor,
    Paused,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::Transition)]
#[states(scoped_entities)]
pub enum GameStateTransition {
    #[default]
    UnloadScene,
    LoadSceneData,
    LoadStatic,
    LoadRest,
    Ready,
    Done
}