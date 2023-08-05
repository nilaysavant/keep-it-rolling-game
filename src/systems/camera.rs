use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    components::{Ground, MyCamera, RollingBall},
    resources::GroundsResource,
};

#[allow(clippy::type_complexity)]
pub fn move_camera_focus_with_grounds(
    mut query_cams: Query<(&mut Transform, &MyCamera)>,
    query_grounds: Query<&Transform, (With<Ground>, Without<MyCamera>)>,
    query_ball: Query<(&Velocity, &Transform), (With<RollingBall>, Without<MyCamera>)>,
    ground_res: Res<GroundsResource>,
    time: Res<Time>,
) {
    let Some(current_ground) = ground_res.current_ground else { return; };
    let Ok(current_ground_transform) = query_grounds.get(current_ground) else { return; };
    let Ok((mut cam_transform, MyCamera { init_transform })) = query_cams.get_single_mut() else { return; };
    let Ok((ball_vel, ball_transform)) = query_ball.get_single() else { return; };
    let cam_transform_lerp_fact = 2.;
    cam_transform.translation = cam_transform.translation.lerp(
        ball_transform.translation + init_transform.translation,
        time.delta_seconds() * cam_transform_lerp_fact,
    );
}
