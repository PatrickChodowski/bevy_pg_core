
use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin, FreeCameraState};
use bevy::prelude::*;

use crate::prelude::GameState;

pub struct PGCoreCameraPlugin;

impl Plugin for PGCoreCameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(FreeCameraPlugin)
        .add_systems(Startup, spawn)
        .add_systems(OnExit(GameState::Play), reset_camera)
        .add_observer(on_camera_mode_changed)
        ;
    }
}

fn on_camera_mode_changed(
    trigger: On<CameraModeChanged>,
    camera: Single<&mut FreeCameraState, With<MainCamera>>
){
    let mut state = camera.into_inner();
    
    match trigger.mode{
        CameraMode::Player => {
            state.enabled = false;
        }
        CameraMode::Dev => {
            state.enabled = true;
        }
    }


}


fn reset_camera(
    mut camera: Single<&mut MainCamera>
){
    camera.reset();
}


fn spawn(
    mut commands: Commands
) {
    let main_camera = MainCamera::default();
    let mut start_camera_transform = Transform::from_translation(main_camera.start);
    start_camera_transform.look_at(main_camera.start_look_at, Vec3::Y);

    commands.spawn((
            Camera3d::default(),
            // Tonemapping::None,
            // Tonemapping::Reinhard,
            // DepthOfField {
            //     mode: DepthOfFieldMode::Gaussian,
            //     focal_distance: 25.0,
            //     ..default()
            // },
            FreeCamera{walk_speed: 25.0, ..default()},
            Projection::Perspective(PerspectiveProjection{far: 100.0, ..default()}),
            start_camera_transform,
            main_camera,
            // DistanceFog {
            //     color: Color::WHITE.with_alpha(0.6),
            //     falloff: FogFalloff::from_visibility_squared(5500.0),                
            //     directional_light_color: Color::NONE,
            //     directional_light_exponent: 8.0,
            // } 
        ));
}


#[derive(Default, Eq, PartialEq, Clone, Copy, Reflect)]
enum CameraMode {
    Dev,
    #[default]
    Player,
}

#[derive(Event)]
pub struct CameraModeChanged {
    pub mode: CameraMode
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MainCamera {
    start: Vec3,
    start_look_at: Vec3,
    sensitivity: f32,
    padding: Vec3,
    speed:   f32,
    mode: CameraMode
}

impl MainCamera {
    pub fn init(
        &mut self, 
        commands:         &mut Commands,
        player_loc:       Vec3,
        camera_entity:    Entity,
        camera_transform: &mut Transform,
        padding: Vec3,
        speed: f32
    ) {
        self.padding = padding;
        self.speed = speed;
        match self.mode {
            CameraMode::Dev => {
                self.set_dev(commands, camera_entity);
            }
            CameraMode::Player => {
                self.set_player(commands, camera_entity, camera_transform, player_loc);
            }
        }
    }


    pub fn toggle(
        &mut self, 
        commands:         &mut Commands,
        player_loc:       Vec3,
        camera_entity:    Entity,
        camera_transform: &mut Transform
    ) {
        match self.mode {
            CameraMode::Player => {
                self.set_dev(commands, camera_entity);
            }
            CameraMode::Dev => {
                self.set_player(commands, camera_entity, camera_transform, player_loc);
            }
        }
    }

    pub fn set_speed(&mut self, padding: Vec3){
        self.padding = padding;
    }

    pub fn set_padding(&mut self, padding: Vec3){
        self.padding = padding;
    }

    pub fn get_padding(&self) -> &Vec3{
        &self.padding
    }

    pub fn get_sensitivity(&self) -> f32 {
        self.sensitivity
    }

    pub fn get_start(&self) -> Vec3 {
        self.start
    }

    pub fn is_player(&self) -> bool {
        self.mode == CameraMode::Player
    }

    pub fn is_dev(&self) -> bool {
        self.mode == CameraMode::Dev
    }

    pub fn set_player(
        &mut self, 
        commands: &mut Commands,
        camera_entity: Entity,
        camera_transform: &mut Transform,
        player_loc: Vec3
    ) {
        self.start = player_loc + self.padding;
        camera_transform.translation = self.start;
        camera_transform.look_at(player_loc, Vec3::Y);
        self.mode = CameraMode::Player;
        commands.trigger(CameraModeChanged{mode: self.mode});
    }

    pub fn set_dev(
        &mut self,
        commands: &mut Commands,
        camera_entity: Entity
    ) {
        if self.mode == CameraMode::Dev {
            return;
        }
        self.mode = CameraMode::Dev;
        commands.trigger(CameraModeChanged{mode: self.mode});
        // self.start = MainCamera::default().start;
    }

    pub fn reset(&mut self){
        *self = MainCamera::default();
    }
}

impl Default for MainCamera {
    fn default() -> Self {
        MainCamera {
            start: Vec3::new(60.0, 60.0, 60.0),
            start_look_at: Vec3::ZERO,
            sensitivity: 0.0001,
            speed:  50.0,
            mode:   CameraMode::Player,
            padding: Vec3::new(21.0, 26.0, 21.0),

        }
    }
}

// Condition functions:
pub fn is_camera_player(camera: Single<&MainCamera>) -> bool {
    return camera.is_player();
}

pub fn is_camera_dev(camera: Single<&MainCamera>) -> bool {
    return camera.is_dev();
}
