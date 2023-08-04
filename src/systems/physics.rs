use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::components::{ActiveGround, BelongsToGround, Ground, GroundSensor, RollingBall};

#[allow(clippy::type_complexity)]
pub fn ball_touching_ground(
    mut commands: Commands,
    balls: Query<Entity, (With<RollingBall>, With<Collider>)>,
    ground_sensors: Query<(Entity, &BelongsToGround), (With<GroundSensor>, With<Collider>)>,
    rapier_context: Res<RapierContext>,
) {
    let Ok(ball_ent) = balls.get_single() else { return; };

    for (sensor_ent, BelongsToGround(ground_ent)) in ground_sensors.iter() {
        let Some(is_intersecting) = rapier_context.intersection_pair(ball_ent, sensor_ent ) else { continue; };
        // if intersecting set ground as active else inactive...
        if is_intersecting {
            commands.entity(*ground_ent).insert(ActiveGround);
        } else {
            commands.entity(*ground_ent).remove::<ActiveGround>();
        }
    }
}
