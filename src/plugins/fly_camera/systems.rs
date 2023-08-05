use bevy::core_pipeline::fxaa::Fxaa;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;

use bevy::window::CursorGrabMode;
use bevy::{input::mouse::MouseMotion, prelude::*};

#[derive(Debug, Resource)]
pub struct FlyCameraSettings {
    pub init_transform: Transform,
    pub init_walk_speed: f32,
    pub init_run_speed: f32,
}

impl Default for FlyCameraSettings {
    fn default() -> Self {
        let mut init_transform = Transform::from_translation(Vec3::ONE);
        init_transform.look_at(Vec3::ZERO, Vec3::Y);
        let controller = CameraController::default();
        Self {
            init_transform,
            init_walk_speed: controller.walk_speed,
            init_run_speed: controller.run_speed,
        }
    }
}

pub fn setup(mut commands: Commands, settings: Res<FlyCameraSettings>) {
    info!("Spawning a controllable 3D perspective camera...");
    let FlyCameraSettings {
        init_transform,
        init_run_speed,
        init_walk_speed,
        ..
    } = *settings;
    commands
        .spawn((
            Visibility::default(),
            ComputedVisibility::default(),
            Camera3dBundle {
                transform: init_transform,
                camera: Camera {
                    is_active: true,
                    // hdr compilation fails for non rgb8uorm textures on web/wasm and requires rgb16f
                    // hdr: true,
                    ..default()
                },
                tonemapping: Tonemapping::TonyMcMapface,
                ..default()
            },
            FogSettings {
                color: Color::rgba(0.1, 0.1, 0.1, 1.0),
                falloff: FogFalloff::Exponential { density: 0.0003 },
                ..default()
            },
        ))
        .insert(Fxaa::default())
        .insert(CameraController {
            walk_speed: init_walk_speed,
            run_speed: init_run_speed,
            ..default()
        })
        .insert(TrackableCamera);
    info!("Spawning a controllable 3D perspective camera... done!");
}

/// Marker component used to enable camera tracking and cycling.
#[derive(Component)]
pub struct TrackableCamera;

#[derive(Debug, Default, Resource)]
pub struct CameraTracker {
    active_index: Option<usize>,
    cameras: Vec<Entity>,
}

impl CameraTracker {
    /// Add camera to be tracked
    pub fn track_camera(&mut self, camera_entity: Entity) {
        self.cameras.push(camera_entity);
    }

    pub fn get_active_camera(&self) -> Option<Entity> {
        self.active_index.map(|i| self.cameras[i])
    }

    pub fn set_active_camera(&mut self, camera_entity: Entity) -> Option<usize> {
        let idx = self.cameras.iter().position(|c| c == &camera_entity);
        self.active_index = idx;
        idx
    }

    pub fn set_next_active(&mut self) -> Option<Entity> {
        let active_index = self.active_index?;
        let new_i = (active_index + 1) % self.cameras.len();
        self.active_index = Some(new_i);
        Some(self.cameras[new_i])
    }
}

pub fn camera_tracker(
    mut camera_tracker: ResMut<CameraTracker>,
    keyboard_input: Res<Input<KeyCode>>,
    mut queries: ParamSet<(
        Query<(Entity, &mut Camera), (Added<Camera>, With<TrackableCamera>)>,
        Query<(Entity, &mut Camera)>,
    )>,
) {
    // track all added scene camera entities
    let added_camera_query = queries.p0();
    for (entity, _) in added_camera_query.iter() {
        camera_tracker.track_camera(entity);
    }
    // Get all active cameras just added
    let added_active_cameras = added_camera_query
        .iter()
        .filter(|(_, cam)| cam.is_active)
        .collect::<Vec<(Entity, &Camera)>>();
    if !added_active_cameras.is_empty() {
        // set only the last active cam (from just added cams) to be tracked as active
        // and set all the other cams to inactive
        let active_cam_ent = added_active_cameras.last().unwrap().0;
        camera_tracker.set_active_camera(active_cam_ent);
        for (ent, mut cam) in queries.p1().iter_mut() {
            if ent != active_cam_ent {
                cam.is_active = false;
            } else {
                cam.is_active = true
            }
        }
    }
    if keyboard_input.just_pressed(KeyCode::C) {
        // disable currently active camera
        if let Some(e) = camera_tracker.get_active_camera() {
            info!("Switching active camera from {:?}", e);
            if let Ok((_, mut camera)) = queries.p1().get_mut(e) {
                camera.is_active = false;
            }
        }

        // enable next active camera
        if let Some(e) = camera_tracker.set_next_active() {
            if let Ok((_, mut camera)) = queries.p1().get_mut(e) {
                camera.is_active = true;
            }
        }
    }
}

#[derive(Component, Debug)]
pub struct CameraController {
    pub enabled: bool,
    pub initialized: bool,
    pub sensitivity: f32,
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_run: KeyCode,
    pub mouse_capture_menu_toggle_key: KeyCode,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub friction: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            initialized: false,
            sensitivity: 0.4,
            key_forward: KeyCode::W,
            key_back: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_up: KeyCode::E,
            key_down: KeyCode::Q,
            key_run: KeyCode::ShiftLeft,
            mouse_capture_menu_toggle_key: KeyCode::AltLeft,
            walk_speed: 20.0,
            run_speed: 120.0,
            friction: 0.5,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
        }
    }
}

pub fn camera_controller(
    time: Res<Time>,
    mut mouse_events: EventReader<MouseMotion>,
    mouse_button_input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    mut move_toggled: Local<bool>,
    mut query: Query<(&Camera, &mut Transform, &mut CameraController), With<Camera>>,
    mut windows: Query<&mut Window>,
) {
    let dt = time.delta_seconds();

    if let Ok((camera, mut transform, mut options)) = query.get_single_mut() {
        let mut window = windows.get_single_mut().unwrap();
        if !camera.is_active {
            // if not active do not apply transformations
            return;
        }
        if !options.initialized {
            let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
            options.yaw = yaw;
            options.pitch = pitch;
            options.initialized = true;
        }
        if !options.enabled {
            return;
        }
        // Handle key input
        let mut axis_input = Vec3::ZERO;
        if key_input.pressed(options.key_forward) {
            axis_input.z += 1.0;
        }
        if key_input.pressed(options.key_back) {
            axis_input.z -= 1.0;
        }
        if key_input.pressed(options.key_right) {
            axis_input.x += 1.0;
        }
        if key_input.pressed(options.key_left) {
            axis_input.x -= 1.0;
        }
        if key_input.pressed(options.key_up) {
            axis_input.y += 1.0;
        }
        if key_input.pressed(options.key_down) {
            axis_input.y -= 1.0;
        }
        if key_input.just_pressed(options.mouse_capture_menu_toggle_key) {
            *move_toggled = !*move_toggled;
        }
        // Apply movement update
        if axis_input != Vec3::ZERO {
            let max_speed = if key_input.pressed(options.key_run) {
                options.run_speed
            } else {
                options.walk_speed
            };
            options.velocity = axis_input.normalize() * max_speed;
        } else {
            let friction = options.friction.clamp(0.0, 1.0);
            options.velocity *= 1.0 - friction;
            if options.velocity.length_squared() < 1e-6 {
                options.velocity = Vec3::ZERO;
            }
        }
        let forward = transform.forward();
        let right = transform.right();
        transform.translation += options.velocity.x * dt * right
            + options.velocity.y * dt * Vec3::Y
            + options.velocity.z * dt * forward;
        // Handle mouse input
        let mut mouse_delta = Vec2::ZERO;
        if *move_toggled {
            // capture cursor
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
            for mouse_event in mouse_events.iter() {
                mouse_delta += mouse_event.delta;
            }
        } else {
            // un-capture cursor
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
        if mouse_delta != Vec2::ZERO {
            // Apply look update
            let (pitch, yaw) = (
                (options.pitch - mouse_delta.y * 0.5 * options.sensitivity * dt).clamp(
                    -0.99 * std::f32::consts::FRAC_PI_2,
                    0.99 * std::f32::consts::FRAC_PI_2,
                ),
                options.yaw - mouse_delta.x * options.sensitivity * dt,
            );
            transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, yaw, pitch);
            options.pitch = pitch;
            options.yaw = yaw;
        }
    }
}
