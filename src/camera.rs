
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use libm::atan2f;

use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::post_process::bloom::Bloom;
use bevy::ecs::message::MessageCursor;
use bevy::input::mouse::MouseMotion;
use bevy_enhanced_input::prelude::*;

pub struct PGCoreCameraPlugin {
    default_padding: Vec3,
    default_speed:   f32
}

impl PGCoreCameraPlugin {
    pub fn new(
        default_padding: Vec3,
        default_speed: f32
    ) -> Self {
        PGCoreCameraPlugin {
            default_padding,
            default_speed
        }
    }
}

impl Plugin for PGCoreCameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(CameraSettings::new(self.default_padding, self.default_speed))
        .add_input_context::<FlyCamController>()
        .init_resource::<InputState>()
        .add_systems(Startup, spawn)
        .add_observer(move_camera_xz)
        .add_observer(move_camera_y)
        .add_observer(pan_look)
        ;
    }
}

fn spawn(
    mut commands: Commands,
    mut state:    ResMut<InputState>, 
) {
    let camera = MainCamera::default();
    let start_camera_transform = Transform::from_translation(camera.start);
    commands.spawn((
            Camera3d::default(),
            // Tonemapping::None,
            // Tonemapping::Reinhard,
            Tonemapping::TonyMcMapface,
            // DepthOfField {
            //     mode: DepthOfFieldMode::Gaussian,
            //     focal_distance: 25.0,
            //     ..default()
            // },
            Projection::Perspective(PerspectiveProjection{far: 1000.0, ..default()}),
            start_camera_transform,
            camera,
            Bloom::default(),
            fly_cam_controller(),
            ContextActivity::<FlyCamController>::INACTIVE
            // DistanceFog {
            //     color: Color::WHITE.with_alpha(0.6),
            //     falloff: FogFalloff::from_visibility_squared(5500.0),                
            //     directional_light_color: Color::NONE,
            //     directional_light_exponent: 8.0,
            // } 
        ));
    state.yaw_pitch(start_camera_transform.rotation);
}


#[derive(Default, Eq, PartialEq, Clone, Copy)]
enum CameraMode {
    Dev,
    #[default]
    Player,
}

#[derive(Event)]
pub struct CameraModeChanged{
    mode: CameraMode
}

#[derive(Resource)]
pub struct CameraSettings {
    pub padding: Vec3,
    pub speed:   f32
}
impl CameraSettings {
    pub fn new(padding: Vec3, speed: f32) -> Self {
        CameraSettings {
            padding, 
            speed
        }
    }
}


#[derive(Component)]
pub struct MainCamera {
    start: Vec3,
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
        settings:         &Res<CameraSettings>   
    ) {
        self.padding = settings.padding;
        self.speed = settings.speed;
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

    pub fn set_padding(&mut self, padding: Vec3){
        self.padding = padding;
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
        commands.entity(camera_entity).insert(
            ContextActivity::<FlyCamController>::INACTIVE,
        );
    }

    pub fn set_dev(
        &mut self,
        commands: &mut Commands,
        camera_entity: Entity,
    ) {
        if self.mode == CameraMode::Dev {
            return;
        }
        commands.entity(camera_entity).insert(
            ContextActivity::<FlyCamController>::ACTIVE,
        );
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
            start: Vec3::new(0.0, 600.0, 0.0),
            sensitivity: 0.0001,
            speed:  1.0,
            mode:   CameraMode::Player,
            padding: Vec3::new(210.0, 260.0, 210.0),

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

#[derive(Resource, Default)]
pub struct InputState {
    reader_motion: MessageCursor<MouseMotion>,
    pitch: f32,
    yaw: f32,
}
impl InputState {
    pub fn yaw_pitch(&mut self, q: Quat){
        self.yaw = atan2f(
            2.0 * q.y * q.w - 2.0 * q.x * q.z,
            1.0 - 2.0 * q.y * q.y - 2.0 * q.z * q.z,
        );
        self.pitch = atan2f(
            2.0 * q.x * q.w - 2.0 * q.y * q.z,
            1.0 - 2.0 * q.x * q.x - 2.0 * q.z * q.z,
        );
        self.pitch = self.pitch.clamp(-1.54, 1.54);
    }
}

#[derive(Component, Reflect)]
pub struct FlyCamController;


pub fn fly_cam_controller() -> impl Bundle {
    return (
        FlyCamController,
        actions!(
            FlyCamController[
                (
                    Action::<MoveCameraXZ>::new(),
                    Down::default(),
                    Bindings::spawn(Cardinal::wasd_keys())
                ),
                (
                    Action::<MoveCameraY>::new(),
                    Down::default(),
                    Bindings::spawn(
                        Bidirectional::<Binding, Binding> {
                            negative: KeyCode::KeyQ.into(), 
                            positive: KeyCode::KeyE.into()
                        }
                    )
                ),
                (
                    Action::<PanLook>::new(),
                    Down::default(),
                    bindings![MouseButton::Middle]
                )
            ]
        )
    );
}


#[derive(InputAction)]
#[action_output(bool)]
struct PanLook;

fn pan_look(
    _trigger:   On<Fire<PanLook>>,
    window:     Single<&Window, With<PrimaryWindow>>,
    motion:     Res<Messages<MouseMotion>>,
    buttons:    Res<ButtonInput<MouseButton>>,
    mut state:  ResMut<InputState>,
    camera:     Single<(&mut Transform, &mut MainCamera)>,
){
    if buttons.pressed(MouseButton::Middle) {
        let delta_state = state.as_mut();
        let (mut camera_transform, main_camera) = camera.into_inner();
        for ev in delta_state.reader_motion.read(&motion) {
            let window_scale = window.height().min(window.width());
            delta_state.pitch -=
                (main_camera.get_sensitivity() * ev.delta.y * window_scale).to_radians();
            delta_state.yaw -=
                (main_camera.get_sensitivity() * ev.delta.x * window_scale).to_radians();
        }
        delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);
        // Order is important to prevent unintended roll
        camera_transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw) * Quat::from_axis_angle(Vec3::X, delta_state.pitch);
    }
}


#[derive(InputAction)]
#[action_output(Vec2)]
struct MoveCameraXZ;

fn move_camera_xz(
    trigger: On<Fire<MoveCameraXZ>>,
    camera:  Single<(&mut Transform, &MainCamera)>,
    time:    Res<Time>
){
    let (mut camera_transform, main_camera) = camera.into_inner();
    let mut velocity = Vec3::ZERO;
    let local_z = camera_transform.local_z();
    let forward = -Vec3::new(local_z.x, 0., local_z.z);
    let right = Vec3::new(local_z.z, 0., -local_z.x);

    match trigger.value.x as i32 {
        1 => {velocity += right}
        -1 => {velocity -= right}
        _ => {}
    }

    match trigger.value.y as i32 {
        1 => {velocity += forward}
        -1 => {velocity -= forward}
        _ => {}
    }
    velocity = velocity.normalize_or_zero();
    camera_transform.translation += velocity * main_camera.speed * time.delta_secs();
    // info!("{}", camera_transform.translation)
}


#[derive(InputAction)]
#[action_output(f32)]
struct MoveCameraY; 

fn move_camera_y(
    trigger: On<Fire<MoveCameraY>>,
    camera:  Single<(&mut Transform, &MainCamera)>,
    time:    Res<Time>
){
    let (mut camera_transform, main_camera) = camera.into_inner();
    let mut velocity = Vec3::ZERO;
    match trigger.value as i32 {
        1 => {velocity += Vec3::Y}
        -1 => {velocity -= Vec3::Y}
        _ => {}
    }
    velocity = velocity.normalize_or_zero();
    camera_transform.translation += velocity * main_camera.speed * time.delta_secs();
}
