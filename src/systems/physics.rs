use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::components::{ActiveGround, Ground, GroundSensor, RollingBall};

pub fn ball_touching_ground(
    mut commands: Commands,
    query_ball: Query<Entity, (With<RollingBall>, With<Collider>)>,
    query_ground: Query<Entity, (With<GroundSensor>, With<Collider>)>,
    rapier_context: Res<RapierContext>,
) {
    let Ok(ball_ent) = query_ball.get_single() else { return; };

    for ground_ent in query_ground.iter() {
        let Some(is_intersecting) = rapier_context.intersection_pair(ball_ent, ground_ent, ) else { continue; };
        // if intersecting set ground as active else inactive...
        if is_intersecting {
            commands.entity(ground_ent).insert(ActiveGround);
        } else {
            commands.entity(ground_ent).remove::<ActiveGround>();
        }
    }
}
