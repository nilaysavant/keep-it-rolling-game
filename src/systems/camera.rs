use bevy::prelude::*;

use crate::{
    components::{Ground, MyCamera},
    resources::GroundsResource,
};

pub fn handle_camera_focus(
    mut query_cams: Query<(&mut Transform, &MyCamera)>,
    query_grounds: Query<&Transform, (With<Ground>, Without<MyCamera>)>,
    ground_res: Res<GroundsResource>,
) {
    let Some(current_ground) = ground_res.current_ground else { return; };
    let Ok(current_ground_transform) = query_grounds.get(current_ground) else { return; };
    let Ok((mut cam_transform, MyCamera { init_transform })) = query_cams.get_single_mut() else { return; };
    cam_transform.translation = current_ground_transform.translation + init_transform.translation;
    cam_transform.look_at(current_ground_transform.translation, Vec3::Y);
}
