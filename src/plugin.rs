use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use bevy_rapier3d::prelude::*;

use crate::{
    events::{SceneEvent, WallEvent},
    materials::glowy::GlowyMaterial,
    plugins::{FlyCameraPlugin, FpsDisplayPlugin},
    resources::{GroundsResource, PreviousScoresRes, ScoresResource, SettingsResource},
    state::GameState,
    systems::{
        camera::move_camera_focus_with_grounds,
        cleanup::cleanup,
        credits::display_credits,
        egui::init_egui_context,
        game_over_sensor::{
            handle_ground_game_over_sensor, move_game_over_sensors_with_current_ground,
        },
        ground::{
            color_grounds,
            handle_ground_sensor,
            handle_mid_ground_sensor,
            // mark_cleanup_prev_grounds,
        },
        lights::move_lighting_with_grounds,
        menu::auto_start_game_on_menu,
        scene::{handle_scene_events, move_to_in_game, scene_setup},
        scoring::{display_scoreboard, setup_scoring, update_grounds_passed, update_stopwatch},
        settings::display_settings,
        walls::{handle_wall_events, pick_ground_point_raycast},
        window::setup_window,
    },
};

pub struct KeepItRollingGamePlugin;

impl Plugin for KeepItRollingGamePlugin {
    fn build(&self, app: &mut App) {
        app //
            // background...
            .insert_resource(ClearColor(Color::BLACK))
            // window...
            .add_systems(Startup, setup_window)
            // physics...
            .insert_resource(RapierConfiguration {
                timestep_mode: TimestepMode::Interpolated {
                    dt: 1.0 / 60.0,
                    time_scale: 1.0,
                    substeps: 1,
                },
                physics_pipeline_active: false, // don't enable physics while starting
                ..default()
            })
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::default(),
                // RapierDebugRenderPlugin::default(),
            ))
            // fly cam
            // .add_plugins(FlyCameraPlugin)
            .add_plugins(FpsDisplayPlugin)
            // materials
            .add_plugins(MaterialPlugin::<GlowyMaterial>::default())
            // state and system sets...
            .add_state::<GameState>()
            .configure_set(
                Update,
                PluginSystemSet::InGame.run_if(in_state(GameState::InGame)),
            )
            // egui
            .add_plugins(EguiPlugin)
            .add_systems(Startup, init_egui_context)
            // settings...
            .insert_resource(SettingsResource::default())
            .add_systems(Update, display_settings)
            // menu...
            .add_systems(OnEnter(GameState::Menu), auto_start_game_on_menu)
            // scoring...
            .insert_resource(PreviousScoresRes::default())
            .insert_resource(ScoresResource::default())
            .add_systems(OnEnter(GameState::InGame), setup_scoring)
            .add_systems(
                Update,
                (update_stopwatch, update_grounds_passed, display_scoreboard)
                    .in_set(PluginSystemSet::InGame),
            )
            // scene...
            .add_event::<SceneEvent>()
            .add_systems(OnEnter(GameState::SceneLoading), scene_setup)
            .add_systems(
                Update,
                move_to_in_game.run_if(in_state(GameState::SceneLoading)),
            )
            .add_systems(
                Update,
                (handle_scene_events,).in_set(PluginSystemSet::InGame),
            )
            // ground...
            .insert_resource(GroundsResource::default())
            .add_systems(
                Update,
                (
                    handle_ground_sensor,
                    handle_mid_ground_sensor,
                    color_grounds,
                    // mark_cleanup_prev_grounds,
                )
                    .in_set(PluginSystemSet::InGame),
            )
            // walls...
            .add_event::<WallEvent>()
            .add_systems(
                Update,
                (pick_ground_point_raycast, handle_wall_events).in_set(PluginSystemSet::InGame),
            )
            // game over sensor...
            .add_systems(
                Update,
                (
                    handle_ground_game_over_sensor,
                    move_game_over_sensors_with_current_ground,
                )
                    .in_set(PluginSystemSet::InGame),
            )
            // camera
            .add_systems(
                Update,
                (move_camera_focus_with_grounds,).in_set(PluginSystemSet::InGame),
            )
            // lights
            .add_systems(
                Update,
                (move_lighting_with_grounds,).in_set(PluginSystemSet::InGame),
            )
            // credits...
            .add_systems(Update, (display_credits,))
            // cleanup
            .add_systems(First, cleanup)
            // debug...
            // .add_plugins(WorldInspectorPlugin::default())
            .register_type::<GroundsResource>()
            // .add_plugins(ResourceInspectorPlugin::<GroundsResource>::default())
            // other...
            .add_systems(Startup, || info!("Game Started..."));
    }
}

/// The Plugin's own system set.
///
/// Used to apply common run conditions based on state etc to
/// a common set of systems.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
enum PluginSystemSet {
    /// Run systems in this set when state is `InGame`.
    InGame,
}
