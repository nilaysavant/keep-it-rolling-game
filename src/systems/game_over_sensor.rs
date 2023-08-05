use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RapierContext};

use crate::{
    components::{GameOverSensor, Ground, RollingBall},
    events::SceneEvent,
    resources::GroundsResource,
};

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn move_game_over_sensors_with_current_ground(
    grounds: Query<&Transform, With<Ground>>,
    mut ground_game_over_sensor: Query<
        (&mut Transform, &GameOverSensor),
        (With<Collider>, Without<Ground>),
    >,
    ground_res: Res<GroundsResource>,
) {
    for (mut sensor_transform, GameOverSensor { init_transform }) in
        ground_game_over_sensor.iter_mut()
    {
        let Some(current_ground) = ground_res.current_ground else { continue; };
        let Ok(curr_ground_transform) = grounds.get(current_ground) else { continue; };
        let mut new_transform = *curr_ground_transform;
        new_transform.translation += init_transform.translation;
        *sensor_transform = new_transform;
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn handle_ground_game_over_sensor(
    balls: Query<Entity, (With<RollingBall>, With<Collider>)>,
    ground_game_over_sensor: Query<Entity, (With<GameOverSensor>, With<Collider>)>,
    mut game_event: EventWriter<SceneEvent>,
    rapier_context: Res<RapierContext>,
) {
    let Ok(ball_ent) = balls.get_single() else { return; };
    for sensor_ent in ground_game_over_sensor.iter() {
        let Some(is_intersecting) = rapier_context.intersection_pair(ball_ent, sensor_ent ) else { continue; };
        if is_intersecting {
            game_event.send(SceneEvent::Restart);
        }
    }
}
